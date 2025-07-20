use hnsw_rs::prelude::AnnT;
//! rust_ingest/src/query.rs
//! Query module for semantic search and retrieval-augmented generation.
//!
//! This module handles querying the pre-built HNSW index for semantic search
//! and integrates with a local language model to provide context-aware responses.
//! It performs vector similarity search to find relevant documents, then uses
//! those documents as context for generating responses via a local LLM API.
//!
//! # Module Structure
//! This is a "module source file" that defines the query module:
//! - Loaded via `mod query;` in main.rs/lib.rs
//! - Functions accessed as `query::run()` from other modules
//! - Complements the ingest module by providing search functionality
//! - Part of the semantic search pipeline: ingest → index → query → response

use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use hnsw_rs::{dist::DistCosine, prelude::*};
use ndarray::Array1;
use serde_json::Value;

use crate::embed;

/// Maximum number of characters to include from each retrieved document.
/// 
/// This limit serves multiple purposes:
/// - Controls the size of context sent to the language model
/// - Prevents token limit overflow in LLM requests
/// - Ensures reasonable response times
/// - Maintains focus on most relevant content
const MAX_CONTEXT_CHARS: usize = 800;

/// Number of similar documents to retrieve for context.
/// 
/// This determines how many semantically similar documents
/// are used to provide context for the language model response.
const NUM_SEARCH_RESULTS: usize = 3;

/// Search effort parameter for HNSW queries.
/// 
/// Higher values provide more accurate results but take longer.
/// This value balances accuracy vs. performance for interactive use.
const SEARCH_EF: usize = 20;

/// Maximum tokens for query embedding requests.
/// 
/// Queries are typically shorter than documents, so this
/// can be smaller than the embedding limit used during ingestion.
const MAX_QUERY_TOKENS: usize = 120;

/// Configuration for query processing.
#[derive(Debug, Clone)]
pub struct QueryConfig {
    /// Root directory containing the data folder with index files.
    pub root_dir: PathBuf,
    /// Maximum characters to extract from each retrieved document.
    pub max_context_chars: usize,
    /// Number of similar documents to retrieve.
    pub num_results: usize,
    /// Search effort parameter for HNSW.
    pub search_ef: usize,
    /// Maximum tokens for query embedding.
    pub max_query_tokens: usize,
    /// Language model endpoint URL.
    pub llm_endpoint: String,
    /// Language model name/identifier.
    pub model_name: String,
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            root_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            max_context_chars: MAX_CONTEXT_CHARS,
            num_results: NUM_SEARCH_RESULTS,
            search_ef: SEARCH_EF,
            max_query_tokens: MAX_QUERY_TOKENS,
            llm_endpoint: "http://127.0.0.1:11434/api/chat".to_string(),
            model_name: "harald-phi4".to_string(),
        }
    }
}

/// Result of a semantic search query.
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// The generated response from the language model.
    pub response: String,
    /// Paths of documents used as context.
    pub context_files: Vec<PathBuf>,
    /// Number of documents retrieved.
    pub num_context_docs: usize,
}

/// Main query function that performs semantic search and generates responses.
///
/// This function implements retrieval-augmented generation (RAG) by:
/// 1. Loading the pre-built HNSW index and metadata
/// 2. Converting the query to an embedding vector
/// 3. Finding semantically similar documents
/// 4. Extracting relevant context from those documents
/// 5. Sending context + query to a local language model
/// 6. Returning the model's response
///
/// # Arguments
/// * `query` - The user's search query or question
///
/// # Returns
/// Returns a `QueryResult` containing the response and metadata.
///
/// # Errors
/// Returns an error if:
/// - Index files cannot be loaded
/// - Embedding generation fails
/// - Language model API request fails
/// - File system operations fail
pub async fn run(query: &str) -> Result<QueryResult> {
    run_with_config(query, QueryConfig::default()).await
}

/// Runs query processing with custom configuration.
///
/// # Arguments
/// * `query` - The user's search query or question
/// * `config` - Configuration parameters for query processing
///
/// # Returns
/// Returns a `QueryResult` containing the response and metadata.
///
/// # Errors
/// Returns an error if any step of the query process fails.
pub async fn run_with_config(query: &str, config: QueryConfig) -> Result<QueryResult> {
    // Load the pre-built index and metadata
    let (index, metadata) = load_index_and_metadata(&config)?;
    
    // Create HTTP client for API requests
    let client = create_http_client()?;
    
    // Perform semantic search
    let search_results = perform_semantic_search(query, &config, &client, &index).await?;
    
    // Build context from retrieved documents
    let (context, context_files) = build_context_from_results(&search_results, &metadata, &config)?;
    
    // Generate response using language model
    let response = generate_llm_response(&context, query, &config, &client).await?;
    
    Ok(QueryResult {
        response,
        context_files,
        num_context_docs: search_results.len(),
    })
}

