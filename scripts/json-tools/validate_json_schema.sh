#!/bin/bash
# validate_json_schema.sh - JSON schema validation utility
#
# This script validates JSON files against predefined schemas for vector data stores
# It works with the vector_stores_registry.json to ensure consistency
#
# Usage:
#   ./validate_json_schema.sh --store <store-id>     # Validate all files for store
#   ./validate_json_schema.sh --file <file-path>     # Validate specific file
#   ./validate_json_schema.sh --generate <store-id>  # Generate schema from existing files
#
# Author: Bryan Chasko
# Date: July 21, 2025

set -e

# Configuration
REGISTRY_FILE="./data/vector_stores_registry.json"
SCHEMA_DIR="./data/schemas"
TARGET="all"
TARGET_FILE=""
STORE_ID=""
MODE="validate"
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
  
  # Check for ajv (JSON Schema validator)
  if ! command -v ajv &> /dev/null; then
    log_warning "ajv is recommended but not installed. For full schema validation, install with: npm install -g ajv-cli"
  fi
  
  if [ "$missing" = true ]; then
    exit 1
  fi
}

ensure_schema_dir() {
  if [ ! -d "$SCHEMA_DIR" ]; then
    mkdir -p "$SCHEMA_DIR"
    log_info "Created schema directory: $SCHEMA_DIR"
  fi
}

get_store_for_file() {
  local file_path="$1"
  local relative_path
  
  # Convert to relative path for comparison
  relative_path=$(echo "$file_path" | sed "s|$(pwd)/||")
  
  jq -r --arg path "$relative_path" '.vectorStores[] | select(.sourceFiles[] | contains($path)) | .id' "$REGISTRY_FILE"
}

validate_file() {
  local file_path="$1"
  local store_id="$2"
  
  if [ ! -f "$file_path" ]; then
    log_error "File not found: $file_path"
    return 1
  fi
  
  # If store_id not provided, try to determine it
  if [ -z "$store_id" ]; then
    store_id=$(get_store_for_file "$file_path")
    if [ -z "$store_id" ]; then
      log_error "Could not determine store ID for file: $file_path"
      log_info "Please specify store ID with --store option"
      return 1
    fi
  fi
  
  local schema_file="$SCHEMA_DIR/${store_id}_schema.json"
  
  # Check if schema exists
  if [ ! -f "$schema_file" ]; then
    log_warning "No schema found for store: $store_id"
    log_info "Use --generate $store_id to create schema"
    return 0
  fi
  
  log_info "Validating $file_path against schema for $store_id"
  
  # Basic validation with jq
  if ! jq empty "$file_path" 2>/dev/null; then
    log_error "File is not valid JSON: $file_path"
    return 1
  fi
  
  # Advanced validation with ajv if available
  if command -v ajv &> /dev/null; then
    if ! ajv validate -s "$schema_file" -d "$file_path" --errors=text 2>/dev/null; then
      log_error "Schema validation failed for: $file_path"
      ajv validate -s "$schema_file" -d "$file_path" --errors=text
      return 1
    else
      log_success "Schema validation passed for: $file_path"
    fi
  else
    # Fallback to basic structure validation with jq
    local expected_keys=$(jq -r 'keys[]' "$schema_file" 2>/dev/null || echo "")
    if [ -n "$expected_keys" ]; then
      for key in $expected_keys; do
        if ! jq -e ".$key" "$file_path" &>/dev/null; then
          log_error "Missing required key in $file_path: $key"
          return 1
        fi
      done
    fi
    log_success "Basic structure validation passed for: $file_path"
  fi
  
  # Store-specific validations
  case "$store_id" in
    "marvel_characters")
      # Specific validation for Marvel characters
      if ! jq -e '.[0].character_name' "$file_path" &>/dev/null; then
        log_error "Marvel character file missing character_name"
        return 1
      fi
      ;;
    "heralds")
      # Specific validation for Heralds
      if ! jq -e '.personality' "$file_path" &>/dev/null; then
        log_warning "Heralds file may be missing personality attributes"
      fi
      ;;
    *)
      # Default validation
      ;;
  esac
  
  return 0
}

