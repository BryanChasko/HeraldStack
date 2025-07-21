#!/bin/bash
# test_text_chunker.sh - Demonstrates the text chunker utility with various inputs and strategies

# Set path to the chunker script
CHUNKER="./scripts/text_chunker.sh"

# Start timing
START_TIME=$(date +%s)

# Function to display timestamp
display_time() {
  local current_time=$(date +"%Y-%m-%d %H:%M:%S")
  echo "🕒 $current_time"
}

# Display start banner
echo "=================================================="
echo "🧩 TEXT CHUNKER UTILITY TEST"
echo "🕒 Started at $(date +"%Y-%m-%d %H:%M:%S")"
echo "=================================================="

# Test size-based chunking
echo
echo "📋 Testing SIZE-BASED CHUNKING (--size)"
echo "--------------------------------------------"
TEST_TEXT="This is a test of the size-based chunking algorithm. It will split this text into chunks of approximately 30 characters, regardless of where word boundaries are."
CHUNK_SIZE=30
echo "Input: $TEST_TEXT"
echo "Chunk size: $CHUNK_SIZE"
echo "Result ($(echo "$TEST_TEXT" | $CHUNKER --size $CHUNK_SIZE --numbered | wc -l | tr -d ' ') chunks):"
echo "$TEST_TEXT" | $CHUNKER --size $CHUNK_SIZE --numbered

# Test character-based chunking
echo
echo "📋 Testing CHARACTER-BASED CHUNKING (--char)"
echo "--------------------------------------------"
TEST_TEXT="This is a test of the character-based chunking algorithm. It will split this text at word boundaries, trying to get close to 30 characters per chunk without breaking words."
CHUNK_SIZE=30
echo "Input: $TEST_TEXT"
echo "Chunk size: $CHUNK_SIZE"
echo "Result ($(echo "$TEST_TEXT" | $CHUNKER --char $CHUNK_SIZE --numbered | wc -l | tr -d ' ') chunks):"
echo "$TEST_TEXT" | $CHUNKER --char $CHUNK_SIZE --numbered

# Test semantic chunking
echo
echo "📋 Testing SEMANTIC CHUNKING (--semantic)"
echo "--------------------------------------------"
TEST_TEXT="This is the first sentence. This is the second sentence. Here's the third one! And a fourth? Yes, that's right."
echo "Input: $TEST_TEXT"
echo "Result ($(echo "$TEST_TEXT" | $CHUNKER --semantic --numbered | wc -l | tr -d ' ') chunks):"
echo "$TEST_TEXT" | $CHUNKER --semantic --numbered

# Test with complex text (names, etc.)
echo
echo "📋 Testing complex text with names"
echo "--------------------------------------------"
TEST_TEXT="Dr. Stephen Strange, also known as Doctor Strange, is a fictional character appearing in American comic books published by Marvel Comics. The character was co-created by Steve Ditko and Stan Lee."
CHUNK_SIZE=50
echo "Using character-based chunking (size: $CHUNK_SIZE)"
echo "Input: $TEST_TEXT"
echo "Result ($(echo "$TEST_TEXT" | $CHUNKER --char $CHUNK_SIZE --numbered | wc -l | tr -d ' ') chunks):"
echo "$TEST_TEXT" | $CHUNKER --char $CHUNK_SIZE --numbered

# Test with JSON input
echo
echo "📋 Testing with JSON input"
echo "--------------------------------------------"
JSON_TEXT='{"character_name":"Vision","first_appearance":"Avengers (1963) #57","affiliations":["Avengers"]}'
CHUNK_SIZE=80
echo "Using size-based chunking (size: $CHUNK_SIZE)"
echo "Input: $JSON_TEXT"
echo "Result ($(echo "$JSON_TEXT" | $CHUNKER --size $CHUNK_SIZE --numbered | wc -l | tr -d ' ') chunks):"
echo "$JSON_TEXT" | $CHUNKER --size $CHUNK_SIZE --numbered
echo
echo "Using character-based chunking (size: $CHUNK_SIZE)"
echo "Result ($(echo "$JSON_TEXT" | $CHUNKER --char $CHUNK_SIZE --numbered | wc -l | tr -d ' ') chunks):"
echo "$JSON_TEXT" | $CHUNKER --char $CHUNK_SIZE --numbered

# Test with markdown input
echo
echo "📋 Testing with markdown input"
echo "--------------------------------------------"
echo "Using semantic chunking"
MARKDOWN_TEXT="# Heading 1
This is paragraph 1. It has multiple sentences. Each should be a separate chunk.

## Heading 2
- List item 1
- List item 2

Final paragraph here."
echo "Input from sample markdown:"
echo "$MARKDOWN_TEXT"
echo "Result ($(echo "$MARKDOWN_TEXT" | $CHUNKER --semantic --numbered | wc -l | tr -d ' ') chunks):"
echo "$MARKDOWN_TEXT" | $CHUNKER --semantic --numbered

# Test with file input
echo
echo "📋 Testing with file input"
echo "--------------------------------------------"
echo "Creating sample file with mixed content..."
TMP_FILE=$(mktemp)
cat > "$TMP_FILE" << EOF
This is a sample file with mixed content. It contains several sentences. Some short ones. And some longer ones that might need to be broken down further if we were using size-based chunking instead of semantic chunking.
# This line looks like a heading
- And this is a list item
The end!
EOF

echo "Using semantic chunking on file"
echo "Result ($(cat "$TMP_FILE" | $CHUNKER --semantic --numbered | wc -l | tr -d ' ') chunks):"
cat "$TMP_FILE" | $CHUNKER --semantic --numbered
rm "$TMP_FILE"

# Test output formats
echo
echo "📋 Testing output formats"
echo "--------------------------------------------"
TEST_ARRAY=("This is chunk 1." "This is chunk 2." "This is chunk 3.")
echo "Standard output:"
printf "%s\n" "${TEST_ARRAY[@]}" | $CHUNKER
echo
echo "JSON output:"
printf "%s\n" "${TEST_ARRAY[@]}" | $CHUNKER --json
echo
echo "Numbered output:"
printf "%s\n" "${TEST_ARRAY[@]}" | $CHUNKER --numbered

# Print summary banner
echo
echo "=================================================="
echo "✅ TEXT CHUNKER TESTS COMPLETE"
echo "=================================================="
echo "🕒 Finished at $(date +"%Y-%m-%d %H:%M:%S")"
