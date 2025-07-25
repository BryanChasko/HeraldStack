use crate::ingest::chunking_utils::chunk_entity_fields;
use anyhow::{Context, Result};
use clap::{Arg, Command};
use serde_json::Value;
use std::fs;
use std::time::{Duration, Instant};
use tokio::time;

// Import our existing utilities through the crate
use crate::core::embedding::ollama_api::OllamaApiClient;

/// Character data structure for Marvel character processing
#[derive(Debug, Clone)]
struct CharacterData {
    character_name: String,
    first_appearance: String,
    affiliations: Vec<String>,
    core_attributes: Vec<String>,
    inspirational_themes: Vec<String>,
    traits: Vec<String>,
    ai_alignment: String,
    description: Option<String>,
}

/// Chunk of character data ready for embedding
#[derive(Debug, Clone)]
struct CharacterChunk {
    label: String,
    content: String,
}


impl CharacterData {
    /// Parse character data from JSON value, robust to missing fields
    fn from_json(value: &Value) -> Result<Self> {
        let character_name = value.get("character_name")
            .and_then(|v| v.as_str())
            .unwrap_or("").to_string();
        let first_appearance = value.get("first_appearance")
            .and_then(|v| v.as_str())
            .unwrap_or("").to_string();
        let affiliations = value.get("affiliations")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
            .unwrap_or_else(Vec::new);
        let core_attributes = value.get("core_attributes")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
            .unwrap_or_else(Vec::new);
        let inspirational_themes = value.get("inspirational_themes")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
            .unwrap_or_else(Vec::new);
        let traits = value.get("traits")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
            .unwrap_or_else(Vec::new);
        let ai_alignment = value.get("ai_alignment")
            .and_then(|v| v.as_str())
            .unwrap_or("").to_string();
        let description = value.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
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

    // Remove to_chunks; use chunk_entity_fields instead
}

/// Generate embedding for a text chunk
async fn generate_embedding(
    client: &OllamaApiClient,
    chunk: &CharacterChunk,
    model: &str,
) -> Result<bool> {
    let text = format!("{}: {}", chunk.label, chunk.content);

    println!("🔄 Generating embedding for text ({} chars)", text.len());

    // If text is too large, warn but proceed
    const MAX_CHUNK_SIZE: usize = 250;
    if text.len() > MAX_CHUNK_SIZE {
        println!(
            "⚠️  Text exceeds recommended size of {} chars",
            MAX_CHUNK_SIZE
        );
        println!("   Will process in smaller chunks");
    }

    let start_time = Instant::now();

    match client.generate_embedding(&text, model).await {
        Ok(embedding) => {
            let elapsed = start_time.elapsed();
            println!(
                "✅ Success - Embedding generated in {:.2}s",
                elapsed.as_secs_f64()
            );
            println!("   Vector dimensions: {}", embedding.len());
            Ok(true)
        }
        Err(e) => {
            println!("❌ Failed - {}", e);
            Ok(false)
        }
    }
}

/// Main entry point for the chunked ingestion tool
#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("ingest_chunked")
        .about("Character-based chunking for Marvel character data")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("JSON file to process")
                .default_value(
                    "/Users/bryanchasko/Code/HARALD/tests/fixtures/test_single_character.json",
                ),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("MODEL")
                .help("Ollama model to use for embeddings")
                .default_value("harald-phi4"),
        )
        .get_matches();

    let file_path = matches.get_one::<String>("file").unwrap();
    let model = matches.get_one::<String>("model").unwrap();

    // Display start banner
    println!("==================================================");
    println!("🚀 HARALD OPTIMIZED INGEST PROCESS (Rust)");
    println!("🔍 Processing character data with size-optimized chunks");
    println!(
        "🕒 Started at {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    println!("==================================================");

    // Initialize Ollama client
    let client = OllamaApiClient::new("http://localhost:11434");

    // Check Ollama API status
    println!("🔍 Checking Ollama API status...");
    match client.check_status().await {
        Ok(_) => println!("✅ Ollama API is available"),
        Err(e) => {
            println!("❌ Ollama API is not responding: {}", e);
            println!("Please check if the service is running properly.");
            return Ok(());
        }
    }

    // Test API with minimal request
    println!("🧪 Testing API with minimal request...");

    match client.generate_embedding("test", model).await {
        Ok(_) => println!("✅ API working correctly"),
        Err(e) => {
            println!("❌ API test failed: {}", e);
            println!("Please check Ollama service status");
            return Ok(());
        }
    }

    // Read and process the input file as JSONL
    println!("📄 Processing JSONL file: {}", file_path);
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let mut total_characters = 0;
    let mut total_chunks = 0;
    let mut total_successful_chunks = 0;
    let mut total_failed_chunks = 0;
    let mut processed_characters = 0;

    let mut any_valid_character = false;
    for (line_num, line) in file_content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let character_json: Value = match serde_json::from_str(line) {
            Ok(val) => val,
            Err(e) => {
                println!("❌ Line {}: Failed to parse JSON: {}", line_num + 1, e);
                total_failed_chunks += 1;
                continue;
            }
        };
        let character = match CharacterData::from_json(&character_json) {
            Ok(c) => c,
            Err(e) => {
                println!("❌ Line {}: Failed to parse character data: {}", line_num + 1, e);
                total_failed_chunks += 1;
                continue;
            }
        };
        if character.character_name.is_empty() {
            println!("⚠️  Line {}: Skipping character with empty name.", line_num + 1);
            total_failed_chunks += 1;
            continue;
        }
        any_valid_character = true;
        println!("\n🔍 Processing character [{}] on line {}", character.character_name, line_num + 1);
        processed_characters += 1;
        let max_embed_len = 250;
        let chunks: Vec<CharacterChunk> = chunk_entity_fields(&character_json, max_embed_len)
            .into_iter()
            .map(|(label, content)| CharacterChunk { label, content })
            .collect();
        println!("🔄 Processing {} chunks for '{}'...", chunks.len(), character.character_name);
        total_chunks += chunks.len();
        for (i, chunk) in chunks.iter().enumerate() {
            println!("--------------------------------------------");
            println!(
                "🔄 Processing chunk {}/{}: {}...",
                i + 1,
                chunks.len(),
                chunk.content.chars().take(30).collect::<String>()
            );
            match generate_embedding(&client, chunk, model).await {
                Ok(true) => {
                    total_successful_chunks += 1;
                }
                Ok(false) | Err(_) => {
                    total_failed_chunks += 1;
                }
            }
            time::sleep(Duration::from_secs(2)).await;
        }
        total_characters += 1;
    }
    if !any_valid_character {
        println!("❌ No valid character data found in file. Please check your JSONL input format.");
        return Ok(());
    }

    // Print summary
    let total_time = chrono::Local::now();
    println!("==================================================");
    if processed_characters == 0 {
        println!("❌ No valid character data found in file. Please check your JSONL input format.");
    } else {
        println!("✅ HARALD CHUNKED INGEST COMPLETE");
        println!("📊 SUMMARY:");
        println!("   Characters processed: {}", processed_characters);
        println!("   Successful chunks: {}", total_successful_chunks);
        println!("   Failed chunks: {}", total_failed_chunks);
        println!("   Total chunks: {}", total_chunks);
        let success_rate = if total_chunks > 0 {
            (total_successful_chunks as f64 / total_chunks as f64) * 100.0
        } else {
            0.0
        };
        println!("   Success rate: {:.1}%", success_rate);
        println!("------------------------------------------------");
        println!("📋 Recommendations:");
        println!("   1. Keep chunks under 250-300 characters for reliable embedding");
        println!("   2. Process character data as separate attributes");
        println!("   3. For larger chunks (500-600 chars), use standalone Ollama server");
        println!("   4. The character limit may be system resource-related, not API-related");
        println!("   5. For best performance, close the Ollama GUI app and run 'ollama serve'");
        println!("==================================================");
        println!("🕒 Finished at {}", total_time.format("%Y-%m-%d %H:%M:%S"));
    }
    Ok(())
}
