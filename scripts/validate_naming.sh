#!/bin/bash
# validate_naming.sh - Validates file and directory naming against project conventions
#
# This script checks that files and directories follow the HARALD project
# naming conventions as defined in docs/naming-conventions.md
#
# Usage:
#   ./validate_naming.sh [--fix] [--verbose] [path]
#
# Options:
#   --fix       Suggest and optionally apply fixes for naming issues
#   --verbose   Show detailed information about checks
#   path        Path to check (defaults to current directory)
#
# Author: Bryan Chasko
# Date: July 21, 2025

set -e

# Configuration
VERBOSE=false
FIX_MODE=false
TARGET_PATH="."

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

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --verbose)
      VERBOSE=true
      shift
      ;;
    --fix)
      FIX_MODE=true
      shift
      ;;
    --help|-h)
      echo "Usage:"
      echo "  ./validate_naming.sh [--fix] [--verbose] [path]"
      echo
      echo "Options:"
      echo "  --fix       Suggest and optionally apply fixes for naming issues"
      echo "  --verbose   Show detailed information about checks"
      echo "  path        Path to check (defaults to current directory)"
      exit 0
      ;;
    *)
      TARGET_PATH="$1"
      shift
      ;;
  esac
done

# Validate directory names
validate_directory_names() {
  local path="$1"
  local issues=0
  local dirs=()
  
  if [ "$VERBOSE" = true ]; then
    log_info "Validating directory names in $path"
  fi
  
  # Find all directories, exclude hidden, generated, and special directories
  while IFS= read -r dir; do
    dirs+=("$dir")
  done < <(find "$path" -type d | grep -v "^\./\." | grep -v "^\./node_modules" | grep -v "target/" | grep -v "^\./venv" | grep -v "^\./\.git" | grep -v "^\./\.vscode" | grep -v "^\./build" | grep -v "^\./dist" | sort)
  
  for dir in "${dirs[@]}"; do
    # Skip the root directory
    if [ "$dir" = "." ]; then
      continue
    fi
    
    # Get just the directory name without path
    local dirname=$(basename "$dir")
    
    # Check for kebab-case in multi-word directory names
    if [[ "$dirname" == *"_"* ]]; then
      log_warning "Directory uses snake_case instead of kebab-case: $dir"
      if [ "$FIX_MODE" = true ]; then
        local new_name=$(echo "$dirname" | tr '_' '-')
        local new_path="${dir%/*}/$new_name"
        echo "  Suggested fix: Rename to $new_path"
        read -p "  Apply this fix? [y/N] " apply_fix
        if [[ "$apply_fix" =~ ^[Yy]$ ]]; then
          mv "$dir" "$new_path"
          log_success "Renamed directory to $new_path"
        fi
      fi
      issues=$((issues + 1))
    fi
    
    # Check for PascalCase in directory names (except for special directories)
    if [[ "$dirname" =~ [A-Z] ]] && \
       [[ "$dir" != *"/ai-entities"* ]] && \
       [[ "$dir" != *"/personality-archetypes"* ]] && \
       [[ "$dir" != *"/.git"* ]] && \
       [[ "$dir" != *"/target"* ]]; then
      log_warning "Directory uses PascalCase instead of kebab-case: $dir"
      if [ "$FIX_MODE" = true ]; then
        local new_name=$(echo "$dirname" | sed 's/\([A-Z]\)/-\L\1/g' | sed 's/^-//')
        local new_path="${dir%/*}/$new_name"
        echo "  Suggested fix: Rename to $new_path"
        read -p "  Apply this fix? [y/N] " apply_fix
        if [[ "$apply_fix" =~ ^[Yy]$ ]]; then
          mv "$dir" "$new_path"
          log_success "Renamed directory to $new_path"
        fi
      fi
      issues=$((issues + 1))
    fi
  done
  
  if [ "$issues" -eq 0 ]; then
    if [ "$VERBOSE" = true ]; then
      log_success "All directory names follow conventions"
    fi
    return 0
  else
    log_warning "Found $issues directory naming issues"
    return 1
  fi
}

