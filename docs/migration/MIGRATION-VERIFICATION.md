# Shell Script to Rust Migration Verification

This document outlines the process for verifying that our Rust implementations
correctly match the behavior of the original shell scripts.

## Overview

As part of our effort to migrate shell scripts to Rust, we need to ensure
that the new implementations maintain the same behavior and output as the
original scripts. This document explains how to use the verification tools
to test the migration.

## Verification Process

To verify the migration from shell scripts to Rust, use the verification script:

```bash
# Run all available tests
./scripts/verify_rust_migration.sh

# Clean previous test outputs before running
./scripts/verify_rust_migration.sh --clean
```

The verification script compares:

- Outputs from original shell scripts (`.legacy` files)
- Against the new Rust implementations

to ensure they behave identically.

## Test Data and Outputs

- Test input data is stored in the `tests/data/` directory
- Test outputs are stored in the `tests/output/` directory
- The verification script automatically creates test data if needed

## Current Migration Status

The following shell scripts have been migrated to Rust:

- `text_chunker.sh` → `src/utils/chunking.rs`
- `test_embedding_size.sh` → `src/core/embedding/embedding_bin.rs`

See the full migration status in `docs/migration/INGEST-MIGRATION.md` and the
cleanup plan in `docs/migration/SCRIPT-CLEANUP-PLAN.md`.

## Extending the Verification Process

To add tests for newly migrated scripts:

1. Update the `verify_rust_migration.sh` script to include the new tests
2. Add appropriate test data to `tests/data/` if needed
3. Run the verification to ensure the new implementation matches the original

## Troubleshooting

If verification tests fail with output differences:

1. Check the output files in `tests/output/` to see the differences
2. Look for whitespace, line ending, or formatting differences
3. Ensure the Rust implementation handles edge cases properly
4. Update the implementation as needed to match the original behavior

## Best Practices

- Always run verification tests before removing original scripts
- Keep legacy scripts until verification is complete and successful
- When making changes to Rust implementations, re-verify against legacy scripts
- Document any intentional behavioral differences between implementations
