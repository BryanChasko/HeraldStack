# Contributing to HARALD

**Created**: July 2025  
**Last Updated**: July 24, 2025  
**Version**: 1.1

This document provides practical guidance for contributing to the HARALD
project. For high-level development philosophy and principles, see
[DEVELOPMENT-PRINCIPLES.md](DEVELOPMENT-PRINCIPLES.md).

## Getting Started

### Core Development Rules

Before contributing, please familiarize yourself with our
[Development Principles](DEVELOPMENT-PRINCIPLES.md), which include:

- **NO NEW SHELL SCRIPTS** - We're migrating to Rust for application logic
- **Documentation over code** - Prefer updating docs to writing new scripts
- **Automation over manual work** - Use and extend our automated tools

For the complete migration strategy and decision framework, refer to the
[Development Principles](DEVELOPMENT-PRINCIPLES.md#-rust-vs-shell-decision-framework)
document.

**Ingestion/Embedding Architecture:** All ingestion and embedding logic must
follow the
[Modular Ingest Refactor Plan](migration/INGEST-MIGRATION-MODULAR-PLAN.md). This
plan defines the canonical, reusable ingest library and the pattern for
domain-specific wrappers (e.g., marvelai_ingest.rs). All new pipelines and
refactors must use this architecture and update documentation accordingly.

## 🔧 Automated Cleanup Tools First

Before manually fixing any formatting or linting issues, **ALWAYS** run our
automated tools:

```bash
# Fix JSON formatting and validation issues
./src/target/release/check_json --fix

# Fix Rust formatting, run clippy, and tests
./scripts/validation/check-rust.sh

# Fix Markdown formatting (line length, spacing, etc.)
./src/target/release/format_md

# Check and optionally fix naming convention problems
./src/target/release/validate_naming --fix --verbose
```

**These tools will automatically resolve most linting and formatting issues.**
Only manually edit files after running the appropriate automated tool.

### 🔎 Tool Paths and Usage

Our development tools are Rust programs that get compiled to executable files
during the build process. When you run `cargo build --release` in a Rust
project, it creates optimized binaries in the `target/release` directory.

In our project, binaries can be found in two different locations:

**Main tools directory**: `./src/target/release/`

- Most Rust tools are here (format_md, validate_naming, etc.)
- Built from the src/Cargo.toml file
- Example: `./src/target/release/format_md`

**Secondary location**: `./target/release/`

- Some newer tools are here (check_json, status, etc.)
- Example: `./src/target/release/check_json`

You need to check both locations when looking for tools. Always refer to
specific documentation for each tool to find its correct path.

Here are the correct commands to run our most common tools from the project root
directory:

```bash
# Format Markdown files with prettier
./src/target/release/format_md path/to/your/file.md

# Check and fix JSON files (wrapper around format_json)
./src/target/release/check_json --fix

# Validate naming conventions across the codebase
./src/target/release/validate_naming --fix --verbose

# Check system status (Ollama services, models, etc)
./src/target/release/status
```

**IMPORTANT:** Always run these tools from the project root directory to ensure
correct path resolution. Running them from other directories may cause file path
errors.

## Development Environment

### Required Tools

- Rust 1.77 or later
- Cargo package manager
- Git
- VS Code (recommended)

### VS Code Configuration

**Required Extensions**:

- rust-analyzer
- Even Better TOML
- markdownlint
- CodeLLDB (for debugging)

**Recommended Settings**:

```json
{
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "rust-analyzer.checkOnSave.command": "clippy",
  "[markdown]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "DavidAnson.vscode-markdownlint"
  },
  "markdownlint.config": {
    "MD013": { "line_length": 80 }
  }
}
```

## Collaboration Standards

We value empathy, transparency, and practical collaboration. Please follow these
standards when working with others:

- **Begin meetings with an emotion check-in** (e.g., "What excites you? What
  worries you?") to build psychological safety and surface risks early.
- **Translate technical details into user language** to ensure features are
  user-focused and accessible.
- **Use small courtesies** (like "thank you") to foster equitable knowledge
  sharing.
- **Communicate authentically**—speak like a human, not a robot.
- **Practice empathy**: it grows with use and is essential for effective
  teamwork.
- **Assume good intent and seek clear impact**—disagree respectfully, document
  decisions, and move forward together.
- **Bias toward co-creation**: sketches, sticky notes, and quick prototypes are
  preferred over long comment threads.
- **Listen past the first answer**—follow-up questions deepen understanding.

> "Empathy is a muscle: left unused, it atrophies; put to work, it grows." —
> Jamil Zaki, Stanford "Minds are mirrors to one another." — David Hume "Seeing
> the world through the eyes of the other, not seeing your world reflected in
> their eyes." — Carl Rogers

For more on our collaboration philosophy, see
[DEVELOPMENT-PRINCIPLES.md](DEVELOPMENT-PRINCIPLES.md).
