# Script Migration Cleanup Plan

This document outlines the plan for cleaning up the migration of shell scripts
to Rust once we're confident the implementations work correctly.

## Current Status

We've successfully migrated the following scripts to Rust:

- ✅ `text_chunker.sh` → Now a wrapper around `src/target/release/text_chunker`
- ✅ `test_embedding_size.sh` → Now a wrapper around
  `src/target/release/embedding_tool`
- ✅ `ingest_chunked.sh` → Direct Rust binary at `target/release/ingest_chunked`
- ✅ `format_json.sh` → Direct Rust binary at `target/release/format_json`

The original scripts have been backed up as:

- `text_chunker.sh.legacy`
- `test_embedding_size.sh.legacy`
- `ingest_chunked.sh.legacy`
- `format_json.sh.legacy`

## Pending Migrations

The following scripts still need to be migrated:

1. `ingest_marvelai.sh` → Integrate into unified ingest tool
2. `test_basic_embedding.sh` → Replace with embedding_tool commands
3. `ingest.sh` → Migrate to core ingest functionality
4. `test_text_chunker.sh` → Replace with proper unit tests
5. `ingest_single_character.sh` → Integrate into unified ingest tool

## Migration Principles

Our migration follows these key principles:

1. **ABSOLUTELY NO NEW SHELL SCRIPTS** - We're migrating TO Rust, not creating
   more shell scripts for application logic. This includes helper scripts,
   reference scripts, or "quick utilities"
2. **Direct Rust binary usage** - Users call `./target/release/binary_name`
   directly
3. **Rust-based testing** - Use `cargo test`, `cargo run`, and direct binary
   testing instead of shell scripts
4. **Clean removal process** - Remove shell scripts entirely after successful
   migration, not create wrappers
5. **Co-location principle** - Related documentation and configuration files
   should migrate to live next to the Rust code
6. **Documentation over scripts** - Information should go in markdown files,
   README sections, or --help flags, NOT in new shell scripts

## Critical Rule: Information Storage

When you need to provide information, use these approaches IN ORDER:

1. **Update existing documentation** (README.md, relevant .md files)
2. **Add --help flags** to existing Rust tools
3. **Create/update markdown documentation** in appropriate directories
4. **NEVER create a new shell script** for information or utilities

## Automated Cleanup Tools - Use These First

Before manually fixing formatting or linting issues, use our automated tools:

### JSON Issues

```bash
# Automatically fix JSON formatting and validation
./scripts/validation/check-json.sh
```

### Rust Issues

```bash
# Automatically fix Rust formatting, run clippy, and tests
./scripts/validation/check-rust.sh
```

### Markdown Issues

```bash
# Automatically fix Markdown formatting (line length, etc.)
./src/target/release/format_md
```

### Naming Convention Issues

```bash
# Check and optionally fix naming convention problems
./src/target/release/validate_naming --fix --verbose
```

**Always run these automated tools before manually editing files!**

### What TO Migrate to Rust ✅

- **Data processing scripts** (text chunking, JSON formatting, validation)
- **Application utilities** (embedding tools, ingestion scripts)
- **Business logic scripts** (character processing, data transformation)
- **Testing utilities** that involve application logic

### What NOT to Migrate to Rust ❌

- **Deployment scripts** (orchestrating external tools like Docker, AWS CLI)
- **Infrastructure scripts** (system administration, environment setup)
- **CI/CD pipeline scripts** (git hooks, build orchestration)
- **Simple file operations** (backup, cleanup, monitoring)
- **Scripts that primarily shell out to other tools**
- **Rust validation scripts** (e.g., `check-rust.sh` - must work even when Rust
  code has issues)

### Guidelines for Decision Making

**Migrate to Rust if the script:**

- Contains complex application logic
- Processes data or performs calculations
- Benefits from type safety and error handling
- Is part of the core application functionality

**Keep as shell script if the script:**

- Primarily orchestrates external tools
- Handles system/infrastructure concerns
- Needs rapid iteration and deployment
- Is better served by shell's ecosystem integration

### Co-location Details

When migrating a script, also move related files:

- **Documentation files** (`.md`) → Move to the same directory as the Rust code
- **Configuration files** (`.json`, `.toml`, etc.) → Move to the same directory
- **Schema files** → Move to the same directory
- **Test data** → Move to appropriate test directory structure

Example structure:

```text
src/utils/json_tools/
├── format_json.rs           # Main Rust implementation
├── validate_json_schema.rs  # Related Rust tool
├── JSON-TOOLS.md           # Documentation
├── schemas/                # Configuration/schema files
└── tests/                  # Module-specific tests
```

Before removing any shell script, the following testing must be completed:

### 1. Functional Equivalence Testing

For each migrated script, verify that the Rust implementation:

```bash
# Example for format_json
# Test help output
./target/release/format_json --help
./scripts/json-tools/format_json.sh.legacy --help

# Test basic functionality
./target/release/format_json --all
./scripts/json-tools/format_json.sh.legacy --all

# Compare outputs (should be identical)
diff <(./target/release/format_json --check) \
     <(./scripts/json-tools/format_json.sh.legacy --check)
```

### 2. Performance Testing

Benchmark the Rust implementation against the shell script:

```bash
# Example timing comparison
time ./target/release/format_json --all
time ./scripts/json-tools/format_json.sh.legacy --all
```

### 3. Edge Case Testing

