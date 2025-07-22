# HARALD Validation & Code Quality Tools

This directory contains automated scripts for validating and formatting files in
the HARALD project.

## 🔧 Automated Cleanup Tools - Use These First

**Before manually fixing any formatting or linting issues, ALWAYS run these
automated tools:**

### Available Scripts

- **`check-json.sh`** - Validates and formats JSON files using jsonlint and
  prettier. **Run this for any JSON formatting issues.**
- **`check-rust.sh`** - Runs formatting (rustfmt), linting (clippy), tests, and
  security audit on Rust code. **Run this for any Rust issues.**
- **`format_md`** - (Rust tool) Formats Markdown files using prettier  
  with consistent line length and spacing. **Run this for any Markdown
  formatting issues.**
- **`validate_naming`** - (Rust tool) Validates file and directory names against
  project conventions. Use `--fix --verbose` for interactive fixes.

## Usage

**Always run these scripts from the project root:**

```bash
# Fix JSON formatting and validation issues
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
