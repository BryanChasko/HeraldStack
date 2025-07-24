# JSON Tools Module

This module contains Rust-based tools for ensuring consistent formatting and
validation of JSON files used for vector embeddings in the HARALD project.

## Tools

- **format_json** (Rust binary): Formats JSON files according to project
  standards and ensures only registered files are processed
- **validate_json_schema** (Rust binary): Validates JSON files against
  predefined schemas for vector data stores

## Usage

```bash
# Build the tools first (from project root)
cd /path/to/HARALD/src
cargo build --release --features cli

# Format all registered JSON files
./target/release/format_json --all

# Register a new JSON file
./target/release/format_json --register ../path/to/file.json

# Check format without modifying
./target/release/format_json --check

# Validate the registry
./target/release/format_json --validate-registry

# Validate all files for a specific store
./target/release/validate_json_schema store marvel_characters

# Validate a specific file
./target/release/validate_json_schema file --file-path ./path/to/file.json

# Generate schema from existing files
./target/release/validate_json_schema generate marvel_characters
```

## File Organization

This directory follows the co-location principle where:

- Rust source code (`format_json.rs`, `validate_json_schema.rs`)
- Related documentation (`JSON-TOOLS.md`)
- Configuration files (if any)

Are all kept together for better maintainability.

## Migration Status

- ✅ `format_json.sh` → `format_json.rs` (Complete)
- ✅ `validate_json_schema.sh` → `validate_json_schema.rs` (Complete)

### Next Migration: validate_json_schema.sh

When migrating the validation script, the following should be co-located:

- Move `scripts/json-tools/validate_json_schema.sh` to `validate_json_schema.rs`
- Create `schemas/` subdirectory for schema files
- Update schema path from `./data/schemas` to `./src/utils/json_tools/schemas`
- Add binary entry to `Cargo.toml` as `validate_json_schema`

## See Also

- [Vector Store Registry Documentation](../../../docs/vector-search/vector-store-registry.md)
- [Migration Documentation](../../../docs/DEVELOPMENT-PRINCIPLES.md)
  (Historical details: [Archive](../../../docs/migration/archive/))

## Related Scripts

- [text_chunker.sh](../text_chunker.sh): Used for optimal text chunking before
  embedding
- [ingest_chunked.sh](../ingest_chunked.sh): Example ingestion script that uses
  chunking