validate_store_files() {
  local store_id="$1"
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
  log_info "Validating $total files for store: $store_id"
  
  for file in "${files[@]}"; do
    echo
    log_info "Validating: $file"
    if validate_file "$(pwd)/$file" "$store_id"; then
      successful=$((successful + 1))
    else
      failed=$((failed + 1))
    fi
  done
  
  echo
  log_info "Validation complete for store $store_id:"
  echo "- Total files: $total"
  echo "- Successfully validated: $successful"
  if [ $failed -gt 0 ]; then
    log_warning "- Failed: $failed"
  else
    echo "- Failed: $failed"
  fi
}

generate_schema() {
  local store_id="$1"
  local files=()
  
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
  
  if [ ${#files[@]} -eq 0 ]; then
    log_error "No files found for store: $store_id"
    return 1
  fi
  
  ensure_schema_dir
  
  local schema_file="$SCHEMA_DIR/${store_id}_schema.json"
  local first_file="$(pwd)/${files[0]}"
  
  log_info "Generating schema for $store_id based on: ${files[0]}"
  
  # Basic schema generation - can be expanded based on specific needs
  jq -n --arg id "$store_id" '{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": $id + " Schema",
    "description": "Schema for ' + $id + ' JSON files"
  }' > "$schema_file"
  
  # Add properties based on first file
  if [ -f "$first_file" ] && jq empty "$first_file" &>/dev/null; then
    # Different handling based on file structure
    if jq -e 'type == "array"' "$first_file" &>/dev/null; then
      # Handle array structure
      jq -n --argjson sample "$(jq '.[0]' "$first_file")" '{
        "type": "array",
        "items": {
          "type": "object",
          "required": ($sample | keys),
          "properties": ($sample | to_entries | map({(.key): {"type": (if .value|type == "object" then "object" elif .value|type == "array" then "array" elif .value|type == "string" then "string" elif .value|type == "number" then "number" else "string" end)}}) | add)
        }
      }' | jq -s '.[0] * .[1]' "$schema_file" - > "${schema_file}.tmp"
      mv "${schema_file}.tmp" "$schema_file"
    else
      # Handle object structure
      jq -n --argjson sample "$(jq '.' "$first_file")" '{
        "type": "object",
        "required": ($sample | keys),
        "properties": ($sample | to_entries | map({(.key): {"type": (if .value|type == "object" then "object" elif .value|type == "array" then "array" elif .value|type == "string" then "string" elif .value|type == "number" then "number" else "string" end)}}) | add)
      }' | jq -s '.[0] * .[1]' "$schema_file" - > "${schema_file}.tmp"
      mv "${schema_file}.tmp" "$schema_file"
    fi
  fi
  
  log_success "Generated schema: $schema_file"
  
  # Add store-specific customizations
  case "$store_id" in
    "marvel_characters")
      # Add specific schema rules for Marvel characters
      jq '.properties.character_name.description = "The name of the Marvel character"' "$schema_file" > "${schema_file}.tmp"
      mv "${schema_file}.tmp" "$schema_file"
      ;;
    *)
      # Default customization
      ;;
  esac
  
  if [ "$VERBOSE" = true ]; then
    echo
    echo "Schema contents:"
    echo "================="
    jq '.' "$schema_file"
  fi
  
  return 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
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
    --generate)
      MODE="generate"
      STORE_ID="$2"
      shift 2
      ;;
    --verbose)
      VERBOSE=true
      shift
      ;;
    --help|-h)
      echo "Usage:"
      echo "  ./validate_json_schema.sh --store <store-id>     # Validate all files for store"
      echo "  ./validate_json_schema.sh --file <file-path>     # Validate specific file"
      echo "  ./validate_json_schema.sh --generate <store-id>  # Generate schema from existing files"
      echo
      echo "Options:"
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
if [ "$MODE" = "generate" ]; then
  if [ -z "$STORE_ID" ]; then
    log_error "No store ID specified for schema generation"
    exit 1
  fi
  generate_schema "$STORE_ID"
  exit $?
elif [ "$TARGET" = "store" ]; then
  if [ -z "$STORE_ID" ]; then
    log_error "No store ID specified"
    exit 1
  fi
  validate_store_files "$STORE_ID"
  exit $?
elif [ "$TARGET" = "file" ]; then
  if [ -z "$TARGET_FILE" ]; then
    log_error "No file path specified"
    exit 1
  fi
  validate_file "$TARGET_FILE" "$STORE_ID"
  exit $?
else
  log_error "Invalid mode or target"
  echo "Use --help for usage information."
  exit 1
fi
