#!/bin/bash
set -e

# Change to the rust_ingest directory where Cargo.toml is located
cd "$(dirname "$0")/../../rust_ingest" || exit 1
echo "Checking Rust files in $(pwd)..."

echo "Running rustfmt..."
cargo fmt -- --check

echo "Running clippy..."
cargo clippy -- -D warnings

echo "Running tests..."
cargo test

echo "Running security audit..."
if command -v cargo-audit > /dev/null 2>&1; then
    cargo audit
else
    echo "cargo-audit not found, skipping security audit"
fi

echo "All checks passed!"
