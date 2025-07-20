# Contributing to rust_ingest

This docs/CONTRIBUTING.md document outlines development standards and practices
for the HeraldStack rust_ingest tool.

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
