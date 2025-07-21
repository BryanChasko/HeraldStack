# JSONL Format for Vector Embedding

This document outlines our approach to using JSON Lines (JSONL) format for
efficient vector embedding and ingestion.

## What is JSONL?

JSONL (JSON Lines) is a convenient format for storing structured data that may
be processed one record at a time. It works well for processing large datasets
and streaming workloads.

Key characteristics:

- Each line is a valid JSON value (typically an object or array)
- Lines are separated by newlines (`\n`)
- UTF-8 encoding is used
- No comma between records (unlike standard JSON arrays)

Example JSONL file:

```jsonl
{"character_name": "Vision", "first_appearance": "Avengers (1963) #57"}
{"character_name": "Tony Stark A.I.", "first_appearance": "Iron Man (2016) #1"}
{"character_name": "N.A.T.A.L.I.E.", "first_appearance": "Ironheart (2018) #1"}
```

## Why JSONL for Vector Embedding?

### Streaming Processing

- Process one record at a time without loading the entire dataset
- Better memory efficiency for large datasets
- Can generate embeddings incrementally

### Reduced Timeout Issues

- Smaller, well-defined chunks for embedding API calls
- Less likely to hit API timeout limits
- More resilient embedding pipeline

### Simplified Error Handling

- Failures affect only individual records, not the entire batch
- Easier to retry specific failed records
- Better visibility into processing status

## Converting JSON to JSONL

To convert standard JSON arrays to JSONL format:

```bash
# Basic conversion of a JSON array to JSONL
jq -c '.[]' input.json > output.jsonl

# Convert and transform each object
jq -c '.[] | {id: .character_name, text: .traits[]}' input.json > output.jsonl
```

## Ollama API Limitations and Chunking Strategy

Through testing, we've discovered that the Ollama API has specific limitations:

- **Character Limit**: ~300-400 characters per request
- **Timeout Behavior**: Longer texts consistently lead to timeouts
- **Performance Impact**: This limit affects how we structure our embedding
  pipeline

This character limit appears to be a system resource constraint rather than a
fundamental API limitation. For optimal performance:

1. **Close the Ollama GUI application** if it's running
2. Use `ollama serve` from the command line to start the standalone server
3. Wait a few seconds for the server to initialize
4. Try embedding requests with somewhat larger chunks (up to 500-600 characters)

For more details, see our
[Ollama API Embedding Limits](./ollama-embedding-limits.md) documentation.

## Implementation in Our Pipeline

We've implemented JSONL processing in our ingest pipeline with these limitations
in mind:

### Step 1: Extract records from the source JSON file

```bash
jq -c '.[]' "$SOURCE_FILE" > "$JSONL_FILE"
```

### Step 2: Validate each JSONL line to ensure proper formatting

```bash
validate_jsonl "$JSONL_FILE"
```

### Step 3: Process each attribute separately to stay within character limits

```bash
# Process character attributes individually
for attribute in "character_name" "first_appearance" "affiliations" "core_attributes"; do
  value=$(jq -r ".$attribute" <<< "$json_record")
  generate_embedding "$value" "$LOG_FILE"
done
```

### Step 4: Store embeddings with proper metadata to maintain relationships

## References

- [JSON Lines Specification](http://jsonlines.org/)
- [JSON Lines Examples](http://jsonlines.org/examples/)
- [Ollama Embedding API](https://ollama.com/blog/embedding-models)
- [HNSW Vector Search in Rust](https://docs.rs/hnsw_rs/)
