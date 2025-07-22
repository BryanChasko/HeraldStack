# Script Migration Cleanup Plan

This document outlines the plan for cleaning up the migration of shell scripts
to Rust once we're confident the implementations work correctly.

## Current Status

We've successfully migrated the following scripts to Rust:

- `text_chunker.sh` → Now a wrapper around `src/target/release/text_chunker`
- `test_embedding_size.sh` → Now a wrapper around
  `src/target/release/embedding_tool`

The original scripts have been backed up as:

- `text_chunker.sh.legacy`
- `test_embedding_size.sh.legacy`

## Pending Migrations

The following scripts still need to be migrated:

1. `ingest_chunked.sh` → Migrate to `src/ingest/chunked_ingest.rs`
2. `ingest_marvelai.sh` → Integrate into unified ingest tool
3. `test_basic_embedding.sh` → Replace with embedding_tool commands
4. `ingest.sh` → Migrate to core ingest functionality
5. `test_text_chunker.sh` → Replace with proper unit tests
6. `ingest_single_character.sh` → Integrate into unified ingest tool

## Cleanup Steps

Once we're confident that the Rust implementations work correctly, we should:

### 1. Update Documentation

- Document the new Rust-based workflow
- Update README files to point to the new tools
- Add examples of using the new tools

### 2. Remove Backup Scripts

- Delete `.legacy` backup files
- Ensure all references to the original scripts are updated

### 3. Simplify Wrapper Scripts

- Remove temporary migration code
- Simplify error handling
- Update comments to reflect permanent status

## Implementation Schedule

### Phase 1 (Current)

- Complete migrations of text chunking and embedding size testing
- Document the migration process
- Test the new Rust implementations

### Phase 2 (Next 2 weeks)

- Migrate ingest-related scripts to Rust
- Create unified ingest CLI tool
- Update documentation and examples

### Phase 3 (Final)

- Migrate remaining test scripts
- Remove backup files
- Complete final documentation

## Success Criteria

By the end of this process, we should have:

1. A unified Rust-based CLI for all operations
2. Minimal shell script wrappers where needed for backward compatibility
3. Complete documentation of the new tools
4. Comprehensive test suite for all functionality
5. No legacy scripts in active use

## Remaining Migrations

The following scripts still need to be migrated:

1. `ingest_chunked.sh` → Migrate to `src/ingest/chunked_ingest.rs`
2. `ingest_marvelai.sh` → Integrate into unified ingest tool
3. `test_basic_embedding.sh` → Replace with embedding_tool commands
4. `ingest.sh` → Migrate to core ingest functionality
5. `test_text_chunker.sh` → Replace with proper unit tests
6. `ingest_single_character.sh` → Integrate into unified ingest tool

## Timeline

- **Week 3**: Complete migrations of ingest-related scripts
- **Week 4**: Complete migrations of test-related scripts
- **Week 5**: Cleanup and finalize documentation

## Final Deliverable

By the end of this process, we should have:

1. A unified Rust-based CLI for all operations
2. Minimal shell script wrappers where needed for backward compatibility
3. Complete documentation of the new tools
4. Comprehensive test suite for all functionality
5. No legacy scripts in active use