Test with:

- Invalid arguments
- Missing files
- Malformed input data
- Large files
- Empty input
- Special characters in paths

### 4. Integration Testing

Verify that other scripts/systems that depend on the output still work
correctly.

## Script Removal Process

Once testing is complete and successful, follow this removal process:

### Phase 1: Mark for Removal (1 week grace period)

1. **Create removal announcement**:

   ```bash
   # Add deprecation notice to the shell script
   echo "# DEPRECATED: This script has been migrated to Rust." >> script.sh
   echo "# Use: ./target/release/binary_name instead" >> script.sh
   echo "# This script will be removed on YYYY-MM-DD" >> script.sh
   ```

2. **Update documentation immediately** to reference Rust binaries
3. **Notify team members** of the upcoming removal

### Phase 2: Remove Shell Scripts

1. **Final verification**:

   ```bash
   # Ensure no other scripts reference the shell script
   grep -r "script_name.sh" . --exclude-dir=target --exclude="*.legacy"
   ```

2. **Remove wrapper scripts**:

   ```bash
   # Remove the shell wrapper (if using direct Rust binary approach)
   rm scripts/path/to/script_name.sh
   ```

3. **Clean up legacy files** (after extended grace period):

   ```bash
   # Remove .legacy backup after 1 month
   rm scripts/path/to/script_name.sh.legacy
   ```

### Phase 3: Documentation Update

1. **Update all references** in documentation
2. **Update README files** with new command examples
3. **Update integration guides** to use Rust binaries
4. **Create migration notes** for external users

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

## Current Migration Checklist

### format_json.sh Migration ✅ COMPLETED

- [x] Rust implementation completed (`src/utils/json_tools/format_json.rs`)
- [x] Binary builds successfully (`target/release/format_json`)
- [x] Help output verified
- [x] Basic functionality tested
- [x] Original script backed up (`format_json.sh.legacy`)
- [x] **Co-location completed**: Documentation moved to
      `src/utils/json_tools/JSON-TOOLS.md`
- [x] **Documentation updated**: References changed to use Rust binary
- [x] **Shell script removed**: No wrapper needed for complete implementation
- [ ] **TODO**: Functional equivalence testing
- [ ] **TODO**: Performance comparison
- [ ] **TODO**: Edge case testing
- [ ] **TODO**: Integration testing

### validate_json_schema.sh Migration ✅ COMPLETED

- [x] Rust implementation completed
      (`src/utils/json_tools/validate_json_schema.rs`)
- [x] Binary builds successfully (`target/release/validate_json_schema`)
- [x] CLI interface implemented with clap subcommands
- [x] Help output verified
- [x] Basic functionality implemented (validate, generate, store commands)
- [x] Original script backed up (`validate_json_schema.sh.legacy`)
- [x] **Co-location completed**: Implementation placed in same directory as
      related tools
- [x] **Documentation updated**: References changed to use Rust binary
- [x] **Shell script removed**: Direct Rust binary usage, no wrapper needed
- [ ] **TODO**: Functional equivalence testing
- [ ] **TODO**: Performance comparison
- [ ] **TODO**: Edge case testing
- [ ] **TODO**: Integration testing

### Migration Template for Future Scripts

For each new migration, copy this checklist:

```markdown
### script_name.sh Migration ⏳ IN PROGRESS

- [ ] Rust implementation started (`src/path/to/module.rs`)
- [ ] Binary builds successfully (`target/release/binary_name`)
- [ ] CLI interface implemented
- [ ] Help output verified
- [ ] Basic functionality tested
- [ ] Original script backed up (`script_name.sh.legacy`)
- [ ] **Co-location completed**: Related files moved to Rust module directory
  - [ ] Documentation (.md files)
  - [ ] Configuration files (.json, .toml, etc.)
  - [ ] Schema files
- [ ] **Documentation updated**: All references changed to use Rust binary
- [ ] **Shell script removed**: No wrapper needed for complete implementation
- [ ] Functional equivalence testing completed
- [ ] Performance comparison completed
- [ ] Edge case testing completed
- [ ] Integration testing completed
```

## Testing Commands Reference

For quick reference, here are the testing commands for completed migrations:

### format_json Testing

```bash
# Build latest version
cargo build --release --features cli

# Test help output
./target/release/format_json --help

# Test basic functionality
./target/release/format_json --all
./target/release/format_json --check

# Run Rust unit tests (when available)
cargo test format_json
```

### validate_json_schema Testing

```bash
# Test help output
./target/release/validate_json_schema --help
./target/release/validate_json_schema store --help
./target/release/validate_json_schema generate --help

# Run Rust unit tests (when available)
cargo test validate_json_schema

# Test with sample data (when available)
./target/release/validate_json_schema store marvel_characters
./target/release/validate_json_schema generate marvel_characters
```

### ingest_chunked Testing

```bash
# Test help output
./target/release/ingest_chunked --help

# Run Rust unit tests (when available)
cargo test ingest_chunked

# Test with sample data (when available)
./target/release/ingest_chunked --file sample.json --model mxbai-embed-large
```

## Recommended Testing Approach

Instead of shell scripts, use:

1. **Rust unit tests** in the same modules (`#[cfg(test)]`)
2. **Integration tests** in `tests/` directory
3. **Direct binary testing** with `cargo run --bin binary_name`
4. **Manual verification** with real data files

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
