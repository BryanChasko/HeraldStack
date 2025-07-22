//! Standalone binary for the Ollama embeddings functionality.
//!
//! This provides a command-line interface to the Ollama embeddings API,
//! matching the features of the various embedding-related scripts.

use anyhow::Result;
use clap::{Parser, Subcommand};
use harald::core::embedding::ollama_api::OllamaApiClient;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;

/// Command-line tool for generating embeddings using Ollama.
#[derive(Parser)]
#[command(
    name = "embedding_tool",
    about = "Generate and manage embeddings using Ollama API",
    long_about = "A tool for generating vector embeddings from text using the Ollama API."
)]
struct Cli {
    /// Subcommands for different embedding operations
    #[command(subcommand)]
    command: Commands,

    /// Base URL for the Ollama API
    #[arg(long, default_value = "http://localhost:11434")]
    base_url: String,

    /// Timeout in seconds for API requests
    #[arg(long, default_value = "30")]
    timeout: u64,

    /// Model to use for embeddings
    #[arg(long, default_value = "harald-phi4")]
    model: String,
}

/// Available commands for the embedding tool
#[derive(Subcommand)]
enum Commands {
    /// Check if the Ollama API is available
    CheckStatus {},

    /// Generate an embedding for a text string
    Generate {
        /// Text to generate an embedding for
        #[arg(required = true)]
        text: String,

        /// Maximum chunk size in characters
        #[arg(long, default_value = "250")]
        max_chunk_size: usize,

        /// Enable chunking for long text
        #[arg(long, default_value = "false")]
        chunked: bool,
    },

    /// Generate embeddings for a file
    GenerateFile {
        /// Path to the file to generate embeddings for
        #[arg(required = true)]
        file_path: String,

        /// Enable chunking for long text
        #[arg(long, default_value = "true")]
        chunked: bool,

        /// Maximum chunk size in characters
        #[arg(long, default_value = "250")]
        max_chunk_size: usize,
    },

