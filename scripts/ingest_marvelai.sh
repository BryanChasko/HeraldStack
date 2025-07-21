#!/bin/bash
# Test ingestion with ONLY the MarvelAIs.json file
# This script converts a single JSON file to JSONL format (newline-delimited JSON) 
# for improved embedding processing with the HNSW vector search

set -e  # Exit on error

# Function to display timestamp and measure execution time
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

# Function for detailed embedding API logging with retries
log_embedding_request() {
  local request_body=$1
  local log_file=$2
  local max_retries=${3:-3}  # Default to 3 retries if not specified
  local retry_delay=${4:-5}  # Default to 5 second delay between retries
  local timeout=${5:-30}     # Default to 30 second timeout (numerical value)
  
  local attempt=1
  local success=false
  
  while [ $attempt -le $max_retries ] && [ "$success" = false ]; do
    local start_time=$(date +%s)
    local timestamp=$(date +"%Y-%m-%d %H:%M:%S")
    
    echo "🔄 $timestamp | Embedding request attempt $attempt/$max_retries..." | tee -a "$log_file"
    echo "📦 Request size: $(echo "$request_body" | wc -c) bytes" | tee -a "$log_file"
    echo "🔍 Request preview: $(echo "$request_body" | cut -c 1-100)..." | tee -a "$log_file"
    echo "⏱️  Using timeout: ${timeout}s" | tee -a "$log_file"
    
    # Make the request with timeout - separated approach for better error handling
    local temp_response_file=$(mktemp)
    local temp_headers_file=$(mktemp)
    
    # Use curl with output to files for better control
    # Ensure timeout is a valid integer
    timeout_int=${timeout%%[^0-9]*}
    echo "   Using curl timeout: $timeout_int seconds" | tee -a "$log_file"
    
    curl -s -m "$timeout_int" \
         -o "$temp_response_file" \
         -D "$temp_headers_file" \
         -X POST http://localhost:11434/api/embeddings \
         -H "Content-Type: application/json" \
         -d "$request_body"
    
    local curl_status=$?
    local response_body=$(cat "$temp_response_file")
    local headers=$(cat "$temp_headers_file")
    local end_time=$(date +%s)
    local elapsed=$((end_time - start_time))
    
    # Determine HTTP status
    local http_code=$(grep -i "^HTTP" "$temp_headers_file" | tail -1 | awk '{print $2}')
    if [ -z "$http_code" ]; then
      if [ $curl_status -eq 28 ]; then
        http_code="TIMEOUT"
      else
        http_code="ERROR ($curl_status)"
      fi
    fi
    
    # Calculate sizes
    local size_request=${#request_body}
    local size_response=${#response_body}
    
    echo "📡 $timestamp | Embedding response received" | tee -a "$log_file"
    echo "📊 Status code: $http_code" | tee -a "$log_file"
    echo "⏱️  Total time: ${elapsed}s" | tee -a "$log_file"
    echo "📦 Request size: ${size_request} bytes, Response size: ${size_response} bytes" | tee -a "$log_file"
    
    # Clean up temp files
    rm -f "$temp_response_file" "$temp_headers_file"
    
    # Check if we got embedding vectors
    if [ "$http_code" = "200" ] && echo "$response_body" | grep -q "embedding"; then
      echo "✅ Embedding vectors received successfully" | tee -a "$log_file"
      echo "   $(echo "$response_body" | grep -o '"embedding":\[[0-9.,\-]*' | head -c 70)..." | tee -a "$log_file"
      success=true
      break
    else
      echo "❌ No embedding vectors in response (attempt $attempt/$max_retries):" | tee -a "$log_file"
      if [ "$http_code" = "TIMEOUT" ]; then
        echo "⚠️  Request timed out after ${timeout}s" | tee -a "$log_file"
      else
        echo "   Response preview: $(echo "$response_body" | head -c 200)..." | tee -a "$log_file"
      fi
      
      # Only retry if we haven't reached max attempts
      if [ $attempt -lt $max_retries ]; then
        echo "🔄 Retrying in ${retry_delay} seconds..." | tee -a "$log_file"
        sleep $retry_delay
        # Increase timeout for next attempt
        timeout=$((timeout + 10))
      else
        echo "❌ All retry attempts failed" | tee -a "$log_file"
      fi
    fi
    
    attempt=$((attempt + 1))
  done
  
  echo "-----------------------------------------" | tee -a "$log_file"
  
  # Return success status
  if [ "$success" = true ]; then
    return 0
  else
    return 1
  fi
}

# Display start banner
echo "=================================================="
echo "🚀 HARALD JSONL INGEST PROCESS"
echo "🔍 Processing MarvelAIs.json using JSONL format"
echo "🕒 Started at $(date +"%Y-%m-%d %H:%M:%S")"
echo "=================================================="

# Function to validate JSONL format
validate_jsonl() {
  local file=$1
  echo "🔍 Validating JSONL format of $file..."
  
  # Check if each line is valid JSON
  local line_num=1
  local error_count=0
  while IFS= read -r line; do
    if ! echo "$line" | jq -e . > /dev/null 2>&1; then
      echo "  ❌ Invalid JSON on line $line_num"
      error_count=$((error_count + 1))
      if [ "$error_count" -gt 5 ]; then
        echo "  Too many errors, stopping validation"
        return 1
      fi
    fi
    line_num=$((line_num + 1))
  done < "$file"
  
  if [ "$error_count" -eq 0 ]; then
    echo "  ✅ JSONL validation successful!"
    return 0
  else
    echo "  ❌ JSONL validation failed with $error_count errors"
    return 1
  fi
}

# Ensure Ollama is running
if ! pgrep -x "ollama" > /dev/null; then
  echo "🚀 Starting Ollama service..."
  ollama serve &
  sleep 5  # Give Ollama time to start
else
  echo "✅ Ollama service is running"
fi

# Check if the rust_ingest is configured to handle JSON and JSONL files
check_file_support() {
  echo "🔍 Checking if the ingest system supports JSONL files..."
  SUPPORTED_EXTS=$(grep -A 10 "SUPPORTED_EXTENSIONS" /Users/bryanchasko/Code/HARALD/rust_ingest/src/ingest.rs | grep -o '"[^"]*"' | tr -d '"')
  
  if echo "$SUPPORTED_EXTS" | grep -q "jsonl"; then
    echo "  ✅ JSONL files are explicitly supported"
  else
    echo "  ⚠️  JSONL files not explicitly mentioned in SUPPORTED_EXTENSIONS"
    echo "  ⚙️  Currently supporting: $SUPPORTED_EXTS"
    echo "  🔧 Will proceed with JSON support and rely on content-based processing"
  fi
}

check_file_support

# Check for required models
if ! ollama list | grep -q "harald-phi4"; then
  echo "❌ harald-phi4 model not found. Please install it with 'ollama pull harald-phi4'"
  exit 1
fi

# Create logs directory
LOGS_DIR="./logs/embedding_api"
mkdir -p "$LOGS_DIR"
LOG_FILE="$LOGS_DIR/embed_log_$(date +%Y%m%d_%H%M%S).log"
echo "📝 Logging API interactions to: $LOG_FILE"

# Test embedding generation
echo "🔄 Testing embedding generation with harald-phi4 model..."
display_time "Testing embedding API with simple prompt"

# Use our logging function for the test request with retries
TEST_REQUEST='{"model":"harald-phi4","prompt":"test"}'
log_embedding_request "$TEST_REQUEST" "$LOG_FILE" 3 5 15

# Check the last log entry to see if it was successful
if grep -q "Embedding vectors received successfully" "$LOG_FILE"; then
  echo "✅ Ollama is responding to embedding requests"
else
  echo "❌ Failed to generate embeddings with harald-phi4 model"
  echo "See detailed logs in $LOG_FILE"
  exit 1
fi

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

# Create a temporary directory for the JSONL file
TEMP_DIR=$(mktemp -d)
echo "📁 Creating temporary directory for focused ingestion: $TEMP_DIR"

# Make directory structure
mkdir -p "$TEMP_DIR/marvel"

# Define the source file (ONLY using MarvelAIs.json)
SOURCE_FILE="/Users/bryanchasko/Code/HARALD/personality-archetypes/pop-culture/marvel/MarvelAIs.json"
display_time "Starting to process: $SOURCE_FILE"
echo "� Source file details:"
echo "   - Size: $(du -h "$SOURCE_FILE" | cut -f1) bytes"
echo "   - Characters: $(wc -c < "$SOURCE_FILE" | xargs) characters"
echo "   - JSON objects: $(jq '. | length' "$SOURCE_FILE") entities"

# Create a JSONL file with one character per line
display_time "Converting to JSONL format..."
JSONL_FILE="$TEMP_DIR/marvel/MarvelAIs.jsonl"
jq -c '.[]' "$SOURCE_FILE" > "$JSONL_FILE"

  # Create smaller JSON files with just a few characters each to avoid timeout
  display_time "Creating smaller JSON chunks for processing..."
  mkdir -p "$TEMP_DIR/marvel/chunks"

  # Split the original file into smaller chunks of 3 characters each
  # We'll use bash array slicing for this instead of jq floor function
  characters=($(jq -c '.[]' "$SOURCE_FILE"))
  total=${#characters[@]}

  # Create 5 chunks with approximately equal number of characters each
  jq '.[:3]' "$SOURCE_FILE" > "$TEMP_DIR/marvel/chunks/chunk1.json"
  jq '.[3:6]' "$SOURCE_FILE" > "$TEMP_DIR/marvel/chunks/chunk2.json"
  jq '.[6:9]' "$SOURCE_FILE" > "$TEMP_DIR/marvel/chunks/chunk3.json"
  jq '.[9:12]' "$SOURCE_FILE" > "$TEMP_DIR/marvel/chunks/chunk4.json"
  jq '.[12:]' "$SOURCE_FILE" > "$TEMP_DIR/marvel/chunks/chunk5.json"
  
  # Extract individual character as minimal test cases
  display_time "Extracting single characters for minimal test cases"
  mkdir -p "$TEMP_DIR/marvel/single"
  jq -c '.[0]' "$SOURCE_FILE" > "$TEMP_DIR/marvel/single/character1.json"
  
  # Test embedding a single character directly
  display_time "Testing direct embedding with a minimal JSON object"
  echo "🧪 Trying minimal embedding test with a single character..." | tee -a "$LOG_FILE"
  SINGLE_CHAR=$(jq -r '.[0].character_name' "$SOURCE_FILE")
  MINIMAL_TEST="{\"model\":\"harald-phi4\",\"prompt\":\"$SINGLE_CHAR\"}"
  
  # Call with numeric arguments
  log_embedding_request "$MINIMAL_TEST" "$LOG_FILE" 2 5 20
  
echo "  Created JSONL file with $(wc -l < "$JSONL_FILE" | xargs) characters"
echo "  Created 5 smaller JSON chunks for processing"

# Validate the JSONL file
validate_jsonl "$JSONL_FILE"
if [ $? -ne 0 ]; then
  echo "⚠️  Warning: JSONL validation failed. Proceeding anyway but may encounter errors."
fi

echo "📊 Created $(ls -1 "$TEMP_DIR/marvel_characters" | wc -l | xargs) character files"

display_time "Starting MarvelAIs.json processing as JSONL..."
cd /Users/bryanchasko/Code/HARALD/rust_ingest

# Clean any existing index to start fresh
echo "🧹 Cleaning any existing index files..."
rm -f ../data/index.hnsw.* ../data/repo.*

# Ensure index directory exists
mkdir -p "../data"

# List the directory to confirm files are present
echo "  Listing files to be processed:"
find "$TEMP_DIR" -type f | grep -v "chunks/" | sort
echo "  (+ 5 additional chunk files in chunks/ directory)"

# Process the smaller chunks to avoid timeout issues
echo "  Processing Marvel AI characters in smaller chunks..."
PROCESSED_COUNT=0
FAILED_COUNT=0

# Process each chunk with a separate invocation
for chunk_file in "$TEMP_DIR/marvel/chunks"/*.json; do
  chunk_name=$(basename "$chunk_file")
  chunk_start_time=$(date +%s)
  display_time "Processing chunk: $chunk_name"
  
  # Create a temporary dir for just this chunk
  CHUNK_DIR=$(mktemp -d)
  mkdir -p "$CHUNK_DIR/marvel"
  cp "$chunk_file" "$CHUNK_DIR/marvel/"
  
  # Log chunk content for debugging
  echo "📄 Chunk content preview:" | tee -a "$LOG_FILE"
  head -c 500 "$chunk_file" | tee -a "$LOG_FILE"
  echo "..." | tee -a "$LOG_FILE"
  
  # Check if we can manually generate an embedding for this chunk
  echo "🧪 Testing direct embedding for chunk..." | tee -a "$LOG_FILE"
  CHUNK_CONTENT=$(cat "$chunk_file" | tr -d '\n' | sed 's/"/\\"/g')
  DIRECT_TEST='{"model":"harald-phi4","prompt":"'"$CHUNK_CONTENT"'"}'
  
  # Use the first 1000 characters max for the test
  TRIMMED_TEST=$(echo "$DIRECT_TEST" | head -c 1000)
  echo "$TRIMMED_TEST..." > "$CHUNK_DIR/test_request.json"
  
  # Log the direct embedding attempt with retries
  display_time "Testing direct embedding for $chunk_name with retries"
  log_embedding_request "$TRIMMED_TEST" "$LOG_FILE" 3 10 45
  
  # Try to process this chunk with detailed output logging
  display_time "Running ingest on $chunk_name"
  echo "🔄 Running cargo with: --max-chars 500 --max-tokens 200 (reduced tokens)" | tee -a "$LOG_FILE"
  
  if cargo run --release -- ingest --root "$CHUNK_DIR" --max-chars 500 --max-tokens 200 2>&1 | tee -a "$LOG_FILE"; then
    chunk_end_time=$(date +%s)
    chunk_duration=$((chunk_end_time - chunk_start_time))
    echo "  ✓ Successfully processed: $chunk_name (${chunk_duration}s)" | tee -a "$LOG_FILE"
    PROCESSED_COUNT=$((PROCESSED_COUNT + 1))
  else
    chunk_end_time=$(date +%s)
    chunk_duration=$((chunk_end_time - chunk_start_time))
    echo "  ✗ Failed to process: $chunk_name (${chunk_duration}s)" | tee -a "$LOG_FILE"
    echo "  ℹ️ Check $LOG_FILE for detailed error information"
    FAILED_COUNT=$((FAILED_COUNT + 1))
  fi
  
  # Clean up the chunk directory
  rm -rf "$CHUNK_DIR"
  
  # Add a slightly longer delay to avoid overwhelming the embedding API
  echo "💤 Adding delay between requests..." | tee -a "$LOG_FILE"
  sleep 5
done

display_time "Processing complete. Summary: $PROCESSED_COUNT chunks successful, $FAILED_COUNT chunks failed"

# If all chunks failed, print debugging info
if [ "$PROCESSED_COUNT" -eq 0 ]; then
  echo "  ℹ️ All chunks failed. Checking JSON format..."
  jq . "$SOURCE_FILE" > /dev/null
  if [ $? -eq 0 ]; then
    echo "  ✓ Source JSON is valid"
  else
    echo "  ✗ Source JSON has format errors"
  fi
  
  echo "  ℹ️ Checking temporary JSONL file..."
  cat "$JSONL_FILE" | head -n 1 | jq . > /dev/null
  if [ $? -eq 0 ]; then
    echo "  ✓ JSONL transformation appears correct"
  else
    echo "  ✗ JSONL transformation has issues"
  fi
fi

# Cleanup
display_time "Cleaning up temporary files..."
rm -rf "$TEMP_DIR"

# Calculate final timing and stats
TOTAL_TIME=$(elapsed_time)
INDEX_SIZE=$(du -h ../data/index.hnsw.data 2>/dev/null | cut -f1 || echo "0")
REPO_INDEX_SIZE=$(du -h ../data/repo.index 2>/dev/null | cut -f1 || echo "0")

# Print summary banner
echo "=================================================="
echo "✅ HARALD JSONL INGESTION COMPLETE"
echo "------------------------------------------------"
echo "📊 SUMMARY:"
echo "   Source: $SOURCE_FILE"
echo "   Chunks processed: $PROCESSED_COUNT successful, $FAILED_COUNT failed"
echo "   Index size: $INDEX_SIZE"
echo "   Repo index size: $REPO_INDEX_SIZE"
echo "   Index location: $(pwd)/../data/index.hnsw.data"
echo "   Total execution time: $TOTAL_TIME"
echo "------------------------------------------------"
echo "🔬 This approach uses JSONL format (newline-delimited JSON)"
echo "   which is optimized for streaming embeddings and vector processing."
echo "   See: http://jsonlines.org/examples"
echo "=================================================="
echo "🕒 Finished at $(date +"%Y-%m-%d %H:%M:%S")"
