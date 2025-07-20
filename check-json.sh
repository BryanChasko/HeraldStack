#!/bin/bash
set -e

echo "Validating JSON files..."
find . -name "*.json" -not -path "./node_modules/*" -not -path "./target/*" | while read file; do
  echo "Checking $file"
  jsonlint -q "$file" || exit 1
done

echo "Formatting JSON files..."
find . -name "*.json" -not -path "./node_modules/*" -not -path "./target/*" | xargs prettier --write --parser json

echo "All JSON files processed successfully!"
