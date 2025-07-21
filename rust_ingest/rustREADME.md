# rust_ingest

In-repo Rust utility that (1) embeds .md / .json files with Ollama, (2) writes a
HNSW index, (3) queries the index for RAG.

## 👋 What This Tool Does

This tool creates a semantic search index for your project documentation. It:

1. **Reads** your Markdown and JSON/JSONL files
2. **Converts** text into numerical vector embeddings that capture meaning
3. **Indexes** these vectors for efficient similarity search
4. **Retrieves** relevant documents when you ask questions

This enables natural language search across your codebase and documentation.

### JSONL Support and Embedding Limits

The system supports [JSON Lines (JSONL)](http://jsonlines.org/examples) format
for more efficient embedding processing:

- **What is JSONL?** A newline-delimited JSON format where each line is a valid
  JSON object
- **Why use it?** Better for streaming, chunking, and processing large datasets
- **Performance benefit:** Reduces timeout issues with embedding APIs by
  allowing record-by-record processing
- **Conversion:** Use `jq -c '.[]' input.json > output.jsonl` to convert JSON
  arrays to JSONL

**Important:** Through testing, we've identified specific limits with the Ollama
embedding API:

- Maximum text length: ~300 characters (requests with 400+ characters timeout)
- Recommended chunk size: Keep chunks under 250 characters for reliable
  processing
- Vector dimensions: harald-phi4 model produces 3072-dimensional embeddings

Note: These limits may be related to system resources rather than API
constraints. For processing larger chunks (500-600 chars), try running the
standalone Ollama server:

```bash
# Close the Ollama GUI app first, then run:
ollama serve
```

See our
[detailed documentation on Ollama embedding limits](../docs/vector-search/ollama-embedding-limits.md).

## 🔎 Quick Overview

**Purpose:** Fast, meaning-based search across project documentation

**How it works:**

- Creates vector embeddings that capture semantic meaning
- Builds a searchable HNSW graph index
- Retrieves contextually relevant documents for queries
- Provides focused answers based on retrieved information

**Key benefits:**

- Context-aware search beyond simple keyword matching
- Single binary with minimal external dependencies
- Designed for projects with extensive documentation

## ⚡ Status

| Date       | Milestone                                                     |
| ---------- | ------------------------------------------------------------- |
| 2025-07-xx | PoC compiles under Rust 1.77, hnsw_rs 0.3.2                   |
| TODO       | Replace blocking `reqwest` with connection-pooled async calls |
| TODO       | Bench vs Python FAISS ingest                                  |

## 🏃‍♂️ How to Use / Running the Project

We've created convenient scripts for ingestion and querying:

```bash
# Step 1: Build the search index for relevant directories
# This script will:
# - Check if Ollama is running and test embedding generation
# - Clean up previous indices if requested
# - Ingest only relevant directories (ai-entities, personality-archetypes)
./scripts/ingest.sh

# Step 2: Query HARALD with your questions
./scripts/query.sh "Harald, who are the other entities you know?"
```

For manual control, you can run the commands directly:

```bash
# For selective ingestion (recommended):
cd rust_ingest
cargo run --release -- ingest --root /Users/bryanchasko/Code/HARALD/ai-entities

# For querying:
cargo run --release -- query "Your question here?"
```

## Unit Testing Standards

| Area            | Approach                             |
| --------------- | ------------------------------------ |
| Code Coverage   | Target 80%+ of core functionality    |
| Mocking         | Use `mockall` for API dependencies   |
| Async Testing   | `tokio::test` for async functions    |
| File Operations | Temporary directories via `tempfile` |

### Key Test Types

### Component Tests

Test individual functions in isolation:

```rust
#[test]
fn test_truncate_content() {
    let content = "a".repeat(1000);
    assert_eq!(truncate_content(&content, 500).len(), 500);
}
```

### Integration Tests

Integration tests ensure that multiple components work together as expected.
These tests simulate real-world usage scenarios and validate the overall
behavior of the application.

| Area                | Approach                                                   |
| ------------------- | ---------------------------------------------------------- |
| End-to-End Testing  | Use `cargo test` with realistic input/output data          |
| Performance Testing | Benchmark ingest and query operations using `criterion`    |
| Error Handling      | Validate edge cases and failure scenarios                  |
| Cross-Platform      | Test on AWS Lambda, Linux, macOS, and Windows environments |

## Mock-Based Tests

Test without external dependencies.

```rust
#[test]
fn test_build_context_from_results() {
    // Create mock search results
    let search_results = vec![
        Neighbour { distance: 0.1, d_id: 0 },
        Neighbour { distance: 0.2, d_id: 1 }
    ];

    // Create temp files with test content
    let temp_dir = std::env::temp_dir();
    let file1 = temp_dir.join("test1.md");
    let file2 = temp_dir.join("test2.md");

    std::fs::write(&file1, "Test content one").unwrap();
    std::fs::write(&file2, "Test content two").unwrap();

    let metadata = vec![file1.clone(), file2.clone()];

    // Configure test parameters
    let config = QueryConfig {
        max_context_chars: 100,
        ..QueryConfig::default()
    };

    // Run function under test
    let result = build_context_from_results(
        &search_results,
        &metadata,
        &config
    );

    // Clean up
    let _ = std::fs::remove_file(&file1);
    let _ = std::fs::remove_file(&file2);

    // Assertions
    assert!(result.is_ok());
    let (context, files) = result.unwrap();
    assert_eq!(files.len(), 2);
    assert!(context.contains("Test content one"));
    assert!(context.contains("Test content two"));
}
```

### 3. Minimal Index Tests

Create small HNSW indices for fast verification without needing real embeddings:

```rust
/// Creates a tiny test HNSW index with sample vectors for testing
///
/// - 'static lifetime: Tells Rust this index can live for the entire program duration
/// - f32: Uses 32-bit floating point numbers (standard for ML vectors)
/// - DistanceType: Determines how similarity is calculated between vectors
fn create_test_index() -> Hnsw<'static, f32, DistanceType> {
    // Create a tiny test index with minimal configuration parameters
    let mut index = Hnsw::<'static, f32, DistanceType>::new(
        8,  // MAX_CONNECTIONS: How many neighbors each point connects to
        10, // EF_CONSTRUCTION: Controls index quality vs. build speed
        4,  // MAX_LAYER: Maximum layers in hierarchical structure
        10, // EF_SEARCH: Controls search accuracy vs. speed
        DistanceType::Cosine // Cosine similarity measure (common for semantic search)
    );

    // Add test vectors with their IDs
    // Vector dimensions should match what your embeddings will have
    index.insert((vec![0.1, 0.2, 0.3].as_slice(), 0)); // Document about "apples"
    index.insert((vec![0.5, 0.6, 0.7].as_slice(), 1)); // Document about "oranges"

    index // Return the populated index
}

// Example of a test using the minimal index
#[test]
fn test_simple_search() {
    let index = create_test_index();

    // Create a test query vector (more similar to second document)
    let query_vec = vec![0.4, 0.5, 0.6];

    // Perform search (find 1 nearest neighbor)
    let results = index.search(&query_vec.as_slice(), 1, 10);

    // Verify we got expected document
    assert_eq!(results[0].d_id, 1); // Should match "oranges" document
}
```

### 4. Integration Tests

End-to-end testing with fixture data (requires the `integration-tests` feature):

```rust
#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_ingest_and_query() {
    // Set up test directory with sample files
    // Run ingest command
    // Run query command
    // Verify results
}
```

## Implementation Plan

- ✅ Basic unit tests for utility functions
- ⏳ Mock-based tests for embedding operations
- ⏳ Small index creation and testing
- ⏳ Add `--sample` flag for quick test runs
- ⏳ Integration tests with mock embeddings

## Running Tests

1. Run all tests:

   ```bash
   cargo test
   ```

2. Run a specific test:

   ```bash
   cargo test test_build_context_from_results
   ```

3. Run integration tests:

   ```bash
   cargo test --features integration-tests
   ```

4. Run with output captured:

   ```bash
   cargo test -- --nocapture
   ```

## CI/CD Integration

Tests automatically run on:

- Pull request creation
- Push to `main` branch

## ❓ Common Questions

**Q: How do I update the index when files change?**  
A: Run the `ingest` command again to rebuild the index.

**Q: What file types are supported?**  
A: Currently Markdown (.md) and JSON (.json) files.

**Q: Can I customize what gets indexed?**  
A: Yes, by modifying the `SUPPORTED_EXTENSIONS` and `SKIP_DIRS` constants in the
code.

## 💡 History

2025-07-15 – Started by taking an existing Python script that used FAISS for
vector search, and rewrote it in Rust. The goal was to make it faster and easier
to deploy as a single, self-contained binary, without needing Python or extra
dependencies.

2025-07-17 – Switched to hnsw_rs, a Rust library for fast vector search using
Hierarchical Navigable Small World (HNSW) graphs. This change made the compiled
program ("binary") smaller and removed the need for BLAS (Basic Linear Algebra
Subprograms) libraries, which are external dependencies often used for
mathematical operations in other vector search tools.

2025-07-18 – Changed the embedding process to run asynchronously (so it doesn't
wait for each file to finish before starting the next). This made the process
about five times faster when tested on a MacBook with an Intel processor.
