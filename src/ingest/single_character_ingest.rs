// single_character_ingest.rs
// Migration of ingest_single_character.sh to Rust
// This tool ingests a minimal single-character JSON file for pipeline validation.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

mod chunking_utils;
use chunking_utils::chunk_entity_fields;
mod ingest_utils;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Single Character Ingest Test", long_about = None)]
struct Args {
    /// Path to the single character JSON file (array of objects)
    #[arg(
        short,
        long,
        default_value = "/Users/bryanchasko/Code/HARALD/tests/fixtures/test_single_character.json"
    )]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let source_file = args.input;
    let model = "harald-phi4";
    // Configurable retry and delay
    let max_retries = std::env::var("EMBED_MAX_RETRIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);
    let retry_delay = std::env::var("EMBED_RETRY_DELAY_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let marvel_dir = temp_dir.path().join("marvel");
    fs::create_dir_all(&marvel_dir).expect("Failed to create marvel dir");

    // Start timer
    let start = Instant::now();

    // Read and parse the input file
    let file_content = fs::read_to_string(&source_file).expect("Failed to read character.json");
    let json_array: serde_json::Value = serde_json::from_str(&file_content).expect("Invalid JSON");
    let arr = json_array.as_array().expect("Expected top-level array");

    // Detect and select the entry for 'Vision'
    let vision_obj = arr.iter().find(|v| {
        v.get("character_name")
            .and_then(|n| n.as_str())
            .map(|n| n.eq_ignore_ascii_case("vision"))
            .unwrap_or(false)
    });
    if vision_obj.is_none() {
        println!("❌ No entry for 'Vision' found in test JSON file.");
        return;
    }
    let vision_obj = vision_obj.unwrap();

    // Prepare JSONL file path (guaranteed supported extension and root placement)
    let jsonl_file = marvel_dir.join("vision.jsonl");

    // Chunk and embed the 'vision' entry
    println!("\n🔍 Chunking and embedding 'Vision' entry...");
    let max_embed_len = 250;
    let chunks = chunk_entity_fields(vision_obj, max_embed_len); // Now always <=250 chars per chunk
    println!("[DEBUG] Chunking produced {} chunks:", chunks.len());
    for (label, chunk) in &chunks {
        println!("    - [{}] ({} chars)", label, chunk.len());
    }
    let client = reqwest::blocking::Client::new();
    // Pre-flight API check
    println!("\n🔎 Checking Ollama API status...");
    let api_status = client
        .get("http://localhost:11434/")
        .timeout(Duration::from_secs(5))
        .send();
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

    let mut failed_chunks = Vec::new();
    let mut successful_chunks = Vec::new();
    let mut failed_chunk_details = Vec::new();
    // Embed all chunks for testing
    if chunks.is_empty() {
        println!("❌ No chunks found to embed.");
        return;
    }
    for (label, chunk) in &chunks {
        println!(
            "  [TEST] Embedding chunk: [{}] ({} chars): {}",
            label,
            chunk.len(),
            chunk.chars().take(80).collect::<String>()
        );
        let test_request = serde_json::json!({
            "model": model,
            "prompt": format!("{}: {}", label, chunk)
        });
        let mut success = false;
        let mut last_error = String::new();
        for attempt in 1..=max_retries {
            println!(
                "    Attempt {}/{}: embedding '{}'",
                attempt, max_retries, label
            );
            let resp = client
                .post("http://localhost:11434/api/embeddings")
                .header("Content-Type", "application/json")
                .timeout(Duration::from_secs(20))
                .body(test_request.to_string())
                .send();
            match resp {
                Ok(r) => {
                    let status = r.status();
                    let body = r.text().unwrap_or_default();
                    if status.is_success() && body.contains("embedding") {
                        println!("      ✅ Embedding vectors received successfully");
                        success = true;
                        successful_chunks.push(label.clone());
                        break;
                    } else {
                        println!("      ❌ API error (status {}):\n{}", status, body);
                        last_error = format!("API error (status {}): {}", status, body);
                    }
                }
                Err(e) => {
                    println!("      ❌ Network error: {}", e);
                    last_error = format!("Network error: {}", e);
                }
            }
            let backoff = retry_delay * attempt;
            println!("      ⏳ Waiting {}s before next attempt...", backoff);
            std::thread::sleep(Duration::from_secs(backoff as u64));
        }
        if !success {
            println!(
                "    ❌ Failed to embed chunk '{}' after {} attempts.",
                label, max_retries
            );
            failed_chunks.push(label.clone());
            failed_chunk_details.push((label.clone(), last_error));
        }
    }
    if !failed_chunks.is_empty() {
        println!(
            "⚠️  Partial ingest: failed to embed {} chunk(s): {:?}",
            failed_chunks.len(),
            failed_chunks
        );
        println!("   Failed chunk details:");
        for (label, err) in &failed_chunk_details {
            println!("   - [{}]: {}", label, err);
        }
        // Log failed chunks for later retry
        let log_path = marvel_dir.join("failed_chunks.log");
        let log_content = failed_chunk_details
            .iter()
            .map(|(l, e)| format!("{}: {}", l, e))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&log_path, log_content).expect("Failed to write failed_chunks.log");
        println!("   Failed chunk details logged to: {}", log_path.display());
        if successful_chunks.is_empty() {
            println!("❌ Aborting ingest: no chunks embedded successfully.");
            return;
        } else {
            println!(
                "✅ Proceeding to ingest {} successful chunk(s)...",
                successful_chunks.len()
            );
        }
    } else {
        println!("✅ All chunks for 'Vision' embedded successfully. Proceeding to ingest...");
    }

    // 6. Run ingestion logic (call Rust ingest binary or function)
    println!("\nRunning ingestion logic on 'Vision' entry...");
    let candidate_bins = [
        "rust_ingest/target/debug/ingest_chunked",
        "rust_ingest/target/release/ingest_chunked",
        "src/target/debug/ingest_chunked",
        "src/target/release/ingest_chunked",
        "target/release/ingest_chunked",
    ];
    let ingest_bin = candidate_bins
        .iter()
        .map(PathBuf::from)
        .find(|p| p.exists());
    if ingest_bin.is_none() {
        println!("❌ No ingest binary found. Please build the project.");
        return;
    }
    let ingest_bin = ingest_bin.unwrap();
    // Add a 'description' field if missing, for ingest compatibility
    let mut vision_obj = vision_obj.clone();
    if vision_obj.get("description").is_none() {
        vision_obj.as_object_mut().map(|obj| {
            obj.insert(
                "description".to_string(),
                serde_json::Value::String("Test description for ingest validation".to_string()),
            )
        });
    }
    let vision_jsonl =
        serde_json::to_string(&vision_obj).expect("Failed to serialize Vision object");
    // Write the JSONL file directly into the root directory for ingest
    fs::write(&jsonl_file, format!("{}\n", vision_jsonl))
        .expect("Failed to write Vision JSONL file");
    let (_valid_lines, invalid_lines) = ingest_utils::validate_jsonl_lines(
        &fs::read_to_string(&jsonl_file).expect("Failed to read JSONL file"),
    );
    if invalid_lines == 0 {
        println!("  ✅ JSONL validation successful!");
    } else {
        panic!("  ❌ JSONL validation failed with {} errors", invalid_lines);
    }
    // Print the contents of the JSONL file for debugging
    println!("\n--- JSONL file contents ---");
    match fs::read_to_string(&jsonl_file) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("❌ Failed to read JSONL file: {}", e),
    }
    println!("--- END JSONL ---\n");

    // Copy the JSONL file into the test output directory for ingest
    let output_dir = PathBuf::from("tests/output");
    fs::create_dir_all(&output_dir).expect("Failed to create test output directory");
    let output_jsonl = output_dir.join("vision.jsonl");
    fs::copy(&jsonl_file, &output_jsonl)
        .expect("Failed to copy JSONL file to test output directory");
    // Run the ingest binary with the test output directory as root
    match Command::new(&ingest_bin)
        .args(&["ingest", "--root", output_dir.to_str().unwrap()])
        .output()
    {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            println!("Ingest stdout:\n{}", stdout);
            println!("Ingest stderr:\n{}", stderr);
            // Parse summary from stdout for JSONL ingest
            let mut success = false;
            let mut failed = 0;
            let mut total = 0;
            for line in stdout.lines() {
                // Removed unused 'processed' variable and assignment
                if line.contains("Successful chunks:") {
                    if let Some(val) = line.split(':').nth(1) {
                        success = val.trim().parse::<usize>().unwrap_or(0) > 0;
                    }
                }
                if line.contains("Failed chunks:") {
                    if let Some(val) = line.split(':').nth(1) {
                        failed = val.trim().parse().unwrap_or(0);
                    }
                }
                if line.contains("Total chunks:") {
                    if let Some(val) = line.split(':').nth(1) {
                        total = val.trim().parse().unwrap_or(0);
                    }
                }
            }
            if total > 0 && success && failed == 0 {
                println!(
                    "✅ Successfully processed all {} chunks for 'Vision' (JSONL)",
                    total
                );
            } else if total > 0 && success {
                println!(
                    "⚠️  Processed 'Vision' with {} successful and {} failed chunks (JSONL)",
                    total - failed,
                    failed
                );
            } else if total == 0 {
                println!("❌ No chunks were processed for 'Vision' character data (JSONL)");
            } else {
                println!("⚠️  Ingest completed, but some chunks may have failed. Please check logs and output above.");
            }
        }
        Err(e) => {
            println!("❌ Failed to run ingest binary: {}", e);
        }
    }

    // 7. Clean up and print summary
    let elapsed = start.elapsed();
    println!("==================================================");
    println!("✅ HARALD SINGLE CHARACTER TEST COMPLETE (Rust)");
    println!("Total execution time: {:.2?}", elapsed);
    println!("==================================================");
}
