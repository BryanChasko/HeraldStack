#!/bin/bash
# format_json.sh - JSON formatting utility for vector data files
#
# This script formats JSON files according to our vector embedding standards
# It only processes files registered in the vector_stores_registry.json
#
# Usage:
#   ./format_json.sh [--all]                # Format all registered files
#   ./format_json.sh --store <store-id>     # Format files for specific store
#   ./format_json.sh --file <file-path>     # Format specific file (must be registered)
#   ./format_json.sh --check                # Check format without modifying
#   ./format_json.sh --register <file-path> # Add a new file to registry
#   ./format_json.sh --validate-registry    # Validate the registry file
#
# Author: Bryan Chasko
# Date: July 21, 2025

set -e

# Configuration
REGISTRY_FILE="./data/vector_stores_registry.json"
MODE="format"
TARGET="all"
TARGET_FILE=""
STORE_ID=""
NEW_FILE=""
SHOW_DIFF=false
VERBOSE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info() {
  echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
  echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
  echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
  echo -e "${RED}[ERROR]${NC} $1"
  return 1
}

check_dependencies() {
  local missing=false
  
  # Check for jq
  if ! command -v jq &> /dev/null; then
    log_error "jq is required but not installed. Please install with: brew install jq"
    missing=true
  fi
  
  # Check for prettier
  if ! command -v prettier &> /dev/null; then
    log_error "prettier is required but not installed. Please install with: npm install -g prettier"
    missing=true
  fi
  
  if [ "$missing" = true ]; then
    exit 1
  fi
}

validate_registry() {
  if [ ! -f "$REGISTRY_FILE" ]; then
    log_error "Registry file not found: $REGISTRY_FILE"
    return 1
  fi
  
  # Validate JSON format
  if ! jq empty "$REGISTRY_FILE" 2>/dev/null; then
    log_error "Registry file is not valid JSON"
    return 1
  fi
  
  # Validate structure
  if ! jq -e '.vectorStores' "$REGISTRY_FILE" &>/dev/null; then
    log_error "Registry file missing 'vectorStores' array"
    return 1
  fi
  
  log_success "Registry validation successful"
  
  # If verbose, show details
  if [ "$VERBOSE" = true ]; then
    echo
    echo "Registry Details:"
    echo "================="
    echo "Total vector stores: $(jq '.vectorStores | length' "$REGISTRY_FILE")"
    echo "Embedding models: $(jq -r '.embeddingModels | map(.id) | join(", ")' "$REGISTRY_FILE")"
    echo
    echo "Registered stores:"
    jq -r '.vectorStores[] | "- " + .id + ": " + .description + " (" + (.sourceFiles | length | tostring) + " files)"' "$REGISTRY_FILE"
  fi
  
  return 0
}

is_file_registered() {
  local file_path="$1"
  local relative_path
  
  # Convert to relative path for comparison
  relative_path=$(echo "$file_path" | sed "s|$(pwd)/||")
  
  if jq -e --arg path "$relative_path" '.vectorStores[] | select(.sourceFiles[] | contains($path))' "$REGISTRY_FILE" &>/dev/null; then
    return 0  # File is registered
  else
    return 1  # File is not registered
  fi
}

get_store_for_file() {
  local file_path="$1"
  local relative_path
  
  # Convert to relative path for comparison
  relative_path=$(echo "$file_path" | sed "s|$(pwd)/||")
  
  jq -r --arg path "$relative_path" '.vectorStores[] | select(.sourceFiles[] | contains($path)) | .id' "$REGISTRY_FILE"
}

format_json_file() {
  local file_path="$1"
  local mode="$2"
  local store_id
  
  if [ ! -f "$file_path" ]; then
    log_error "File not found: $file_path"
    return 1
  fi
  
  # Validate JSON format
  if ! jq empty "$file_path" 2>/dev/null; then
    log_error "File is not valid JSON: $file_path"
    return 1
  fi
  
  store_id=$(get_store_for_file "$file_path")
  
  if [ "$mode" = "check" ]; then
    # Create formatted version in temp file
    local temp_file=$(mktemp)
    prettier --parser json "$file_path" > "$temp_file"
    
    # Compare with original
    if diff -q "$file_path" "$temp_file" &>/dev/null; then
      log_success "File already properly formatted: $file_path"
      rm "$temp_file"
      return 0
    else
      log_warning "File needs formatting: $file_path"
      if [ "$SHOW_DIFF" = true ]; then
        echo
        echo "Diff for $file_path:"
        diff -u "$file_path" "$temp_file" | grep -v '^@@' | grep -v '^---' | grep -v '^+++'
        echo
      fi
      rm "$temp_file"
      return 1
    fi
  else
    # Format the file in place
    local temp_file=$(mktemp)
    prettier --parser json --write "$file_path" > "$temp_file" 2>&1
    rm "$temp_file"
    
    # Check schema rules specific to vector stores
    if [ -n "$store_id" ]; then
      log_info "Applying schema rules for store: $store_id"
      
      # For now we just do basic validation; in the future we could add more 
      # store-specific validation rules here
      jq empty "$file_path" &>/dev/null || log_error "JSON validation failed after formatting!"
    fi
    
    log_success "Formatted file: $file_path"
    return 0
  fi
}

