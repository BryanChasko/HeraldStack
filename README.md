# HeraldStack

[![Version](https://img.shields.io/badge/version-0.0.1-blue.svg)](https://semver.org)

> A context-aware, emotionally adaptive AI framework built exclusively for Bryan
> Chasko

## Core Vision

HeraldStack is an ambient intelligence system that integrates memory, emotion,
and modular execution across a trusted cohort of AI entities to restore
momentum, anchor decisions, and evolve alongside Bryan's ongoing personal and
professional journey.

## 🚨 Critical Development Principles

### No New Shell Scripts for Application Logic

**We are migrating FROM shell scripts TO Rust.** Do not create new shell scripts
for any application functionality. Instead:

- **Add features to existing Rust binaries**
- **Update documentation** (README.md, .md files)
- **Add --help flags** to existing tools

#### Exceptions: Scripts That Remain as Shell Scripts

The following should remain as shell scripts:

- **Deployment scripts** that orchestrate external tools (e.g., AWS CLI)
- **check-rust.sh** which must work even when Rust code has issues
- **CI/CD pipeline scripts** for infrastructure tasks

### Use Automated Cleanup Tools First

Before manually fixing linting/formatting issues, run our automated tools:

```bash
./scripts/validation/check-json.sh     # Fix JSON issues
./scripts/validation/check-rust.sh     # Fix Rust issues
./src/target/release/format_md      # Fix Markdown issues
```

**See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for complete development
guidelines.**

## Key Components

- **🦊 HARALD** – Default entity for emotional mirroring, decision anchoring,  
  and continuity management
- **🧠 Herald Entity Cohort** – Specialized assistants with distinct
  personalities and roles
- **🌐 Cloud Capable Infrastructure** – Modular architecture using AWS
- **📚 Narrative-Aware UX** – Long-memory interactions rooted in Bryan's story
  arcs
- **🔍 Vector Search** – Optimized embedding process with character-based
  chunking
  - [Vector Store Registry](docs/vector-search/vector-store-registry.md) for
    managing JSON data
  - [Character-Based Chunking](docs/vector-search/character-based-chunking.md)
    for optimal text processing
  - [Ollama API Limitations](docs/vector-search/ollama-embedding-limits.md) and
    workarounds

## Core Capabilities

- Persistent awareness of Bryan's preferences, goals, and activities
- Collaboration modes: Co-Pilot, Auto, and Recall
- Consent-based logging for conversations and insights
- Calendar intelligence for optimal scheduling
- JSONL-optimized vector ingestion for efficient embedding processing
- Thought organization with automatic categorization
- Weekly review for continuous system improvement

## Technical Stack

| Component       | Technology      |
| --------------- | --------------- |
| Compute         | AWS Lambda      |
| Storage         | Amazon S3       |
| State Tracking  | Amazon DynamoDB |
| Semantic Memory | Pinecone        |
| Core Logic      | Rust            |
| Infrastructure  | Shell Scripts   |

## Build & Deploy

### Building Rust Components

HeraldStack uses Rust for core application logic (data processing, JSON tools,
embedding utilities):

```bash
# Build all Rust binaries
cd src && cargo build --release --features cli

# Available binaries:
# - format_json          (JSON formatting and validation)
# - validate_json_schema  (Schema validation and generation)
# - ingest_chunked       (Character-based data ingestion)
# - embedding_tool       (Embedding generation and testing)
# - text_chunker         (Text processing utilities)
```

### Deployment

Deployment uses shell scripts for infrastructure orchestration:

```bash
# Quick build (useful for CI/CD)
./scripts/deploy/deploy.sh --build-only

# Deploy to development (default)
./scripts/deploy/deploy.sh

# Deploy to production
./scripts/deploy/deploy.sh prod

# Skip tests for faster deployment
./scripts/deploy/deploy.sh staging --no-tests
```

**Why Shell for Deployment?** Infrastructure scripts remain as shell because
they orchestrate external tools (Docker, AWS CLI) and need rapid iteration -
perfect for shell's ecosystem integration.

**Why Rust for Application Logic?** Data processing, JSON validation, and
embedding tools benefit from Rust's type safety, performance, and error
handling.

## Development Standards

- [Naming Conventions](docs/naming-conventions.md) - Standards for files and
  directories
- **Build & Deploy**: Use `./scripts/deploy/deploy.sh` for deployment (see
  [DEPLOY.md](scripts/deploy/DEPLOY.md) for usage)
- **JSON Tools**: Rust-based JSON processing utilities in `src/utils/json_tools`
  (see [JSON-TOOLS.md](src/utils/json_tools/JSON-TOOLS.md))
- **Shell vs Rust**: Infrastructure scripts use shell, application logic uses
  Rust
- [Project Structure](docs/migration/RECOMMENDED-STRUCTURE.md) - Recommended
  organization
- [Migration Documentation](docs/migration/) - Detailed migration information

## Operating Model

All interactions flow through HARALD, who routes context and emotion to
specialized entities as needed. The system provides emotional intelligence,
pragmatic execution, and narrative continuity.

## Documentation

- System Design Details
- Entity Descriptions
- Infrastructure
- Integrations
- Memory Architecture
- Personality Models
- Workflows
- [JSONL Format for Vector Embedding](docs/vector-search/jsonl-ingestion.md)
- [Migration Documentation](docs/migration/) - Shell-to-Rust migration details

## Ethics & Consent

HeraldStack operates on consent-based principles and follows clear ethical
guidelines including those defined in
[LawsOfRobotics.json](config/ethics/LawsOfRobotics.json).

## Development Tools

- **Rust Binaries**: Core application tools built with
  `cargo build --release --features cli`
- **Deployment**: Shell-based deployment script at `scripts/deploy/deploy.sh`
- **Models**: Model configurations can be found in `config/models/`
- **Test Data**: Test fixtures are available in `tests/fixtures/` (see
  [FIXTURES.md](tests/fixtures/FIXTURES.md) for details)

## Further Information

See docs/DETAILED.md for more information.

---

Shared under MIT Open License 2025 Bryan Chasko
