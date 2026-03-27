import os
import json
import time
import ssl
import http.client
import urllib.request
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import urlparse

OPENROUTER_API_KEY = (os.environ.get("OPENROUTER_API_KEY") or "").strip() or None
OLLAMA_HOST = os.environ.get("OLLAMA_HOST", "http://host.docker.internal:11434")

MODEL_MAP = {
    "minimax-free":  "minimax/minimax-m2.5:free",
    "minimax":       "minimax/minimax-m2.5",
    "qwen-reason":   "qwen/qwen3-235b-a22b-2507",
    "qwen-code":     "qwen/qwen3-coder-next",
    "qwen-deep":     "qwen/qwen3-235b-a22b-thinking-2507",
}

DEFAULT_MODEL = "minimax-free"

CODE_KEYWORDS = {"code", "implement", "debug", "refactor", "function", "class", "bug", "compile", "syntax", "repo", "commit", "diff", "patch", "test", "unittest"}
REASON_KEYWORDS = {"reason", "analyze", "plan", "prove", "math", "logic", "step-by-step", "compare", "evaluate", "architecture", "design", "strategy"}
DEEP_KEYWORDS = {"proof", "theorem", "formal", "derive", "long-form", "exhaustive", "comprehensive"}

REQUEST_LOG = []
MAX_REQUESTS_PER_MINUTE = 30


def log(msg):
    print(f"[Proxy {time.strftime('%H:%M:%S')}] {msg}", flush=True)


def classify_request(messages):
    last_user = ""
    for msg in reversed(messages):
        if msg.get("role") == "user":
            last_user = msg.get("content", "").lower()
            break
    if not last_user:
        return DEFAULT_MODEL
    words = set(last_user.split())
    if words & DEEP_KEYWORDS:
        return "qwen-deep"
    if words & CODE_KEYWORDS:
        return "qwen-code"
    if words & REASON_KEYWORDS:
        return "qwen-reason"
    if len(last_user) > 2000:
        return "minimax"
    return DEFAULT_MODEL


def check_rate_limit():
    now = time.time()
    REQUEST_LOG[:] = [t for t in REQUEST_LOG if now - t < 60]
    if len(REQUEST_LOG) >= MAX_REQUESTS_PER_MINUTE:
        return False
    REQUEST_LOG.append(now)
    return True


def strip_streaming(data):
    """Goose sends stream:true but this proxy buffers full responses.
    Must strip for both providers or resp.read() hangs on chunked SSE."""
    data.pop("stream", None)
    data.pop("stream_options", None)


MODELS_RESPONSE = json.dumps({
    "object": "list",
    "data": [
        {"id": k, "object": "model", "owned_by": "heraldstack-proxy"}
        for k in MODEL_MAP
    ]
}).encode()


class ProxyHandler(BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        pass

    def do_GET(self):
        if self.path == "/health":
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b"OK")
        elif self.path == "/v1/models":
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            self.wfile.write(MODELS_RESPONSE)
        else:
            self.send_response(404)
            self.end_headers()

    def do_POST(self):
        if self.path != "/v1/chat/completions":
            self.send_response(404)
            self.end_headers()
            return

        if not check_rate_limit():
            self._error(429, "Rate limit: max 30 req/min")
            return

        content_length = int(self.headers.get("Content-Length", 0))
        data = json.loads(self.rfile.read(content_length))
        model = data.get("model", "")

        if model.startswith("llama") or model.startswith("ollama"):
            return self.handle_ollama(data)

        resolved = model if model in MODEL_MAP else classify_request(data.get("messages", []))
        openrouter_model = MODEL_MAP[resolved]
        log(f"route model={model} -> {resolved} -> {openrouter_model}")
        return self.handle_openrouter(openrouter_model, data)

    def handle_openrouter(self, mapped_model, data):
        """Uses http.client instead of urllib — urllib strips Authorization
        headers on redirects, which breaks auth behind Cloudflare."""
        if not OPENROUTER_API_KEY:
            self._error(500, "OPENROUTER_API_KEY not set")
            return

        data["model"] = mapped_model
        strip_streaming(data)
        payload = json.dumps(data).encode()

        key_prefix = OPENROUTER_API_KEY[:8]
        log(f"openrouter key={key_prefix}... model={mapped_model} bytes={len(payload)}")

        ctx = ssl.create_default_context()
        conn = http.client.HTTPSConnection("openrouter.ai", 443, timeout=120, context=ctx)
        t0 = time.time()
        try:
            conn.request("POST", "/api/v1/chat/completions", body=payload, headers={
                "Authorization": f"Bearer {OPENROUTER_API_KEY}",
                "Content-Type": "application/json",
                "HTTP-Referer": "https://github.com/bryanchasko/heraldstack",
                "X-Title": "HeraldStack Goose Proxy",
            })
            resp = conn.getresponse()
            body = resp.read()
            elapsed = time.time() - t0
            log(f"openrouter status={resp.status} elapsed={elapsed:.1f}s bytes={len(body)}")

            self.send_response(resp.status)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            self.wfile.write(body)
        except Exception as e:
            elapsed = time.time() - t0
            log(f"openrouter FAIL elapsed={elapsed:.1f}s error={e}")
            self._error(502, str(e), "openrouter")
        finally:
            conn.close()

    def handle_ollama(self, data):
        """Ollama: no auth, no Cloudflare — urllib is fine here.
        Only change from original: strip streaming (same buffering issue)."""
        strip_streaming(data)
        post_data = json.dumps(data).encode()
        req = urllib.request.Request(
            f"{OLLAMA_HOST}/v1/chat/completions",
            data=post_data,
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        t0 = time.time()
        try:
            with urllib.request.urlopen(req, timeout=120) as resp:
                body = resp.read()
                elapsed = time.time() - t0
                log(f"ollama status={resp.getcode()} elapsed={elapsed:.1f}s")
                self.send_response(resp.getcode())
                self.send_header("Content-Type", "application/json")
                self.end_headers()
                self.wfile.write(body)
        except Exception as e:
            err = str(e)
            if hasattr(e, "read"):
                try: err = e.read().decode()
                except: pass
            elapsed = time.time() - t0
            log(f"ollama FAIL elapsed={elapsed:.1f}s error={err}")
            self._error(502, err, "ollama")

    def _error(self, code, msg, provider="proxy"):
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps({
            "error": {"message": msg, "type": f"{provider}_error"}
        }).encode())


if __name__ == "__main__":
    key_status = f"{OPENROUTER_API_KEY[:8]}..." if OPENROUTER_API_KEY else "MISSING"
    log(f"OpenRouter key: {key_status}")
    log(f"Ollama host: {OLLAMA_HOST}")
    log(f"Default model: {DEFAULT_MODEL} -> {MODEL_MAP[DEFAULT_MODEL]}")
    log(f"Models: {', '.join(MODEL_MAP.keys())}")
    HTTPServer(('', 4000), ProxyHandler).serve_forever()
