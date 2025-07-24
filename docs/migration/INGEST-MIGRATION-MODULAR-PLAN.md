# Refactor to Modular Ingestion Architecture

## Motivation

To support both general-purpose and domain-specific ingestion pipelines (e.g., MarvelAI, other entity types), we are refactoring the ingestion system to use a modular, reusable architecture. This will:

- Eliminate code duplication between `src/ingest/` and `rust_ingest/`
- Make it easy to add new pipelines by reusing core logic
- Ensure all ingestion, chunking, and embedding logic is robust, tested, and consistent

## Step-by-Step Plan

### 1. Refactor `rust_ingest/` as a Library Crate

- Move all reusable ingest, chunking, embedding, and validation logic into `rust_ingest/src/lib.rs`.
- Expose a clean API for chunking, embedding, and indexing.
- Keep a CLI binary in `rust_ingest/src/bin/` for general ingest operations.

### 2. Update `marvelai_ingest.rs` to Use the Library

- Refactor `src/ingest/marvelai_ingest.rs` to:
  - Import and use the `rust_ingest` library for all core ingest steps.
  - Only handle Marvel-specific configuration, data selection, and CLI interface.
  - Remove any duplicated ingest logic.

### 3. Remove Duplicates and Legacy Code

- Once the new architecture is validated:
  - Remove any legacy or duplicate ingest code from `src/ingest/` that is now handled by the library.
  - Ensure only one canonical ingest implementation exists for each function.

### 4. Update Documentation and Examples

- Update `README.md`, `copilot-instructions.md`, and CLI usage docs to:
  - Show how to use the new modular ingest system for both general and domain-specific pipelines.
  - Document the API and CLI for the `rust_ingest` library and binaries.

### 5. Validate and Test

- Run all tests and validation tools to ensure the new architecture works end-to-end.
- Add new tests for the library API if needed.

## Benefits

- Single source of truth for ingest logic
- Easy to add new pipelines (just write a thin wrapper)
- Consistent, maintainable, and testable codebase
- Clear separation between core logic and domain-specific configuration

---

_This plan supersedes previous duplication-prone migration steps and aligns with the modular, Rust-first architecture described in project documentation._
