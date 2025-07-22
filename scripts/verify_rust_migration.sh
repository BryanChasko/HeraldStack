#!/bin/bash
# verify_rust_migration.sh - Test Rust implementations against original shell scripts
# Usage: ./verify_rust_migration.sh [--clean] [--all]

set -e

# Set base directories
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
RUST_BIN_DIR="${PROJECT_ROOT}/src/target/release"
TEST_DATA_DIR="${PROJECT_ROOT}/tests/data"
TEST_OUTPUT_DIR="${PROJECT_ROOT}/tests/output"

# Ensure test directories exist
mkdir -p "${TEST_DATA_DIR}"
mkdir -p "${TEST_OUTPUT_DIR}"

# Create a test file if it doesn't exist
if [ ! -f "${TEST_DATA_DIR}/test_text.txt" ]; then
  echo "Creating test data file..."
  cat > "${TEST_DATA_DIR}/test_text.txt" << EOF
This is a test file for comparing the shell script and Rust implementations.
We want to make sure that both versions produce identical results.

Here are some paragraphs to chunk:

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec a diam lectus. 
Sed sit amet ipsum mauris. Maecenas congue ligula ac quam viverra nec 
consectetur ante hendrerit. Donec et mollis dolor.

Praesent et diam eget libero egestas mattis sit amet vitae augue. Nam tincidunt 
congue enim, ut porta lorem lacinia consectetur. Donec ut libero sed arcu vehicula 
ultricies a non tortor. Lorem ipsum dolor sit amet, consectetur adipiscing elit.

End of test file.
EOF
fi

# Clean previous test outputs if requested
if [[ "$1" == "--clean" || "$2" == "--clean" ]]; then
  echo "Cleaning previous test outputs..."
  rm -f "${TEST_OUTPUT_DIR}/rust_"*
  rm -f "${TEST_OUTPUT_DIR}/shell_"*
fi

# Check if binaries exist and build if needed
if [ ! -f "${RUST_BIN_DIR}/text_chunker" ]; then
  echo "Building Rust binaries..."
  (cd "${PROJECT_ROOT}/src" && cargo build --release)
fi

# Function to run a specific test
run_test() {
  local test_name=$1
  local shell_script=$2
  local rust_script=$3
  local args=$4
  
  echo "Running test: ${test_name}"
  echo "  Shell script: ${shell_script}"
  echo "  Rust script: ${rust_script}"
  echo "  Args: ${args}"
  
  # Run shell version
  echo "  Running original shell script..."
  ${shell_script} ${args} > "${TEST_OUTPUT_DIR}/shell_${test_name}.out" 2> "${TEST_OUTPUT_DIR}/shell_${test_name}.err"
  
  # Run Rust version
  echo "  Running Rust implementation..."
  ${rust_script} ${args} > "${TEST_OUTPUT_DIR}/rust_${test_name}.out" 2> "${TEST_OUTPUT_DIR}/rust_${test_name}.err"
  
  # Compare outputs
  if diff -q "${TEST_OUTPUT_DIR}/shell_${test_name}.out" "${TEST_OUTPUT_DIR}/rust_${test_name}.out" > /dev/null; then
    echo "  ✅ Output matches!"
  else
    echo "  ❌ Output differs!"
    echo "  Differences:"
    diff "${TEST_OUTPUT_DIR}/shell_${test_name}.out" "${TEST_OUTPUT_DIR}/rust_${test_name}.out" | head -n 10
  fi
  
  # Compare error outputs
  if diff -q "${TEST_OUTPUT_DIR}/shell_${test_name}.err" "${TEST_OUTPUT_DIR}/rust_${test_name}.err" > /dev/null; then
    echo "  ✅ Error output matches!"
  else
    echo "  ❌ Error output differs!"
    echo "  Differences:"
    diff "${TEST_OUTPUT_DIR}/shell_${test_name}.err" "${TEST_OUTPUT_DIR}/rust_${test_name}.err" | head -n 10
  fi
  
  echo ""
}

# Test text_chunker implementations
if [[ "$1" == "--all" || "$2" == "--all" || -z "$1" ]]; then
  # Find original script backup
  if [ -f "${SCRIPT_DIR}/text_chunker.sh.legacy" ]; then
    TEXT_CHUNKER_LEGACY="${SCRIPT_DIR}/text_chunker.sh.legacy"
  else
    echo "⚠️  Original text_chunker.sh.legacy not found, skipping text chunker tests."
    TEXT_CHUNKER_LEGACY=""
  fi
  
  if [ -n "${TEXT_CHUNKER_LEGACY}" ]; then
    echo "Testing text_chunker implementation..."
    
    # Character chunking test
    run_test "text_chunker_character" \
      "${TEXT_CHUNKER_LEGACY}" \
      "${SCRIPT_DIR}/text_chunker.sh" \
      "--input ${TEST_DATA_DIR}/test_text.txt --output ${TEST_OUTPUT_DIR}/chunked_text.txt --strategy character --size 100"
    
    # Size chunking test  
    run_test "text_chunker_size" \
      "${TEXT_CHUNKER_LEGACY}" \
      "${SCRIPT_DIR}/text_chunker.sh" \
      "--input ${TEST_DATA_DIR}/test_text.txt --output ${TEST_OUTPUT_DIR}/chunked_text.txt --strategy size --size 200"
  fi
  
  # Find original embedding test script backup
  if [ -f "${SCRIPT_DIR}/test_embedding_size.sh.legacy" ]; then
    EMBEDDING_TEST_LEGACY="${SCRIPT_DIR}/test_embedding_size.sh.legacy"
  else
    echo "⚠️  Original test_embedding_size.sh.legacy not found, skipping embedding tests."
    EMBEDDING_TEST_LEGACY=""
  fi
  
  if [ -n "${EMBEDDING_TEST_LEGACY}" ]; then
    echo "Testing embedding size implementation..."
    
    # Basic embedding test (if Ollama is running)
    if curl -s http://localhost:11434/api/embeddings > /dev/null; then
      run_test "embedding_size" \
        "${EMBEDDING_TEST_LEGACY}" \
        "${SCRIPT_DIR}/test_embedding_size.sh" \
        "--model llama3 --text \"This is a short test text\" --log ${TEST_OUTPUT_DIR}/embed_test.log"
    else
      echo "⚠️  Ollama API not available, skipping embedding tests."
    fi
  fi
fi

echo "All tests completed!"
