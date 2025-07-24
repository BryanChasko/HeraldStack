#!/bin/bash
# Queries HARALD's knowledge base

set -e  # Exit on error

# Ensure Ollama is running
if ! pgrep -x "ollama" > /dev/null; then
  echo "🚀 Starting Ollama service..."
  ollama serve &
  sleep 5  # Give Ollama time to start
else
  echo "✅ Ollama service is running"
fi

# Check for required models
if ! ollama list | grep -q "harald-phi4"; then
  echo "❌ harald-phi4 model not found. Please install it with 'ollama pull harald-phi4'"
  exit 1
fi

# Check if an index exists
if [ ! -f "/Users/bryanchasko/Code/HARALD/data/index.hnsw.data" ]; then
  echo "❌ No index found. Run ./scripts/ingest.sh first."
  exit 1
fi

# Accept the query as command arguments
if [ "$#" -lt 1 ]; then
  echo "❌ No query provided."
  echo "Usage: ./scripts/query.sh \"Your question here?\""
  exit 1
fi

# Combine all arguments into a single query
QUERY="$*"

# Log the query with timestamp
mkdir -p /Users/bryanchasko/Code/HARALD/logs
echo "$(date '+%Y-%m-%d %H:%M:%S') - $QUERY" >> /Users/bryanchasko/Code/HARALD/logs/queries.log

echo "🔍 Querying HARALD: $QUERY"
echo "--------------------------------------"
cd /Users/bryanchasko/Code/HARALD/rust_ingest
cargo run --release -- query --index ../data --query "$QUERY"