register_new_file() {
  local file_path="$1"
  
  if [ ! -f "$file_path" ]; then
    log_error "File not found: $file_path"
    return 1
  fi
  
  # Validate JSON format
  if ! jq empty "$file_path" 2>/dev/null; then
    log_error "File is not valid JSON: $file_path"
    return 1
  fi
  
  # Convert to relative path for storage
  local relative_path
  relative_path=$(echo "$file_path" | sed "s|$(pwd)/||")
  
  # Check if already registered
  if is_file_registered "$file_path"; then
    log_warning "File is already registered: $relative_path"
    return 0
  fi
  
  # Get store details from user
  echo
  echo "Registering new file: $relative_path"
  
  # Ask for store ID
  local store_ids
  store_ids=$(jq -r '.vectorStores[].id' "$REGISTRY_FILE")
  echo
  echo "Existing stores:"
  echo "$store_ids" | sed 's/^/- /'
  echo
  read -p "Enter store ID (existing or new): " new_store_id
  
  if jq -e --arg id "$new_store_id" '.vectorStores[] | select(.id == $id)' "$REGISTRY_FILE" &>/dev/null; then
    # Add to existing store
    jq --arg id "$new_store_id" --arg file "$relative_path" \
      '(.vectorStores[] | select(.id == $id).sourceFiles) += [$file]' "$REGISTRY_FILE" > "${REGISTRY_FILE}.tmp"
    mv "${REGISTRY_FILE}.tmp" "$REGISTRY_FILE"
    log_success "File added to existing store: $new_store_id"
  else
    # Create new store
    echo
    read -p "Enter store description: " store_description
    read -p "Enter data location (path): " data_location
    read -p "Enter ingest script (path): " ingest_script
    read -p "Enter embedding model [ollama]: " embedding_model
    embedding_model=${embedding_model:-ollama}
    read -p "Enter chunking strategy [character-based]: " chunking_strategy
    chunking_strategy=${chunking_strategy:-character-based}
    read -p "Enter max chunk size [250]: " max_chunk_size
    max_chunk_size=${max_chunk_size:-250}
    
    # Add new store
    jq --arg id "$new_store_id" \
       --arg desc "$store_description" \
       --arg file "$relative_path" \
       --arg location "$data_location" \
       --arg script "$ingest_script" \
       --arg model "$embedding_model" \
       --arg strategy "$chunking_strategy" \
       --argjson size "$max_chunk_size" \
      '.vectorStores += [{
        "id": $id,
        "description": $desc,
        "sourceFiles": [$file],
        "dataLocation": $location,
        "ingestScript": $script,
        "embeddingModel": $model,
        "chunkingStrategy": $strategy,
        "maxChunkSize": $size
      }]' "$REGISTRY_FILE" > "${REGISTRY_FILE}.tmp"
    mv "${REGISTRY_FILE}.tmp" "$REGISTRY_FILE"
    log_success "New store created: $new_store_id"
  fi
  
  # Update lastUpdated date
  current_date=$(date +"%Y-%m-%d")
  jq --arg date "$current_date" '.lastUpdated = $date' "$REGISTRY_FILE" > "${REGISTRY_FILE}.tmp"
  mv "${REGISTRY_FILE}.tmp" "$REGISTRY_FILE"
  
  return 0
}

