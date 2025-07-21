#!/bin/bash
# Ingests selected directories into HARALD's knowledge base
# This script focuses specifically on ingesting only the most relevant files

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

# Test embedding generation
echo "🔄 Testing embedding generation with harald-phi4 model..."
EMBED_TEST=$(curl -s -X POST http://localhost:11434/api/embeddings -d '{"model":"harald-phi4","prompt":"test"}')
if ! echo "$EMBED_TEST" | grep -q "embedding"; then
  echo "❌ Failed to generate embeddings with harald-phi4 model"
  echo "Response was: $EMBED_TEST"
  exit 1
fi

echo "✅ Ollama is running and responding to embedding requests"

# Clean previous data if needed
if [ -e "data/index.hnsw.data" ] || [ -e "data/repo.index" ]; then
  read -p "Previous index found. Delete and rebuild? (y/n) " -n 1 -r
  echo
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🧹 Removing previous index..."
    rm -f data/index.hnsw.* data/repo.*
  fi
fi

# Ensure data directory exists
mkdir -p data

# Only ingest the most important directories
echo "🧠 Ingesting AI entities individually to avoid timeout issues..."
cd /Users/bryanchasko/Code/HARALD/rust_ingest

# Process AI entity files one by one to avoid overloading Ollama
for file in /Users/bryanchasko/Code/HARALD/ai-entities/*.md; do
  filename=$(basename "$file")
  echo "  📄 Processing $filename..."
  cargo run --release -- ingest --root "$file" || echo "    ⚠️ Failed to process $filename"
  # Give Ollama a moment to recover between files
  sleep 2
done

# Process JSON files
for file in /Users/bryanchasko/Code/HARALD/ai-entities/*.json; do
  filename=$(basename "$file")
  echo "  � Processing $filename..."
  cargo run --release -- ingest --root "$file" || echo "    ⚠️ Failed to process $filename"
  sleep 2
done

echo "📚 Ingesting personality archetypes..."
for file in /Users/bryanchasko/Code/HARALD/personality-archetypes/*.{md,json}; do
  if [ -f "$file" ]; then
    filename=$(basename "$file")
    echo "  📄 Processing $filename..."
    cargo run --release -- ingest --root "$file" || echo "    ⚠️ Failed to process $filename"
    sleep 2
  fi
done

echo "✅ Ingestion complete!"
echo "The index is now available at: $(pwd)/../data/index.hnsw.data"
