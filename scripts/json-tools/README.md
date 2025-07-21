# JSON Formatting and Validation Tools

This directory contains tools for ensuring consistent formatting and validation
of JSON files used for vector embeddings in the HARALD project.

## Tools

- **format_json.sh**: Formats JSON files according to project standards and
  ensures only registered files are processed
- **validate_json_schema.sh**: Validates JSON files against predefined schemas

## Usage

See detailed documentation in
[Vector Store Registry](../../docs/vector-search/vector-store-registry.md)

## Quick Start

```bash
# Format all registered JSON files
./format_json.sh --all

# Register a new JSON file
./format_json.sh --register ../path/to/file.json

# Generate a schema for a store
./validate_json_schema.sh --generate store_id

# Validate files against schema
./validate_json_schema.sh --store store_id
```

## Related Scripts

- [text_chunker.sh](../text_chunker.sh): Used for optimal text chunking before
  embedding
- [ingest_chunked.sh](../ingest_chunked.sh): Example ingestion script that uses
  chunking
