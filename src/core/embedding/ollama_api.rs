//! Ollama API client for embedding generation.
//!
//! This module provides a client for interacting with the Ollama API,
//! particularly for generating embeddings and checking the API status.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Response from the Ollama embedding API
#[derive(Debug, Deserialize)]
pub struct EmbeddingResponse {
    pub embedding: Vec<f32>,
}

/// Request to the Ollama embedding API
#[derive(Debug, Serialize)]
pub struct EmbeddingRequest<'a> {
    pub model: &'a str,
    pub prompt: &'a str,
}

/// Client for interacting with the Ollama API.
pub struct OllamaApiClient {
    /// Base URL of the Ollama API
    base_url: String,

    /// Timeout for API requests in seconds
    timeout: Duration,

    /// HTTP client for making requests
    client: reqwest::Client,
}

impl OllamaApiClient {
    /// Create a new OllamaApiClient with the given base URL.
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            timeout: Duration::from_secs(30),
            client: reqwest::Client::new(),
        }
    }

    /// Set the timeout for API requests.
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout = Duration::from_secs(timeout_secs);
        self
    }

    /// Check if the Ollama API is available.
    pub async fn check_status(&self) -> Result<bool> {
        let url = format!("{}/api/version", self.base_url);
        let response = self
            .client
            .get(&url)
            .timeout(self.timeout)
            .send()
            .await
            .context("Failed to connect to Ollama API")?;

        Ok(response.status().is_success())
    }

    /// Generate an embedding for the given text using the specified model.
    pub async fn generate_embedding(&self, text: &str, model: &str) -> Result<Vec<f32>> {
        let url = format!("{}/api/embeddings", self.base_url);
        let request = EmbeddingRequest {
            model,
            prompt: text,
        };

        let response = self
            .client
            .post(&url)
            .timeout(self.timeout)
            .json(&request)
            .send()
            .await
            .context("Failed to send embedding request")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "No response body".to_string());
            anyhow::bail!("API error ({}): {}", status, text);
        }

        let embedding_response: EmbeddingResponse = response
            .json()
            .await
            .context("Failed to parse embedding response")?;

        Ok(embedding_response.embedding)
    }

    /// Generate an embedding with automatic chunking for long text.
    ///
    /// This function will automatically break down long text into smaller chunks
    /// and generate embeddings for each chunk. It's useful for handling text that
    /// might exceed the model's context window.
    pub async fn generate_embedding_chunked(
        &self,
        text: &str,
        model: &str,
        max_chunk_size: usize,
    ) -> Result<Vec<Vec<f32>>> {
        use crate::utils::chunking::{chunk_text, ChunkerOptions, ChunkingStrategy};

        // If text is under the limit, just generate a single embedding
        if text.len() <= max_chunk_size {
            let embedding = self.generate_embedding(text, model).await?;
            return Ok(vec![embedding]);
        }

        // Otherwise, chunk the text and generate embeddings for each chunk
        let options = ChunkerOptions {
            strategy: ChunkingStrategy::Character(max_chunk_size),
            ..Default::default()
        };

        let chunks = chunk_text(text, options);
        let mut embeddings = Vec::with_capacity(chunks.len());

        for chunk in chunks {
            let embedding = self.generate_embedding(&chunk, model).await?;
            embeddings.push(embedding);
        }

        Ok(embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = OllamaApiClient::new("http://localhost:11434");
        assert_eq!(client.base_url, "http://localhost:11434");
    }

    #[tokio::test]
    async fn test_client_with_timeout() {
        let client = OllamaApiClient::new("http://localhost:11434").with_timeout(60);
        assert_eq!(client.timeout, std::time::Duration::from_secs(60));
    }

    // Additional tests will be implemented when needed
}
