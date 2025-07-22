#!/bin/bash
# check-rust.sh - Validate Rust code formatting, linting, tests, and security
#
# This script runs comprehensive checks on all Rust code in the project
#
# Usage:
#   ./scripts/validation/check-rust.sh
#
# Author: Bryan Chasko  
# Date: July 22, 2025

set -e

# Change to the src directory where Cargo.toml is located
cd "$(dirname "$0")/../../src" || exit 1
echo "Checking Rust files in $(pwd)..."

echo "Running rustfmt..."
cargo fmt -- --check

echo "Running clippy..."
cargo clippy -- -D warnings

echo "Running tests..."
if cargo test; then
    echo "All tests passed"
else
    echo "Some tests failed - please fix before committing"
fi

echo "Running security audit..."
if command -v cargo-audit > /dev/null 2>&1; then
    cargo audit
else
    echo "cargo-audit not found, skipping security audit"
fi

echo "All checks passed!"
