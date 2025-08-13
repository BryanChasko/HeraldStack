# HNSW Best Practices

<!-- markdownlint-disable MD052 MD013 MD024 MD051 -->

This file contains code examples with syntax that may trigger MD052 "Reference
not found" warnings, but they are valid code samples. This rule has been
disabled at the top of the file.

<!-- markdownlint-disable-file MD052 MD013 MD024 MD051 -->

This document outlines best practices for working with Hierarchical Navigable
Small World (HNSW) indices in the HARALD project.

## Table of Contents

- [Quick Reference](#quick-reference)
- [Introduction](#introduction)
- [Configuration Parameters](#configuration-parameters)
- [Performance Considerations](#performance-considerations)
- [Memory Usage](#memory-usage)
- [Index Persistence](#index-persistence)
- [Query Optimization](#query-optimization)
- [Common Pitfalls](#common-pitfalls)
- [Implementation Lessons](#implementation-lessons)
- [API Reference](#api-reference)
- [References](#references)

## Quick Referenceå

| Task               | Method                    | Example                                           |
| ------------------ | ------------------------- | ------------------------------------------------- |
| Create index       | `Hnsw::new()`             | `Hnsw::new(384, 100_000, 16, 200, DistCosine {})` |
| Insert data        | `index.insert()`          | `index.insert((vector.as_slice(), id))`           |
| Batch insert       | `index.parallel_insert()` | `index.parallel_insert(&data_points)`             |
| Search             | `index.search()`          | `index.search(&query, 5, 50)`                     |
| Batch search       | `index.parallel_search()` | `index.parallel_search(&queries, 5, 50)`          |
| Set search quality | `index.set_ef()`          | `index.set_ef(50)`                                |
| Save index         | `index.file_dump()`       | `index.file_dump(&path_dir, "index_basename")?`   |
| Load index         | `HnswIo::load_hnsw()`     | `HnswIo::new(&path_dir, "basename").load_hnsw()?` |

### Key Parameters

- **Dimensions**: Match your embedding size (e.g., 384 for OpenAI Ada2)
- **Max Elements**: ~1.5x your expected dataset size
- **M**: 12-16 (general), 5-8 (memory-constrained), 20-30 (speed-critical)
- **EF Construction**: 100 (faster build), 200 (balanced), 400+ (highest
  quality)
- **EF Search**: 20-50 (interactive), 100+ (accuracy-critical), 400+
  (exhaustive)

### Common Issues

- **Persistence**: Use `file_dump()` and `HnswIo::load_hnsw()`, not
  `save_hnsw()`
- **Lifetimes**: Handle index lifetimes carefully when loading
- **Type Safety**: Be explicit with generic type parameters
- **Memory**: Monitor usage with `dimension × 4 + M × 12` bytes per vector

## Introduction

HNSW (Hierarchical Navigable Small World) is an algorithm for finding similar
items in large datasets without comparing against every single item. Think of it
like looking for a book in a well-organized library rather than searching
through every book on every shelf.

When dealing with vector embeddings (like those produced by language models), we
need to quickly find which vectors are most similar to a query vector.
Traditional methods that check every vector become too slow when dealing with
millions of items. HNSW solves this by creating multiple "layers" of connections
between vectors, with each higher layer being a shortcut through the data.

The `hnsw_rs` crate (version 0.3.2) is a Rust library that implements this
algorithm with several practical features:

- **Fast Search**: Finds similar vectors quickly (logarithmic time complexity)
  even with millions of vectors

- **Different Similarity Measures**: Supports various ways of measuring how
  similar vectors are (cosine similarity for semantic meaning, euclidean for
  spatial distance, and more)

- **Save & Load**: Can save your vector index to disk and load it later,
  avoiding the need to rebuild it from scratch

- **Thread Safety**: Multiple parts of your program can safely use the index at
  the same time

- **Adjustable Accuracy**: Lets you trade search accuracy for speed depending on
  your needs

## Configuration Parameters

The `hnsw_rs` library requires several key parameters when initializing an HNSW
index. Think of these as the blueprint for how your search index will be built
and behave.

Here's a typical example of creating an HNSW index:

```rust
let dimensions = 384; // Vector dimensions (must match your embedding size)
let max_elements = 10_000; // Maximum number of elements in the index
let ef_construction = 200; // Controls index quality (higher = better, slower)
let m = 16; // Maximum number of connections per element
let ef_search = 20; // Controls search quality (higher = better, slower query)

let index = hnsw_rs::Hnsw::<f32, DistCosine>::new(
    dimensions,
    max_elements,
    m,
    ef_construction,
    DistCosine {}
);
```

### Key Parameter Recommendations

- **Dimensions (d)**: This must exactly match the size of your vector
  embeddings. For example, if you're using OpenAI's text-embedding-3-small model
  which outputs 1536-dimensional vectors, you would set `dimensions = 1536`.

- **Max Elements**: How many vectors your index can hold. Think of this as
  pre-allocating memory. Set this to about 1.5 times your expected dataset size
  to avoid costly memory reallocations. For example, if you expect to store
  100,000 documents, set this to 150,000.

- **M**: This is the maximum number of connections each point can have to other
  points. Higher values create more shortcuts, making searches faster but using
  more memory:
  - 12-16 for general use (good balance)
  - 5-8 for memory-constrained systems
  - 20-30 for speed-critical applications with available memory

- **EF Construction**: This controls how thoroughly the algorithm explores the
  space when building the index. Higher values create better quality indices but
  take longer to build:
  - 100: Faster building, slightly lower quality
  - 200: Good balance for most applications
  - 400+: Very high quality, but much slower to build

- **EF Search**: Controls how thoroughly the algorithm searches the graph at
  query time. Higher values improve accuracy but slow down searches:
  - 20-50: Good for interactive applications where speed matters
  - 100+: Better for applications where accuracy is critical
  - 400+: Very high accuracy, but significantly slower

## Performance Considerations

When building HARALD's semantic memory system using HNSW, performance becomes
crucial as the knowledge base grows. Bryan should consider these practical
performance optimizations:

### Index Building Speed

The `hnsw_rs` library builds indices on a single thread by default. This means
adding thousands of items can become slow on complex datasets. For HARALD's
extensive knowledge base:

```rust
// Option 1: Break data into chunks and build separate indices
let mut indices = Vec::new();
for chunk in data.chunks(10_000) {
    let mut index = create_index();
    // Add chunk to index
    indices.push(index);
}

// Option 2: Build an index partitioned by topic/domain
let mut personal_journal_index = create_index();
let mut work_projects_index = create_index();
let mut research_notes_index = create_index();
// Populate indices separately
```

### Vector Representation

The library expects vectors as `Vec<f32>` or `ndarray::Array1<f32>`. When
working with embeddings from different sources:

```rust
// Convert from OpenAI API response format
fn convert_openai_embedding(response: &OpenAiResponse) -> Vec<f32> {
    response.data[0].embedding.clone()
}

// Convert from tensor format if using local models
fn convert_from_tensor(tensor: &Tensor) -> Vec<f32> {
    tensor.to_vec1()
}
```

### Memory Efficiency Techniques

For Bryan's MacBook Air with limited RAM:

```rust
// 1. Batch processing for better cache locality
for batch in vectors.chunks(1000) {
    for vector in batch {
        index.insert(vector, id, None)?;
        id += 1;
    }
}

// 2. Memory-mapping for large indices
let mmap_index = MmapIndex::new(index, "path/to/disk_storage")?;
```

### Benchmarking for Bryan's Workflow

Different types of information benefit from different parameters. For HARALD,
benchmark using Bryan's actual search patterns:

```rust
// Test typical queries Bryan might make
fn benchmark_ef_values() -> Vec<(u32, Duration, f32)> {
    let queries = vec![
        "meeting notes from last week",
        "ideas for creative projects",
        "health tracking data",
    ];

    let ef_values = vec![10, 20, 50, 100, 200];
    let mut results = Vec::new();

    for ef in ef_values {
        index.set_ef(ef);
        let (duration, accuracy) = test_queries(&index, &queries);
        results.push((ef, duration, accuracy));
    }
    results
}
```

### Real-time vs. Background Processing

For HARALD's user experience, separate time-sensitive and background operations:

```rust
// Quick search for interactive use
fn quick_search(
    query: &str,
    index: &Hnsw<f32, DistCosine>
) -> Vec<SearchResult> {
    // Lower dimension or faster model
    let embedding = embed_text_quickly(query);
    index.set_ef(20); // Lower quality, faster search
    index.search(&embedding, 3, 20)
}

// Deep search for background analysis
fn deep_search(
    query: &str,
    index: &Hnsw<f32, DistCosine>
) -> Vec<SearchResult> {
    let embedding = embed_text_high_quality(query);
    index.set_ef(100); // Higher quality, slower search
    index.search(&embedding, 10, 100)
}
```

## Memory Usage

Understanding memory consumption is especially important for HARALD's system
running on Bryan's MacBook Air with 8GB of RAM. The good news is that HNSW is
relatively memory-efficient compared to some other vector search algorithms.

### What Consumes Memory in HNSW?

Memory usage comes from two main components:

1. **Vector Data**: This is the raw embedding data that represents Bryan's
   information. Each floating point number takes 4 bytes of memory.

   ```plaintext
   Vector Memory = Number of items × Dimensions × 4 bytes
   ```

1. **Graph Structure**: This is the network of connections between items that
   makes fast searching possible. It depends primarily on the M parameter
   (maximum connections per node).

   ```plaintext
   Graph Memory = Number of items × M × ~12 bytes
   ```

### Memory Usage Formula

For practical planning, use this approximate formula:

```plaintext
Total Memory (bytes) ≈ num_vectors × (dimension × 4 + M × 12)
```

### Real-World Examples for HARALD

Let's consider some realistic scenarios for Bryan's system:

#### Small Knowledge Base (10,000 items)

For 10,000 items with 384-dimensional vectors and M=16:

- Vector data: 10,000 × 384 × 4 bytes = ~15 MB
- Graph structure: 10,000 × 16 × 12 bytes = ~1.9 MB
- Total: ~17 MB (easily fits in memory)

#### Medium Knowledge Base (100,000 items)

For 100,000 items with 384-dimensional vectors and M=16:

- Vector data: 100,000 × 384 × 4 bytes = ~153 MB
- Graph structure: 100,000 × 16 × 12 bytes = ~19 MB
- Total: ~172 MB (still comfortable)

#### Large Knowledge Base (1 million items)

For 1,000,000 items with 384-dimensional vectors and M=16:

- Vector data: 1,000,000 × 384 × 4 bytes = ~1.5 GB
- Graph structure: 1,000,000 × 16 × 12 bytes = ~192 MB
- Total: ~1.7 GB (significant but manageable)

### Memory Optimization Strategies for HARALD

If Bryan starts reaching memory limits as his knowledge base grows:

1. **Dimension Reduction**: Consider using smaller embeddings when appropriate.
   Going from 384 to 256 dimensions can save ~30% memory with minimal loss in
   quality for many use cases.

1. **Adjust M Parameter**: Reducing M from 16 to 8 halves the graph memory
   requirement with some trade-off in search quality.

1. **Segmented Indices**: Split data into multiple specialized indices by topic,
   time period, or importance:

   ```rust
   // Create separate indices for different domains
   let personal_index = Hnsw::new(384, 100_000, 16, 200, DistCosine {});
   let work_index = Hnsw::new(384, 50_000, 16, 200, DistCosine {});
   let reference_index = Hnsw::new(384, 200_000, 16, 200, DistCosine {});

   // Search across multiple indices when needed
   fn search_all_indices(query: &[f32]) -> Vec<SearchResult> {
       let mut results = Vec::new();
       results
   }
   ```

1. **Memory-Mapped Files**: For very large indices, consider memory mapping to
   keep only active portions in RAM:

   ```rust
   // Save indices to disk
   personal_index.file_dump(Path::new("path/to"), "personal_idx")?;

   // Load only when needed using memory mapping
   // Note: Proper memory mapping requires additional configuration
   let mut hnsw_loader = HnswIo::new(Path::new("path/to"), "personal_idx");
   let mapped_index = hnsw_loader.load_hnsw()?;
   ```

1. **Quantization**: For extreme memory constraints, consider quantizing the
   vector values from 32-bit floats to 8-bit integers, reducing vector memory by
   75% with some accuracy loss.

These strategies allow HARALD to scale efficiently as Bryan's knowledge base
grows over time, even on hardware with memory constraints.

## Index Persistence

One of the key features of HNSW is the ability to save your index to disk and
reload it later. This is critical for HARALD's system, as it allows Bryan to
preserve his knowledge base between sessions without having to rebuild the index
from scratch (which would be time-consuming).

### Basic Saving and Loading

The `hnsw_rs` library offers methods to persist indices:

```rust
// Save index to disk using file_dump
// This creates two files:
// - path/to/basename.hnsw.data
// - path/to/basename.hnsw.graph
index.file_dump(Path::new("path/to"), "basename")?;

// Load index from disk using HnswIo
let mut hnsw_loader = HnswIo::new(Path::new("path/to"), "basename");
let loaded_index = hnsw_loader.load_hnsw()?;
```

### Comprehensive Persistence Strategy for HARALD

For Bryan's knowledge management system, a more robust approach is needed:

```rust
// 1. Create a timestamp for versioning
let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();

// 2. Save the index with version info
let data_dir = Path::new(root_dir).join("data");
let basename = format!("hnsw_index_{}", timestamp);
index.file_dump(&data_dir, &basename)?;

// 3. Save metadata separately (with the same version tag)
let metadata_path = format!("{}/data/hnsw_meta_{}.json", root_dir, timestamp);
let metadata_file = fs::File::create(&metadata_path)?;
serde_json::to_writer(metadata_file, &document_metadata)?;

// 4. Update a pointer file to the latest version
let pointer_path = format!("{}/data/latest_index.txt", root_dir);
fs::write(&pointer_path, format!("{}\n{}", index_path, metadata_path))?;
```

### Best Practices for HARALD's Knowledge Base

1. **Separate Index and Metadata**: Always store the actual document content and
   metadata separately from the index. The index should contain only:

- The vector embeddings
- IDs that can be used to look up the original content

1. **Implement Validation on Load**: Before using a loaded index, verify its
   integrity:

   ```rust
   fn validate_index(
       index: &Hnsw<f32, DistCosine>,
       metadata: &Vec<DocumentInfo>
   ) -> Result<()> {
       // Check dimension consistency
       if index.get_dimensions() != EXPECTED_DIMENSIONS {
           return Err(anyhow!("Index dimensions mismatch"));
       }

       // Check point count matches metadata
       if index.get_point_count() != metadata.len() {
           return Err(anyhow!("Index and metadata count mismatch"));
       }

       // Test a simple search to verify functionality
       let test_query = vec![0.0; EXPECTED_DIMENSIONS];
       let _ = index.search(&test_query, 1, 10)?;

       Ok(())
   }
   ```

1. **Version Your Indices**: Implement a versioning system for Bryan's growing
   knowledge base:

- Keep a history of index versions
- Implement a rollback mechanism
- Log changes between versions

1. **Implement Recovery Strategies**: Prepare for potential corruption:

   ```rust
   fn load_index_with_fallback(
       root_dir: &Path
   ) -> Result<(Hnsw<f32, DistCosine>, Vec<DocumentInfo>)> {
       // Try loading the latest version
       let result = load_latest_index(root_dir);

       if result.is_ok() {
           return result;
       }

       // If failed, try loading the previous version
       log::warn!("Failed to load latest index, trying previous version");
       let previous_version = find_previous_index_version(root_dir)?;

       // Load using proper HnswIo interface
       let dir_path = Path::new(&previous_version.dir_path);
       let basename = &previous_version.basename;

       let mut hnsw_loader = HnswIo::new(dir_path, basename);
       let index = hnsw_loader.load_hnsw()?;

       let metadata = load_metadata(&previous_version.metadata_path)?;

       // Rebuild latest from previous
       rebuild_latest_index(&index, &metadata, root_dir)?;

       Ok((index, metadata))
   }
   ```

**Regular Backups**: Especially before large updates to the index, create
complete backups of Bryan's knowledge base.

By following these practices, HARALD can maintain a reliable and recoverable
knowledge base that grows alongside Bryan without risking data loss.

## Query Optimization

When Bryan asks HARALD to retrieve relevant information or find similar content,
the quality and speed of these searches depend heavily on how we configure and
execute queries against the HNSW index.

Here's how to optimize queries for Bryan's experience:

```rust
// Set search parameters for better quality
let ef_search = 100; // Higher for more accurate results
index.set_ef(ef_search);

// Perform search
let query_vector = vec![0.1, 0.2, ..., 0.3];
let search_results = index.search(&query_vector, 5, ef_search);

// Process results
for neighbor in search_results {
    println!("Found item {} with distance {}",
             neighbor.d_id, neighbor.distance);
}
```

### Understanding Query Parameters

- **EF Search Parameter**: This controls the trade-off between search speed and
  accuracy. Think of it like how carefully you look through a bookshelf:
  - **Low values (10-20)**: Quick but might miss relevant items
  - **Medium values (50-100)**: Good balance for most of Bryan's queries
  - **High values (200+)**: Very thorough but slower, best for critical research

- **Number of Results**: The second parameter in the `search()` function (5 in
  the example) determines how many "nearest neighbors" to return. For HARALD's
  use cases:
  - **3-5 results**: Good for quick contextual lookups
  - **10-20 results**: Better for comprehensive research when Bryan needs depth
  - **50+ results**: Only when exhaustive coverage is needed

### Advanced Optimization Techniques

- **Context-Aware EF**: Adjust `ef_search` dynamically based on Bryan's needs:

  ```rust
  // For casual browsing
  let ef_search = 20;

  // For important work projects
  let ef_search = 100;

  // For critical research where accuracy is paramount
  let ef_search = 200;
  ```

- **Batch Processing**: When Bryan wants to compare multiple concepts at once:

  ```rust
  // Much faster than running individual searches
  let query_vectors = vec![vector1, vector2, vector3];
  let batch_results = index.parallel_search(&query_vectors, 5, ef_search);
  ```

- **Query Caching**: For Bryan's frequently accessed topics:

  ```rust
  // Simple cache implementation
  let mut cache = HashMap::new();
  if let Some(results) = cache.get(&query_hash) {
      return results.clone();
  }
  let results = index.search(&query_vector, 5, ef_search);
  cache.insert(query_hash, results.clone());
  ```

- **Time-Sensitive Searches**: For recency-weighted results that favor Bryan's
  latest information:

  ```rust
  // Apply time decay to search results
  let results = index.search(&query_vector, 10, ef_search)
      .iter()
      .map(|n| {
          // Adjust score based on document age
          let adjusted_score = n.distance * time_decay_factor(&metadata[n.d_id]);
          (n, adjusted_score)
      })
      .collect();
  ```

## Common Pitfalls

When implementing HNSW for HARALD's memory system, several common issues can
arise. Understanding these in advance will save Bryan significant debugging time
and ensure more reliable performance.

### Dimension Mismatch

**Problem**: One of the most common and frustrating errors occurs when query
vectors don't match the dimensions of the index.

**Example**: If HARALD's embeddings from a language model are 384-dimensional,
but you query with a 1536-dimensional vector from a different model, the search
will fail or return meaningless results.

**Solution**: Always verify embedding dimensions match across your entire
pipeline. For HARALD's system:

```rust
// Good practice: Make dimensions a configurable constant
const EMBEDDING_DIMENSIONS: usize = 384;

// Validate vector dimensions before insertion
fn validate_vector(vector: &[f32]) -> Result<()> {
    if vector.len() != EMBEDDING_DIMENSIONS {
        return Err(anyhow!("Vector dimension mismatch: expected {}, got {}",
            EMBEDDING_DIMENSIONS, vector.len()));
    }
    Ok(())
}
```

### Distance Metric Confusion

**Problem**: Each distance metric has different interpretations of "similarity"
and different score ranges.

**Example**: With `DistCosine`, lower values (closer to 0) mean more similar,
while with `DistDot`, higher values indicate greater similarity.

**Solution**: Normalize scores according to the distance metric and establish
consistent thresholds:

```rust
// For cosine distance (1 - cosine similarity)
// Range: 0 (identical) to 2 (opposite)
fn is_relevant_cosine(distance: f32) -> bool {
    distance < 0.15 // Highly similar (85%+ similarity)
}

// For dot product (varies based on vector magnitudes)
fn is_relevant_dot(dot_product: f32, magnitude_threshold: f32) -> bool {
    dot_product > magnitude_threshold
}
```

### Missing Error Handling

**Problem**: HNSW operations can fail due to I/O errors, memory constraints, or
invalid parameters.

**Solution**: Implement comprehensive error handling, especially for operations
that Bryan depends on for critical tasks:

```rust
// Always use proper error handling with context
match index.search(&query, 5, ef_search) {
    Ok(results) => {
        // Process results
    },
    Err(e) => {
        log::error!("Search failed: {}", e);
        // Implement fallback strategy
        fallback_search_strategy(&query)?;
    }
}
```

### Memory Limitations

**Problem**: Large indices with millions of high-dimensional vectors can exceed
available memory, especially on Bryan's MacBook Air with 8GB RAM.

**Solution**: Implement progressive loading, sharding, or disk-based fallback:

```rust
// Monitor memory usage and implement adaptive strategies
fn load_appropriate_index() -> Result<Hnsw<f32, DistCosine>> {
    let available_memory = get_available_memory_mb()?;

    if available_memory < 1000 {  // Less than 1GB free
        // Load smaller, core knowledge index only
        return load_core_index();
    } else {
        // Load full index
        return load_complete_index();
    }
}
```

### Not Tuning Parameters

**Problem**: Default parameters work for basic use cases but won't deliver
optimal performance for HARALD's specific knowledge patterns.

**Solution**: Benchmark and tune parameters based on Bryan's actual usage
patterns:

```rust
// Different parameter presets for different use cases
let browsing_config = HnswConfig {
    ef_construction: 100,
    m: 12,
    ef_search: 20,
};

let research_config = HnswConfig {
    ef_construction: 200,
    m: 16,
    ef_search: 100,
};

// Select configuration based on Bryan's current context
let config = if is_deep_research_mode() {
    &research_config
} else {
    &browsing_config
};
```

## Implementation Lessons

When implementing HNSW in HARALD's codebase, we encountered several challenges
that required careful attention. This section documents these experiences to
help avoid similar issues in the future.

### Persistence API Changes

**Challenge**: The API documentation examples suggested using `save_hnsw()` and
`load_hnsw()` methods, but these weren't correctly implemented in our version of
the library.

**Solution**: We discovered that the correct method for persisting indices is
`file_dump()`, which takes a directory path and basename:

```rust
// The correct way to save an HNSW index:
// Takes a directory path and a basename
index.file_dump(output_dir, "index")?;
// This creates two files: index.hnsw.data and index.hnsw.graph
```

**Lesson**: Always check the source code or extensive API documentation when
methods aren't working as expected. The method signatures matter:

```rust
// Incorrect (doesn't exist):
index.save_hnsw("path/to/index")?;

// Correct:
index.file_dump(&path_dir, "index_basename")?;
```

### Lifetime Management

**Challenge**: Loading indices created lifetime issues where the index was tied
to the loader object, causing compile errors when trying to return the loaded
index.

**Solution**: Proper construction and lifetime management is required:

```rust
// Create the loader and obtain the loaded index
let mut hnsw_loader = HnswIo::new(&data_dir, "index");
let loaded_index: Hnsw<'_, f32, DistCosine> = hnsw_loader.load_hnsw()?;

// Convert to 'static lifetime for storage (requires unsafe)
let index: Hnsw<'static, f32, DistCosine> =
    unsafe { std::mem::transmute(loaded_index) };
```

**Lesson**: The HNSW index loading process creates references tied to the
loader. When returning the index from functions, you need to carefully manage
these lifetimes or extend them with appropriate mechanisms.

### Type System Complexity

**Challenge**: The `hnsw_rs` library has a complex type system with generic
parameters and trait bounds that can be confusing, especially with newer
versions making breaking changes.

**Solution**: Be explicit about type parameters and check compiler errors
carefully:

```rust
// Be explicit about types when working with the library
let mut hnsw_loader = HnswIo::new(&data_dir, "index");
let loaded_index: Hnsw<'_, f32, DistCosine> = hnsw_loader.load_hnsw()?;
```

### Struct Field Changes

**Challenge**: The `Neighbour` struct fields changed between library versions,
with a new `p_id` field required of type `PointId`.

**Solution**: Update code to use the correct struct definition:

```rust
// Before: This no longer compiles
let neighbor = Neighbour {
    d_id: 0,
    distance: 0.1,
};

// After: Correct with new field
let neighbor = Neighbour {
    d_id: 0,
    p_id: PointId(0, 0), // Takes layer (u8) and index (i32)
    distance: 0.1,
};
```

**Lesson**: When upgrading dependencies, carefully check struct definitions for
breaking changes. The `PointId` type requires both a layer and an index
parameter.

### Testing Challenges

**Challenge**: Mock testing with HNSW indices can be complex due to the internal
structures and lifetimes.

**Solution**: Use more integration-style tests or isolate the HNSW interactions
behind interfaces that can be mocked separately.

### Performance Insights

**Challenge**: Initial implementation showed slower than expected query times.

**Solution**: Implemented batched operations and adjusted the EF parameters as
suggested in the documentation:

```rust
// Setting appropriate EF values for our use case
index.set_ef(50);  // Found this to be a good balance for our queries
```

## API Reference

This section provides a detailed reference of key `hnsw_rs` methods, their
parameters, return types, and usage examples based on our implementation.

### Core Methods

#### Creating an Index

```rust
pub fn new(
    nb_dimensions: usize,    // Vector dimension (must match embedding size)
    max_elements: usize,     // Maximum number of elements the index can hold
    max_nb_connection: usize, // Maximum connections per node (M parameter)
    ef_construction: usize,  // Controls index quality during construction
    dist_f: D,               // Distance function (DistCosine, DistL2, etc.)
) -> Hnsw<'static, T, D>
```

**Example:**

```rust
// Creating a 384-dimensional index with cosine distance
let index = Hnsw::<f32, DistCosine>::new(
    384,        // Dimensions - match your embeddings
    100_000,    // Max elements - 1.5x your expected data size
    16,         // M parameter - connections per node
    200,        // EF construction - build quality
    DistCosine {}
);
```

#### Inserting Data

```rust
pub fn insert(&self, point: (impl AsRef<[T]>, usize)) -> PointId
```

**Parameters:**

- `point`: A tuple containing:
  - Vector data as slice (`&[T]`)
  - Data identifier (`usize`)

**Returns:** `PointId` - Physical identifier in the index

**Example:**

```rust
// Single insertion
let vector = generate_embedding("document content");
let doc_id = 42;
index.insert((vector.as_slice(), doc_id));
```

#### Batch Insertion

```rust
pub fn parallel_insert(&self, points: &[(impl AsRef<[T]> + Sync, usize)])
```

**Parameters:**

- `points`: Slice of tuples, each containing a vector and its ID

**Example:**

```rust
// Batch insertion for better performance
let data_points: Vec<(&[f32], usize)> = documents
    .iter()
    .enumerate()
    .map(|(id, doc)| {
        let embedding = generate_embedding(doc);
        (embedding.as_slice(), id)
    })
    .collect();

index.parallel_insert(&data_points);
```

#### Searching

```rust
pub fn search(
    &self,
    query: impl AsRef<[T]>, // Query vector
    k_nearest: usize,       // Number of results to return
    ef_search: usize        // Search quality parameter
) -> Vec<Neighbour>
```

**Parameters:**

- `query`: Query vector as slice
- `k_nearest`: Number of nearest neighbors to return
- `ef_search`: Controls search quality/speed tradeoff

**Returns:** Vector of `Neighbour` structures

**Example:**

```rust
// Search for 5 nearest neighbors with medium quality
let query_vec = generate_embedding("search query");
let neighbors = index.search(&query_vec, 5, 50);

for n in neighbors {
    println!("Doc ID: {}, Distance: {}", n.d_id, n.distance);
}
```

#### Batch Searching

```rust
pub fn parallel_search(
    &self,
    queries: &[impl AsRef<[T]> + Sync],
    k_nearest: usize,
    ef_search: usize
) -> Vec<Vec<Neighbour>>
```

**Parameters:**

- `queries`: Slice of query vectors
- `k_nearest`: Number of nearest neighbors to return per query
- `ef_search`: Controls search quality/speed tradeoff

**Returns:** Vector of vectors of `Neighbour` structures

**Example:**

```rust
// Search multiple queries in parallel
let query_vecs = vec![
    generate_embedding("first query"),
    generate_embedding("second query"),
    generate_embedding("third query"),
];

let batch_results = index.parallel_search(&query_vecs, 5, 50);
```

#### Setting Search Parameters

```rust
pub fn set_ef(&self, ef: usize) -> bool
```

**Parameters:**

- `ef`: New EF value for search

**Returns:** Boolean indicating success

**Example:**

```rust
// Adjust search quality dynamically based on use case
if needs_high_accuracy {
    index.set_ef(200);  // More thorough search
} else {
    index.set_ef(20);   // Faster search
}
```

### Persistence Methods

#### Saving an Index

```rust
pub fn file_dump(
    &self,
    path: &std::path::Path,  // Directory path
    basename: &str           // Base filename
) -> Result<String>
```

**Parameters:**

- `path`: Path to directory where files will be saved
- `basename`: Base filename for the index files

**Returns:** Result with path string or error

**Example:**

```rust
// Save index to disk
let data_dir = Path::new("/path/to/data");
let result = index.file_dump(&data_dir, "knowledge_index")?;

// Creates two files:
// - /path/to/data/knowledge_index.hnsw.data
// - /path/to/data/knowledge_index.hnsw.graph
```

#### Loading an Index

```rust
// HnswIo constructor
pub fn new(dir_path: &Path, basename: &str) -> HnswIo

// Loading method
pub fn load_hnsw<'b, T, D>(&mut self) -> Result<Hnsw<'b, T, D>>
where
    T: 'static + Serialize + DeserializeOwned + Clone + Send + Sync + Debug,
    D: Distance<T> + Default + Send + Sync
```

**Parameters:**

- For constructor:
  - `dir_path`: Directory containing the index files
  - `basename`: Base filename of the index
- For `load_hnsw()`: No parameters, uses state from constructor

**Returns:** Result with HNSW index or error

**Example:**

```rust
// Load index from disk
let data_dir = Path::new("/path/to/data");
let mut hnsw_loader = HnswIo::new(&data_dir, "knowledge_index");
let index = hnsw_loader.load_hnsw()?;

// Handle lifetime issues if returning from function:
let index: Hnsw<'static, f32, DistCosine> =
    unsafe { std::mem::transmute(loaded_index) };
```

### Helper Methods

#### Getting Index Info

```rust
pub fn get_nb_point(&self) -> usize  // Get number of points in index
pub fn get_dimensions(&self) -> usize // Get vector dimensions
```

**Example:**

```rust
// Check index stats
let point_count = index.get_nb_point();
let dimensions = index.get_dimensions();
println!("Index contains {} points of {} dimensions", point_count, dimensions);
```

### Structures

#### Neighbour

Represents a search result:

```rust
pub struct Neighbour {
    pub d_id: usize,             // Data ID (corresponds to your insertion ID)
    pub p_id: PointId,           // Physical ID (internal identifier)
    pub distance: f32,           // Distance from query
}

pub struct PointId(pub u8, pub i32);  // Layer and index
```

**Example:**

```rust
// Processing search results
for neighbor in search_results {
    println!(
        "Data ID: {}, Distance: {}, Internal ID: {:?}",
        neighbor.d_id,
        neighbor.distance,
        neighbor.p_id
    );
}
```

## References

- [hnsw_rs Documentation](https://docs.rs/hnsw_rs/0.3.2/hnsw_rs/)
- [hnsw_rs GitHub Repository](https://github.com/jean-pierreBoth/hnswlib-rs)
- [HNSW Algorithm Paper](https://arxiv.org/abs/1603.09320) - "Efficient and  
  robust approximate nearest neighbor search using Hierarchical Navigable  
  Small World graphs", Yu. A. Malkov, D.A Yashunin (2016, 2018)
- [Rust Vector Database Comparison](https://github.com/unum-cloud/usearch)
- [HNSW Parameter Tuning Guide](https://github.com/nmslib/hnswlib/blob/master/ALGO_PARAMS.md)

## Library Overview

The `hnsw_rs` crate is a Rust implementation of the Hierarchical Navigable Small
World (HNSW) algorithm. In Rust programming, a "crate" is a package or module
that contains reusable code - similar to a library in other languages. Think of
it like a toolbox you can add to your project.

For HARALD's needs, this crate provides everything needed to build, maintain,
and search a semantic memory system that can quickly find information relevant
to Bryan's queries.

### What is a Struct?

In Rust, a "struct" (short for structure) is a custom data type that lets you
bundle multiple related values together. It's similar to a class in other
programming languages but more focused on data organization.

In the HNSW library, structs are used to represent important concepts like the
index itself, search results, and distance calculations.

### Key Components

- **`Hnsw<T, D>`**: The main structure representing the entire search index.
  This is the heart of the system where Bryan's knowledge is organized for quick
  retrieval.

  The type parameters are:
  - `T`: The data type of vector elements (usually `f32` for 32-bit floats)
  - `D`: The distance metric to use (like `DistCosine` or `DistL2`)

- **Key methods include**:
  - `new()` - Creates a fresh index for storing information
  - `insert()` - Adds a new memory (vector) to the index
  - `insert_parallel()` - Adds multiple memories simultaneously for efficiency
  - `search()` - Finds memories similar to a query
  - `parallel_search()` - Finds multiple similar memories at once
  - `file_dump()` - Saves Bryan's memory to disk for persistence
  - Using `HnswIo::load_hnsw()` - Loads previously saved memories

- **`Neighbour`**: A struct representing a single search result - one piece of
  relevant information found during a query. Contains:
  - `d_id` - The identifier HARALD can use to retrieve the original content
  - `distance` - How similar/different this item is from the query

- **`PointDistance`**: A type that represents how "far apart" two memories are
  in the semantic space. Lower distances typically mean more similar concepts.

- **`DataId`**: An identifier type for each piece of information. In HARALD's
  system, this could be a UUID or path to a specific memory entry.

- **`FilterT`**: A trait (interface) for creating custom filters. This lets
  HARALD filter search results based on metadata like:
  - Time periods ("only show memories from last week")
  - Sources ("only show journal entries")
  - Emotion tags ("only show positive experiences")

### Understanding Distance Metrics

Distance metrics are mathematical ways of measuring how "similar" or "different"
two vectors (pieces of information) are. Choosing the right metric is crucial
for HARALD's ability to understand the relationships between Bryan's memories,
notes, and other data.

Each metric has different properties and is best suited for different types of
data:

- **`DistL1`** **(Manhattan Distance)**:

  Measures the sum of absolute differences between vector elements. Imagine
  walking along city blocks - you can only move along the grid, not diagonally.

  ```plaintext
  distance = |x1 - y1| + |x2 - y2| + ... + |xn - yn|
  ```

  **Best for**: Data with independent features, where changes in one dimension
  shouldn't affect others. Good for sparse binary features.

  **HARALD use case**: Could be used for tagging systems where features are
  distinct categories.

- **`DistL2`** **(Euclidean Distance)**:

  The "as-the-crow-flies" straight-line distance between two points.

  ```plaintext
  distance = sqrt((x1 - y1)² + (x2 - y2)² + ... + (xn - yn)²)
  ```

  **Best for**: Data where the scale and correlation between dimensions matters.

  **HARALD use case**: Useful for comparing numerical data or when dimensions
  have meaningful relationships to each other.

- **`DistCosine`** **(Cosine Distance)**:

  Measures the angle between two vectors, normalized by their lengths. This
  focuses on the direction of vectors rather than their magnitude.

  ```plaintext
  distance = 1 - (dot_product(x, y) / (|x| * |y|))
  ```

  **Best for**: Text embeddings and semantic similarity where the relative
  importance of concepts matters more than their intensity.

  **HARALD use case**: Ideal for most of Bryan's text-based memories and notes,
  as it captures semantic meaning well. This is likely the most useful metric
  for HARALD's primary knowledge base.

- **`DistDot`** **(Dot Product)**:

  The sum of the products of corresponding elements. Unlike cosine, this is
  affected by vector magnitude.

  ```plaintext
  distance = -(x1*y1 + x2*y2 + ... + xn*yn)
  ```

  **Best for**: Cases where both direction and magnitude matter.

  **HARALD use case**: Could be used when intensity of features is important,
  such as in emotion analysis of Bryan's journal entries.

- **`DistJeffreys`** **(Jeffreys Divergence)**:

  A measure for comparing probability distributions.

  **Best for**: Comparing normalized histograms or probability distributions.

  **HARALD use case**: Could be useful for comparing distributions of topics or
  themes across different time periods in Bryan's life.

- **`DistJensenShannon`** **(Jensen-Shannon Divergence)**:

  Another probability distribution comparison metric, but smoother than KL
  divergence.

  **Best for**: Comparing probability distributions where symmetry matters.

  **HARALD use case**: Could be used for comparing topic models or analyzing
  changes in Bryan's interests over time.

### Practical Metric Selection for HARALD

For Bryan's use cases, the choice of distance metric should align with the type
of data and similarity concept:

1. **For text and semantic memories**: Use `DistCosine` - it works best with
   language model embeddings by focusing on conceptual similarity.

1. **For numerical data** (like time series or sensor data): Use `DistL2` to
   capture magnitude differences.

1. **For mixed categorical data** (like tags or attributes): Consider `DistL1`
   for its treatment of independent features.

1. **For emotion analysis or intensity-based comparisons**: `DistDot` might
   capture both the type and strength of emotions in journal entries.

By selecting the appropriate distance metric for each type of data HARALD stores
about Bryan, we can ensure the most relevant and meaningful results when
retrieving memories or insights.
