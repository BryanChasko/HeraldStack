# Copilot Instructions for HeraldStack

## Project Overview
- **HeraldStack** is a Rust-based, context-aware AI framework for personal and professional productivity, integrating memory, emotion, and modular execution across a cohort of AI entities.
- The system is migrating all application logic from shell scripts to Rust binaries. Shell scripts are only used for infrastructure orchestration (e.g., deployment, CI/CD, AWS CLI).

## Architecture & Key Components
- **src/**: Core Rust code for all application logic, including data processing, JSON/JSONL tools, embedding, and ingestion.
- **ai-entities/**: Definitions and metadata for AI personalities (see `entity-registry.json`, individual `.md` files).
- **config/**: Schemas, ethics, and model configuration.
- **docs/**: System design, migration, and vector search documentation.
- **scripts/**: Shell scripts for deployment and validation (do not add new app logic here).
- **rust_ingest/**: Rust CLI tools for ingestion and embedding (see `marvelai_ingest.rs`, `ingest.rs`).
- **data/**: Vector store registry and ingested data.

## Developer Workflows
- **Build Rust Binaries:**
  ```bash
  cd src && cargo build --release --features cli
  # Binaries: format_json, validate_json_schema, ingest_chunked, embedding_tool, text_chunker
  ```
- **Run Ingestion Pipeline:**
  - Use Rust binaries for all data ingestion and embedding.
  - Chunking and embedding logic is in `src/ingest/` and `rust_ingest/`.
  - Embeddings are saved as `.emb.json` files alongside chunked data.
- **Deploy:**
  ```bash
  ./scripts/deploy/deploy.sh [--build-only|prod|staging|--no-tests]
  ```
- **Validation & Linting:**
  ```bash
  ./scripts/validation/check-json.sh
  ./scripts/validation/check-rust.sh
  ./src/target/release/format_md
  ```

## Project-Specific Conventions
- **No new shell scripts for app logic.** All new features must be implemented in Rust.
- **Chunking for Embedding:**
  - All text for embedding must be chunked to ≤250 characters (see `chunking_utils.rs`).
  - Embeddings are generated per sub-chunk and saved as `<chunkfile>.<entity>.<field>.<start>-<end>.emb.json`.
- **JSONL as Standard:**
  - All ingestion and embedding pipelines use JSONL for input/output.
- **Error Handling:**
  - Use Rust's `anyhow` and `Result` for error propagation.
  - All ingestion failures must be logged with actionable diagnostics.
- **Documentation:**
  - Update `README.md` and relevant `.md` files for any new workflow or tool.
  - See `docs/vector-search/ollama-embedding-limits.md` for API constraints.

## Integration & External Dependencies
- **Ollama API** for embedding (see `docs/vector-search/ollama-embedding-limits.md`).
- **AWS Lambda, S3, DynamoDB, Pinecone** for compute, storage, state, and semantic memory.
- **No direct shell logic in Rust binaries.**

## Examples
- To add a new ingestion feature, extend `src/ingest/marvelai_ingest.rs` and document the CLI usage.
- To validate JSON, use `src/utils/json_tools/` Rust binaries, not shell scripts.
- To add a new entity, update `ai-entities/entity-registry.json` and add a corresponding `.md` file.

## References
- [README.md](../README.md)
- [docs/vector-search/character-based-chunking.md](../docs/vector-search/character-based-chunking.md)
- [docs/vector-search/ollama-embedding-limits.md](../docs/vector-search/ollama-embedding-limits.md)
- [docs/CONTRIBUTING.md](../docs/CONTRIBUTING.md)

---

## Copilot Chat Preferences

If you are unsure about a workflow or convention, check the documentation including the Root "HARALD" directory README.md and the appropriate .md file in the working directory, as well as general documentation in `docs/` paying special attention to the `migration/` directory for migration guidelines and the `CONTRIBUTING.md` for contribution standards.

- When suggesting code or pipeline changes, do not offer multiple alternatives or open‑ended questions. Clearly state the proposed fix and tell me you will apply it after I say “yes.” Only ask me to reply “yes” when confirmation is absolutely necessary before executing the change.