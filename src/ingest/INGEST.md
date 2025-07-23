# HARALD Ingest Module

This directory contains the Rust implementation of the HARALD ingestion
pipeline, migrated from the previous `rust_ingest` directory as part of our
organizational improvements.

## Important Notes & Best Practices

- **Chunking Limits:** All text fields are chunked to ≤250 characters for
  reliable embedding. This is enforced in the code and recommended for API
  stability and resource management.
- **Error Handling:** The pipeline includes robust error handling and
  diagnostics. Common error messages (e.g., "No chunks were processed") are
  documented below.
- **Debug Output:** Enable debug output to see chunk sizes and counts. This
  helps diagnose chunking and embedding issues.
- **CLI Features:** Some binaries require specific features (e.g.,
  `--features="cli"`). Always check the documentation for required flags.
- **Input/Output Formats:** Ingestion expects JSONL input and outputs vector
  data to the `data/` directory. Validate your input and output files for
  correct structure.

## Troubleshooting

- **No chunks were processed:** This message means the input file was empty or
  incorrectly formatted. Check your JSONL file and ensure it contains valid
  character objects.
- **Network/API errors:** If embedding fails, check Ollama API status and logs.
  Retry logic is built-in for transient failures.
- **Debugging chunking:** Use the debug output to verify chunk sizes and counts.
  If chunking does not match expectations, review the chunking logic and input
  data.

## Key Components

## Key Components

- `main.rs` - CLI entry point for ingest and query operations
- `lib.rs` - Library exports for use in other modules
- `ingest.rs` - Core ingestion functionality
- `embed.rs` - Vector embedding generation
- `query.rs` - Query processing and retrieval

## Building and Running

From the `src` directory:

```bash
# Build the project
cargo build

# Run ingestion (with CLI features if required)
cargo run --bin single_character_ingest --features="cli"

# Run main ingest
cargo run -- ingest

# Run query
cargo run -- query "your query here"
```

## Debugging & Validation

- To enable debug output for chunking, run the test harness or ingest binary and
  review the printed chunk sizes/counts.
- Always validate your JSONL input before running ingestion. Use `jq` or similar
  tools to check structure.

## Migration Notes

## Migration Notes

This code was migrated from the original `rust_ingest` directory as part of the
project reorganization. The code maintains the same functionality but with an
updated module structure that better fits our overall architecture.

## Integration Points

- Outputs vector data to the `data/` directory
- Reads from project directories based on configuration
- Provides both a library API and command-line interface
- Expects JSONL input for ingestion; output files are validated and summarized
  in logs

## Common Pitfalls

- Forgetting to enable required CLI features (e.g., `--features="cli"`)
- Incorrectly formatted JSONL input (empty or missing fields)
- Not keeping chunk sizes ≤250 characters, leading to API errors
- Overlooking debug output for diagnostics