    /// Test embedding with different text sizes to find the limit
    TestSizes {
        /// Log file directory
        #[arg(long, default_value = "./logs")]
        log_dir: String,

        /// Custom sizes to test (comma-separated)
        #[arg(long)]
        sizes: Option<String>,

        /// Start size for testing
        #[arg(long, default_value = "10")]
        start_size: usize,

        /// End size for testing
        #[arg(long, default_value = "3000")]
        end_size: usize,

        /// Step size for testing
        #[arg(long, default_value = "100")]
        step_size: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let rt = Runtime::new()?;

    // Create API client
    let client = OllamaApiClient::new(&cli.base_url).with_timeout(cli.timeout);

    match cli.command {
        Commands::CheckStatus {} => {
            rt.block_on(check_status(client))?;
        }
        Commands::Generate {
            text,
            max_chunk_size,
            chunked,
        } => {
            rt.block_on(generate_embedding(
                client,
                text,
                cli.model,
                max_chunk_size,
                chunked,
            ))?;
        }
        Commands::GenerateFile {
            file_path,
            max_chunk_size,
            chunked,
        } => {
            rt.block_on(generate_embeddings_for_file(
                client,
                file_path,
                cli.model,
                max_chunk_size,
                chunked,
            ))?;
        }
        Commands::TestSizes {
            log_dir,
            sizes,
            start_size,
            end_size,
            step_size,
        } => {
            rt.block_on(test_embedding_sizes(
                client, cli.model, log_dir, sizes, start_size, end_size, step_size,
            ))?;
        }
    }

    Ok(())
}

async fn check_status(client: OllamaApiClient) -> Result<()> {
    println!("🔍 Checking Ollama API status...");

    match client.check_status().await {
        Ok(true) => {
            println!("✅ Ollama API is available");
            Ok(())
        }
        Ok(false) => {
            println!("❌ Ollama API is not responding properly");
            anyhow::bail!("API returned unsuccessful status")
        }
        Err(e) => {
            println!("❌ Failed to connect to Ollama API: {}", e);
            Err(e)
        }
    }
}

async fn generate_embedding(
    client: OllamaApiClient,
    text: String,
    model: String,
    max_chunk_size: usize,
    chunked: bool,
) -> Result<()> {
    println!("🔄 Generating embedding for text ({} chars)", text.len());

    let start = std::time::Instant::now();

    if chunked && text.len() > max_chunk_size {
        println!(
            "⚠️  Text exceeds recommended size of {} chars",
            max_chunk_size
        );
        println!("   Will process in smaller chunks");

        let embeddings = client
            .generate_embedding_chunked(&text, &model, max_chunk_size)
            .await?;

        println!(
            "✅ Success - Generated {} chunk embeddings in {:?}",
            embeddings.len(),
            start.elapsed()
        );

        for (i, embedding) in embeddings.iter().enumerate() {
            println!("   Chunk {}: Vector dimensions: {}", i + 1, embedding.len());
        }
    } else {
        let embedding = client.generate_embedding(&text, &model).await?;

        println!("✅ Success - Embedding generated in {:?}", start.elapsed());
        println!("   Vector dimensions: {}", embedding.len());
    }

    Ok(())
}

async fn generate_embeddings_for_file(
    client: OllamaApiClient,
    file_path: String,
    model: String,
    max_chunk_size: usize,
    chunked: bool,
) -> Result<()> {
    use std::fs;

    println!("📄 Processing file: {}", file_path);

    let content = fs::read_to_string(&file_path)?;
    println!("📝 File content size: {} characters", content.len());

    generate_embedding(client, content, model, max_chunk_size, chunked).await
}

/// Test embedding with different text sizes to find the limit
async fn test_embedding_sizes(
    client: OllamaApiClient,
    model: String,
    log_dir: String,
    custom_sizes: Option<String>,
    start_size: usize,
    end_size: usize,
    step_size: usize,
) -> Result<()> {
    use chrono::Local;
    use std::fs;

    println!("==================================================");
    println!("🔍 HARALD EMBEDDING SIZE TEST");
    println!("🕒 Started at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("==================================================");

    // Create logs directory
    fs::create_dir_all(&log_dir)?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let log_file = format!("{}/embedding_size_test_{}.log", log_dir, timestamp);
    println!("📝 Logging to: {}", log_file);

    let mut log = File::create(&log_file)?;
    writeln!(log, "HARALD EMBEDDING SIZE TEST")?;
    writeln!(
        log,
        "Started at: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;

    // Determine sizes to test
    let sizes = if let Some(sizes_str) = custom_sizes {
        sizes_str
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect::<Vec<_>>()
    } else {
        (start_size..=end_size)
            .step_by(step_size)
            .collect::<Vec<_>>()
    };

    // Define timeouts based on size
    let get_timeout = |size: usize| -> u64 {
        if size <= 100 {
            15
        } else if size <= 500 {
            30
        } else if size <= 1000 {
            60
        } else if size <= 2000 {
            90
        } else {
            120
        }
    };

    println!("🔄 Testing with text sizes: {:?}", sizes);
    writeln!(log, "\nTesting sizes: {:?}", sizes)?;

    let mut max_success_size = 0;

    for size in sizes {
        println!("--------------------------------------------");
        writeln!(log, "\n--------------------------------------------")?;
        writeln!(log, "Testing size: {} characters", size)?;

        let timeout = get_timeout(size);
        let test_text = "X".repeat(size);

        println!(
            "🧪 Testing embedding with text size: {} chars (timeout: {}s)",
            size, timeout
        );

        let start_time = Instant::now();

        // Try to generate embedding
        let result = tokio::time::timeout(
            Duration::from_secs(timeout),
            client.generate_embedding(&test_text, &model),
        )
        .await;

        let elapsed = start_time.elapsed();

        match result {
            // Timeout occurred
            Err(_) => {
                println!("⚠️  Request timed out after {}s", timeout);
                println!("   Text size {} is too large for the embedding API", size);

                writeln!(log, "TIMEOUT after {}s", timeout)?;
                writeln!(log, "Text size {} is too large", size)?;

                // Break the loop, we've found our limit
                break;
            }

            // Got a response (success or error)
            Ok(embedding_result) => match embedding_result {
                // Successful embedding
                Ok(embedding) => {
                    println!("✅ Success - Embedding generated in {:?}", elapsed);
                    println!("   Vector dimensions: {}", embedding.len());

                    writeln!(log, "SUCCESS - Embedding generated in {:?}", elapsed)?;
                    writeln!(log, "Vector dimensions: {}", embedding.len())?;

                    // Save the detail log
                    let detail_log = format!("{}.{}", log_file, size);
                    let mut detail_file = File::create(&detail_log)?;
                    writeln!(detail_file, "Embedding size test for {} characters", size)?;
                    writeln!(detail_file, "Generated {} dimensions", embedding.len())?;

                    max_success_size = size;
                }

                // API error
                Err(e) => {
                    println!("❌ Failed - Error: {}", e);
                    writeln!(log, "ERROR: {}", e)?;

                    // Break the loop, we've found our limit
                    break;
                }
            },
        }

        // Small delay between tests
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    // Print summary
    println!("==================================================");
    println!("✅ HARALD EMBEDDING SIZE TEST COMPLETE");
    println!("📊 Results:");
    println!(
        "   Maximum successful text size: {} characters",
        max_success_size
    );

    if max_success_size > 0 {
        println!(
            "   Recommendation: Keep chunks under {} characters for reliable embedding",
            max_success_size
        );
    } else {
        println!("   Warning: Even the smallest test failed. Check Ollama API health.");
    }

    println!("🔍 Logs saved to: {}", log_file);
    println!("==================================================");
    println!(
        "🕒 Finished at {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    writeln!(
        log,
        "\n===================================================="
    )?;
    writeln!(log, "TEST COMPLETE")?;
    writeln!(
        log,
        "Maximum successful text size: {} characters",
        max_success_size
    )?;
    writeln!(
        log,
        "Finished at: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;

    Ok(())
}
