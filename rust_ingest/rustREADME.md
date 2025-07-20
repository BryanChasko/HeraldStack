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

2025-07-15 – Started by taking an existing Python script that used FAISS for
 vector search, and rewrote it in Rust. The goal was to make it faster and
 easier to deploy as a single, self-contained binary, without needing Python
 or extra dependencies.

2025-07-17 – Switched to hnsw_rs, a Rust library for fast vector search
 using Hierarchical Navigable Small World (HNSW) graphs. This change made
 the compiled program ("binary") smaller and removed the need for BLAS 
 (Basic Linear Algebra Subprograms) libraries, which are external 
 dependencies often used for mathematical operations in other 
 vector search tools.

2025-07-18 – Changed the embedding process to run asynchronously (so it 
doesn't wait for each file to finish before starting the next). This made
the process about five times faster when tested on a MacBook with an Intel 
processor.
```

```text

```
