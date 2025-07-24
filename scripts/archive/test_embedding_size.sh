#!/bin/bash
# test_embedding_size.sh - Wrapper around the Rust implementation for testing embedding size limits
# This script replaces the original test_embedding_size.sh with a Rust implementation

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

# Parse command line arguments
SIZES=""
START_SIZE=10
END_SIZE=3000
STEP_SIZE=100
LOG_DIR="./logs"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --sizes)
      SIZES="$2"
      shift 2
      ;;
    --start)
      START_SIZE="$2"
      shift 2
      ;;
    --end)
      END_SIZE="$2"
      shift 2
      ;;
    --step)
      STEP_SIZE="$2"
      shift 2
      ;;
    --log-dir)
      LOG_DIR="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: $0 [--sizes 10,50,100,200] [--start 10] [--end 3000] [--step 100] [--log-dir ./logs]"
      exit 1
      ;;
  esac
done

# Build the command
CMD="$BINARY_PATH test-sizes --log-dir $LOG_DIR --start-size $START_SIZE --end-size $END_SIZE --step-size $STEP_SIZE"
if [ -n "$SIZES" ]; then
  CMD="$CMD --sizes $SIZES"
fi

# Execute the command
$CMD
