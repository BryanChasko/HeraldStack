#!/bin/bash
set -e

echo "Running rustfmt..."
cargo fmt -- --check

echo "Running clippy..."
cargo clippy -- -D warnings

echo "Running tests..."
cargo test

echo "Running security audit..."
cargo audit

echo "All checks passed!"
