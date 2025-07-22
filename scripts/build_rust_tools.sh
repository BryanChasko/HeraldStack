#!/bin/bash
# build_rust_tools.sh - Ensures all Rust binaries are built and up to date

set -e

PROJECT_ROOT="/Users/bryanchasko/Code/HARALD"
RUST_DIR="$PROJECT_ROOT/src"

echo "🔨 Building HARALD Rust tools..."

cd "$RUST_DIR"

# Build all binaries with CLI features
cargo build --bins --features cli

echo "✅ All Rust tools built successfully!"
echo ""
echo "Available tools:"
echo "  - $RUST_DIR/target/debug/harald_ingest (main ingest tool)"
echo "  - $RUST_DIR/target/debug/text_chunker (text chunking utility)"  
echo "  - $RUST_DIR/target/debug/embedding_tool (embedding utilities)"
echo "  - $RUST_DIR/target/debug/ingest_chunked (character data ingestion)"
echo ""
echo "💡 You can run these directly or add $RUST_DIR/target/debug to your PATH"
