#!/bin/bash
# Script to diagnose embedding issues with Ollama API
# This focuses on finding the size limits for embedding requests

set -e  # Exit on error

# Display start banner
echo "=================================================="
echo "🔍 HARALD EMBEDDING SIZE TEST"
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
LOG_FILE="./logs/embedding_size_test_$(date +%Y%m%d_%H%M%S).log"
echo "📝 Logging to: $LOG_FILE"

# Define a function to test embedding with increasing text sizes
test_embedding_size() {
  local text_size=$1
  local timeout=$2
  local test_text=$(printf '%*s' "$text_size" | tr ' ' 'X')
  
  echo "🧪 Testing embedding with text size: $text_size chars (timeout: ${timeout}s)" | tee -a "$LOG_FILE"
  
  local start_time=$(date +%s)
  local request='{"model":"harald-phi4","prompt":"'"$test_text"'"}'
  
  # Make the request
  curl -s -m "$timeout" \
       -X POST http://localhost:11434/api/embeddings \
       -H "Content-Type: application/json" \
       -d "$request" \
       -o "$LOG_FILE.$text_size" 2>> "$LOG_FILE"
  
  local curl_status=$?
  local end_time=$(date +%s)
  local elapsed=$((end_time - start_time))
  
  if [ $curl_status -eq 28 ]; then
    echo "⚠️  Request timed out after ${timeout}s" | tee -a "$LOG_FILE"
    echo "   Text size $text_size is too large for the embedding API" | tee -a "$LOG_FILE"
    return 1
  elif [ -f "$LOG_FILE.$text_size" ] && grep -q "embedding" "$LOG_FILE.$text_size"; then
    echo "✅ Success - Embedding generated in ${elapsed}s" | tee -a "$LOG_FILE"
    local vector_size=$(grep -o '"embedding":\[[^]]*\]' "$LOG_FILE.$text_size" | tr -cd ',' | wc -c)
    echo "   Vector dimensions: $((vector_size + 1))" | tee -a "$LOG_FILE"
    return 0
  else
    echo "❌ Failed - No embedding vector returned" | tee -a "$LOG_FILE"
    if [ -f "$LOG_FILE.$text_size" ]; then
      echo "   Error: $(cat "$LOG_FILE.$text_size")" | tee -a "$LOG_FILE"
    fi
    return 1
  fi
}

# Start with small text and increase size until failure
echo "🔄 Testing with increasing text sizes to find the limit..."
sizes=(10 50 100 200 300 400 500 1000 2000 3000)
timeouts=(15 15 15 30 30 45 60 90 120 180)
max_success_size=0

for i in "${!sizes[@]}"; do
  size=${sizes[$i]}
  timeout=${timeouts[$i]}
  echo "--------------------------------------------"
  if test_embedding_size "$size" "$timeout"; then
    max_success_size=$size
  else
    echo "🛑 Stopping tests at size $size"
    break
  fi
  
  # Small delay between tests
  sleep 2
done

# Print summary
echo "=================================================="
echo "✅ HARALD EMBEDDING SIZE TEST COMPLETE"
echo "📊 Results:"
echo "   Maximum successful text size: $max_success_size characters"
if [ $max_success_size -gt 0 ]; then
  echo "   Recommendation: Keep chunks under $max_success_size characters for reliable embedding"
else
  echo "   Warning: Even the smallest test failed. Check Ollama API health."
fi
echo "🔍 Logs saved to: $LOG_FILE.*"
echo "=================================================="
echo "🕒 Finished at $(date +"%Y-%m-%d %H:%M:%S")"
