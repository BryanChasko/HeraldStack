# Recommended Project Structure

**Created**: July 2025  
**Last Updated**: July 24, 2025  
**Version**: 1.0  
**Status**: Planning Document (will be moved to docs/project-planning/)

This document outlines recommended organizational improvements for the HARALD
project based on developer best practices.

## Current vs Future Structure

### Current Structure

```bash
HARALD/
├── ai-entities/          # AI entity definitions
├── data/                 # Data files and schemas
├── docs/                 # Documentation
├── rust_ingest/          # Rust implementation for ingestion
├── scripts/              # Various scripts with mixed purposes
│   ├── json-tools/       # JSON formatting and validation
│   ├── validation/       # Code validation
│   └── [many scripts]    # Ingest, query, and utility scripts
└── [various files]       # Configuration and other files
```

### Recommended Structure

```bash
HARALD/
├── src/                  # NEW: Main source code directory
│   ├── api/              # API endpoints and handlers
│   ├── core/             # Core application logic
│   │   ├── embedding/    # Embedding-related logic
│   │   ├── entities/     # Entity management logic
│   │   └── memory/       # Memory handling logic
│   ├── ingest/           # Ingestion pipeline (from rust_ingest)
│   └── utils/            # Shared utilities and helpers
├── ai-entities/          # UNCHANGED: AI entity definitions
├── config/               # NEW: Configuration files
│   ├── default.json      # Default configuration
│   └── schemas/          # JSON schemas (from data/schemas)
├── data/                 # REORGANIZED: Data files
│   ├── raw/              # Raw input data
│   └── processed/        # Processed data
├── docs/                 # UNCHANGED: Documentation
├── scripts/              # REORGANIZED: Utility scripts
│   ├── dev/              # Development scripts
│   ├── validation/       # Validation scripts
│   └── deploy/           # Deployment scripts
└── tests/                # NEW: Tests directory
    ├── unit/             # Unit tests
    ├── integration/      # Integration tests
    └── fixtures/         # Test fixtures
```

## Specific Improvements

### 1. Move Code into `src`

- Organize application code in a standard src directory
- Separate application code from configuration and resources
- Make imports and file paths more consistent
- Follow standard practices for most languages

### 2. Create a `config` Directory

- Centralize all configuration files in one location
- Separate configuration from code and data
- Make configuration management more straightforward

### 3. Improve `scripts` Organization

- Add dev/ for development-specific scripts
- Keep validation/ for validation scripts (already exists)
- Create deploy/ for deployment and infrastructure scripts

### 4. Add a `tests` Directory

- Separate tests from implementation code
- Make it easier to run specific test categories
- Provide clear location for test fixtures

### 5. Data Pipeline Organization

- Add raw/ and processed/ subdirectories in data
- Create clear separation of input and output data
- Better track data transformations

## Migration Strategy

1. Start with new feature development in the new structure
2. Gradually migrate existing code as it's modified
3. Update import paths and documentation incrementally
4. Use the validation tools to ensure consistency during migration
