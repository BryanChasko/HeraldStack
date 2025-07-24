#!/bin/bash
# build-ingest.sh - Build and test the migrated ingest module
#
# This script builds and tests the Rust ingest module in its new location
#
# Usage:
#   ./scripts/dev/build-ingest.sh [--test]
#
# Options:
#   --test     Run tests for the ingest module
#
# Author: Bryan Chasko
# Date: July 21, 2025

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
SRC_DIR="$PROJECT_ROOT/src"
INGEST_DIR="$SRC_DIR/ingest"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
RUN_TESTS=false
for arg in "$@"; do
  case $arg in
    --test)
      RUN_TESTS=true
      shift
      ;;
  esac
done

echo -e "${BLUE}[INFO]${NC} Building ingest module at $INGEST_DIR"

# Navigate to src directory (contains Cargo.toml)
cd "$SRC_DIR"

# Build the project
echo -e "${BLUE}[INFO]${NC} Building Rust project..."
cargo build

# Run tests if requested
if [ "$RUN_TESTS" = true ]; then
  echo -e "${BLUE}[INFO]${NC} Running tests..."
  cargo test
fi

echo -e "${GREEN}[SUCCESS]${NC} Build completed successfully"
