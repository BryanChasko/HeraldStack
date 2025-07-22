// marvelai_ingest.rs
// Migration of ingest_marvelai.sh to Rust
// Ingests MarvelAIs.json, splits into chunks, validates, tests embedding API, and runs ingestion.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

use clap::Parser;
use reqwest::Client;
use tokio::runtime::Runtime;
mod embed;
use embed::{embed_with_config, EmbedConfig};

#[derive(Parser, Debug)]
#[command(author, version, about = "MarvelAI Ingest Tool", long_about = None)]
struct Args {
    /// Path to the MarvelAIs.json file
    #[arg(
        short,
        long,
        default_value = "/Users/bryanchasko/Code/HARALD/personality-archetypes/pop-culture/marvel/MarvelAIs.json"
    )]
    input: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let source_file = args.input;
    let model = "harald-phi4";
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let marvel_dir = temp_dir.path().join("marvel");
    fs::create_dir_all(&marvel_dir).expect("Failed to create marvel dir");

    let start = Instant::now();
    println!("==================================================");
    println!("🚀 HARALD MARVELAI INGEST (Rust)");
    println!("🔍 Processing MarvelAIs.json using JSONL format");
    println!("==================================================");

    // 1. Copy source file to temp dir
    let dest_file = marvel_dir.join("MarvelAIs.json");
    if let Err(e) = fs::copy(&source_file, &dest_file) {
        eprintln!("❌ Failed to copy MarvelAIs.json: {}", e);
        return;
    }

    // 2. Convert to JSONL (one object per line)
    let jsonl_file = marvel_dir.join("MarvelAIs.jsonl");
    println!("Converting {:?} to JSONL at {:?}", dest_file, jsonl_file);
    let file_content = match fs::read_to_string(&dest_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to read MarvelAIs.json: {}", e);
            return;
        }
    };
    let json_array: serde_json::Value = match serde_json::from_str(&file_content) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("❌ Invalid JSON in MarvelAIs.json: {}", e);
            return;
        }
    };
    let arr = match json_array.as_array() {
        Some(a) => a,
        None => {
            eprintln!("❌ Expected top-level array in MarvelAIs.json");
            return;
        }
    };
    let mut jsonl = String::new();
    for obj in arr {
        let line = match serde_json::to_string(obj) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("❌ Failed to serialize object to JSONL: {}", e);
                continue;
            }
        };
        jsonl.push_str(&line);
        jsonl.push('\n');
    }
    if let Err(e) = fs::write(&jsonl_file, jsonl) {
        eprintln!("❌ Failed to write JSONL file: {}", e);
        return;
    }
    println!("✅ JSONL conversion complete: {} lines", arr.len());

    // 3. Validate JSONL (each line is valid JSON)
    println!("Validating JSONL file: {:?}", jsonl_file);
    let jsonl_content = match fs::read_to_string(&jsonl_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to read JSONL file: {}", e);
            return;
        }
    };
    let mut valid_lines = 0;
    let mut invalid_lines = 0;
    for (i, line) in jsonl_content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<serde_json::Value>(line) {
            Ok(_) => valid_lines += 1,
            Err(e) => {
                println!("  ❌ Invalid JSON on line {}: {}", i + 1, e);
                invalid_lines += 1;
                if invalid_lines > 5 {
                    println!("  Too many errors, stopping validation");
                    break;
                }
            }
        }
    }
    if invalid_lines == 0 {
        println!("  ✅ JSONL validation successful!");
    } else {
        println!("  ❌ JSONL validation failed with {} errors", invalid_lines);
    }

    // 4. Test embedding API (Ollama) using shared embed.rs logic
    println!("Testing embedding API with model {}", model);
    let test_prompt = "test";
    let client = Client::new();
    let embed_config = EmbedConfig::default();
    let mut success = false;
    for attempt in 1..=3 {
        println!("  Attempt {}/3: embedding '{}'", attempt, test_prompt);
        let result = embed_with_config(test_prompt, 100, &client, embed_config.clone()).await;
        match result {
            Ok(embedding) => {
                println!(
                    "  ✅ Embedding vectors received successfully (dim: {})",
                    embedding.len()
                );
                success = true;
                break;
            }
            Err(e) => {
                println!("  ❌ Embedding request failed: {}", e);
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    if !success {
        println!("  ❌ Failed to generate embeddings with harald-phi4 model after 3 attempts");
    }

    // 5. Split JSON into smaller chunks and process each chunk
    println!("Splitting JSON into chunks...");
    let chunks_dir = marvel_dir.join("chunks");
    if let Err(e) = fs::create_dir_all(&chunks_dir) {
        eprintln!("❌ Failed to create chunks dir: {}", e);
        return;
    }
    let total = arr.len();
    let chunk_size = (total as f64 / 5.0).ceil() as usize;
    for i in 0..5 {
        let start = i * chunk_size;
        let end = ((i + 1) * chunk_size).min(total);
        if start >= end {
            break;
        }
        let chunk: Vec<_> = arr[start..end].to_vec();
        let chunk_path = chunks_dir.join(format!("chunk{}.json", i + 1));
        if let Err(e) = fs::write(&chunk_path, serde_json::to_string_pretty(&chunk).unwrap()) {
            eprintln!("❌ Failed to write chunk file {:?}: {}", chunk_path, e);
            continue;
        }
        println!(
            "  Wrote chunk {} ({} items) to {:?}",
            i + 1,
            chunk.len(),
            chunk_path
        );
    }

    // 6. For each chunk: test embedding, run ingestion, log results, clean up
    println!("Processing each chunk: test embedding, run ingestion, log results, clean up");
    let mut processed_count = 0;
    let mut failed_count = 0;
    for i in 0..5 {
        let chunk_path = chunks_dir.join(format!("chunk{}.json", i + 1));
        if !chunk_path.exists() {
            continue;
        }
        println!("\n--- Processing chunk {} ---", i + 1);
        // Read chunk and get first character name for embedding test
        let chunk_content = match fs::read_to_string(&chunk_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ Failed to read chunk file {:?}: {}", chunk_path, e);
                failed_count += 1;
                continue;
            }
        };
        let chunk_json: Vec<serde_json::Value> = match serde_json::from_str(&chunk_content) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("❌ Invalid chunk JSON in {:?}: {}", chunk_path, e);
                failed_count += 1;
                continue;
            }
        };
        let char_name = chunk_json
            .get(0)
            .and_then(|v| v.get("character_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("test");
        let mut chunk_success = false;
        for attempt in 1..=2 {
            println!("  Attempt {}/2: embedding '{}'", attempt, char_name);
            let result = embed_with_config(char_name, 100, &client, embed_config.clone()).await;
            match result {
                Ok(embedding) => {
                    println!(
                        "  ✅ Embedding vectors received successfully (dim: {})",
                        embedding.len()
                    );
                    chunk_success = true;
                    break;
                }
                Err(e) => {
                    println!("  ❌ Embedding request failed: {}", e);
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
        if !chunk_success {
            println!(
                "  ❌ Failed to generate embeddings for chunk {} after 2 attempts",
                i + 1
            );
            failed_count += 1;
            continue;
        }

        // Run ingestion logic (call harald_ingest binary)
        let candidate_bins = [
            "rust_ingest/target/debug/harald_ingest",
            "rust_ingest/target/release/harald_ingest",
            "src/target/debug/harald_ingest",
            "src/target/release/harald_ingest",
            "target/debug/harald_ingest",
            "target/release/harald_ingest",
        ];
        let ingest_bin = candidate_bins
            .iter()
            .map(PathBuf::from)
            .find(|p| p.exists())
            .unwrap_or_else(|| {
                println!("❌ Could not find harald_ingest binary in any known location.");
                PathBuf::from("harald_ingest_NOT_FOUND")
            });
        if !ingest_bin.exists() {
            println!("❌ harald_ingest binary not found. Please build it in rust_ingest/target/{{debug,release}}/");
            failed_count += 1;
            continue;
        }
        let chunk_dir = match tempfile::tempdir() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("❌ Failed to create chunk temp dir: {}", e);
                failed_count += 1;
                continue;
            }
        };
        let chunk_marvel_dir = chunk_dir.path().join("marvel");
        if let Err(e) = fs::create_dir_all(&chunk_marvel_dir) {
            eprintln!("❌ Failed to create chunk marvel dir: {}", e);
            failed_count += 1;
            continue;
        }
        let chunk_file_path = chunk_marvel_dir.join("chunk.json");
        if let Err(e) = fs::write(
            &chunk_file_path,
            serde_json::to_string_pretty(&chunk_json).unwrap(),
        ) {
            eprintln!("❌ Failed to write chunk file {:?}: {}", chunk_file_path, e);
            failed_count += 1;
            continue;
        }
        let output = Command::new(&ingest_bin)
            .arg("ingest")
            .arg("--root")
            .arg(chunk_dir.path())
            .output();
        match output {
            Ok(out) => {
                println!("Ingest stdout:\n{}", String::from_utf8_lossy(&out.stdout));
                println!("Ingest stderr:\n{}", String::from_utf8_lossy(&out.stderr));
                if out.status.success() {
                    println!("✅ Successfully processed chunk {}", i + 1);
                    processed_count += 1;
                } else {
                    println!("❌ Failed to process chunk {}", i + 1);
                    failed_count += 1;
                }
            }
            Err(e) => {
                println!("❌ Failed to run ingest binary: {}", e);
                failed_count += 1;
            }
        }
        // Clean up chunk temp dir automatically
    }

    // 7. Print summary and clean up
    let elapsed = start.elapsed();
    println!("==================================================");
    println!("✅ HARALD MARVELAI INGEST COMPLETE (Rust)");
    println!("Total execution time: {:.2?}", elapsed);
    println!("Chunks processed successfully: {}", processed_count);
    println!("Chunks failed: {}", failed_count);
    println!("==================================================");
}
