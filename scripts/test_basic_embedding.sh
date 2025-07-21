#!/bin/bash
# Very simple embedding test with a single character from MarvelAIs.json
# This script generates embeddings for just one character to verify the pipeline

set -e  # Exit on error

# Display start banner
echo "=================================================="
echo "🧪 HARALD EMBEDDING TEST - SINGLE CHARACTER"
echo "🕒 Started at $(date +"%Y-%m-%d %H:%M:%S")"
echo "=================================================="

# Check if Ollama is running
if ! pgrep -x "ollama" > /dev/null; then
  echo "🚀 Starting Ollama service..."
  ollama serve &
  sleep 5
else
  echo "✅ Ollama service is running"
fi

# Create logs directory
mkdir -p "./logs"
LOG_FILE="./logs/single_character_test_$(date +%Y%m%d_%H%M%S).log"

# Define the source file (our single character test file)
SOURCE_FILE="/Users/bryanchasko/Code/HARALD/tests/fixtures/test_single_character.json"

# Extract character name
CHAR_NAME=$(jq -r '.[0].character_name' "$SOURCE_FILE")
echo "📄 Testing with character: $CHAR_NAME"

# Test a simple embedding request for the character name
echo "🔄 Simple test: Generating embedding for character name only"
SIMPLE_TEST='{"model":"harald-phi4","prompt":"'$CHAR_NAME'"}'
echo "   Request: $SIMPLE_TEST"

echo "🔄 Sending request to Ollama API..." | tee -a "$LOG_FILE"
curl -s -m 30 \
     -X POST http://localhost:11434/api/embeddings \
     -H "Content-Type: application/json" \
     -d "$SIMPLE_TEST" \
     -o "$LOG_FILE.simple"

# Check if we got a response with embedding vectors
if grep -q "embedding" "$LOG_FILE.simple"; then
  echo "✅ Simple test passed - Embedding vectors received!"
  EMBEDDING_DIMENSION=$(grep -o '"embedding":\[[^]]*\]' "$LOG_FILE.simple" | tr -cd ',' | wc -c)
  echo "   Vector dimensions: $((EMBEDDING_DIMENSION + 1))"
else
  echo "❌ Simple test failed - No embedding vectors in response"
  cat "$LOG_FILE.simple"
  exit 1
fi

# Now try with more character content - using a simpler approach
echo "🔄 Full test: Generating embedding for character details"

# Extract relevant fields as text
NAME=$(jq -r '.[0].character_name' "$SOURCE_FILE")
APPEAR=$(jq -r '.[0].first_appearance' "$SOURCE_FILE")
AFFILIATIONS=$(jq -r '.[0].affiliations | join(", ")' "$SOURCE_FILE")
ATTRIBUTES=$(jq -r '.[0].core_attributes | join(", ")' "$SOURCE_FILE")
THEMES=$(jq -r '.[0].inspirational_themes | join(", ")' "$SOURCE_FILE")
TRAITS=$(jq -r '.[0].traits | join(", ")' "$SOURCE_FILE")
ALIGNMENT=$(jq -r '.[0].ai_alignment' "$SOURCE_FILE")

# Create a text prompt
CHARACTER_TEXT="Character: $NAME | First Appearance: $APPEAR | Affiliations: $AFFILIATIONS | Attributes: $ATTRIBUTES | Themes: $THEMES | Traits: $TRAITS | AI Alignment: $ALIGNMENT"

FULL_TEST=$(printf '{"model":"harald-phi4","prompt":"%s"}' "$CHARACTER_TEXT")

echo "   Request content size: $(echo "$FULL_TEST" | wc -c) bytes"
echo "   Character data converted to text format"
echo "🔄 Sending full character data to Ollama API..." | tee -a "$LOG_FILE"

echo "⏱️  Using timeout: 90 seconds"
curl -s -m 90 \
     -X POST http://localhost:11434/api/embeddings \
     -H "Content-Type: application/json" \
     -d "$FULL_TEST" \
     -o "$LOG_FILE.full"

CURL_STATUS=$?

# Check if curl timed out
if [ $CURL_STATUS -eq 28 ]; then
  echo "⚠️  Request timed out after 90 seconds"
  echo "   This indicates the Ollama API is struggling with the request size"
  echo "   Consider using smaller chunks of text for embedding"
  FULL_TEST_SUCCESS=false
elif [ -f "$LOG_FILE.full" ] && grep -q "embedding" "$LOG_FILE.full"; then
  echo "✅ Full test passed - Embedding vectors received!"
  EMBEDDING_DIMENSION=$(grep -o '"embedding":\[[^]]*\]' "$LOG_FILE.full" | tr -cd ',' | wc -c)
  echo "   Vector dimensions: $((EMBEDDING_DIMENSION + 1))"
  FULL_TEST_SUCCESS=true
else
  echo "❌ Full test failed - No embedding vectors in response"
  if [ -f "$LOG_FILE.full" ]; then
    cat "$LOG_FILE.full"
  fi
  FULL_TEST_SUCCESS=false
fi

# Print summary
echo "=================================================="
echo "✅ HARALD EMBEDDING TEST COMPLETE"
echo "✅ Character name test: SUCCESS"
if [ "$FULL_TEST_SUCCESS" = true ]; then
  echo "✅ Full character test: SUCCESS"
else
  echo "❌ Full character test: FAILED"
  echo "   This indicates we need to work with smaller text chunks"
fi
echo "🔍 Logs saved to: $LOG_FILE.*"
echo "=================================================="
echo "🕒 Finished at $(date +"%Y-%m-%d %H:%M:%S")"
