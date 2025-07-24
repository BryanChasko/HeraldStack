# Ollama API Embedding Limits

**Created**: July 2025  
**Last Updated**: July 24, 2025  
**Version**: 1.0

## Overview

This document outlines key findings about Ollama API embedding limits and
optimal chunking strategies. These insights were gained through systematic
testing of text embedding generation through the Ollama API during our ingestion
process. These findings should guide our approach to processing text for vector
embedding generation.

## Key Findings

Through systematic testing, we have established:

1. **Character Limit**: The Ollama API has a practical limit of around 300-400
   characters per request
   - Requests with more than 400 characters frequently timeout
   - This may be a system resource constraint rather than a fundamental API
     limitation
   - For reliable processing, keep chunks under 250 characters

2. **Timeout Behavior**: Longer texts consistently lead to API timeouts
   - Timeout occurs without error feedback
   - Increasing the timeout parameter doesn't reliably solve the issue

3. **Embedding Dimensions**: The harald-phi4 model produces 3072-dimensional
   embeddings
   - This dimensionality is consistent regardless of input text length

## Impact on Ingestion Process

These limitations significantly affect how we process text data for embedding:

1. **Small Chunks Required**: Documents must be broken into small chunks (< 250
   chars)
2. **Semantic Quality Trade-off**: Small chunks may lose important context
3. **Processing Overhead**: More chunks means more API calls and higher
   processing time

## Recommended Approach

### Character-Based Attribute Processing

For structured data like our Marvel character JSON:

```json
{
  "character_name": "Vision",
  "first_appearance": "Avengers #57",
  "core_attributes": ["Synthetic being", "AI consciousness"],
  ...
}
```

This approach requires modifying the ingestion pipeline to:

1. Process each attribute/field separately
2. Generate separate embeddings for each attribute
3. Store metadata to maintain relationships between chunks

### System Optimization

If you need to process larger chunks (500-600 characters):

1. Close the Ollama GUI application if running
2. Use the command line to run the standalone server:

   ```bash
   ollama serve
   ```

3. Wait a few seconds for the server to initialize
4. Try embedding requests with larger chunks

The character limit may be a constraint related to system resources, not a
fundamental API limitation. Using the standalone server without the GUI
application may allow processing of larger chunks.

## References

- [Rust Ingest README](../../rust_ingest/rustREADME.md)
- [JSONL Ingestion Documentation](./jsonl-ingestion.md)
- [HNSW Best Practices](./hnsw-best-practices.md)
