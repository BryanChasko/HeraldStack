# Vector Data Store Registry and JSON Formatting

## Overview

The Vector Data Store Registry provides a centralized mechanism for managing and
tracking all JSON files that need to be processed for vector embeddings in
HARALD. This system helps ensure consistent formatting and validation of JSON
files before they're ingested into the vector database.

## Registry Structure

The registry is maintained in `data/vector_stores_registry.json` and contains:

1. **Vector Store Definitions**: Metadata about each store including:

- Source files that should be processed
- Location where embeddings are stored
- Chunking strategy and parameters
- Associated ingest scripts

2. **Embedding Model Information**: Details on supported embedding models and
   their limitations.

## Tools

### JSON Formatting Utility

The `format_json.sh` script provides tools for:

- Formatting JSON files according to project standards
- Only processing files that are registered in the vector store registry
- Validating the registry itself
- Adding new files to the registry

```bash
# Format all registered files
./scripts/format_json.sh --all

# Format files for a specific store
./scripts/format_json.sh --store marvel_characters

# Format a specific file (must be registered)
./scripts/format_json.sh --file ./personality-archetypes/Heralds.json

# Check format without modifying
./scripts/format_json.sh --check

# Register a new file
./scripts/format_json.sh --register ./path/to/new/file.json

# Validate the registry
./scripts/format_json.sh --validate-registry
```

### JSON Schema Validation

The `validate_json_schema.sh` script provides tools for:

- Validating JSON files against predefined schemas
- Generating schemas based on existing files
- Store-specific validation rules

```bash
# Validate all files for a store
./scripts/validate_json_schema.sh --store marvel_characters

# Validate a specific file
./scripts/validate_json_schema.sh --file ./path/to/file.json

# Generate a schema from existing files
./scripts/validate_json_schema.sh --generate marvel_characters
```

## Workflow

The recommended workflow for managing vector data:

1. **Register** new JSON files in the vector store registry
2. **Format** the files according to project standards
3. **Generate** schemas for consistent validation
4. **Validate** files against schemas
5. **Process** files using the appropriate ingest script

## Integration with Text Chunking

The registry includes configuration for text chunking strategies to use with
different vector stores. This ensures that:

1. Different data types can use appropriate chunking strategies
2. Each store can maintain consistent chunking parameters
3. Chunking is aligned with embedding model limitations

## Best Practices

1. **Always register new files** before formatting or processing them
2. **Maintain schema definitions** for complex data structures
3. **Use consistent chunking strategies** for similar data types
4. **Document store-specific requirements** in schema files
5. **Validate files** before ingestion to catch issues early

## Adding New Vector Stores

To add a new vector store:

1. Use the registration tool:

   ```bash
   ./scripts/format_json.sh --register ./path/to/first/file.json
   ```

2. Follow the prompts to define store properties
3. Generate a schema for the new store:

   ```bash
   ./scripts/validate_json_schema.sh --generate new_store_id
   ```

4. Update the schema as needed for specific validation requirements

## Related Documentation

- [Ollama Embedding Limits](./ollama-embedding-limits.md)
- [Character-Based Chunking](./character-based-chunking.md)
- [JSONL Ingestion](./jsonl-ingestion.md)
