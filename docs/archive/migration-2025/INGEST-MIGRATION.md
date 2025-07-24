````markdown
# Ingest Module Migration

This document records the migration of the Rust ingest code from `rust_ingest/`
to `src/ingest/`.

## Migration Steps Completed

1. Created `src/ingest/` directory
2. Copied Rust files from `rust_ingest/src/*.rs` to `src/ingest/`
3. Updated package name from `rust_ingest` to `harald_ingest`
4. Created new Cargo.toml in the `src` directory
5. Updated imports in main.rs to use new package name
6. Fixed E0433 errors related to unresolved module references
7. Created README.md with documentation
8. Created build script at `scripts/dev/build-ingest.sh`
9. Updated path references in the code
10. Successfully built the migrated code

## Next Steps

1. See
   [INGEST-MIGRATION-MODULAR-PLAN.md](docs/migration/INGEST-MIGRATION-MODULAR-PLAN.md)
   for the current step-by-step plan to refactor to a modular, reusable
   ingestion architecture.

2. Run tests on the migrated code:

   ```bash
   ./scripts/dev/build-ingest.sh --test
   ```

3. Update any scripts that referenced the old `rust_ingest` directory

4. After successful validation, deprecate the old `rust_ingest/` directory:

   ```bash
   # After verification that everything works
   mv rust_ingest rust_ingest.old
   ```

## Benefits of New Structure

1. All source code is now in the `src` directory, following standard conventions
2. The code is organized by domain rather than technology
3. Module boundaries are clearer in the new structure
4. Future functionality can be added to the `src` directory with consistent
   organization

## Shell Scripts Migration Plan

This section outlines the plan to migrate essential shell scripts to Rust. The
goal is to replace critical bash scripts with more maintainable, performant, and
type-safe Rust implementations.

### Migration Candidates (Prioritized)

1. **text_chunker.sh** - Text chunking utility
2. **ingest_chunked.sh** - Character-based chunking for Marvel character data
3. **test_embedding_size.sh** - Embedding size testing utility
4. **status** - (Rust tool) System status checking
5. **JSON tools** (format_json.sh, validate_json_schema.sh)

### Migration Strategy

#### Phase 1: Core Text Chunking Utilities

1. **Create text chunking module** in `src/utils/chunking.rs`
   - Implement character-based chunking
   - Implement size-based chunking
   - Implement semantic chunking
   - Support all functionality from text_chunker.sh

2. **API Design**:

   ```rust
   pub enum ChunkingStrategy {
       Size(usize),      // max_size
       Character(usize), // target_size
       Semantic,         // natural breaks
   }

   pub struct ChunkerOptions {
       strategy: ChunkingStrategy,
       preserve_whitespace: bool,
       delimiter: Option<String>,
   }

   pub fn chunk_text(text: &str, options: ChunkerOptions) -> Vec<String>;
   ```

#### Phase 2: Embed API Integration

1. **Create API module** in `src/core/embedding/ollama_api.rs`:
   - Implement functions for checking Ollama API status
   - Implement embedding generation with proper error handling
   - Support timeout and chunking for larger texts

2. **API Design**:

   ```rust
   pub struct OllamaApiClient {
       base_url: String,
       timeout: Duration,
   }

   impl OllamaApiClient {
       pub fn new(base_url: &str) -> Self;
       pub fn check_status(&self) -> Result<bool>;
       pub fn generate_embedding(&self, text: &str, model: &str) -> Result<Vec<f32>>;
   }
   ```

#### Phase 3: Ingest Chunked Implementation

1. **Extend existing ingest module**:
   - Add character-based chunking to `src/ingest/ingest.rs`
   - Support for JSON field extraction and processing
   - Progress logging and status reporting

2. **Command-line interface extensions**:

   ```rust
   // CLI Options for chunked ingestion
   pub struct ChunkedIngestOptions {
       source_file: PathBuf,
       chunk_size: usize,
       model: String,
       log_file: Option<PathBuf>,
   }
   ```

#### Phase 4: CLI Enhancements

1. **Unified CLI interface** in `src/main.rs`:

   ```rust
   fn main() {
       let matches = clap::Command::new("harald")
           .subcommand(
               clap::Command::new("chunk")
                   .about("Chunk text using various strategies")
                   // options
           )
           .subcommand(
               clap::Command::new("ingest")
                   .about("Ingest data into the vector database")
                   // options
           )
           // other subcommands
           .get_matches();

       // handle commands
   }
   ```

### Implementation Roadmap

1. **Week 1**: Implement text chunking module
   - ✅ Create chunking module in src/utils/chunking.rs
   - ✅ Create binary wrapper in src/utils/chunker_bin.rs
   - ✅ Update build configuration in Cargo.toml
   - ✅ Create compatibility wrapper scripts

