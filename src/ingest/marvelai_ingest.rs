use anyhow::{Context, Result};
use clap::Parser;
use harald::ingest::embed;
use harald::ingest::{run_with_config, IngestConfig};
use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "MarvelAI Marvel Ingest Tool", long_about = None)]
struct Args {
    /// Path to the MarvelAIs.json file
    #[arg(
        short,
        long,
        default_value = "personality-archetypes/pop-culture/marvel/MarvelAIs.json"
    )]
    input: PathBuf,

    /// Maximum characters to read per file
    #[arg(long, default_value_t = 800)]
    max_chars: usize,

    /// Maximum tokens for embedding requests
    #[arg(long, default_value_t = 600)]
    max_tokens: usize,

    /// Maximum number of files to process concurrently
    #[arg(long)]
    max_concurrent_files: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("==================================================");
    println!("🚀 HARALD MARVELAI INGEST (Rust)");
    println!("🔍 Processing MarvelAIs.json using JSONL format");
    println!("==================================================");

    // Verify input file exists
    if !args.input.exists() {
        eprintln!("❌ Input file not found: {}", args.input.display());
        std::process::exit(1);
    }

    // Test embedding API first - exit early if it fails
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10)) // Reduced from 30 to 10 seconds
        .build()
        .context("Failed to create HTTP client")?;

    println!("Testing embedding API with model harald-phi4");

    // Test with a simple string first
    match test_embedding_api(&client, args.max_tokens).await {
        Ok(_) => println!("  ✅ Embedding API test successful"),
        Err(e) => {
            eprintln!("  ❌ Embedding API test failed: {}", e);
            eprintln!(
                "❌ Cannot proceed without working embedding API. Please check Ollama is running."
            );
            std::process::exit(1);
        }
    }

    // Convert JSON to JSONL if needed
    let jsonl_path = prepare_jsonl_input(&args.input)?;

    // Create a temporary directory for processing
    let temp_dir = tempfile::TempDir::new().context("Failed to create temporary directory")?;

    // Copy JSONL to temp directory
    let temp_jsonl = temp_dir.path().join("MarvelAIs.jsonl");
    fs::copy(&jsonl_path, &temp_jsonl).context("Failed to copy JSONL to temp directory")?;

    // Configure ingestion to use the temp directory
    let config = IngestConfig {
        root_dir: temp_dir.path().to_path_buf(),
        max_chars: args.max_chars,
        max_tokens: args.max_tokens,
        max_concurrent_files: args.max_concurrent_files,
    };

    // Run the standard harald_ingest logic
    match run_with_config(config).await {
        Ok(stats) => {
            println!("✅ MarvelAI ingest completed successfully!");
            println!("📁 Processed: {} files", stats.files_processed);
            println!("⏭️  Skipped: {} files", stats.files_skipped);
            println!("💾 Output: {}", stats.output_dir.display());
        }
        Err(e) => {
            eprintln!("❌ MarvelAI ingest failed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Test the embedding API with a simple request to ensure it's working
async fn test_embedding_api(client: &Client, max_tokens: usize) -> Result<()> {
    let test_text = "test";

    // Create a fast-fail config for testing - using localhost endpoint as recommended
    let test_config = embed::EmbedConfig {
        model: "harald-phi4".to_string(),
        endpoint: "http://localhost:11434/api/embeddings".to_string(),
        timeout_secs: 15, // Longer timeout to account for model loading
        max_retries: 2,   // Allow 2 attempts for initial API warmup
    };

    println!("  Testing: embedding '{}' (using {})", test_text, test_config.endpoint);
    println!("  Model warmup may take a moment on first request...");

    match embed::embed_with_config(test_text, max_tokens, client, test_config).await {
        Ok(embedding) => {
            if embedding.is_empty() {
                return Err(anyhow::anyhow!("Received empty embedding vector"));
            }
            println!("  ✅ Embedding vectors received successfully ({} dimensions)", embedding.len());
            Ok(())
        }
        Err(e) => {
            println!("  ❌ Request failed: {}", e);
            
            // Provide helpful debugging information
            eprintln!("  💡 Troubleshooting tips:");
            eprintln!("     • Ensure 'ollama serve' is running in a terminal");
            eprintln!("     • Verify harald-phi4 model is available: ollama list");
            eprintln!("     • Check API endpoint: curl http://localhost:11434/api/version");
            
            Err(anyhow::anyhow!(
                "Failed to generate embeddings with harald-phi4 model (fast test failed)"
            ))
        }
    }
}

/// Prepare JSONL input file from the MarvelAIs.json file
fn prepare_jsonl_input(input_path: &PathBuf) -> Result<PathBuf> {
    // If it's already JSONL, return as-is
    if input_path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
        return Ok(input_path.clone());
    }

    // Read and parse JSON file
    let json_content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read JSON file: {}", input_path.display()))?;

    let json_value: Value = serde_json::from_str(&json_content)
        .with_context(|| format!("Failed to parse JSON file: {}", input_path.display()))?;

    // Create JSONL output path
    let mut jsonl_path = input_path.clone();
    jsonl_path.set_extension("jsonl");

    // Convert to JSONL
    let jsonl_content = match json_value {
        Value::Array(items) => {
            // Array of objects - convert each to a line
            items
                .iter()
                .map(|item| serde_json::to_string(item))
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to serialize JSON items")?
                .join("\n")
        }
        _ => {
            // Single object - just one line
            serde_json::to_string(&json_value).context("Failed to serialize JSON object")?
        }
    };

    // Write JSONL file
    fs::write(&jsonl_path, &jsonl_content)
        .with_context(|| format!("Failed to write JSONL file: {}", jsonl_path.display()))?;

    let line_count = jsonl_content.lines().count();
    println!(
        "Converting \"{}\" to JSONL at \"{}\"",
        input_path.display(),
        jsonl_path.display()
    );
    println!("✅ JSONL conversion complete: {} lines", line_count);

    Ok(jsonl_path)
}
