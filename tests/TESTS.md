# HARALD Tests

This directory contains tests for the HARALD project.

## Directory Structure

- `unit/` - Unit tests for individual components
- `integration/` - Integration tests for testing multiple components together
- `fixtures/` - Test data and fixtures
- `data/` - Test data files used for migration verification
- `output/` - Output files from migration verification tests

## Running Tests

### Standard Tests

TBD - Add instructions for running standard tests

### Shell Script Migration Tests

To verify the migration from shell scripts to Rust, use the verification script:

```bash
# Run all available tests
./scripts/verify_rust_migration.sh

# Clean previous test outputs before running
./scripts/verify_rust_migration.sh --clean
```

The verification script compares the output of original shell scripts (`.legacy`
files) with the new Rust implementations to ensure identical results.

## Migration Verification

Currently, the following shell scripts have been migrated to Rust:

- `text_chunker.sh` → `src/utils/chunking.rs`
- `test_embedding_size.sh` → `src/core/embedding/embedding_bin.rs`

See the full migration status in `docs/migration/INGEST-MIGRATION.md` and the
cleanup plan in `docs/migration/SCRIPT-CLEANUP-PLAN.md`.

## Best Practices

1. Write tests for all new functionality
2. Maintain high test coverage
3. Keep tests fast and independent
4. Use descriptive test names
5. Follow the AAA pattern: Arrange, Act, Assert
6. When migrating scripts, verify that outputs match exactly
