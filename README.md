# HeraldStack

[![Version](https://img.shields.io/badge/version-0.0.1-blue.svg)](https://semver.org)

> A context-aware, emotionally adaptive AI framework built exclusively for Bryan
> Chasko

## Core Vision

HeraldStack is an am## Directory Structure Overview

For a detailed, canonical description of the project's directory structure, see:

- [docs/DETAILED.md](docs/DETAILED.md) – **Directory Structure and Naming Best
  Practices** (includes a `tree` overview and rationale)
- [docs/naming-conventions.md](docs/naming-conventions.md) – **Directory and
  file naming conventions**
- [docs/DEVELOPMENT-PRINCIPLES.md](docs/DEVELOPMENT-PRINCIPLES.md) –
  **Development principles and migration history**

Historical migration documents have been moved to
[docs/migration/archive/](docs/migration/archive/) for reference.gence system
that integrates memory, emotion, and modular execution across a trusted cohort
of AI entities to restore momentum, anchor decisions, and evolve alongside
Bryan's ongoing personal and professional journey.

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
# Fix JSON formatting and validation issues
./src/target/release/check_json --fix

# Fix Rust formatting, run clippy, and tests
./scripts/validation/check-rust.sh

# Fix Markdown formatting (line length, spacing, etc.)
./src/target/release/format_md

# Check and optionally fix naming convention problems
./src/target/release/validate_naming --fix --verbose
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

# Available binaries in src/target/release/:
# - check_json           (JSON formatting and validation wrapper)
# - format_json          (JSON formatting and validation)
# - validate_json_schema (Schema validation and generation)
# - ingest_chunked       (Character-based data ingestion)
# - embedding_tool       (Embedding generation and testing)
# - text_chunker         (Text processing utilities)
# - format_md            (Markdown formatting)
# - validate_naming      (Naming convention validation)
# - status               (System status checking)
# - harald_ingest        (Main ingestion tool)
# - marvelai_ingest      (Marvel-specific ingestion)
```

### Using Rust Binaries

All binaries are located in `src/target/release/` and should be run from the
project root:

```bash
# Format and validate JSON files
./src/target/release/check_json --fix

# Format Markdown files
./src/target/release/format_md path/to/file.md

# Validate naming conventions
./src/target/release/validate_naming --fix --verbose

# Check system status (Ollama services, models, etc.)
./src/target/release/status

# Process text for embedding
./src/target/release/text_chunker --input file.txt --mode char --size 250

# Run any tool with --help to see available options
./src/target/release/format_json --help
```

**Note**: These Rust binaries have replaced the previous shell scripts for
application logic. The old shell scripts in `scripts/validation/` have been
migrated to these type-safe, performant Rust implementations.

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

- [Development Principles](docs/DEVELOPMENT-PRINCIPLES.md) - Core development
  principles and migration guidelines
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
- **Ingestion/Embedding Architecture:** All ingestion and embedding logic must
  follow the
  [Modular Ingest Refactor Plan](docs/migration/INGEST-MIGRATION-MODULAR-PLAN.md).
  This plan defines the canonical, reusable ingest library and the pattern for
  domain-specific wrappers (e.g., marvelai_ingest.rs). All new pipelines and
  refactors must use this architecture and update documentation accordingly.

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
- [Modular Ingest Refactor Plan](docs/migration/INGEST-MIGRATION-MODULAR-PLAN.md)
  – Step-by-step plan for refactoring to a reusable, component-based ingestion
  architecture

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

## Directory Structure Overview

For a detailed, canonical description of the project’s directory structure, see:

- [docs/DETAILED.md](docs/DETAILED.md) – **Directory Structure and Naming Best
  Practices** (includes a `tree` overview and rationale)
- [docs/naming-conventions.md](docs/naming-conventions.md) – **Directory and
  file naming conventions**

Other structure-related documents in `docs/migration/` (such as
`RECOMMENDED-STRUCTURE.md`, `DIRECTORY-REORGANIZATION.md`, and
`IMPLEMENTATION-PLAN.md`) are project planning artifacts and will be moved to a
`docs/project-planning/` subdirectory.

---

Shared under MIT Open License 2025 Bryan Chasko
