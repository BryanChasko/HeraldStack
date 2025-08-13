//! Chunked Ingest Library
//!
//! Library module for character-based chunking ingestion functionality.
//! CLI has been moved to src/bin/ingest_chunked.rs

use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::time::Instant;

use crate::ingest::chunking_utils::chunk_entity_fields;
use crate::core::embedding::ollama_api::OllamaApiClient;

/// Configuration for chunked ingestion processing
#[derive(Debug, Clone)]
pub struct ChunkedIngestConfig {
    /// Model name for embeddings
    pub model_name: String,
    /// Maximum chunk size in characters
    pub max_chunk_size: usize,
    /// Ollama API endpoint
    pub api_endpoint: String,
    /// Timeout for API requests in seconds
    pub timeout_secs: u64,
}

impl Default for ChunkedIngestConfig {
    fn default() -> Self {
        Self {
            model_name: "harald-phi4".to_string(),
            max_chunk_size: 250,
            api_endpoint: "http://localhost:11434".to_string(),
            timeout_secs: 30,
        }
    }
}

/// Result of chunked ingestion processing
#[derive(Debug, Clone)]
pub struct ChunkedIngestResult {
    /// Number of characters processed
    pub characters_processed: usize,
    /// Number of chunks created
    pub chunks_created: usize,
    /// Number of embeddings generated successfully
    pub embeddings_generated: usize,
    /// Number of failed embedding attempts
    pub failed_embeddings: usize,
    /// Processing time in seconds
    pub processing_time_secs: f64,
    /// Success status
    pub success: bool,
    /// Error message if any
    pub error: Option<String>,
}

/// Character data structure for Marvel character processing
#[derive(Debug, Clone)]
pub struct CharacterData {
    pub character_name: String,
    pub first_appearance: String,
    pub affiliations: Vec<String>,
    pub core_attributes: Vec<String>,
    pub inspirational_themes: Vec<String>,
    pub traits: Vec<String>,
    pub ai_alignment: String,
    pub description: Option<String>,
}

impl CharacterData {
    /// Create CharacterData from JSON value
    pub fn from_json(json: &Value) -> Result<Self> {
        let character_name = json
            .get("character_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let first_appearance = json
            .get("first_appearance")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let affiliations = json
            .get("affiliations")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let core_attributes = json
            .get("core_attributes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let inspirational_themes = json
            .get("inspirational_themes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let traits = json
            .get("traits")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let ai_alignment = json
            .get("ai_alignment")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let description = json
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(CharacterData {
            character_name,
            first_appearance,
            affiliations,
            core_attributes,
            inspirational_themes,
            traits,
            ai_alignment,
            description,
        })
    }
}

/// Represents a chunk of character data
#[derive(Debug, Clone)]
pub struct CharacterChunk {
    pub label: String,
    pub content: String,
}

/// Process a file with chunked ingestion
///
/// This function handles the complete workflow:
/// 1. Reading and parsing the JSONL file
/// 2. Processing each character entry
/// 3. Chunking the character data
/// 4. Generating embeddings for each chunk
/// 5. Returning processing statistics
///
/// # Arguments
/// * `file_path` - Path to the JSONL file to process
/// * `config` - Configuration for processing
///
/// # Returns
/// Returns a `ChunkedIngestResult` with processing statistics and status.
pub async fn process_file(file_path: &str, config: &ChunkedIngestConfig) -> Result<ChunkedIngestResult> {
    let start_time = Instant::now();
    
    // Initialize Ollama client
    let client = OllamaApiClient::new(&config.api_endpoint);

    // Check API status
    client.check_status().await
        .context("Ollama API is not responding")?;

    // Test API with minimal request
    client.generate_embedding("test", &config.model_name).await
        .context("API test failed")?;

    // Read and process the input file as JSONL
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let mut stats = ProcessingStats::new();

    for (line_num, line) in file_content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let character_json: Value = match serde_json::from_str(line) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("❌ Line {}: Failed to parse JSON: {}", line_num + 1, e);
                stats.failed_embeddings += 1;
                continue;
            }
        };

        let character = match CharacterData::from_json(&character_json) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ Line {}: Failed to parse character data: {}", line_num + 1, e);
                stats.failed_embeddings += 1;
                continue;
            }
        };

        if character.character_name.is_empty() {
            eprintln!("⚠️  Line {}: Skipping character with empty name.", line_num + 1);
            stats.failed_embeddings += 1;
            continue;
        }

        // Process this character
        let result = process_character(&character_json, &character, &client, config).await?;
        stats.accumulate(&result);
    }

    if stats.characters_processed == 0 {
        return Err(anyhow::anyhow!("No valid characters found in file"));
    }

    let processing_time = start_time.elapsed().as_secs_f64();

    Ok(ChunkedIngestResult {
        characters_processed: stats.characters_processed,
        chunks_created: stats.chunks_created,
        embeddings_generated: stats.embeddings_generated,
        failed_embeddings: stats.failed_embeddings,
        processing_time_secs: processing_time,
        success: true,
        error: None,
    })
}

