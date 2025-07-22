# HeraldStack Validation & Code Quality Tools

This directory contains scripts used for validating and formatting files in the
HeraldStack project.

## Available Scripts

- `check-json.sh` - Validates and formats JSON files in the project using
  jsonlint and prettier
- `check-rust.sh` - Runs formatting (rustfmt), linting (clippy), tests, and
  security audit on Rust code
- `format_md` (Rust) - Formats Markdown files in the project using prettier
- `validate_naming` (Rust) - Validates file and directory names against project
  naming conventions

## Usage

Run these scripts from the project root for best results:

```bash
# Validate and format JSON files
./scripts/validation/check-json.sh

# Validate Rust code
./scripts/validation/check-rust.sh

# Format Markdown files
./src/target/release/format_md

# Validate file and directory names
./src/target/release/validate_naming

# Fix naming issues interactively
./src/target/release/validate_naming --fix --verbose
```

These scripts are designed to be used both locally during development and in
CI/CD pipelines.
