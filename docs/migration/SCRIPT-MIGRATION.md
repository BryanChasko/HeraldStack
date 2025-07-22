# Script Migration to Rust Plan

This document outlines our strategy for migrating shell scripts to Rust
implementations.

## Core Migration Principles

1. **No new shell scripts** - We're migrating TO Rust, not creating
   more shell scripts
2. **Direct Rust binary usage** - Users call `./src/target/release/binary_name`
   directly
3. **Rust-based testing** - Use `cargo test`, `cargo run`, and direct binary
   testing instead of shell scripts
4. **Clean removal process** - Remove shell scripts entirely after successful
   migration, not create wrappers
5. **Co-location principle** - Related documentation and configuration files
   should migrate to live next to the new Rust code

## Successfully Migrated Scripts

The following shell scripts have been successfully migrated to Rust:

- ✅ `validate_naming.sh` → Now `src/target/release/validate_naming`
- ✅ `format-md.sh` → Now `src/target/release/format_md`
- ✅ `format_json.sh` → Now `src/target/release/format_json`
- ✅ `status.sh` → Now `src/target/release/status`
- ✅ `text_chunker.sh` → Now `src/target/release/text_chunker`
- ✅ `ingest_chunked.sh` → Now `src/target/release/ingest_chunked`
- ✅ `validate_json_schema.sh` → Now `src/target/release/validate_json_schema`
- ✅ `check-json.sh` → Now `src/target/release/check_json`

## Scripts That Should Remain as Shell Scripts

Some scripts should intentionally remain as shell scripts due to their nature:

- **`check-rust.sh`** - Must work even when Rust code has issues
- **Deployment scripts** - Scripts that orchestrate external tools like AWS CLI
- **CI/CD pipeline scripts** - Build and deployment orchestration

## Pending Migrations

The following scripts still need to be migrated:

1. `ingest_marvelai.sh` → Integrate into unified ingest tool
2. `test_basic_embedding.sh` → Replace with embedding_tool commands
3. `ingest.sh` → Migrate to core ingest functionality
4. `test_text_chunker.sh` → Replace with proper unit tests
5. `ingest_single_character.sh` → Integrate into unified ingest tool

## Migration Process

For each script to be migrated:

1. **Create Rust implementation** in the appropriate directory (e.g.,
   `src/utils/system/` or `src/utils/validation/`)
1. **Add entry to Cargo.toml** under `[[bin]]` section
1. **Build and test** the Rust implementation
1. **Update documentation** to reference the new Rust binary
1. **Remove the original shell script** once migration is complete and tested

## Rust Implementation Structure

When migrating a script to Rust, organize the code in the following structure:

```text
src/
├── utils/
│   ├── validation/       # Validation tools
│   │   ├── validate_naming.rs
│   │   └── format_md.rs
│   ├── json_tools/       # JSON tools
│   │   ├── format_json.rs
│   │   └── validate_json_schema.rs
│   └── system/           # System tools
│       └── status.rs
├── ingest/               # Ingestion tools
│   └── chunked_ingest.rs
└── core/                 # Core functionality
    └── embedding/
        └── embedding_bin.rs
```