2. **Week 2**: Implement Ollama API client
   - ✅ Create Ollama API client module in src/core/embedding/ollama_api.rs
   - ⏳ Implement chunking-aware embedding generation
   - ⏳ Add proper error handling and logging
   - ⏳ Create test cases for API client

3. **Week 3**: Extend ingest module with chunked ingestion
   - ⏳ Integrate text chunking with ingest process
   - ⏳ Implement character-based chunking for large fields
   - ⏳ Support semantic chunking for description fields
   - ⏳ Add progress reporting and better error messages

4. **Week 4**: Create unified CLI and compatibility wrappers
   - ⏳ Design comprehensive CLI interface
   - ⏳ Implement subcommands (ingest, query, chunk, etc.)
   - ⏳ Create compatibility wrappers for all scripts
   - ⏳ Update documentation

### Current Status (Updated)

✅ **Successfully migrated ingest_chunked.sh to Rust**

- Created complete Rust implementation with character data processing
- Implemented async embedding generation with OllamaApiClient
- Added comprehensive CLI interface with clap
- Compiled and tested successfully with proper error handling
- Original script backed up as `ingest_chunked.sh.legacy`
- New implementation at `src/ingest/chunked_ingest.rs`
- **Status**: Ready for shell script removal after testing protocol

✅ **Successfully migrated format_json.sh to Rust**

- Created comprehensive JSON formatting utility with registry management
- Implemented file validation and interactive registration features
- Added support for prettier and jq integration
- Compiled and tested successfully with full CLI interface
- Original script backed up as `format_json.sh.legacy`
- New implementation at `src/utils/json_tools/format_json.rs`
- **Co-location completed**: Moved documentation to
  `src/utils/json_tools/JSON-TOOLS.md`
- **Shell script removed**: Direct Rust binary usage, no wrapper needed
- **Status**: Migration complete, ready for testing protocol

✅ **Successfully migrated test_embedding_size.sh to Rust**

- Implemented as part of a comprehensive embedding_tool CLI
- Added test-sizes command with flexible configuration
- Created detailed logging and reporting functionality
- Original script backed up as `test_embedding_size.sh.legacy`
- New implementation at `src/core/embedding/embedding_bin.rs`

✅ **Created a robust Ollama API client module**

- Implemented check_status functionality
- Added embedding generation with timeout handling
- Added support for chunked embeddings for long text
- Implemented proper error handling and reporting
- New implementation at `src/core/embedding/ollama_api.rs`

✅ **Created wrapper scripts for backwards compatibility**

- `text_chunker.sh` - Now a wrapper around the Rust implementation
- `test_embedding_size.sh` - Now a wrapper around the Rust implementation
- Automatic Rust binary rebuilding when source changes
- Error handling and fallback mechanisms

📝 **Created cleanup documentation**

- Migration tracking document at `docs/DEVELOPMENT-PRINCIPLES.md` (Historical
  details: `docs/migration/archive/`)
- Implementation timeline and roadmap
- Testing and verification strategies

✅ **Successfully migrated validate_json_schema.sh to Rust**

- Created comprehensive JSON schema validation utility with generate/validate
  commands
- Implemented store-specific validation and schema generation features
- Added support for both ajv and fallback validation methods
- Compiled and tested successfully with full CLI interface using subcommands
- Original script backed up as `validate_json_schema.sh.legacy`
- New implementation at `src/utils/json_tools/validate_json_schema.rs`
- **Co-location completed**: Implementation co-located with related JSON tools
- **Shell script removed**: Direct Rust binary usage, no wrapper needed
- **Status**: Migration complete, ready for testing protocol

### Scripts Pending Migration

The following scripts are still pending migration to Rust:

1. 🔄 `ingest_marvelai.sh` - Marvel AI data ingestion (Medium Priority)
2. 🔄 `test_basic_embedding.sh` - Basic embedding testing (Medium Priority)
3. 🔄 `ingest.sh` - Main ingestion script (High Priority)
4. 🔄 `test_text_chunker.sh` - Tests for text chunking (Low Priority)
5. 🔄 `ingest_single_character.sh` - Single character ingestion (Medium
   Priority)

### Script Removal Protocol

For completed migrations, follow the cleanup process documented in
`docs/DEVELOPMENT-PRINCIPLES.md`: (Historical details:
`docs/migration/archive/SCRIPT-CLEANUP-PLAN.md`):

1. **Testing Phase**: Complete functional equivalence, performance, edge case,
   and integration testing
2. **Grace Period**: Add deprecation notices and allow 1 week for transition
3. **Removal Phase**: Remove shell scripts and update documentation
4. **Legacy Cleanup**: Remove .legacy files after 1 month grace period

### Testing Strategy

1. Create unit tests for each component
2. Create integration tests that compare output with existing shell scripts
3. Benchmark performance against shell script implementations

### Compatibility Considerations

During the transition period:

1. Maintain shell script wrappers that call the Rust implementations
2. Ensure consistent output formats and logging
3. Document migration details for users

```

```
````
