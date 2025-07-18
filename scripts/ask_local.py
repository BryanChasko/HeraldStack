#!/usr/bin/env python3
import sys, json, pathlib, requests, numpy as np, faiss

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
index = faiss.read_index(str(DATA / "repo.index"))
meta  = json.load(open(DATA / "repo.meta.json"))

def embed(prompt: str) -> np.ndarray:
    r = requests.post("http://127.0.0.1:11434/api/embeddings",
                      json={"model":"harald-phi4","prompt":prompt,"stream":False},
                      timeout=600)
    return np.asarray(r.json()["embedding"], dtype=np.float32)

question = " ".join(sys.argv[1:]) or "List all entity names."
q_vec = embed(question)
D, I = index.search(q_vec.reshape(1, -1), k=3)

context = "\n\n".join(
    pathlib.Path(meta[i]["path"]).read_text(encoding="utf-8", errors="ignore")[:800]
    for i in I[0]
)

payload = {
    "model": "harald-phi4",
    "messages": [{"role":"user","content": f"{context}\n\n{question}"}],
    "stream": False
}
answer = requests.post("http://127.0.0.1:11434/api/chat", json=payload).json()
print(answer["message"]["content"])
