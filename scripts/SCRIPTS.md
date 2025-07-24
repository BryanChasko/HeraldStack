# Scripts Organization

This directory contains only infrastructure and essential validation scripts.
All application logic has been migrated to Rust binaries in
`src/target/release/`.

## Current Directory Structure

- `deploy/` - Deployment and infrastructure orchestration scripts
- `validation/` - Essential validation scripts (must work even when Rust fails)
- `archive/` - Archived application logic scripts (superseded by Rust binaries)

## Active Scripts

### Infrastructure Scripts

- `build_rust_tools.sh` - Orchestrates Rust binary compilation
- `deploy/deploy.sh` - Deployment orchestration using AWS CLI

### Essential Validation Scripts

- `validation/check-rust.sh` - Rust code quality checks (must work when Rust
  fails)
- `validation/check-json.sh` - JSON validation orchestration

## Migration to Rust

All application logic scripts have been migrated to type-safe, performant Rust
binaries:

**Use Rust binaries instead of shell scripts:**

```bash
# Build all tools
cd src && cargo build --release --features cli

# Use self-documenting tools
./src/target/release/format_json --help
./src/target/release/text_chunker --help
./src/target/release/marvelai_ingest --help
```

## Archived Scripts

Historical shell scripts are preserved in `archive/` for reference. See
[archive/README.md](archive/README.md) for details on what was migrated and why.

## Best Practices

1. **No new application logic scripts** - Use Rust binaries instead
2. Infrastructure scripts only for AWS CLI, Docker, deployment orchestration
3. Essential validation scripts that must work even when Rust code fails
4. All scripts should be executable (`chmod +x script.sh`)
5. Include usage information in script headers
6. Handle errors gracefully with meaningful error messages
