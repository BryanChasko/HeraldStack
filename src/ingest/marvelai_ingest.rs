mod chunking_utils;
use chunking_utils::chunk_entity_fields;
// marvelai_ingest.rs
// Migration of ingest_marvelai.sh to Rust
// Ingests MarvelAIs.json, splits into chunks (≤250 chars per field), validates, tests embedding API, and runs ingestion.
// 
// Lessons learned: 
// - All chunked fields must be ≤250 characters for reliable embedding/API stability.
// - Error messages should be clear and actionable.
// - Debug output is essential for diagnosing chunking and embedding issues.
// - CLI usage and input/output formats must be explicit.
// - Troubleshooting hints help avoid common pitfalls.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

use clap::Parser;
use reqwest::Client;

mod embed;
mod ingest_utils;
use embed::{embed_with_config, EmbedConfig};
use ingest_utils::*;

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
    /// Enable verbose debug output
    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let source_file = args.input;
    let debug = args.debug;
    let model = "harald-phi4";
    // Configurable retry and delay
    let max_retries = std::env::var("EMBED_MAX_RETRIES").ok().and_then(|v| v.parse().ok()).unwrap_or(3);
    let retry_delay = std::env::var("EMBED_RETRY_DELAY_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(5);
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let marvel_dir = temp_dir.path().join("marvel");
    fs::create_dir_all(&marvel_dir).expect("Failed to create marvel dir");

    let start = Instant::now();
    println!("==================================================");
    println!("🚀 HARALD MARVELAI INGEST (Rust)");
    println!("🔍 Processing MarvelAIs.json using JSONL format");
    println!("  - All chunked fields are ≤250 characters for reliable embedding");
    println!("  - Use --debug for verbose output and diagnostics");
    println!("==================================================");

    // 1. Copy source file to temp dir
    let dest_file = marvel_dir.join("MarvelAIs.json");
    if let Err(e) = fs::copy(&source_file, &dest_file) {
        eprintln!("❌ Failed to copy {:?} to {:?}: {}", source_file, dest_file, e);
        return;
    }

    // 2. Convert to JSONL (one object per line)
    let jsonl_file = marvel_dir.join("MarvelAIs.jsonl");
    println!("Converting {:?} to JSONL at {:?}", dest_file, jsonl_file);
    let file_content = match fs::read_to_string(&dest_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to read {:?}: {}", dest_file, e);
            return;
        }
    };
    let arr: Vec<serde_json::Value> = match serde_json::from_str(&file_content) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("❌ Invalid JSON array in {:?}: {}", dest_file, e);
            return;
        }
    };
    let mut jsonl = String::new();
    for (i, obj) in arr.iter().enumerate() {
        let line = match serde_json::to_string(obj) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("❌ Failed to serialize object {} to JSON: {}", i, e);
                continue;
            }
        };
        jsonl.push_str(&line);
        jsonl.push('\n');
    }
    if let Err(e) = fs::write(&jsonl_file, &jsonl) {
        eprintln!("❌ Failed to write to {:?}: {}", jsonl_file, e);
        return;
    }
    println!("✅ JSONL conversion complete: {} lines", arr.len());

    // 3. Validate JSONL (each line is valid JSON)
    println!("Validating JSONL file: {:?}", jsonl_file);
    let jsonl_content = match fs::read_to_string(&jsonl_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to read {:?}: {}", jsonl_file, e);
            return;
        }
    };
    let (_, invalid_lines) = validate_jsonl_lines(&jsonl_content);
    if invalid_lines == 0 {
        println!("  ✅ JSONL validation successful!");
    } else {
        println!("  ❌ JSONL validation failed with {} errors", invalid_lines);
        println!("  ⚠️ Please check your input file for formatting issues.");
    }

    // 4. Test embedding API (Ollama) using shared embed.rs logic
    println!("Testing embedding API with model {}", model);
    let test_prompt = "test";
    let client = Client::new();
    let embed_config = EmbedConfig::default();
    // Pre-flight Ollama API status check
    println!("\n🔎 Checking Ollama API status...");
    let api_status = client.get("http://localhost:11434/")
        .timeout(std::time::Duration::from_secs(5)).send().await;
    match api_status {
        Ok(resp) if resp.status().is_success() => {
            println!("  ✅ Ollama API reachable.");
        }
        Ok(resp) => {
            println!("  ❌ Ollama API returned error status: {}", resp.status());
            println!("     Aborting before embedding.");
            return;
        }
        Err(e) => {
            println!("  ❌ Ollama API unreachable: {}", e);
            println!("     Aborting before embedding.");
            return;
        }
    }
    let mut success = false;
    for attempt in 1..=max_retries {
        println!("  Attempt {}/{}: embedding '{}'", attempt, max_retries, test_prompt);
        let result = embed_with_config(test_prompt, 100, &client, embed_config.clone()).await;
        match result {
            Ok(embedding) => {
                println!("  ✅ Embedding vectors received successfully (dim: {})", embedding.len());
                success = true;
                break;
            }
            Err(e) => {
                println!("  ❌ Embedding request failed: {}", e);
            }
        }
        let backoff = retry_delay * attempt;
        println!("      ⏳ Waiting {}s before next attempt...", backoff);
        tokio::time::sleep(std::time::Duration::from_secs(backoff as u64)).await;
    }
    if !success {
        println!("  ❌ Failed to generate embeddings with harald-phi4 model after {} attempts", max_retries);
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
        if debug {
            println!("    [DEBUG] Chunk {} contents:", i + 1);
            for (j, obj) in chunk.iter().enumerate() {
                println!("      - Object {}: {} chars", j + 1, serde_json::to_string(obj).unwrap_or_default().chars().count());
            }
        }
    }

    // 6. For each chunk: test embedding, run ingestion, log results, clean up
    println!("Processing each chunk: test embedding, run ingestion, log results, clean up");
    let mut processed_count = 0;
    let mut failed_count = 0;
    for i in 0..5 {
        let chunk_path = chunks_dir.join(format!("chunk{}.json", i + 1));
        if !chunk_path.exists() {
            println!("  ⚠️ Chunk file {:?} does not exist, skipping.", chunk_path);
            continue;
        }
        println!("\n--- Processing chunk {} ---", i + 1);
        if debug {
            println!("    [DEBUG] Validating chunk JSON...");
        }
        // Read chunk and validate
        let chunk_content = match fs::read_to_string(&chunk_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ Failed to read chunk file {:?}: {}", chunk_path, e);
                failed_count += 1;
                continue;
            }
        };
        // Validate chunk JSON
        let chunk_json: Vec<serde_json::Value> = match serde_json::from_str(&chunk_content) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("❌ Invalid chunk JSON in {:?}: {}", chunk_path, e);
                failed_count += 1;
                continue;
            }
        };
        if chunk_json.is_empty() {
            println!("  ⚠️ Chunk {} is empty, skipping.", i + 1);
            continue;
        }
        // Validate each object in chunk
        let mut valid_objs = 0;
        let mut invalid_objs = 0;
        for (idx, obj) in chunk_json.iter().enumerate() {
            match serde_json::to_string(obj) {
                Ok(line) => {
                    if serde_json::from_str::<serde_json::Value>(&line).is_ok() {
                        valid_objs += 1;
                    } else {
                        println!(
                            "    ❌ Invalid JSON object at index {} in chunk {}",
                            idx,
                            i + 1
                        );
                        invalid_objs += 1;
                    }
                }
                Err(e) => {
                    println!(
                        "    ❌ Failed to serialize object at index {} in chunk {}: {}",
                        idx,
                        i + 1,
                        e
                    );
                    invalid_objs += 1;
                }
            }
        }
        if invalid_objs > 0 {
            println!(
                "  ❌ Chunk {} has {} invalid objects, skipping.",
                i + 1,
                invalid_objs
            );
            println!("  Valid objects in chunk {}: {}", i + 1, valid_objs);
            failed_count += 1;
            continue;
        }
        // Proceed with embedding and ingestion
        let char_name = chunk_json
            .get(0)
            .and_then(|v| v.get("character_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("test");

        // Modular chunking for long fields before embedding
        let max_embed_len = 250;
        let mut chunk_success = true;
        for obj in &chunk_json {
        for (field, chunk) in chunk_entity_fields(obj, max_embed_len) {
            if debug {
                println!("    [DEBUG] Field '{}' chunk size: {} chars", field, chunk.chars().count());
            }
            let mut last_error = String::new();
            let mut success = false;
            for attempt in 1..=max_retries {
                println!("  Attempt {}/{}: embedding '{}.{}' ({} chars)", attempt, max_retries, field, char_name, chunk.chars().count());
                let result = embed_with_config(&chunk, 100, &client, embed_config.clone()).await;
                match result {
                    Ok(embedding) => {
                        println!("  ✅ Embedding vectors received successfully (dim: {})", embedding.len());
                        success = true;
                        break;
                    }
                    Err(e) => {
                        println!("  ❌ Embedding request failed for '{}.{}': {}", field, char_name, e);
                        last_error = format!("Embedding request failed: {}", e);
                    }
                }
                let backoff = retry_delay * attempt;
                println!("      ⏳ Waiting {}s before next attempt...", backoff);
                tokio::time::sleep(std::time::Duration::from_secs(backoff as u64)).await;
            }
            if !success {
                println!("  ❌ Failed to generate embeddings for '{}.{}' after {} attempts. Aborting further chunk processing.", field, char_name, max_retries);
                failed_count += 1;
                // Log failed chunk for later retry
                let log_path = marvel_dir.join("failed_chunks.log");
                let log_content = format!("{}: {}\n", field, last_error);
                fs::write(&log_path, log_content).expect("Failed to write failed_chunks.log");
                break;
            }
        }
        if !chunk_success {
            break;
        }
        }
        if !chunk_success {
            break;
        }

        // Run ingestion logic (call harald_ingest binary)
        let candidate_bins = [
            "rust_ingest/target/debug/rust_ingest",
            "rust_ingest/target/release/rust_ingest",
            "src/target/debug/rust_ingest",
            "src/target/release/rust_ingest",
            "target/debug/rust_ingest",
            "target/release/rust_ingest",
        ];
        let ingest_bin = candidate_bins
            .iter()
            .map(PathBuf::from)
            .find(|p| p.exists())
            .unwrap_or_else(|| {
                println!("❌ Could not find rust_ingest binary in any known location. Aborting further chunk processing.");
                PathBuf::from("rust_ingest_NOT_FOUND")
            });
        if !ingest_bin.exists() {
            println!("❌ rust_ingest binary not found. Please build it in rust_ingest/target/{{debug,release}}/. Ingest process interrupted.");
            failed_count += 1;
            break;
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
                    println!(
                        "❌ Failed to process chunk {}. Aborting further chunk processing.",
                        i + 1
                    );
                    failed_count += 1;
                    break;
                }
            }
            Err(e) => {
                println!(
                    "❌ Failed to run ingest binary: {}. Aborting further chunk processing.",
                    e
                );
                failed_count += 1;
                break;
            }
        }
        // Clean up chunk temp dir automatically
    }

    // 7. Print summary and clean up
    let elapsed = start.elapsed();
    println!("==================================================");
    println!("==================================================");
    println!("✅ HARALD MARVELAI INGEST COMPLETE (Rust)");
    println!("Total execution time: {:.2?}", elapsed);
    println!("Chunks processed successfully: {}", processed_count);
    println!("Chunks failed: {}", failed_count);
    println!("--------------------------------------------------");
    println!("Tips:");
    println!("- Keep chunk sizes ≤250 characters for reliable embedding");
    println!("- Use --debug for detailed diagnostics");
    println!("- Validate JSONL input before running ingestion");
    println!("- If you see network/API errors, check Ollama status and logs");
    println!("==================================================");
}
