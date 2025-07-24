#!/bin/bash
# embedding_tool.sh - Wrapper around the Rust embedding_tool implementation
# Provides embedding generation functionality using the Ollama API
#
# Usage:
# ./embedding_tool.sh check-status                            - Check if the Ollama API is available
# ./embedding_tool.sh generate "Your text"                    - Generate an embedding for the text
# ./embedding_tool.sh generate --chunked "Your long text"     - Generate chunked embeddings for long text
# ./embedding_tool.sh generate-file path/to/file.txt          - Generate embeddings for a file

set -e  # Exit on error

# Project paths
PROJECT_ROOT="/Users/bryanchasko/Code/HARALD"
RUST_DIR="$PROJECT_ROOT/src"
BINARY_PATH="$RUST_DIR/target/release/embedding_tool"

# Function to ensure Rust binary is built
ensure_rust_binary() {
  if [ ! -f "$BINARY_PATH" ] || [ "$BINARY_PATH" -ot "$RUST_DIR/core/embedding/embedding_bin.rs" ]; then
    echo "🔨 Building embedding_tool binary..."
    (cd "$RUST_DIR" && cargo build --release --features cli --bin embedding_tool)
    
    if [ $? -ne 0 ]; then
      echo "❌ Failed to build embedding_tool binary."
      exit 1
    fi
    
    echo "✅ Successfully built embedding_tool binary"
  fi
}

# Check if Ollama is running
ensure_ollama_running() {
  if ! pgrep -x "ollama" > /dev/null; then
    echo "🚀 Starting Ollama service..."
    ollama serve &
    sleep 5  # Give Ollama time to start
  else
    echo "✅ Ollama service is running"
  fi
}

# Make sure the binary is built
ensure_rust_binary

# Make sure Ollama is running
ensure_ollama_running

# Call the Rust implementation with all arguments
"$BINARY_PATH" "$@"
