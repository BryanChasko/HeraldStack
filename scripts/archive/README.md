# Archived Scripts

This directory contains shell scripts that have been superseded by Rust binaries
as part of the HARALD project's migration from shell scripts to Rust for
application logic.

## Archive Purpose

These scripts are preserved for:

- **Historical reference** - Understanding previous implementation approaches
- **Migration context** - Comparing shell vs Rust implementations
- **Emergency backup** - Last resort if Rust implementations need debugging

## Archived Scripts

### Application Logic Scripts (Superseded by Rust binaries)

#### Ingestion and Embedding

- `ingest_chunked.sh.legacy` - Superseded by `src/target/release/ingest_chunked`
- `ingest_chunked.sh.new` - Superseded by `src/target/release/ingest_chunked`
- `ingest.sh` - Superseded by `src/target/release/harald_ingest`
- `ingest_marvelai.sh` - Superseded by `src/target/release/marvelai_ingest`
- `ingest_single_character.sh` - Superseded by embedding tools
- `embedding_tool.sh` - Superseded by `src/target/release/embedding_tool`
- `query.sh` - Superseded by Rust query tools

#### Text Processing

- `text_chunker.sh` - Superseded by `src/target/release/text_chunker`
- `text_chunker.sh.legacy` - Superseded by `src/target/release/text_chunker`
- `text_chunker_rust_wrapper.sh` - Wrapper no longer needed

#### JSON Tools

- `json-tools/format_json.sh` - Superseded by `src/target/release/format_json`
- `json-tools/format_json.sh.legacy` - Superseded by
  `src/target/release/format_json`
- `json-tools/validate_json_schema.sh.legacy` - Superseded by
  `src/target/release/validate_json_schema`

#### Testing Scripts

- `test_embedding_size.sh.legacy` - Superseded by Rust tests
- `test_embedding_size.sh` - Superseded by Rust tests
- `test_embedding_size_rust.sh` - Superseded by Rust tests
- `test_basic_embedding.sh` - Superseded by Rust tests
- `test_text_chunker.sh` - Superseded by Rust tests
- `test_migration.sh` - Migration testing no longer needed
- `verify_rust_migration.sh` - Migration verification complete

#### Validation Tools

- `validation/format-md.sh` - Superseded by `src/target/release/format_md`
- `validation/validate_naming.sh.new` - Superseded by
  `src/target/release/validate_naming`

#### Development Tools

- `dev-reference.sh` - Development reference script
- `dev/build-ingest.sh` - Build script superseded by cargo build

## Current Active Scripts

The following scripts remain active in the main `scripts/` directory:

### Infrastructure Scripts (Keep)

- `deploy/deploy.sh` - Deployment orchestration using AWS CLI
- `build_rust_tools.sh` - Rust binary build orchestration

### Essential Validation Scripts (Keep)

- `validation/check-rust.sh` - Must work even when Rust code fails
- `validation/check-json.sh` - JSON validation orchestration

## Usage Note

**These archived scripts should NOT be used in active development.** They are
preserved for historical context only. All application logic has been migrated
to type-safe, performant Rust implementations.

For current development, use the Rust binaries:

```bash
# Build all tools
cd src && cargo build --release --features cli

# Use tools with --help for complete documentation
./src/target/release/format_json --help
./src/target/release/text_chunker --help
./src/target/release/marvelai_ingest --help
```

---

_Archived: July 24, 2025_
