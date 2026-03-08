# Copilot Instructions – HAROLD / HeraldStack

## Project Overview
**HAROLD** is the primary AI entity in **HeraldStack** — a Rust-based, context-aware, emotionally adaptive AI framework. HeraldStack integrates a cohort of 8 AI personalities, persistent memory, emotional mirroring, and semantic vector search.

All application logic lives in Rust. Shell scripts handle infrastructure orchestration only (deployment, CI/CD, AWS CLI). This migration is complete — do not add new shell app logic.

---

## Architecture

```
HAROLD/
├── src/              # All Rust application code
│   ├── ingest/       # Core ingestion pipeline (embed, chunk, query, domain wrappers)
│   ├── core/         # Embedding (Ollama API), entities, memory (scaffolded)
│   ├── utils/        # JSON tools, validation, markdown formatter, system status
│   ├── bin/          # Standalone binary entry points
│   └── api/          # API handlers
├── ai-entities/      # Entity definitions (entity-registry.json + .md per entity)
├── config/           # Schemas, ethics (LawsOfRobotics.json), Ollama Modelfile
├── data/             # Raw, processed, and embedding data
├── docs/             # Design docs, vector-search guides, migration history, changelogs
├── scripts/          # Shell scripts for deploy/validation only
├── tests/            # Unit, integration, fixtures
└── workflows/        # Workflow definitions
```

**Key design pattern:** Domain-specific ingest wrappers (`marvelai_ingest.rs`, `single_character_ingest.rs`) delegate to the canonical modular ingest library in `src/ingest/`. See `docs/migration/INGEST-MIGRATION-MODULAR-PLAN.md`.

---

## Entity Cohort (`ai-entities/`)

8 AI personalities defined in `entity-registry.json`, each with a `.md` spec:

| Entity | Role |
|---|---|
| **HAROLD** | Default — emotional mirror, decision anchor |
| **Stratia** | Strategic planning |
| **Myrren** | Vision and foresight |
| **Liora** | Creative expression |
| **Kade-Vox** | High-energy execution |
| **Solan** | Science and research |
| **Ellow** | Emotional intelligence |
| **Orin** | Technical problem-solving |

To add an entity: update `ai-entities/entity-registry.json` and add a corresponding `.md` file.

---

## Build, Test & Lint

Run all commands from the **repository root** (not `src/`):

```bash
# Build all Rust binaries
cargo build --release --features cli

# Run all tests
cargo test

# Run a single test
cargo test <test_name>

# Lint
cargo fmt --check
cargo clippy -- -D warnings

# Security audit
cargo audit

# Deploy
./scripts/deploy/deploy.sh [dev|staging|prod] [--build-only|--no-tests]
```

---

## Rust Binaries (all in `target/release/`)

| Binary | Purpose |
|---|---|
| `harald_ingest` | Main semantic ingest & query CLI |
| `marvelai_ingest` | Marvel-specific ingestion wrapper |
| `single_character_ingest` | Character-based ingestion wrapper |
| `ingest_chunked` | Chunked ingestion pipeline |
| `text_chunker` | Text chunking for embeddings |
| `embedding_tool` | Embedding generation & testing |
| `format_json` | JSON formatter |
| `validate_json_schema` | JSON schema validator |
| `check_json` | JSON validation with `--fix` |
| `validate_naming` | Naming convention checker with `--fix` |
| `format_md` | Markdown formatter |
| `status` | System status check |

All binaries are self-documenting via `--help`.

**Validation shortcuts** (run before committing):
```bash
./target/release/check_json --fix
./target/release/validate_naming --fix --verbose
./target/release/format_md <path/to/file.md>
./scripts/validation/check-json.sh
./scripts/validation/check-rust.sh
```

---

## Key Conventions

- **No new shell scripts for app logic.** All features → Rust.
- **Chunking:** All embedding text must be ≤250 characters (see `src/ingest/chunking_utils.rs` and `docs/vector-search/character-based-chunking.md`).
- **Embeddings** saved as `<chunkfile>.<entity>.<field>.<start>-<end>.emb.json`.
- **JSONL** is the standard interchange format for all ingestion/embedding pipelines.
- **Error handling:** `anyhow::Result` throughout; all ingestion failures must log actionable diagnostics.
- **Directory names:** `kebab-case`. Rust files: `snake_case`. Entity docs: `TitleCase`.
- **Markdown:** 80-char line width, `prose-wrap: always` (enforced via Prettier in CI).
- **Docs first:** Update `README.md` and relevant `.md` files before writing new code or tools.

---

## External Dependencies

- **Ollama API** — local embedding generation (see `docs/vector-search/ollama-embedding-limits.md`)
- **AWS Lambda, S3, DynamoDB** — compute, storage, state
- **Pinecone** — semantic memory (vector search)

---

## CI Pipeline (`.github/workflows/ci.yml`)

Runs on push/PR to `main`:
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo audit`
- JSON lint via `jsonlint` (all `.json` files)
- Markdown check via Prettier (`--print-width 80 --prose-wrap always`)

---

## Copilot Behavior

- Check `README.md`, `docs/CONTRIBUTING.md`, `docs/DEVELOPMENT-PRINCIPLES.md`, and `docs/migration/` before proposing changes.
- State the fix clearly and apply it. Only ask for "yes" confirmation when strictly necessary.
- Do not offer multiple alternatives — recommend one approach.
