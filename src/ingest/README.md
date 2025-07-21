# HARALD Ingest Module

This directory contains the Rust implementation of the HARALD ingestion pipeline,
migrated from the previous `rust_ingest` directory as part of our organizational
improvements.

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

# Run ingestion
cargo run -- ingest

# Run query
cargo run -- query "your query here"
```

## Migration Notes

This code was migrated from the original `rust_ingest` directory as part of the
project reorganization. The code maintains the same functionality but with an
updated module structure that better fits our overall architecture.

## Integration Points

- Outputs vector data to the `data/` directory
- Reads from project directories based on configuration
- Provides both a library API and command-line interface