process_all_files() {
  local mode="$1"
  local total=0
  local successful=0
  local failed=0
  local files=()
  
  # Get all registered files
  while IFS= read -r file; do
    files+=("$file")
  done < <(jq -r '.vectorStores[].sourceFiles[]' "$REGISTRY_FILE")
  
  total=${#files[@]}
  log_info "Processing $total registered files..."
  
  for file in "${files[@]}"; do
    echo
    log_info "Processing: $file"
    if format_json_file "$(pwd)/$file" "$mode"; then
      successful=$((successful + 1))
    else
      failed=$((failed + 1))
    fi
  done
  
  echo
  log_info "Processing complete:"
  echo "- Total files: $total"
  echo "- Successfully processed: $successful"
  if [ $failed -gt 0 ]; then
    log_warning "- Failed: $failed"
  else
    echo "- Failed: $failed"
  fi
}

process_store_files() {
  local store_id="$1"
  local mode="$2"
  local files=()
  local total=0
  local successful=0
  local failed=0
  
  # Check if store exists
  if ! jq -e --arg id "$store_id" '.vectorStores[] | select(.id == $id)' "$REGISTRY_FILE" &>/dev/null; then
    log_error "Store not found: $store_id"
    echo "Available stores:"
    jq -r '.vectorStores[].id' "$REGISTRY_FILE" | sed 's/^/- /'
    return 1
  fi
  
  # Get files for this store
  while IFS= read -r file; do
    files+=("$file")
  done < <(jq -r --arg id "$store_id" '.vectorStores[] | select(.id == $id) | .sourceFiles[]' "$REGISTRY_FILE")
  
  total=${#files[@]}
  log_info "Processing $total files for store: $store_id"
  
  for file in "${files[@]}"; do
    echo
    log_info "Processing: $file"
    if format_json_file "$(pwd)/$file" "$mode"; then
      successful=$((successful + 1))
    else
      failed=$((failed + 1))
    fi
  done
  
  echo
  log_info "Processing complete for store $store_id:"
  echo "- Total files: $total"
  echo "- Successfully processed: $successful"
  if [ $failed -gt 0 ]; then
    log_warning "- Failed: $failed"
  else
    echo "- Failed: $failed"
  fi
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --all)
      TARGET="all"
      shift
      ;;
    --store)
      TARGET="store"
      STORE_ID="$2"
      shift 2
      ;;
    --file)
      TARGET="file"
      TARGET_FILE="$2"
      shift 2
      ;;
    --check)
      MODE="check"
      shift
      ;;
    --register)
      MODE="register"
      NEW_FILE="$2"
      shift 2
      ;;
    --validate-registry)
      MODE="validate"
      shift
      ;;
    --diff)
      SHOW_DIFF=true
      shift
      ;;
    --verbose)
      VERBOSE=true
      shift
      ;;
    --help|-h)
      echo "Usage:"
      echo "  ./format_json.sh [--all]                # Format all registered files"
      echo "  ./format_json.sh --store <store-id>     # Format files for specific store"
      echo "  ./format_json.sh --file <file-path>     # Format specific file (must be registered)"
      echo "  ./format_json.sh --check                # Check format without modifying"
      echo "  ./format_json.sh --register <file-path> # Add a new file to registry"
      echo "  ./format_json.sh --validate-registry    # Validate the registry file"
      echo
      echo "Options:"
      echo "  --diff     Show diff when checking files"
      echo "  --verbose  Show detailed output"
      echo "  --help     Show this help message"
      exit 0
      ;;
    *)
      echo "Error: Unknown option: $1"
      echo "Use --help for usage information."
      exit 1
      ;;
  esac
done

# Check for required dependencies
check_dependencies

# Process based on mode and target
if [ "$MODE" = "validate" ]; then
  validate_registry
  exit $?
elif [ "$MODE" = "register" ]; then
  if [ -z "$NEW_FILE" ]; then
    log_error "No file specified for registration"
    exit 1
  fi
  register_new_file "$NEW_FILE"
  exit $?
elif [ "$TARGET" = "all" ]; then
  process_all_files "$MODE"
elif [ "$TARGET" = "store" ]; then
  if [ -z "$STORE_ID" ]; then
    log_error "No store ID specified"
    exit 1
  fi
  process_store_files "$STORE_ID" "$MODE"
elif [ "$TARGET" = "file" ]; then
  if [ -z "$TARGET_FILE" ]; then
    log_error "No file path specified"
    exit 1
  fi
  
  # Check if file is registered
  if ! is_file_registered "$TARGET_FILE"; then
    log_error "File is not registered in the vector store registry"
    log_info "Use --register to add it to the registry first"
    exit 1
  fi
  
  format_json_file "$TARGET_FILE" "$MODE"
else
  log_error "Invalid mode or target"
  echo "Use --help for usage information."
  exit 1
fi
