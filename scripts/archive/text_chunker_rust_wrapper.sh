#!/bin/bash
# text_chunker.sh - Wrapper around the Rust text_chunker implementation
# Maintains compatibility with existing scripts during the migration period
#
# Usage: Same as the original text_chunker.sh
# ./text_chunker.sh --size <max-size> "Your text to chunk"
# ./text_chunker.sh --char <target-size> "Your text to chunk"
# ./text_chunker.sh --semantic "Your text to chunk with. Multiple sentences. Will be split."
# ./text_chunker.sh --size <max-size> --file input.txt
# cat input.txt | ./text_chunker.sh --char <target-size>

set -e  # Exit on error

# Project paths
PROJECT_ROOT="/Users/bryanchasko/Code/HARALD"
RUST_DIR="$PROJECT_ROOT/src"
BINARY_PATH="$RUST_DIR/target/release/text_chunker"
LEGACY_PATH="$PROJECT_ROOT/scripts/text_chunker.sh.legacy"

# Check if the original text_chunker.sh exists and rename it if needed
if [ -f "$PROJECT_ROOT/scripts/text_chunker.sh" ] && [ ! -f "$LEGACY_PATH" ]; then
  echo "Creating backup of original text_chunker.sh as text_chunker.sh.legacy..."
  cp "$PROJECT_ROOT/scripts/text_chunker.sh" "$LEGACY_PATH"
  chmod +x "$LEGACY_PATH"
fi

# Function to ensure Rust binary is built
ensure_rust_binary() {
  if [ ! -f "$BINARY_PATH" ] || [ "$BINARY_PATH" -ot "$RUST_DIR/utils/chunking.rs" ]; then
    echo "🔨 Building text_chunker binary..."
    (cd "$RUST_DIR" && cargo build --release --features cli --bin text_chunker)
    
    if [ $? -ne 0 ]; then
      echo "❌ Failed to build text_chunker binary. Falling back to legacy implementation."
      if [ -f "$LEGACY_PATH" ]; then
        "$LEGACY_PATH" "$@"
        exit $?
      else
        echo "❌ Legacy implementation not found. Exiting."
        exit 1
      fi
    fi
    
    echo "✅ Successfully built text_chunker binary"
  fi
}

# Make sure the binary is built
ensure_rust_binary "$@"

# Call the Rust implementation with all arguments
"$BINARY_PATH" "$@"
