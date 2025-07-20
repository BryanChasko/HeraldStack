use hnsw_rs::prelude::AnnT;
//! File ingestion module for semantic search indexing.
//!
//! This module handles the ingestion of files into a searchable vector index.
//! It processes files, generates embeddings, and builds an HNSW index for semantic search
//! using the HNSW algorithm for efficient nearest neighbor search in high-dimensional spaces.
//! This creates a searchable database of file contents based on their semantic meaning.
//!
//! # Module Structure
//! In Rust, this .rs file defines a module named "ingest":
//! - If main.rs/lib.rs contains `mod ingest;`, Rust loads this file as the ingest module
//! - Functions here are accessed as `ingest::run()` from other modules
//! - This is a "module source file" - a unit of compilation within our crate
//! - Part of the flat module style (modern) vs ingest/mod.rs (legacy)

use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use hnsw_rs::{dist::DistCosine, prelude::*};
use serde_json::json;
use walkdir::WalkDir;

use crate::embed;

/// Directories to skip during file traversal.
/// 
/// These directories typically contain:
/// - Version control metadata (.git)
/// - Virtual environments (.venv)
/// - Build artifacts (target, node_modules)
const SKIP_DIRS: &[&str] = &[".git", ".venv", "target", "node_modules"];

/// Maximum number of characters to read from each file for embedding.
/// 
/// This limit serves multiple purposes:
/// - Controls API costs for embedding services
/// - Prevents memory issues with extremely large files
/// - Ensures consistent processing time per file
const MAX_FILE_CHARS: usize = 800;

/// Maximum number of tokens for embedding API requests.
const MAX_EMBEDDING_TOKENS: usize = 600;

/// HNSW index construction parameters optimized for semantic search.
const HNSW_MAX_CONNECTIONS: usize = 16;
const HNSW_EF_CONSTRUCTION: usize = 200;
const HNSW_MAX_LAYER: usize = 16;
const HNSW_EF_SEARCH: usize = 20;

/// Progress reporting interval (number of files).
const PROGRESS_INTERVAL: usize = 10;

/// Supported file extensions for semantic indexing.
const SUPPORTED_EXTENSIONS: &[&str] = &["md", "json"];

/// Configuration for the ingestion process.
#[derive(Debug, Clone)]
pub struct IngestConfig {
    /// Root directory to start ingestion from.
    pub root_dir: PathBuf,
    /// Maximum characters to read per file.
    pub max_chars: usize,
    /// Maximum tokens for embedding requests.
    pub max_tokens: usize,
}

impl Default for IngestConfig {
    fn default() -> Self {
        Self {
            root_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            max_chars: MAX_FILE_CHARS,
            max_tokens: MAX_EMBEDDING_TOKENS,
        }
    }
}

/// Statistics from the ingestion process.
#[derive(Debug, Clone)]
pub struct IngestStats {
    /// Total number of files processed.
    pub files_processed: usize,
    /// Total number of files skipped.
    pub files_skipped: usize,
    /// Output directory path.
    pub output_dir: PathBuf,
}

/// Main ingestion function that processes files and builds a searchable vector index.
///
/// This function creates the foundation for semantic search across the codebase by:
/// 1. Traversing the file system to find relevant files
/// 2. Reading and preprocessing file contents
/// 3. Generating embeddings for semantic representation
/// 4. Building an HNSW index for efficient similarity search
/// 5. Persisting the index and metadata for later use
///
/// # Returns
/// Returns `IngestStats` containing information about the ingestion process.
///
/// # Errors
/// Returns an error if:
/// - File system operations fail
/// - Embedding API requests fail
/// - Index serialization fails
/// - Directory creation fails
pub async fn run() -> Result<IngestStats> {
    run_with_config(IngestConfig::default()).await
}

/// Runs ingestion with custom configuration.
///
/// # Arguments
/// * `config` - Configuration parameters for the ingestion process
///
/// # Returns
/// Returns `IngestStats` containing information about the ingestion process.
///
/// # Errors
/// Returns an error if any step of the ingestion process fails.
pub async fn run_with_config(config: IngestConfig) -> Result<IngestStats> {
    let client = create_http_client()?;
    let index = create_hnsw_index();
    let mut file_metadata = Vec::new();
    let mut stats = IngestStats {
        files_processed: 0,
        files_skipped: 0,
        output_dir: config.root_dir.join("data"),
    };

    // Process all files in the directory tree
    process_directory_tree(&config, &client, &index, &mut file_metadata, &mut stats).await?;

    // Persist the index and metadata
    persist_index_data(&index, &file_metadata, &stats.output_dir)?;

    println!(
        "Ingestion complete: {} files processed, {} files skipped → {}",
        stats.files_processed,
        stats.files_skipped,
        stats.output_dir.join("index.hnsw.*").display()
    );

    Ok(stats)
}

