#!/bin/bash
# Checks the status of HARALD system components

echo "🔍 Checking HARALD System Status"
echo "--------------------------------"

# Check if Ollama is running
if pgrep -x "ollama" > /dev/null; then
  echo "✅ Ollama service: RUNNING"
  
  # Get Ollama PID and memory usage
  OLLAMA_PID=$(pgrep -x "ollama")
  if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS memory check
    MEM_USAGE=$(ps -o rss= -p $OLLAMA_PID | awk '{printf "%.2f GB", $1/1024/1024}')
  else
    # Linux memory check
    MEM_USAGE=$(ps -o rss= -p $OLLAMA_PID | awk '{printf "%.2f GB", $1/1024/1024}')
  fi
  echo "   Memory usage: $MEM_USAGE"
  
  # Check API connectivity
  if curl -s "http://localhost:11434/api/version" > /dev/null; then
    VERSION=$(curl -s "http://localhost:11434/api/version" | grep -o '"version":"[^"]*"' | cut -d'"' -f4)
    echo "   Version: $VERSION"
    
    # Try embedding test
    EMBED_TEST=$(curl -s -X POST http://localhost:11434/api/embeddings -d '{"model":"harald-phi4","prompt":"test"}' 2>/dev/null)
    if echo "$EMBED_TEST" | grep -q "embedding"; then
      echo "✅ Embedding API: WORKING"
    else
      echo "❌ Embedding API: NOT RESPONDING"
    fi
  else
    echo "❌ Ollama API: NOT RESPONDING at http://localhost:11434"
  fi
else
  echo "❌ Ollama service: NOT RUNNING (start with 'ollama serve')"
fi

# Check for available models
echo -n "📚 Available models: "
ollama list | grep -v "NAME" | awk '{print $1}' | tr '\n' ', ' | sed 's/,$/\n/'

# Check harald-phi4 model details if available
if ollama list | grep -q "harald-phi4"; then
  MODEL_SIZE=$(ollama list | grep "harald-phi4" | awk '{print $2}')
  echo "   harald-phi4 model size: $MODEL_SIZE"
fi

# Check if index exists
if [ -f "/Users/bryanchasko/Code/HARALD/data/index.hnsw.data" ]; then
  echo "✅ Knowledge index: EXISTS"
  FILESIZE=$(du -h "/Users/bryanchasko/Code/HARALD/data/index.hnsw.data" | cut -f1)
  echo "   Index size: $FILESIZE"
  
  # Count indexed files
  if [ -f "/Users/bryanchasko/Code/HARALD/data/repo.index" ]; then
    DOC_COUNT=$(wc -l < "/Users/bryanchasko/Code/HARALD/data/repo.index")
    echo "📄 Indexed documents: $DOC_COUNT"
  else
    echo "❌ Metadata file missing"
  fi
else
  echo "❌ Knowledge index: NOT FOUND (run ./scripts/ingest.sh first)"
fi

# Check for rust_ingest binary
if [ -f "/Users/bryanchasko/Code/HARALD/rust_ingest/target/release/rust_ingest" ]; then
  echo "✅ rust_ingest binary: FOUND"
else
  echo "❌ rust_ingest binary: NOT FOUND (run 'cd rust_ingest && cargo build --release')"
fi

# Check code status
cd /Users/bryanchasko/Code/HARALD/rust_ingest
if cargo check --quiet; then
  echo "✅ Rust code: COMPILES SUCCESSFULLY"
else
  echo "❌ Rust code: HAS COMPILATION ERRORS"
fi

# Check recent activity
echo "--------------------------------"
echo "🕒 Recent Activity:"

if [ -f "/Users/bryanchasko/Code/HARALD/data/repo.index" ]; then
  MODIFIED=$(stat -f "%Sm" -t "%Y-%m-%d %H:%M:%S" /Users/bryanchasko/Code/HARALD/data/repo.index)
  echo "   Last index update: $MODIFIED"
fi

# Look for recent queries in system logs (last 3)
echo "   Recent queries:"
if [ -d "/Users/bryanchasko/Code/HARALD/logs" ]; then
  if [ -f "/Users/bryanchasko/Code/HARALD/logs/queries.log" ]; then
    tail -n 3 "/Users/bryanchasko/Code/HARALD/logs/queries.log" | sed 's/^/     /'
  else
    echo "     No query logs found"
  fi
else
  echo "     No logs directory found"
fi

echo "--------------------------------"
echo "🧠 To ingest knowledge: ./scripts/ingest.sh"
echo "🔍 To query HARALD: ./scripts/query.sh \"Your question?\""
echo "🔎 To check system status: ./scripts/status.sh"
echo "--------------------------------"
