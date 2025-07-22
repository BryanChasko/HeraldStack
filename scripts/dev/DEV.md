# Development Scripts

This directory contains scripts used during development.

## Scripts

- `ingest.sh` - Core ingestion script
- `query.sh` - Query the vector database
- `status` - (Rust tool) Check system status
- `text_chunker.sh` - Text chunking utilities

## Usage

All scripts should be run from the project root directory:

```bash
./scripts/dev/script_name.sh [arguments]
```

For detailed usage of each script, run with the --help flag:

```bash
./scripts/dev/script_name.sh --help
```