# Validate Rust file names
validate_rust_file_names() {
  local path="$1"
  local issues=0
  local files=()
  
  if [ "$VERBOSE" = true ]; then
    log_info "Validating Rust file names in $path"
  fi
  
  # Find all Rust files
  while IFS= read -r file; do
    files+=("$file")
  done < <(find "$path" -name "*.rs" | sort)
  
  for file in "${files[@]}"; do
    # Get just the file name without path and extension
    local filename=$(basename "$file" .rs)
    
    # Skip special Rust files
    if [[ "$filename" == "main" || "$filename" == "lib" ]]; then
      continue
    fi
    
    # Check for snake_case
    if [[ "$filename" == *"-"* ]]; then
      log_warning "Rust file uses kebab-case instead of snake_case: $file"
      if [ "$FIX_MODE" = true ]; then
        local new_name=$(echo "$filename" | tr '-' '_').rs
        local new_path="${file%/*}/$new_name"
        echo "  Suggested fix: Rename to $new_path"
        read -p "  Apply this fix? [y/N] " apply_fix
        if [[ "$apply_fix" =~ ^[Yy]$ ]]; then
          mv "$file" "$new_path"
          log_success "Renamed file to $new_path"
        fi
      fi
      issues=$((issues + 1))
    fi
    
    # Check for PascalCase or camelCase
    if [[ "$filename" =~ [A-Z] ]]; then
      log_warning "Rust file uses PascalCase/camelCase instead of snake_case: $file"
      if [ "$FIX_MODE" = true ]; then
        local new_name=$(echo "$filename" | sed 's/\([A-Z]\)/_\L\1/g' | sed 's/^_//').rs
        local new_path="${file%/*}/$new_name"
        echo "  Suggested fix: Rename to $new_path"
        read -p "  Apply this fix? [y/N] " apply_fix
        if [[ "$apply_fix" =~ ^[Yy]$ ]]; then
          mv "$file" "$new_path"
          log_success "Renamed file to $new_path"
        fi
      fi
      issues=$((issues + 1))
    fi
  done
  
  if [ "$issues" -eq 0 ]; then
    if [ "$VERBOSE" = true ]; then
      log_success "All Rust file names follow conventions"
    fi
    return 0
  else
    log_warning "Found $issues Rust naming issues"
    return 1
  fi
}

# Validate Markdown file names
validate_markdown_file_names() {
  local path="$1"
  local issues=0
  local files=()
  
  if [ "$VERBOSE" = true ]; then
    log_info "Validating Markdown file names in $path"
  fi
  
  # Find all Markdown files
  while IFS= read -r file; do
    files+=("$file")
  done < <(find "$path" -name "*.md" | sort)
  
  for file in "${files[@]}"; do
    # Get just the file name without path and extension
    local filename=$(basename "$file" .md)
    
    # Skip special Markdown files
    if [[ "$filename" =~ ^[A-Z]+$ ]]; then
      continue
    fi
    
    # Handle entity files differently
    if [[ "$file" == *"/ai-entities/"* && ! "$filename" == *"-"* && ! "$filename" == *"_"* ]]; then
      # Entity files should be lowercase to match directory structure
      if [[ "$filename" =~ [A-Z] ]]; then
        log_warning "Entity markdown file should use lowercase: $file"
        if [ "$FIX_MODE" = true ]; then
          local new_name=$(echo "$filename" | tr '[:upper:]' '[:lower:]').md
          local new_path="${file%/*}/$new_name"
          echo "  Suggested fix: Rename to $new_path"
          read -p "  Apply this fix? [y/N] " apply_fix
          if [[ "$apply_fix" =~ ^[Yy]$ ]]; then
            mv "$file" "$new_path"
            log_success "Renamed file to $new_path"
          fi
        fi
        issues=$((issues + 1))
      fi
    else
      # Regular documentation should use kebab-case
      if [[ "$filename" == *"_"* ]]; then
        log_warning "Markdown file uses snake_case instead of kebab-case: $file"
        if [ "$FIX_MODE" = true ]; then
          local new_name=$(echo "$filename" | tr '_' '-').md
          local new_path="${file%/*}/$new_name"
          echo "  Suggested fix: Rename to $new_path"
          read -p "  Apply this fix? [y/N] " apply_fix
          if [[ "$apply_fix" =~ ^[Yy]$ ]]; then
            mv "$file" "$new_path"
            log_success "Renamed file to $new_path"
          fi
        fi
        issues=$((issues + 1))
      fi
    fi
  done
  
  if [ "$issues" -eq 0 ]; then
    if [ "$VERBOSE" = true ]; then
      log_success "All Markdown file names follow conventions"
    fi
    return 0
  else
    log_warning "Found $issues Markdown naming issues"
    return 1
  fi
}

