#!/bin/bash
find . -name "*.md" -not -path "./node_modules/*" | xargs prettier --write --parser markdown --print-width 80 --prose-wrap always
