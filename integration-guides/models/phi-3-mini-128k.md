# Phi-3 Mini 128K – Local LLM Best Practices

Complement to Microsoft GenAI – Secure, Scalable Architecture. Focuses on offline inference via Ollama / llama.cpp on Intel-based macOS.

## 1. Installation & Runtime

| Step | Command | Notes |
|------|---------|-------|
| Install Ollama | `brew install ollama` | Installs daemon & CLI system-wide |
| Start daemon | `brew services start ollama` | Flash Attention ≈ +15% tokens/s |
|  | `OLLAMA_FLASH_ATTENTION=1 ollama serve &` | Alternative startup with flash attention |
| Pull model | `ollama pull phi3:mini` | Downloads ≈ 2.3 GB GGUF (Q4_K_M quant) |
| REPL | `ollama run phi3:mini` | Starts interactive chat |
| REST | `POST http://127.0.0.1:11434/api/chat` | JSON schema identical to OpenAI, streaming by default |

### Runtime Flags

- `OLLAMA_KV_CACHE_TYPE=q8_0` → -30% KV-RAM
- `OLLAMA_CONTEXT_LENGTH=128000` (optional) sets full 128K when using llama.cpp directly
- `LLAMA_METAL=1` during llama.cpp build enables GPU assist on Intel Iris Plus

## 2. Prompt Engineering

```
<|system|>You are HARALD, Bryan's ambient intelligence…<|end|>
<|user|>…<|end|>
<|assistant|>
```

**Best Practices:**
- Keep system prompt explicit; pack persona traits here
- Place few-shot examples between `<|assistant|>` / `<|user|>` pairs
- Use chain-of-thought sparingly—Phi-3 obeys; suppress with "think step-by-step, but answer concisely"

## 3. Retrieval-Augmented Generation (RAG)

### Embed query locally:

```bash
curl -s http://127.0.0.1:11434/api/embeddings \
  -d '{"model":"phi3:mini","prompt":"<query>"}' | jq '.embedding'
```

### Process:
1. Search Pinecone / FAISS → top-k docs
2. Stuff docs (≲10K each) before user turn; maintain total ≤128K

```json
{
  "id": "2025-07-11-harald-thought",
  "metadata": {"entity_name":"harald","task_type":"rag_response"},
  "values": [0.018,-0.072,…]
}
```

## 4. Fine-Tuning & Adapters

| Method | Tooling | Quick cmd |
|--------|---------|-----------|
| LoRA | peft, trl | `python sft.py --model phi3 --r 8 --alpha 16` |
| QLoRA | BitsAndBytes | Reduce VRAM, slower training |

Store resulting adapters in `ai-entities/adapters/<entity>.safetensors` and load with ollama create:

```bash
ollama create harald-phi3 -m phi3:mini -f adapter.lora
```

## 5. Evaluation Loop

```bash
pipx run lm-eval-harness \
  --model huggingface --model_args "pretrained=phi3:mini,use_accelerate=True" \
  --tasks mmlu,gsm8k,humaneval --device cpu
```

**Schedule:**
- Run weekly
- Store metrics in `/docs/weekly-reviews/`
- Alert if AGIEval drops >5 pts

## 6. Security & Cost

- All compute local → $0 cloud spend
- Enforce Zero-Trust on Pinecone & Git remotes
- Disable daemon on battery: `brew services stop ollama`
- Prune unused blobs quarterly: `ollama prune`

## 7. Troubleshooting

| Symptom | Fix |
|---------|-----|
| "model manifest: file does not exist" | Use `ollama tags \| grep ^phi3` to locate valid tag |
| Segfault on load | Re-pull model; ensure LLAMA_METAL=1 compile flag matches CPU arch |
| Out-of-memory swap | Set OLLAMA_MAX_LOADED_MODELS=1; downgrade