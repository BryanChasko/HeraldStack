# HeraldStack

[![Version](https://img.shields.io/badge/version-0.0.1-blue.svg)](https://semver.org)

> A context-aware, emotionally adaptive AI framework built exclusively for Bryan
> Chasko

## Core Vision

| HeraldStack is an ambient intelligence system that in | Component  | Technology       |
| ----------------------------------------------------- | ---------- | ---------------- |
| Compute                                               | AWS Lambda |
| Data Structure                                        | JSONL      |
| Storage                                               | Amazon S3  | memory, emotion, |

and modular execution across a trusted cohort of AI entities to restore
momentum, anchor decisions, and evolve alongside Bryan's ongoing personal and
professional journey.

## 🚨 Critical Development Principles

### No New Shell Scripts for Application Logic

**We are have bias to write our functionality in Rust.** Do not create new shell
scripts for any application functionality. Instead:

- **Add features to existing Rust binaries**
- **Update documentation** (README.md, .md files)
- **Add comprehensive --help flags** to existing tools for self-documenting
  usage

#### Exceptions: Scripts That Remain as Shell Scripts

The following should remain as shell scripts:

- **Deployment scripts** that orchestrate external tools (e.g., AWS CLI)
- **check-rust.sh** which must work even when Rust code has issues
- **CI/CD pipeline scripts** for infrastructure tasks

### Use Automated Cleanup Tools First

Before manually fixing linting/formatting issues, run our automated tools:

```bash
# Fix JSON formatting and validation issues
./target/release/check_json --fix

# Fix Rust formatting, run clippy, and tests
./scripts/validation/check-rust.sh

# Fix Markdown formatting (line length, spacing, etc.)
./target/release/format_md

# Check and optionally fix naming convention problems
./target/release/validate_naming --fix --verbose
```

**See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for complete development
guidelines.**

## Directory Structure Overview

HARALD follows a standard project structure designed for maintainability and
clear separation of concerns:

### Recommended Project Layout

```text
HARALD/
├── src/                  # Main source code directory
│   ├── api/              # API endpoints and handlers
│   ├── core/             # Core application logic
│   │   ├── embedding/    # Embedding-related logic
│   │   ├── entities/     # Entity management logic
│   │   └── memory/       # Memory handling logic
│   ├── ingest/           # Ingestion pipeline and tools
│   └── utils/            # Shared utilities and helpers
│       ├── json-tools/   # JSON formatting and validation
│       ├── validation/   # Code validation utilities
│       └── system/       # System utilities
├── ai-entities/          # AI entity definitions and metadata
├── config/               # Configuration files and schemas
│   ├── schemas/          # JSON schemas
│   ├── ethics/           # Ethics guidelines
│   └── models/           # Model configurations
├── data/                 # Data files and registries
│   ├── raw/              # Raw input data
│   └── processed/        # Processed data and embeddings
├── docs/                 # Documentation
│   ├── migration/        # Migration documentation and archive
│   └── vector-search/    # Vector search documentation
├── scripts/              # Infrastructure and deployment scripts
│   ├── deploy/           # Deployment scripts
│   └── validation/       # Validation scripts (shell-based)
└── tests/                # Tests and fixtures
    ├── unit/             # Unit tests
    ├── integration/      # Integration tests
    └── fixtures/         # Test fixtures and sample data
```

### Key Principles

- **`src/`** - Central location for all application code written in Rust
  - Sub-modules organized by functionality (api, core, ingest, utils)
  - All Rust binaries built from this directory tree
- **`config/`** - Centralized configuration management
  - JSON schemas, model configs, ethics guidelines
- **`data/`** - Raw and processed data with clear separation
  - Vector store registries and embedded data
- **`scripts/`** - Infrastructure scripts only (deployment, CI/CD)
  - Application logic has been migrated to Rust in `src/`
- **`tests/`** - Comprehensive testing structure
  - Unit, integration, and fixture organization

### Migration Status

This structure represents the target state. Current migration progress:

- ✅ Core Rust tools implemented in `src/utils/`
- ✅ Ingestion pipeline moved to `src/ingest/`
- ✅ Shell scripts migrated to Rust binaries
- 🔄 Ongoing migration of remaining application logic to `src/`

For detailed documentation:

- [docs/DETAILED.md](docs/DETAILED.md) – Complete directory descriptions
- [docs/naming-conventions.md](docs/naming-conventions.md) – Naming standards
- [docs/DEVELOPMENT-PRINCIPLES.md](docs/DEVELOPMENT-PRINCIPLES.md) – Development
  principles and migration history

## Archive Policy

### Historical Documents and Legacy Code

HARALD maintains archived materials for historical reference and context. These
archives are excluded from active development workflows:

#### Archive Locations

- **`docs/migration/archive/`** - Historical migration documentation
  - Shell script prevention strategies
  - Detailed migration plans and checklists
  - Step-by-step cleanup procedures
  - Legacy decision documentation

- **`scripts/*.legacy`** - Archived shell scripts (when present)
  - Backup copies of migrated scripts
  - Reference implementations for comparison
  - Historical functionality documentation

- **Early experiments and prototypes** (project-specific locations)
  - Proof-of-concept implementations
  - Alternative approaches that were not adopted
  - Research and exploration code

#### Archive Characteristics

- **Ignored by automation** - Archive directories are excluded from:
  - Linting and formatting tools
  - Build processes and validation
  - Automated testing suites
  - Code quality checks

- **Historical reference only** - Archived materials:
  - Preserve context for past decisions
  - Document migration rationale and process
  - Provide examples of previous approaches
  - Should not be modified or actively maintained

- **Documentation over deletion** - We prefer archiving to deletion because:
  - Historical context aids future decision-making
  - Migration patterns can be reused
  - Past approaches inform current best practices
  - Preserves institutional knowledge

#### When to Archive

Archive materials when:

1. **Migrating functionality** from shell scripts to Rust implementations
2. **Consolidating documentation** to eliminate duplication
3. **Refactoring approaches** that replace previous patterns
4. **Completing experiments** that informed current architecture

#### Accessing Archives

- Use archived materials for **historical context only**
- Reference archives when **documenting decisions**
- Consult archives to **understand migration patterns**
- **Do not** use archived code in active development

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

## HARALD Model Demonstration

HeraldStack includes a custom-trained Ollama model (`harald-phi4`) that has been
fine-tuned with project-specific knowledge and Bryan's personal context. Here's
an example interaction showing successful knowledge retrieval:

### Test Ollama Custom Model Query Example

```bash
ollama run harald-phi4 "Hello HARALD, please introduce yourself briefly."
```

### Response

```json
{
  "response": "I am HARALD—Bryan Chasko's default ambient-intelligence entity
  within HeraldStack designed to assist with pragmatic tasks and information
  retrieval."
}
```

### Additional Knowledge Verification

```bash
ollama run harald-phi4 "What Marvel AIs are you aware of?"
```

The model demonstrates comprehensive knowledge of Marvel AI characters,
referencing Vision, FRIDAY, EDITH, and other AI entities from the Marvel
universe, showing successful integration of the training data.

This demonstrates that:

- ✅ **Custom model training successful** - HARALD understands its role and
  context
- ✅ **Project knowledge integration** - Model recalls HeraldStack-specific
  terminology
- ✅ **Domain expertise** - Successfully retrieves Marvel AI information from
  training data
- ✅ **Structured responses** - Returns JSON format suitable for programmatic
  use
- ✅ **Personal context awareness** - Recognizes Bryan as the primary user

The model serves as the foundation for all AI interactions within the
HeraldStack ecosystem, providing contextually-aware responses while maintaining
the established personality framework.## Core Capabilities

- Persistent awareness of Bryan's preferences, goals, and activities
- Collaboration modes: Co-Pilot, Auto, and Recall
- Consent-based logging for conversations and insights
- Calendar intelligence for optimal scheduling
- JSONL-optimized vector ingestion for efficient embedding processing
- Thought organization with automatic categorization
- Weekly review for continuous system improvement

## Technical Stack

| Component              | Technology      |
| ---------------------- | --------------- |
| Compute                | AWS Lambda      |
| Data Structure } JSONL |
| Storage                | Amazon S3       |
| State Tracking         | Amazon DynamoDB |
| Semantic Memory        | Pinecone        |
| Core Logic             | Rust            |
| Deployment             | Shell Scripts   |

## Build & Deploy

### Building Rust Components

HeraldStack uses Rust for core application logic (data processing, JSON tools,
embedding utilities):

```bash
# Build all Rust binaries
cargo build --release --features cli

# Available binaries in target/release/:
# - check_json           (JSON formatting and validation wrapper)
# - embedding_tool       (Embedding generation and testing)
# - format_json          (JSON formatting and validation)
# - format_md            (Markdown formatting)
# - harald_ingest        (General semantic search ingestion and query tool)
# - marvelai_ingest      (Marvel-specific ingestion)
# - status               (System status checking)
# - text_chunker         (Text processing utilities)
# - validate_json_schema (Schema validation and generation)
# - validate_naming      (Naming convention validation)
```

### Using Rust Binaries

All binaries are located in `target/release/` and should be run from the project
root. **Each tool includes comprehensive `--help` documentation**:

```bash
# Get detailed usage for any tool
./target/release/format_json --help
./target/release/validate_naming --help
./target/release/text_chunker --help
```

#### Common Usage Examples

```bash
# Format and validate JSON files
./target/release/check_json --fix

# Format Markdown files
./target/release/format_md path/to/file.md

# Validate naming conventions
./target/release/validate_naming --fix --verbose

# Check system status (Ollama services, models, etc.)
./target/release/status

# Process text for embedding with detailed options
./target/release/text_chunker --char 250 --file input.txt --json
```

**Self-Documenting Design**: Instead of maintaining separate documentation, each
binary provides complete usage instructions via `--help`. This ensures usage
information stays current with the code and reduces documentation maintenance
overhead.

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
- **JSON Tools**: Rust-based JSON processing utilities in `src/utils/json-tools`
  (see [JSON-TOOLS.md](src/utils/json-tools/JSON-TOOLS.md))
- **Shell vs Rust**: Infrastructure scripts use shell, application logic uses
  Rust
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

---

Shared under MIT Open License 2025 Bryan Chasko
