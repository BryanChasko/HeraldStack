# HeraldStack

[![Version](https://img.shields.io/badge/version-0.0.1-blue.svg)](https://semver.org)

> A context-aware, emotionally adaptive AI framework built exclusively for Bryan
> Chasko

## Core Vision

HeraldStack is an ambient intelligence system that integrates memory, emotion,
and modular execution across a trusted cohort of AI entities to restore
momentum, anchor decisions, and evolve alongside Bryan's ongoing personal and
professional journey.

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

## Development Standards

- [Naming Conventions](docs/naming-conventions.md) - Standards for files and
  directories
- Automated validation scripts in `scripts/validation`
  (see [VALIDATION.md](scripts/validation/VALIDATION.md))
- JSON tooling and vector store registry in `scripts/json-tools`
  (see [JSON-TOOLS.md](scripts/json-tools/JSON-TOOLS.md))
- [Project Structure](docs/migration/RECOMMENDED-STRUCTURE.md) - Recommended organization

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
- [Directory Structure](docs/migration/RECOMMENDED-STRUCTURE.md) - Organization
  standards
- [Implementation Plan](docs/migration/IMPLEMENTATION-PLAN.md) - Migration
  strategy
- [Ingest Migration](docs/migration/INGEST-MIGRATION.md) - Rust code migration
  notes
- [Directory Reorganization](docs/migration/DIRECTORY-REORGANIZATION.md) - File
  reorganization details
- [Migration Documentation](docs/migration/MIGRATION.md) - Migration overview

## Ethics & Consent

HeraldStack operates on consent-based principles and follows clear ethical
guidelines including those defined in
[LawsOfRobotics.json](config/ethics/LawsOfRobotics.json).

## Development Tools

- **Code Quality & Validation**: Scripts for checking and formatting code are
  available in `scripts/validation/`. See
  [VALIDATION.md](scripts/validation/VALIDATION.md) for usage details.
- **Models**: Model configurations can be found in `config/models/`.
- **Test Data**: Test fixtures are available in `tests/fixtures/`. See
  [FIXTURES.md](tests/fixtures/FIXTURES.md) for details.

## Further Information

See docs/DETAILED.md for more information.

---

Shared under MIT Open License 2025 Bryan Chasko
