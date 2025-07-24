# HARALD Validation & Code Quality Tools

This directory contains essential validation scripts that must work even when
Rust code fails. All application logic validation has been migrated to
self-documenting Rust binaries.

## 🔧 Essential Validation Scripts

These scripts provide infrastructure-level validation and orchestration:

### Available Scripts

- **`check-json.sh`** - Orchestrates JSON validation using Rust tools
- **`check-rust.sh`** - Orchestrates Rust code quality checks (formatting,
  linting, tests, security audit). Must work even when Rust code fails.

## Rust Validation Tools

For day-to-day validation, use the self-documenting Rust binaries:

```bash
# Build all validation tools
cd src && cargo build --release --features cli

# Use self-documenting validation tools
./src/target/release/format_md --help
./src/target/release/validate_naming --help --fix --verbose
./src/target/release/check_json --help
./src/target/release/format_json --help
```

## Usage

**Always run these scripts from the project root:**

```bash
# Essential validation (infrastructure-level)
./scripts/validation/check-json.sh

# Fix Rust formatting, linting, run tests
./scripts/validation/check-rust.sh

# Fix Markdown formatting (line length, spacing, etc.)
./src/target/release/format_md

# Check and fix naming convention problems
./src/target/release/validate_naming --fix --verbose
```

## When to Use Each Tool

- **Got JSON linting errors?** → Run `check-json.sh`
- **Got Rust clippy warnings or formatting issues?** → Run `check-rust.sh`
- **Got Markdown line length or formatting issues?** → Run `format_md`
- **Got file naming convention warnings?** → Run `validate_naming --fix`

## Integration with Development

These scripts are designed to be used:

- **Locally during development** - Run before committing changes
- **In CI/CD pipelines** - Automated quality checks
- **When encountering linting errors** - Fix automatically instead of manually

**Key principle: Let automation handle formatting, focus on logic and content.**
