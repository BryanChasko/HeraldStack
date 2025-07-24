# Directory Reorganization Summary

This document summarizes the changes made to improve the organization of the
HARALD project directory structure.

## Files Moved

| File                         | Original Location | New Location      |
| ---------------------------- | ----------------- | ----------------- |
| `LawsOfRobotics.json`        | Root              | `config/ethics/`  |
| `Modelfile`                  | Root              | `config/models/`  |
| `rustfmt.toml`               | Root              | `src/`            |
| `test_single_character.json` | Root              | `tests/fixtures/` |
| `GITHUB.md`                  | Root              | `docs/`           |
| `IMPLEMENTATION-PLAN.md`     | Root              | `docs/migration/` |
| `RECOMMENDED-STRUCTURE.md`   | Root              | `docs/migration/` |
| `SCRIPT-MIGRATION.md`        | Root              | `docs/migration/archive/` |
| `INGEST-MIGRATION.md`        | Root              | `docs/migration/` |

## README Updates

The main `README.md` has been updated with links to the new file locations.

## Completion Status

✅ Review the moved files to ensure they work in their new locations  
✅ Remove the original files from the root directory once verified  
✅ Update any code or scripts that may reference the old file paths  
✅ Retire shell scripts migrated to Rust implementations  
⬜️ Commit the changes with a descriptive message about the reorganization

### Updated Scripts

The following scripts were updated to reference the new file locations:

- `/scripts/ingest_chunked.sh`
- `/scripts/test_basic_embedding.sh`
- `/scripts/ingest_single_character.sh`

### Retired Scripts

The following shell scripts have been migrated to Rust and retired:

- `text_chunker.sh` → Replaced by `src/target/debug/text_chunker`
- `test_text_chunker.sh` → Replaced by unit tests in Rust
- `test_embedding_size.sh` → Replaced by `src/target/debug/embedding_tool`
- `ingest_chunked.sh` → Replaced by `src/target/debug/ingest_chunked`

### Migration to Native Rust Binaries

Instead of shell script wrappers, the project now uses native Rust binaries
directly:

```bash
# Old approach (shell script wrapper)
./scripts/ingest_chunked.sh --file data.json --model harald-phi4

# New approach (direct Rust binary)
./src/target/debug/ingest_chunked --file data.json --model harald-phi4

# Or build all tools at once
./scripts/build_rust_tools.sh
```

## Benefits

This reorganization provides several benefits:

- Cleaner root directory with fewer files
- Better organization of configuration files
- Proper location for test fixtures
- Consolidated documentation files
- More maintainable project structure
- Improved performance and reliability through Rust implementations
- Type safety and better error handling with compiled code
- Consistent API for previously disparate script functionality
