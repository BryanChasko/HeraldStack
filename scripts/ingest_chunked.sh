#!/bin/bash
# Modified ingestion script for Marvel character data
# Uses smaller text chunks to avoid embedding API timeouts

set -e  # Exit on# Use our text_chunker.sh utility for optimal chunking
CHUNKER="./scripts/text_chunker.sh"

# Extract character data first
CHARACTER_DATA=$(jq -r '.[0]' "$SOURCE_FILE")

# Extract chunks from each field - with character-based chunking where appropriate
CHUNKS=()

# Add character name - use as-is since it's typically short
CHAR_NAME=$(echo "$CHARACTER_DATA" | jq -r '.character_name')
CHUNKS+=("Name: $CHAR_NAME")

# Add first appearance - use as-is since it's typically short
FIRST_APPEARANCE=$(echo "$CHARACTER_DATA" | jq -r '.first_appearance')
CHUNKS+=("First Appearance: $FIRST_APPEARANCE")

# Process affiliations with character-based chunking if long
AFFILIATIONS=$(echo "$CHARACTER_DATA" | jq -r '.affiliations | join(", ")')
if [ ${#AFFILIATIONS} -gt 250 ]; then
  echo "⚠️  Long affiliations text (${#AFFILIATIONS} chars), using character-based chunking"
  readarray -t AFF_CHUNKS < <(echo "$AFFILIATIONS" | $CHUNKER --char 250)
  for chunk in "${AFF_CHUNKS[@]}"; do
    CHUNKS+=("Affiliations: $chunk")
  done
else
  CHUNKS+=("Affiliations: $AFFILIATIONS")
fi

# Process attributes with character-based chunking if long
ATTRIBUTES=$(echo "$CHARACTER_DATA" | jq -r '.core_attributes | join(", ")')
if [ ${#ATTRIBUTES} -gt 250 ]; then
  echo "⚠️  Long attributes text (${#ATTRIBUTES} chars), using character-based chunking"
  readarray -t ATTR_CHUNKS < <(echo "$ATTRIBUTES" | $CHUNKER --char 250)
  for chunk in "${ATTR_CHUNKS[@]}"; do
    CHUNKS+=("Attributes: $chunk")
  done
else
  CHUNKS+=("Attributes: $ATTRIBUTES")
fi

# Process themes with character-based chunking
THEMES=$(echo "$CHARACTER_DATA" | jq -r '.inspirational_themes | join(", ")')
if [ ${#THEMES} -gt 250 ]; then
  echo "⚠️  Long themes text (${#THEMES} chars), using character-based chunking"
  readarray -t THEME_CHUNKS < <(echo "$THEMES" | $CHUNKER --char 250)
  for chunk in "${THEME_CHUNKS[@]}"; do
    CHUNKS+=("Themes: $chunk")
  done
else
  CHUNKS+=("Themes: $THEMES")
fi

# Process traits with character-based chunking
TRAITS=$(echo "$CHARACTER_DATA" | jq -r '.traits | join(", ")')
if [ ${#TRAITS} -gt 250 ]; then
  echo "⚠️  Long traits text (${#TRAITS} chars), using character-based chunking"
  readarray -t TRAIT_CHUNKS < <(echo "$TRAITS" | $CHUNKER --char 250)
  for chunk in "${TRAIT_CHUNKS[@]}"; do
    CHUNKS+=("Traits: $chunk")
  done
else
  CHUNKS+=("Traits: $TRAITS")
fi

# Add AI alignment - typically short
AI_ALIGNMENT=$(echo "$CHARACTER_DATA" | jq -r '.ai_alignment')
CHUNKS+=("AI Alignment: $AI_ALIGNMENT")

# For any description field, use semantic chunking
if echo "$CHARACTER_DATA" | jq -e '.description' > /dev/null 2>&1; then
  DESCRIPTION=$(echo "$CHARACTER_DATA" | jq -r '.description')
  if [ -n "$DESCRIPTION" ]; then
    echo "📝 Processing description field with semantic chunking"
    readarray -t DESC_CHUNKS < <(echo "$DESCRIPTION" | $CHUNKER --semantic)
    for chunk in "${DESC_CHUNKS[@]}"; do
      CHUNKS+=("Description: $chunk")
    done
  fi
fition to display timestamp and measure execution time
START_TIME=$(date +%s)
display_time() {
  local current_time=$(date +"%Y-%m-%d %H:%M:%S")
  echo "⏱️  $current_time | $1"
}

# Function to calculate elapsed time
elapsed_time() {
  local current_time=$(date +%s)
  local elapsed=$((current_time - START_TIME))
  local mins=$((elapsed / 60))
  local secs=$((elapsed % 60))
  echo "${mins}m ${secs}s"
}

# Function to generate embeddings with reliable size limits
generate_embedding() {
  local text="$1"
  local model="$2"
  local log_file="$3"
  local max_chunk_size=250  # Characters per chunk, based on our testing
  local timeout=30
  local success=false
  
  echo "🔄 Generating embedding for text (${#text} chars)" | tee -a "$log_file"
  
  # If text is too large, warn but proceed (we'll chunk it later)
  if [ ${#text} -gt $max_chunk_size ]; then
    echo "⚠️  Text exceeds recommended size of $max_chunk_size chars" | tee -a "$log_file"
    echo "   Will process in smaller chunks" | tee -a "$log_file"
  fi
  
  # Make sure text is under the size limit
  local request_text="${text:0:$max_chunk_size}"
  local request='{"model":"'$model'","prompt":"'$request_text'"}'
  
  echo "📦 Request size: $(echo "$request" | wc -c) bytes" | tee -a "$log_file"
  
  local start_time=$(date +%s)
  local temp_file=$(mktemp)
  
  echo "⏱️  Using timeout: ${timeout}s" | tee -a "$log_file"
  curl -s -m "$timeout" \
       -X POST http://localhost:11434/api/embeddings \
       -H "Content-Type: application/json" \
       -d "$request" \
       -o "$temp_file"
  
  local curl_status=$?
  local end_time=$(date +%s)
  local elapsed=$((end_time - start_time))
  
  if [ $curl_status -eq 28 ]; then
    echo "⚠️  Request timed out after ${timeout}s" | tee -a "$log_file"
    success=false
  elif [ -f "$temp_file" ] && grep -q "embedding" "$temp_file"; then
    echo "✅ Success - Embedding generated in ${elapsed}s" | tee -a "$log_file"
    local vector_size=$(grep -o '"embedding":\[[^]]*\]' "$temp_file" | tr -cd ',' | wc -c)
    echo "   Vector dimensions: $((vector_size + 1))" | tee -a "$log_file"
    success=true
  else
    echo "❌ Failed - No embedding vector returned" | tee -a "$log_file"
    if [ -f "$temp_file" ]; then
      echo "   Error: $(cat "$temp_file")" | tee -a "$log_file"
    fi
    success=false
  fi
  
  rm -f "$temp_file"
  return $([ "$success" = true ] && echo 0 || echo 1)
}

# Display start banner
echo "=================================================="
echo "🚀 HARALD OPTIMIZED INGEST PROCESS"
echo "🔍 Processing MarvelAIs.json with size-optimized chunks"
echo "🕒 Started at $(date +"%Y-%m-%d %H:%M:%S")"
echo "=================================================="

# Check Ollama running status and determine if it's the GUI app or standalone server
OLLAMA_PROCESS=$(pgrep -x "ollama" || echo "")
OLLAMA_GUI_APP=$(pgrep -f "Ollama.app" || echo "")

if [ -z "$OLLAMA_PROCESS" ]; then
  echo "🚀 Starting Ollama standalone server..."
  ollama serve &
  sleep 5  # Give Ollama time to start
  echo "✅ Ollama standalone server started"
elif [ -n "$OLLAMA_GUI_APP" ]; then
  echo "⚠️  Warning: Ollama is running through the GUI application"
  echo "   For optimal embedding performance with larger chunks (500-600 chars):"
  echo "   1. Close the Ollama GUI application"
  echo "   2. Run 'ollama serve' from the command line"
  echo "   3. Re-run this script"
  
  read -p "Continue with potentially limited performance? (y/n) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Aborted. Please restart with the standalone server for better performance."
    exit 1
  fi
else
  echo "✅ Ollama standalone server is running"
  echo "   This configuration allows for optimal embedding performance"
fi

# Verify API is responding
echo "🔍 Checking Ollama API status..."
if curl -s -o /dev/null -w "%{http_code}" http://localhost:11434/api/version | grep -q "200"; then
  echo "✅ Ollama API is available"
else
  echo "❌ Ollama API is not responding. Please check if the service is running properly."
  exit 1
fi

# Create logs directory
LOGS_DIR="./logs"
mkdir -p "$LOGS_DIR"
LOG_FILE="$LOGS_DIR/ingest_log_$(date +%Y%m%d_%H%M%S).log"
echo "📝 Logging to: $LOG_FILE"

# Define the source file (our single character test file)
SOURCE_FILE="/Users/bryanchasko/Code/HARALD/tests/fixtures/test_single_character.json"
display_time "Starting to process: $SOURCE_FILE"
echo "📄 Processing single character test file"

# Verify the API works with a minimal test
echo "🧪 Testing API with minimal request..."
if generate_embedding "test" "harald-phi4" "$LOG_FILE"; then
  echo "✅ API working correctly"
else
  echo "❌ API test failed"
  echo "Please check Ollama service status"
  exit 1
fi

# Extract character information
CHAR_NAME=$(jq -r '.[0].character_name' "$SOURCE_FILE")
echo "📄 Processing character: $CHAR_NAME"

# Use our text_chunker.sh utility for optimal chunking
CHUNKER="./scripts/text_chunker.sh"
display_time "Extracting character data using character-based chunking"

# Extract character data first
CHARACTER_DATA=$(jq -r '.[0]' "$SOURCE_FILE")
CHAR_NAME=$(echo "$CHARACTER_DATA" | jq -r '.character_name')

echo "🔍 Processing character: $CHAR_NAME"
echo "🔄 Using character-based chunking for large fields"

# Extract chunks from each field - with character-based chunking where appropriate
CHUNKS=()

# Add character name - use as-is since it's typically short
CHUNKS+=("Name: $CHAR_NAME")

# Add first appearance - use as-is since it's typically short
FIRST_APPEARANCE=$(echo "$CHARACTER_DATA" | jq -r '.first_appearance')
CHUNKS+=("First Appearance: $FIRST_APPEARANCE")

# Process affiliations with character-based chunking if long
AFFILIATIONS=$(echo "$CHARACTER_DATA" | jq -r '.affiliations | join(", ")')
if [ ${#AFFILIATIONS} -gt 250 ]; then
  echo "⚠️  Long affiliations text (${#AFFILIATIONS} chars), using character-based chunking"
  while IFS= read -r chunk; do
    CHUNKS+=("Affiliations: $chunk")
  done < <(echo "$AFFILIATIONS" | $CHUNKER --char 250)
else
  CHUNKS+=("Affiliations: $AFFILIATIONS")
fi

# Process attributes with character-based chunking if long
ATTRIBUTES=$(echo "$CHARACTER_DATA" | jq -r '.core_attributes | join(", ")')
if [ ${#ATTRIBUTES} -gt 250 ]; then
  echo "⚠️  Long attributes text (${#ATTRIBUTES} chars), using character-based chunking"
  while IFS= read -r chunk; do
    CHUNKS+=("Attributes: $chunk")
  done < <(echo "$ATTRIBUTES" | $CHUNKER --char 250)
else
  CHUNKS+=("Attributes: $ATTRIBUTES")
fi

# Process themes with character-based chunking
THEMES=$(echo "$CHARACTER_DATA" | jq -r '.inspirational_themes | join(", ")')
if [ ${#THEMES} -gt 250 ]; then
  echo "⚠️  Long themes text (${#THEMES} chars), using character-based chunking"
  while IFS= read -r chunk; do
    CHUNKS+=("Themes: $chunk")
  done < <(echo "$THEMES" | $CHUNKER --char 250)
else
  CHUNKS+=("Themes: $THEMES")
fi

# Process traits with character-based chunking
TRAITS=$(echo "$CHARACTER_DATA" | jq -r '.traits | join(", ")')
if [ ${#TRAITS} -gt 250 ]; then
  echo "⚠️  Long traits text (${#TRAITS} chars), using character-based chunking"
  while IFS= read -r chunk; do
    CHUNKS+=("Traits: $chunk")
  done < <(echo "$TRAITS" | $CHUNKER --char 250)
else
  CHUNKS+=("Traits: $TRAITS")
fi

# Add AI alignment - typically short
AI_ALIGNMENT=$(echo "$CHARACTER_DATA" | jq -r '.ai_alignment')
CHUNKS+=("AI Alignment: $AI_ALIGNMENT")

# For any description field, use semantic chunking
if echo "$CHARACTER_DATA" | jq -e '.description' > /dev/null 2>&1; then
  DESCRIPTION=$(echo "$CHARACTER_DATA" | jq -r '.description')
  if [ -n "$DESCRIPTION" ] && [ "$DESCRIPTION" != "null" ]; then
    echo "📝 Processing description field with semantic chunking"
    while IFS= read -r chunk; do
      CHUNKS+=("Description: $chunk")
    done < <(echo "$DESCRIPTION" | $CHUNKER --semantic)
  fi
fi

# Process each chunk
SUCCESSFUL_CHUNKS=0
FAILED_CHUNKS=0

echo "🔄 Processing ${#CHUNKS[@]} character data chunks..."
for i in "${!CHUNKS[@]}"; do
  chunk="${CHUNKS[$i]}"
  echo "--------------------------------------------"
  echo "🔄 Processing chunk $((i+1))/${#CHUNKS[@]}: ${chunk:0:30}..."
  
  if generate_embedding "$chunk" "harald-phi4" "$LOG_FILE"; then
    SUCCESSFUL_CHUNKS=$((SUCCESSFUL_CHUNKS + 1))
  else
    FAILED_CHUNKS=$((FAILED_CHUNKS + 1))
  fi
  
  # Small delay between chunks
  sleep 2
done

# Print summary
TOTAL_TIME=$(elapsed_time)
echo "=================================================="
echo "✅ HARALD CHUNKED INGEST COMPLETE"
echo "📊 SUMMARY:"
echo "   Character: $CHAR_NAME"
echo "   Successful chunks: $SUCCESSFUL_CHUNKS"
echo "   Failed chunks: $FAILED_CHUNKS"
echo "   Total chunks: ${#CHUNKS[@]}"
echo "   Success rate: $(( (SUCCESSFUL_CHUNKS * 100) / ${#CHUNKS[@]} ))%"
echo "   Total execution time: $TOTAL_TIME"
echo "------------------------------------------------"
echo "📋 Recommendations:"
echo "   1. Keep chunks under 250-300 characters for reliable embedding"
echo "   2. Process character data as separate attributes"
echo "   3. For larger chunks (500-600 chars), use standalone Ollama server"
echo "   4. The character limit may be system resource-related, not API-related"
echo "   5. For best performance, close the Ollama GUI app and run 'ollama serve'"
echo "=================================================="
echo "🕒 Finished at $(date +"%Y-%m-%d %H:%M:%S")"