/// Creates an HTTP client for embedding API requests.
fn create_http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("Failed to create HTTP client")
}

/// Creates and configures an HNSW index for vector similarity search.
fn create_hnsw_index() -> Hnsw<f32, DistCosine> {
    Hnsw::<f32, DistCosine>::new(
        HNSW_MAX_CONNECTIONS,
        HNSW_EF_CONSTRUCTION,
        HNSW_MAX_LAYER,
        HNSW_EF_SEARCH,
        DistCosine::default(),
    )
}

/// Processes all files in the directory tree.
async fn process_directory_tree(
    config: &IngestConfig,
    client: &reqwest::Client,
    index: &Hnsw<f32, DistCosine>,
    file_metadata: &mut Vec<PathBuf>,
    stats: &mut IngestStats,
) -> Result<()> {
    for entry in WalkDir::new(&config.root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if should_skip_path(path) {
            stats.files_skipped += 1;
            continue;
        }

        if !is_supported_file(path) {
            stats.files_skipped += 1;
            continue;
        }

        match process_single_file(path, config, client, index, file_metadata, stats.files_processed).await {
            Ok(()) => {
                stats.files_processed += 1;
                if stats.files_processed % PROGRESS_INTERVAL == 0 {
                    println!("Processed {} files…", stats.files_processed);
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to process file {}: {}", path.display(), e);
                stats.files_skipped += 1;
            }
        }
    }

    Ok(())
}

/// Determines if a path should be skipped during traversal.
fn should_skip_path(path: &std::path::Path) -> bool {
    path.is_dir() && SKIP_DIRS.iter().any(|&dir| path.ends_with(dir))
}

/// Checks if a file has a supported extension for indexing.
fn is_supported_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext| SUPPORTED_EXTENSIONS.contains(&ext))
}

/// Processes a single file and adds it to the index.
async fn process_single_file(
    path: &std::path::Path,
    config: &IngestConfig,
    client: &reqwest::Client,
    index: &Hnsw<f32, DistCosine>,
    file_metadata: &mut Vec<PathBuf>,
    file_id: usize,
) -> Result<()> {
    // Read and truncate file content
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    
    let truncated_content = truncate_content(&content, config.max_chars);

    // Generate embedding vector
    let embedding = embed::embed(truncated_content, config.max_tokens, client)
        .await
        .with_context(|| format!("Failed to generate embedding for file: {}", path.display()))?;

    // Insert into index
    index.insert((embedding.as_slice(), file_id));

    // Store file path metadata
    file_metadata.push(path.to_path_buf());

    Ok(())
}

/// Truncates content to the specified maximum length.
fn truncate_content(content: &str, max_chars: usize) -> &str {
    if content.len() <= max_chars {
        content
    } else {
        &content[..max_chars]
    }
}

/// Persists the HNSW index and file metadata to disk.
fn persist_index_data(
    index: &Hnsw<f32, DistCosine>,
    file_metadata: &[PathBuf],
    output_dir: &std::path::Path,
) -> Result<()> {
    // Create output directory
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    // Save HNSW index
    let index_path = output_dir.join("index");
    index
        .dump(&index_path)
        .with_context(|| format!("Failed to save HNSW index to: {}", index_path.display()))?;

    // Save metadata as JSON
    let metadata_path = output_dir.join("meta.json");
    let metadata_file = File::create(&metadata_path)
        .with_context(|| format!("Failed to create metadata file: {}", metadata_path.display()))?;
    
    serde_json::to_writer(metadata_file, &json!(file_metadata))
        .with_context(|| format!("Failed to write metadata to: {}", metadata_path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_should_skip_path() {
        assert!(should_skip_path(Path::new(".git")));
        assert!(should_skip_path(Path::new("project/.git")));
        assert!(should_skip_path(Path::new("target")));
        assert!(!should_skip_path(Path::new("src")));
        assert!(!should_skip_path(Path::new("README.md")));
    }

    #[test]
    fn test_is_supported_file() {
        assert!(is_supported_file(Path::new("README.md")));
        assert!(is_supported_file(Path::new("config.json")));
        assert!(!is_supported_file(Path::new("binary.exe")));
        assert!(!is_supported_file(Path::new("script.py")));
    }

    #[test]
    fn test_truncate_content() {
        let long_content = "a".repeat(1000);
        assert_eq!(truncate_content(&long_content, 500).len(), 500);
        
        let short_content = "short";
        assert_eq!(truncate_content(short_content, 500), "short");
    }

    #[test]
    fn test_ingest_config_default() {
        let config = IngestConfig::default();
        assert_eq!(config.max_chars, MAX_FILE_CHARS);
        assert_eq!(config.max_tokens, MAX_EMBEDDING_TOKENS);
    }
}
