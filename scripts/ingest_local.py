#!/usr/bin/env python3
"""
Embed all *.md and *.json files under the HARALD folder
and store a FAISS index plus a parallel metadata file.

Run:  python scripts/ingest_local.py
"""

import pathlib, json, requests
import numpy as np
import faiss  # pip install faiss-cpu
from typing import Optional

ROOT = pathlib.Path(__file__).resolve().parents[1]   # HARALD/
DATA = ROOT / "data"
DATA.mkdir(exist_ok=True)

def embed(text: str) -> Optional[np.ndarray]:
    try:
        r = requests.post(
            "http://127.0.0.1:11434/api/embeddings",
            json={"model": "harald-phi4", "prompt": text, "stream": False},
            timeout=600,
        )
        r.raise_for_status()
        response_data = r.json()
        
        if "embedding" not in response_data:
            print(f"Warning: No embedding in response")
            return None
            
        return np.array(response_data["embedding"], dtype=np.float32)
    except requests.exceptions.RequestException as e:
        print(f"Error making embedding request: {e}")
        return None
    except (KeyError, ValueError, json.JSONDecodeError) as e:
        print(f"Error processing embedding response: {e}")
        return None

docs: list[np.ndarray] = []
meta: list[dict[str, object]] = []
file_count = 0

for path in ROOT.rglob("*"):
    if path.suffix.lower() in {".md", ".json"} and path.stat().st_size > 0:
        try:
            chunk = path.read_text(encoding="utf-8", errors="ignore")[:800]
            embedding = embed(chunk)
            
            if embedding is not None:
                docs.append(embedding)
                meta.append({"path": str(path), "bytes": path.stat().st_size})
                file_count += 1
                if file_count % 10 == 0:
                    print(f"Processed {file_count} files...")
            else:
                print(f"Skipping {path} due to embedding failure")
        except Exception as e:
            print(f"Error processing file {path}: {e}")

if docs:
    vecs: np.ndarray = np.stack(docs)
    index: faiss.IndexFlatL2 = faiss.IndexFlatL2(vecs.shape[1])
    index.add(vecs)
    faiss.write_index(index, str(DATA / "repo.index"))
    with open(DATA / "repo.meta.json", "w") as f:
        json.dump(meta, f)
    print(f"Ingested {len(meta)} files → {DATA}/repo.index")
else:
    print("No files ingested. Check your embedding service at http://127.0.0.1:11434")
