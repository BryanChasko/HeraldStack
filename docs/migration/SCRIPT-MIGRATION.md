# Script Migration Plan

This document outlines how to migrate existing scripts to the new directory
structure.

## Development Scripts

Move to `scripts/dev/`:

- `ingest.sh`
- `ingest_chunked.sh`
- `ingest_marvelai.sh`
- `ingest_single_character.sh`
- `query.sh`
- `status.sh`
- `test_basic_embedding.sh`
- `test_embedding_size.sh`
- `test_text_chunker.sh`
- `text_chunker.sh`

## Validation Scripts

Already in `scripts/validation/`:

- `validate_naming.sh` (moved from scripts root)
- Other validation scripts

## Deployment Scripts

Create new scripts in `scripts/deploy/`:

- `deploy.sh` - Deployment script
- `backup.sh` - Database/vector store backup
- `monitor.sh` - System monitoring

## JSON Tools

Already in `scripts/json-tools/`:

- Format and validation tools for JSON files

## Migration Commands

```bash
# Move development scripts
for script in ingest*.sh query.sh status.sh test_*.sh text_chunker.sh; do
  git mv scripts/$script scripts/dev/
done

# Move validation script
git mv scripts/validate_naming.sh scripts/validation/

# Create placeholder deployment scripts
touch scripts/deploy/{deploy,backup,monitor}.sh
chmod +x scripts/deploy/{deploy,backup,monitor}.sh
```
