# HARALD Source Code

This directory contains the main source code for the HARALD project.

## Directory Structure

- `api/` - API endpoints and handlers
- `core/` - Core application logic
  - `embedding/` - Embedding-related logic
  - `entities/` - Entity management logic
  - `memory/` - Memory handling logic
- `ingest/` - Ingestion pipeline (migrated from rust_ingest/)
- `utils/` - Shared utilities and helpers

## Migration Strategy

1. Migrate the rust_ingest/ code directly into src/ingest/
   - Don't create duplicate structures
   - Don't keep both rust_ingest/ AND src/ingest/
2. Create clear API boundaries between components
3. Implement new features following this structure
4. Gradually migrate existing functionality from scripts into proper modules
