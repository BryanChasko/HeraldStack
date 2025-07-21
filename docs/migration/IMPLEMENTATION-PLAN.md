# Implementation Plan for Directory Restructuring

This document outlines the step-by-step plan for migrating from the current
directory structure to the recommended structure.

## Current vs. Target Structure

### Current Root Structure

```
HARALD/
├── ai-entities/               # AI entity definitions
├── data/                     # Data files and schemas
├── docs/                     # Documentation
├── rust_ingest/              # Rust implementation
├── scripts/                  # Various scripts
└── [other files]
```

### Target Root Structure

```
HARALD/
├── ai-entities/               # AI entity definitions (unchanged)
├── config/                   # Configuration files (NEW)
├── data/                     # Data files (reorganized)
├── docs/                     # Documentation (unchanged)
├── scripts/                  # Scripts (reorganized into subdirs)
├── src/                      # Main source code (NEW)
│   └── ingest/               # Includes code from rust_ingest
└── tests/                    # Tests directory (NEW)
```

## Phase 1: Preparation (Week 1)

1. Create the new directory structure
   - Create `src` directory with subdirectories
   - Create `config` directory
   - Create `tests` directory
   - Reorganize `scripts` into subdirectories

2. Document migration plan
   - Create file mappings from old to new structure
   - Update READMEs with new organization information
   - Document exclusions from validation

3. Update validation tools
   - Update naming convention validators
   - Create directory structure validators
   - Ensure all tools respect the new structure

## Phase 2: Rust Code Migration (Week 2)

1. Migrate Rust code from rust_ingest/ to src/ingest/
   - DO NOT create both rust_ingest/ and src/ingest/rust/ (avoid duplication)
   - Copy files from rust_ingest/ to src/ingest/ directly
   - Update imports and paths in the Rust code
   - Update build scripts to work with the new location
   - Test functionality to ensure nothing is broken
   - Once verified, deprecate the old rust_ingest/ directory

2. Create proper module structure
   - Define clear API boundaries
   - Separate concerns between modules
   - Update documentation

## Phase 3: Scripts Migration (Week 3)

1. Move development scripts to scripts/dev/
   - Update paths in scripts
   - Update documentation references
   - Test functionality

2. Create deployment scripts in scripts/deploy/
   - Implement proper deployment workflow
   - Add monitoring and logging
   - Document deployment process

## Phase 4: Data and Configuration (Week 4)

1. Organize data directory
   - Create raw/ and processed/ subdirectories
   - Document data pipeline
   - Add validation for data files

2. Set up configuration management
   - Move configuration to config/
   - Create schemas for validation
   - Document configuration options

## Phase 5: Final Review (Week 5)

1. Conduct full code review
   - Ensure all components work together
   - Verify documentation accuracy
   - Check for any missed files

2. Clean up transitional files
   - Remove old directories after migration
   - Update all documentation references
   - Ensure CI/CD pipeline works with new structure

3. Release notes
   - Document changes for team members
   - Update external documentation
   - Train team on new structure