# Validate JSON file names
validate_json_file_names() {
  local path="$1"
  local issues=0
  local files=()
  
  if [ "$VERBOSE" = true ]; then
    log_info "Validating JSON file names in $path"
  fi
  
  # Find all JSON files
  while IFS= read -r file; do
    files+=("$file")
  done < <(find "$path" -name "*.json" | sort)
  
  for file in "${files[@]}"; do
    # Get just the file name without path and extension
    local filename=$(basename "$file" .json)
    local parent_dir=$(basename "$(dirname "$file")")
    
    # Skip dot files
    if [[ "$filename" == .* ]]; then
      continue
    fi
    
    # Entity and personality files should use TitleCase
    if [[ "$file" == *"/personality-archetypes/"* || "$filename" == *"Registry"* ]]; then
      # First letter should be uppercase
      if [[ ! "$filename" =~ ^[A-Z] ]]; then
        log_warning "Personality/entity JSON file should use TitleCase: $file"
        if [ "$FIX_MODE" = true ]; then
          local new_name=$(echo "$filename" | sed 's/^[a-z]/\U&/').json
          local new_path="${file%/*}/$new_name"
          echo "  Suggested fix: Rename to $new_path"
          read -p "  Apply this fix? [y/N] " apply_fix
          if [[ "$apply_fix" =~ ^[Yy]$ ]]; then
            mv "$file" "$new_path"
            log_success "Renamed file to $new_path"
          fi
        fi
        issues=$((issues + 1))
      fi
    # Schema files should use kebab-case
    elif [[ "$file" == *"/data/schemas/"* || "$file" == *"/data/vector"* || "$file" == *"-config.json" ]]; then
      if [[ "$filename" == *"_"* ]]; then
        log_warning "Schema/config JSON file should use kebab-case: $file"
        if [ "$FIX_MODE" = true ]; then
          local new_name=$(echo "$filename" | tr '_' '-').json
          local new_path="${file%/*}/$new_name"
          echo "  Suggested fix: Rename to $new_path"
          read -p "  Apply this fix? [y/N] " apply_fix
          if [[ "$apply_fix" =~ ^[Yy]$ ]]; then
            mv "$file" "$new_path"
            log_success "Renamed file to $new_path"
          fi
        fi
        issues=$((issues + 1))
      fi
    # Memory schema files should use kebab-case
    elif [[ "$file" == *"/memory-schemas/"* ]]; then
      if [[ "$filename" == *"_"* ]]; then
        log_warning "Memory schema JSON file should use kebab-case: $file"
        if [ "$FIX_MODE" = true ]; then
          local new_name=$(echo "$filename" | tr '_' '-').json
          local new_path="${file%/*}/$new_name"
          echo "  Suggested fix: Rename to $new_path"
          read -p "  Apply this fix? [y/N] " apply_fix
          if [[ "$apply_fix" =~ ^[Yy]$ ]]; then
            mv "$file" "$new_path"
            log_success "Renamed file to $new_path"
          fi
        fi
        issues=$((issues + 1))
      fi
    fi
  done
  
  if [ "$issues" -eq 0 ]; then
    if [ "$VERBOSE" = true ]; then
      log_success "All JSON file names follow conventions"
    fi
    return 0
  else
    log_warning "Found $issues JSON naming issues"
    return 1
  fi
}

# Run all validations
run_validations() {
  local path="$1"
  local exit_code=0
  
  echo "Validating naming conventions in $path"
  echo "========================================"
  
  # Run each validation
  validate_directory_names "$path" || exit_code=1
  validate_rust_file_names "$path" || exit_code=1
  validate_markdown_file_names "$path" || exit_code=1
  validate_json_file_names "$path" || exit_code=1
  
  echo "========================================"
  if [ "$exit_code" -eq 0 ]; then
    log_success "All naming conventions validated successfully!"
    if [ "$VERBOSE" = true ]; then
      echo
      echo "Naming conventions reference:"
      echo "- Directories: kebab-case (e.g., vector-search)"
      echo "- Rust files:  snake_case (e.g., embed.rs)"
      echo "- Markdown:    kebab-case for docs (e.g., character-based-chunking.md)"
      echo "              lowercase for entities (e.g., harald.md)"
      echo "- JSON:        kebab-case for config/schema (e.g., vector-stores-registry.json)"
      echo "              TitleCase for entities/personalities (e.g., Heralds.json)"
    fi
  else
    log_warning "Some naming convention checks failed"
    echo
    echo "For more information on naming conventions, see:"
    echo "docs/naming-conventions.md"
  fi
  
  return $exit_code
}

# Execute main function
run_validations "$TARGET_PATH"
