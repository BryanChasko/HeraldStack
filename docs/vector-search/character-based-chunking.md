# Character-Based Text Chunking for Optimal Embeddings

## Overview

Character-based text chunking is a method for dividing text into smaller units
that respects word boundaries and meaningful content segments. This approach was
developed to address the specific limitations of embedding APIs while preserving
semantic meaning.

## Why Character-Based Chunking?

When generating embeddings for text, we face a fundamental trade-off:

1. **Larger chunks** preserve more context but may exceed API limits
2. **Smaller chunks** stay within API limits but may lose important context
3. **Arbitrary splitting** often breaks words and semantic units

Character-based chunking addresses these challenges by:

- Preserving whole words rather than cutting mid-word
- Respecting natural breaks in text like punctuation
- Maintaining semantic units where possible
- Staying within API character limits

## Comparison with Other Chunking Methods

<!-- markdownlint-disable MD013 -->

| Method              | Description                                 | Pros                       | Cons                                            |
| ------------------- | ------------------------------------------- | -------------------------- | ----------------------------------------------- |
| **Size-based**      | Splits text at exact character counts       | Simple, predictable        | Breaks words, loses semantics                   |
| **Character-based** | Splits at word boundaries near target size  | Preserves words, readable  | May have variable chunk sizes                   |
| **Semantic**        | Splits at natural sentence/paragraph breaks | Maintains meaning, context | Often results in chunks too large for embedding |

## Implementation

Our character-based chunker follows this algorithm:

```sh
function characterBasedChunk(text, targetSize):
    if length(text) <= targetSize:
        return [text]

    chunks = []
    remaining = text

    while length(remaining) > targetSize:
        # Find the last word boundary before targetSize
        cutPoint = findLastWordBoundary(remaining, targetSize)

        # Extract chunk and add to results
        chunk = remaining.substring(0, cutPoint)
        chunks.add(chunk)

        # Update remaining text
        remaining = remaining.substring(cutPoint).trim()

    # Add final chunk if anything remains
    if length(remaining) > 0:
        chunks.add(remaining)

    return chunks
```

## Application to Different Content Types

Character-based chunking is particularly effective for:

### Structured Data

```json
{
  "character_name": "Vision",
  "first_appearance": "Avengers #57",
  "core_attributes": ["Synthetic being", "AI consciousness"]
}
```

With structured data, we can:

1. Process each field separately
2. Preserve entity names and attributes intact
3. Maintain the relationship between fields through metadata

### Natural Text

For natural language text like documentation, articles, or stories:

1. Split at paragraph or sentence boundaries when possible
2. Otherwise split at word boundaries near the target size
3. Keep important phrases together when recognized

## Best Practices

For optimal embedding results:

1. **Choose appropriate chunk size**: 200-250 characters is ideal for Ollama
2. **Process structured data by attributes**: Keep related fields together
3. **Preserve entity names**: Don't split character names or key terms
4. **Store metadata**: Track relationships between chunks for reassembly
5. **Consider semantic units**: Try to keep sentences intact when possible

## Implementation in HARALD

In the HARALD project, we've implemented character-based chunking:

1. In our `text_chunker.sh` utility with the `--char` mode
2. In our structured data processing for Marvel character data
3. In our documentation processing pipeline

## References

- [Ollama API Embedding Limits](./ollama-embedding-limits.md)
- [JSONL Ingestion Documentation](./jsonl-ingestion.md)
- [HNSW Best Practices](./hnsw-best-practices.md)