/// Internal statistics tracking
struct ProcessingStats {
    characters_processed: usize,
    chunks_created: usize,
    embeddings_generated: usize,
    failed_embeddings: usize,
}

impl ProcessingStats {
    fn new() -> Self {
        Self {
            characters_processed: 0,
            chunks_created: 0,
            embeddings_generated: 0,
            failed_embeddings: 0,
        }
    }

    fn accumulate(&mut self, result: &CharacterProcessingResult) {
        self.characters_processed += 1;
        self.chunks_created += result.chunks_created;
        self.embeddings_generated += result.embeddings_generated;
        self.failed_embeddings += result.failed_embeddings;
    }
}

/// Result of processing a single character
struct CharacterProcessingResult {
    chunks_created: usize,
    embeddings_generated: usize,
    failed_embeddings: usize,
}

/// Process a single character entry
async fn process_character(
    character_json: &Value,
    character: &CharacterData,
    client: &OllamaApiClient,
    config: &ChunkedIngestConfig,
) -> Result<CharacterProcessingResult> {
    // Create chunks for this character
    let chunks: Vec<CharacterChunk> = chunk_entity_fields(character_json, config.max_chunk_size)
        .into_iter()
        .map(|(label, content)| CharacterChunk { label, content })
        .collect();

    let mut result = CharacterProcessingResult {
        chunks_created: chunks.len(),
        embeddings_generated: 0,
        failed_embeddings: 0,
    };

    // Process each chunk
    for chunk in &chunks {
        match client.generate_embedding(&chunk.content, &config.model_name).await {
            Ok(_) => {
                result.embeddings_generated += 1;
            }
            Err(e) => {
                eprintln!("❌ Failed to generate embedding for chunk '{}': {}", chunk.label, e);
                result.failed_embeddings += 1;
            }
        }
    }

    Ok(result)
}

/// Validate a character JSON entry
pub fn validate_character_entry(character: &Value) -> Result<(), String> {
    if !character.is_object() {
        return Err("Character entry must be a JSON object".to_string());
    }

    let obj = character.as_object().unwrap();
    
    if !obj.contains_key("character_name") {
        return Err("Character entry must have 'character_name' field".to_string());
    }

    if let Some(name) = obj.get("character_name").and_then(|n| n.as_str()) {
        if name.trim().is_empty() {
            return Err("Character name cannot be empty".to_string());
        }
    } else {
        return Err("Character name must be a string".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_chunked_ingest_config_default() {
        let config = ChunkedIngestConfig::default();
        assert_eq!(config.model_name, "harald-phi4");
        assert_eq!(config.max_chunk_size, 250);
        assert_eq!(config.api_endpoint, "http://localhost:11434");
        assert_eq!(config.timeout_secs, 30);
    }

    #[test]
    fn test_character_data_from_json() {
        let json = json!({
            "character_name": "Vision",
            "first_appearance": "Avengers #57",
            "affiliations": ["Avengers", "West Coast Avengers"],
            "core_attributes": ["Synthetic being", "AI consciousness"],
            "inspirational_themes": ["Identity", "Humanity"],
            "traits": ["Logical", "Empathetic"],
            "ai_alignment": "Lawful Good",
            "description": "An android created by Ultron"
        });

        let character = CharacterData::from_json(&json).unwrap();
        assert_eq!(character.character_name, "Vision");
        assert_eq!(character.first_appearance, "Avengers #57");
        assert_eq!(character.affiliations.len(), 2);
        assert_eq!(character.core_attributes.len(), 2);
        assert!(character.description.is_some());
    }

    #[test]
    fn test_validate_character_entry_valid() {
        let character = json!({
            "character_name": "Vision",
            "description": "A test character"
        });
        
        let result = validate_character_entry(&character);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_character_entry_missing_name() {
        let character = json!({
            "description": "A test character"
        });
        
        let result = validate_character_entry(&character);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("character_name"));
    }

    #[test]
    fn test_validate_character_entry_empty_name() {
        let character = json!({
            "character_name": "",
            "description": "A test character"
        });
        
        let result = validate_character_entry(&character);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }
}
