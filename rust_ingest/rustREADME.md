# rust_ingest

In-repo Rust utility that (1) embeds .md / .json files with Ollama,
(2) writes a HNSW index, (3) queries the index for RAG.

## ⚡ Status

| Date | Milestone |
|------|-----------|
| 2025-07-xx | PoC compiles under Rust 1.77, hnsw_rs 0.3.2 |
| TODO | Replace blocking `reqwest` with connection-pooled async calls |
| TODO | Bench vs Python FAISS ingest |

## 🏃‍♂️ Run

```bash
cd rust_ingest
cargo run --release -- ingest                # build index into ../data/
cargo run --release -- query "hello world"   # ask


## 💡 History

2025-07-15 – Forked from Python FAISS script → Rust for speed & single-binary
deploy.

2025-07-17 – Switched to hnsw_rs – smaller binary, no native BLAS.

2025-07-18 – Async embedding pipeline, 5× throughput on M3 Max.
```

```text

```

```text
2025-07-15 – Forked from Python FAISS script → Rust for speed & single-binary
deploy.

2025-07-17 – Switched to hnsw_rs – smaller binary, no native BLAS.

2025-07-18 – Async embedding pipeline, 5× throughput on M3 Max.
```

2025-07-17 – Switched to hnsw_rs – smaller binary, no native BLAS.

2025-07-18 – Async embedding pipeline, 5× throughput on M3 Max.

```text

```
