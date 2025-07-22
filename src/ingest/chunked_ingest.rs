use anyhow::{Context, Result};
use clap::{Arg, Command};
use serde_json::Value;
use std::fs;
use std::time::{Duration, Instant};
use tokio::time;

// Import our existing utilities through the harald crate
use harald::core::embedding::ollama_api::OllamaApiClient;
use harald::utils::chunking::{chunk_text, ChunkerOptions, ChunkingStrategy};

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

/// Statistics for the ingestion process
#[derive(Debug)]
struct IngestionStats {
    successful_chunks: usize,
    failed_chunks: usize,
    total_chunks: usize,
    start_time: Instant,
}

impl IngestionStats {
    fn new() -> Self {
        Self {
            successful_chunks: 0,
            failed_chunks: 0,
            total_chunks: 0,
            start_time: Instant::now(),
        }
    }

    fn success_rate(&self) -> f64 {
        if self.total_chunks == 0 {
            0.0
        } else {
            (self.successful_chunks as f64 / self.total_chunks as f64) * 100.0
        }
    }

    fn elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl CharacterData {
    /// Parse character data from JSON value
    fn from_json(value: &Value) -> Result<Self> {
        let character_name = value["character_name"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();

        let first_appearance = value["first_appearance"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();

        let affiliations = value["affiliations"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        let core_attributes = value["core_attributes"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        let inspirational_themes = value["inspirational_themes"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        let traits = value["traits"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        let ai_alignment = value["ai_alignment"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();

        let description = value["description"].as_str().map(|s| s.to_string());

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

    /// Convert character data into chunks for embedding
    fn to_chunks(&self) -> Result<Vec<CharacterChunk>> {
        let mut chunks = Vec::new();
        const MAX_CHUNK_SIZE: usize = 250;

        // Add character name - use as-is since it's typically short
        chunks.push(CharacterChunk {
            label: "Name".to_string(),
            content: self.character_name.clone(),
        });

        // Add first appearance - use as-is since it's typically short
        chunks.push(CharacterChunk {
            label: "First Appearance".to_string(),
            content: self.first_appearance.clone(),
        });

        // Process affiliations with character-based chunking if long
        let affiliations_text = self.affiliations.join(", ");
        if affiliations_text.len() > MAX_CHUNK_SIZE {
            println!(
                "⚠️  Long affiliations text ({} chars), using character-based chunking",
                affiliations_text.len()
            );
            let options = ChunkerOptions {
                strategy: ChunkingStrategy::Character(MAX_CHUNK_SIZE),
                ..Default::default()
            };
            let chunked = chunk_text(&affiliations_text, options);
            for chunk in chunked {
                chunks.push(CharacterChunk {
                    label: "Affiliations".to_string(),
                    content: chunk,
                });
            }
        } else {
            chunks.push(CharacterChunk {
                label: "Affiliations".to_string(),
                content: affiliations_text,
            });
        }

        // Process attributes with character-based chunking if long
        let attributes_text = self.core_attributes.join(", ");
        if attributes_text.len() > MAX_CHUNK_SIZE {
            println!(
                "⚠️  Long attributes text ({} chars), using character-based chunking",
                attributes_text.len()
            );
            let options = ChunkerOptions {
                strategy: ChunkingStrategy::Character(MAX_CHUNK_SIZE),
                ..Default::default()
            };
            let chunked = chunk_text(&attributes_text, options);
            for chunk in chunked {
                chunks.push(CharacterChunk {
                    label: "Attributes".to_string(),
                    content: chunk,
                });
            }
        } else {
            chunks.push(CharacterChunk {
                label: "Attributes".to_string(),
                content: attributes_text,
            });
        }

        // Process themes with character-based chunking
        let themes_text = self.inspirational_themes.join(", ");
        if themes_text.len() > MAX_CHUNK_SIZE {
            println!(
                "⚠️  Long themes text ({} chars), using character-based chunking",
                themes_text.len()
            );
            let options = ChunkerOptions {
                strategy: ChunkingStrategy::Character(MAX_CHUNK_SIZE),
                ..Default::default()
            };
            let chunked = chunk_text(&themes_text, options);
            for chunk in chunked {
                chunks.push(CharacterChunk {
                    label: "Themes".to_string(),
                    content: chunk,
                });
            }
        } else {
            chunks.push(CharacterChunk {
                label: "Themes".to_string(),
                content: themes_text,
            });
        }

        // Process traits with character-based chunking
        let traits_text = self.traits.join(", ");
        if traits_text.len() > MAX_CHUNK_SIZE {
            println!(
                "⚠️  Long traits text ({} chars), using character-based chunking",
                traits_text.len()
            );
            let options = ChunkerOptions {
                strategy: ChunkingStrategy::Character(MAX_CHUNK_SIZE),
                ..Default::default()
            };
            let chunked = chunk_text(&traits_text, options);
            for chunk in chunked {
                chunks.push(CharacterChunk {
                    label: "Traits".to_string(),
                    content: chunk,
                });
            }
        } else {
            chunks.push(CharacterChunk {
                label: "Traits".to_string(),
                content: traits_text,
            });
        }

        // Add AI alignment - typically short
        chunks.push(CharacterChunk {
            label: "AI Alignment".to_string(),
            content: self.ai_alignment.clone(),
        });

        // For any description field, use semantic chunking
        if let Some(description) = &self.description {
            if !description.is_empty() {
                println!("📝 Processing description field with semantic chunking");
                let options = ChunkerOptions {
                    strategy: ChunkingStrategy::Semantic,
                    ..Default::default()
                };
                let chunked = chunk_text(description, options);
                for chunk in chunked {
                    chunks.push(CharacterChunk {
                        label: "Description".to_string(),
                        content: chunk,
                    });
                }
            }
        }

        Ok(chunks)
    }
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

    // Read and parse the input file
    println!("📄 Processing file: {}", file_path);
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let json_data: Value =
        serde_json::from_str(&file_content).with_context(|| "Failed to parse JSON")?;

    // Extract the first character (assuming array format)
    let character_json = json_data
        .as_array()
        .and_then(|arr| arr.first())
        .ok_or_else(|| anyhow::anyhow!("No character data found in file"))?;

    let character = CharacterData::from_json(character_json)?;
    println!("🔍 Processing character: {}", character.character_name);
    println!("🔄 Using character-based chunking for large fields");

    // Convert character to chunks
    let chunks = character.to_chunks()?;

    // Process each chunk
    let mut stats = IngestionStats::new();
    stats.total_chunks = chunks.len();

    println!("🔄 Processing {} character data chunks...", chunks.len());

    for (i, chunk) in chunks.iter().enumerate() {
        println!("--------------------------------------------");
        println!(
            "🔄 Processing chunk {}/{}: {}...",
            i + 1,
            chunks.len(),
            chunk.content.chars().take(30).collect::<String>()
        );

        if generate_embedding(&client, chunk, model).await? {
            stats.successful_chunks += 1;
        } else {
            stats.failed_chunks += 1;
        }

        // Small delay between chunks
        time::sleep(Duration::from_secs(2)).await;
    }

    // Print summary
    let total_time = stats.elapsed_time();
    println!("==================================================");
    println!("✅ HARALD CHUNKED INGEST COMPLETE");
    println!("📊 SUMMARY:");
    println!("   Character: {}", character.character_name);
    println!("   Successful chunks: {}", stats.successful_chunks);
    println!("   Failed chunks: {}", stats.failed_chunks);
    println!("   Total chunks: {}", stats.total_chunks);
    println!("   Success rate: {:.1}%", stats.success_rate());
    println!("   Total execution time: {:.2}s", total_time.as_secs_f64());
    println!("------------------------------------------------");
    println!("📋 Recommendations:");
    println!("   1. Keep chunks under 250-300 characters for reliable embedding");
    println!("   2. Process character data as separate attributes");
    println!("   3. For larger chunks (500-600 chars), use standalone Ollama server");
    println!("   4. The character limit may be system resource-related, not API-related");
    println!("   5. For best performance, close the Ollama GUI app and run 'ollama serve'");
    println!("==================================================");
    println!(
        "🕒 Finished at {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    Ok(())
}