/// Loads the HNSW index and file metadata from disk.
fn load_index_and_metadata(config: &QueryConfig) -> Result<(Hnsw<'_, f32, DistCosine>, Vec<PathBuf>)> {
    let data_dir = config.root_dir.join("data");
    
    // Load the HNSW index using the correct API
    // The file_load function doesn't exist, use the proper loading function
    let index: Hnsw<'_, f32, DistCosine> = hnsw_rs::Hnsw::load_hnsw(&data_dir.join("index"))
        .context("Failed to load HNSW index - ensure ingestion has been run")?;
    
    // Load file metadata
    let metadata_file = fs::File::open(data_dir.join("meta.json"))
        .context("Failed to open metadata file")?;
    let metadata: Vec<PathBuf> = serde_json::from_reader(metadata_file)
        .context("Failed to parse metadata JSON")?;
    
    Ok((index, metadata))
}

/// Creates an HTTP client for API requests.
fn create_http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60)) // Longer timeout for LLM responses
        .build()
        .context("Failed to create HTTP client")
}

/// Performs semantic search to find relevant documents.
async fn perform_semantic_search(
    query: &str,
    config: &QueryConfig,
    client: &reqwest::Client,
    index: &Hnsw<'_, f32, DistCosine>,
) -> Result<Vec<Neighbour>> {
    // Convert query to embedding vector
    let query_embedding = embed::embed(query, config.max_query_tokens, client)
        .await
        .context("Failed to generate embedding for query")?;
    
    // Search for similar documents
    let search_results = index.search(&Array1::from(query_embedding), config.num_results, config.search_ef);
    
    if search_results.is_empty() {
        return Err(anyhow::anyhow!("No similar documents found"));
    }
    
    Ok(search_results)
}

/// Builds context string from search results.
fn build_context_from_results(
    search_results: &[Neighbour],
    metadata: &[PathBuf],
    config: &QueryConfig,
) -> Result<(String, Vec<PathBuf>)> {
    let mut context = String::new();
    let mut context_files = Vec::new();
    
    for neighbor in search_results.iter().take(config.num_results) {
        let file_path = &metadata[neighbor.d_id];
        context_files.push(file_path.clone());
        
        // Read file content
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read context file: {}", file_path.display()))?;
        
        // Truncate content and add to context
        let truncated = truncate_content(&content, config.max_context_chars);
        context.push_str(truncated);
        context.push_str("\n\n");
    }
    
    Ok((context, context_files))
}

/// Generates a response using the language model API.
async fn generate_llm_response(
    context: &str,
    query: &str,
    config: &QueryConfig,
    client: &reqwest::Client,
) -> Result<String> {
    let request_body = serde_json::json!({
        "model": config.model_name,
        "messages": [{
            "role": "user",
            "content": format!("{}\n\n{}", context, query)
        }],
        "stream": false
    });
    
    let response: Value = client
        .post(&config.llm_endpoint)
        .json(&request_body)
        .send()
        .await
        .context("Failed to send request to language model")?
        .json()
        .await
        .context("Failed to parse language model response")?;
    
    let content = response["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_owned();
    
    if content.is_empty() {
        return Err(anyhow::anyhow!("Empty response from language model"));
    }
    
    Ok(content)
}

/// Truncates content to the specified maximum length.
fn truncate_content(content: &str, max_chars: usize) -> &str {
    if content.len() <= max_chars {
        content
    } else {
        &content[..max_chars]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_truncate_content() {
        let long_content = "a".repeat(1000);
        assert_eq!(truncate_content(&long_content, 500).len(), 500);
        
        let short_content = "short";
        assert_eq!(truncate_content(short_content, 500), "short");
    }

    #[test]
    fn test_query_config_default() {
        let config = QueryConfig::default();
        assert_eq!(config.max_context_chars, MAX_CONTEXT_CHARS);
        assert_eq!(config.num_results, NUM_SEARCH_RESULTS);
        assert_eq!(config.search_ef, SEARCH_EF);
        assert_eq!(config.max_query_tokens, MAX_QUERY_TOKENS);
        assert_eq!(config.model_name, "harald-phi4");
    }

    #[test]
    fn test_query_result_creation() {
        let result = QueryResult {
            response: "test response".to_string(),
            context_files: vec![PathBuf::from("test.md")],
            num_context_docs: 1,
        };
        
        assert_eq!(result.response, "test response");
        assert_eq!(result.num_context_docs, 1);
        assert_eq!(result.context_files.len(), 1);
    }

    // Integration tests would require setting up test index files
    #[cfg(feature = "integration-tests")]
    mod integration {
        use super::*;
        
        #[tokio::test]
        async fn test_query_with_mock_data() {
            // This would test with actual index files in a test environment
            // Requires careful setup of test data and mock LLM responses
        }
    }
}
