#!/bin/bash
# test_migration.sh - Automated testing script for shell-to-Rust migrations
#
# Usage: ./test_migration.sh <script_name> [options]
#
# This script helps verify that Rust implementations are functionally
# equivalent to their shell script predecessors.

set -euo pipefail

SCRIPT_NAME="$1"
RUST_BINARY="./target/release/${SCRIPT_NAME}"
LEGACY_SCRIPT="./scripts/**/${SCRIPT_NAME}.sh.legacy"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

test_help_output() {
    log_info "Testing help output..."
    
    echo "=== Rust Implementation ==="
    if ! $RUST_BINARY --help; then
        log_error "Rust binary help failed"
        return 1
    fi
    
    echo -e "\n=== Legacy Implementation ==="
    if ! $LEGACY_SCRIPT --help 2>/dev/null; then
        log_warn "Legacy script help not available or failed"
    fi
    
    log_info "Help output test completed"
}

test_build() {
    log_info "Building Rust binary..."
    if cargo build --release; then
        log_info "Build successful"
    else
        log_error "Build failed"
        return 1
    fi
}

test_basic_functionality() {
    log_info "Testing basic functionality..."
    
    # This is a placeholder - specific tests should be added per script
    case "$SCRIPT_NAME" in
        "format_json")
            test_format_json_basic
            ;;
        "ingest_chunked")
            test_ingest_chunked_basic
            ;;
        *)
            log_warn "No specific tests defined for $SCRIPT_NAME"
            ;;
    esac
}

test_format_json_basic() {
    log_info "Testing format_json basic functionality..."
    
    # Test check mode (should not modify files)
    if $RUST_BINARY --check; then
        log_info "format_json --check passed"
    else
        log_warn "format_json --check failed or returned warnings"
    fi
    
    # Test registry validation
    if $RUST_BINARY --validate-registry; then
        log_info "format_json --validate-registry passed"
    else
        log_warn "format_json --validate-registry failed"
    fi
}

test_ingest_chunked_basic() {
    log_info "Testing ingest_chunked basic functionality..."
    
    # Just test that help works - actual ingestion requires data files
    log_info "ingest_chunked requires data files for full testing"
    log_info "Consider running with actual data files manually"
}

test_performance() {
    log_info "Performance testing not implemented yet"
    log_warn "TODO: Add performance comparison tests"
}

test_edge_cases() {
    log_info "Edge case testing not implemented yet"
    log_warn "TODO: Add edge case tests (invalid args, missing files, etc.)"
}

main() {
    if [[ $# -eq 0 ]]; then
        echo "Usage: $0 <script_name> [test_type]"
        echo "Example: $0 format_json"
        exit 1
    fi
    
    log_info "Starting migration test for $SCRIPT_NAME"
    
    # Verify files exist
    if [[ ! -f "$RUST_BINARY" ]]; then
        log_warn "Rust binary not found at $RUST_BINARY, attempting build..."
        test_build
    fi
    
    # Run tests
    test_help_output
    test_basic_functionality
    
    log_info "Migration test completed for $SCRIPT_NAME"
    log_warn "Remember to run manual tests with real data!"
}

main "$@"
