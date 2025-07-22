// single_character_ingest.rs
// Migration of ingest_single_character.sh to Rust
// This tool ingests a minimal single-character JSON file for pipeline validation.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

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
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let marvel_dir = temp_dir.path().join("marvel");
    fs::create_dir_all(&marvel_dir).expect("Failed to create marvel dir");

    // 1. Print start banner and timing
    let start = Instant::now();
    println!("==================================================");
    println!("🚀 HARALD SINGLE CHARACTER TEST (Rust)");
    println!("🔍 Processing a single character JSON file for testing");
    println!("==================================================");

    // 2. Copy test file to temp dir
    let dest_file = marvel_dir.join("character.json");
    fs::copy(&source_file, &dest_file).expect("Failed to copy test file");

    // 3. Convert to JSONL (one object per line)
    let jsonl_file = marvel_dir.join("character.jsonl");
    println!("Converting {:?} to JSONL at {:?}", dest_file, jsonl_file);
    let file_content = fs::read_to_string(&dest_file).expect("Failed to read character.json");
    let json_array: serde_json::Value = serde_json::from_str(&file_content).expect("Invalid JSON");
    let arr = json_array.as_array().expect("Expected top-level array");
    let mut jsonl = String::new();
    for obj in arr {
        let line = serde_json::to_string(obj).expect("Failed to serialize object");
        jsonl.push_str(&line);
        jsonl.push('\n');
    }
    fs::write(&jsonl_file, jsonl).expect("Failed to write JSONL file");
    println!("✅ JSONL conversion complete: {} lines", arr.len());

    // 4. Validate JSONL (each line is valid JSON)
    println!("Validating JSONL file: {:?}", jsonl_file);
    let jsonl_content = fs::read_to_string(&jsonl_file).expect("Failed to read JSONL file");
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
            }
        }
    }
    if invalid_lines == 0 {
        println!("  ✅ JSONL validation successful!");
    } else {
        println!("  ❌ JSONL validation failed with {} errors", invalid_lines);
    }

    // 5. Test embedding API (Ollama) with retries and logging
    println!("Testing embedding API with model {}", model);
    let char_name = arr
        .get(0)
        .and_then(|v| v.get("character_name"))
        .and_then(|v| v.as_str())
        .unwrap_or("test");
    let test_request = serde_json::json!({
        "model": model,
        "prompt": char_name
    });
    let client = reqwest::blocking::Client::new();
    let mut success = false;
    for attempt in 1..=3 {
        println!("  Attempt {}/3: embedding '{}'", attempt, char_name);
        let resp = client
            .post("http://localhost:11434/api/embeddings")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(15))
            .body(test_request.to_string())
            .send();
        match resp {
            Ok(r) => {
                let status = r.status();
                let body = r.text().unwrap_or_default();
                if status.is_success() && body.contains("embedding") {
                    println!("  ✅ Embedding vectors received successfully");
                    success = true;
                    break;
                } else {
                    println!(
                        "  ❌ No embedding vectors in response (status {}): {}",
                        status,
                        body.chars().take(200).collect::<String>()
                    );
                }
            }
            Err(e) => {
                println!("  ❌ Request failed: {}", e);
            }
        }
        std::thread::sleep(Duration::from_secs(5));
    }
    if !success {
        println!("  ❌ Failed to generate embeddings with harald-phi4 model after 3 attempts");
    }

    // 6. Run ingestion logic (call Rust ingest binary or function)
    println!("Running ingestion logic on single character data...");
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
    } else {
        let ingest_root = temp_dir.path();
        let output = Command::new(&ingest_bin)
            .arg("--root")
            .arg(ingest_root)
            .output();
        match output {
            Ok(out) => {
                println!("Ingest stdout:\n{}", String::from_utf8_lossy(&out.stdout));
                println!("Ingest stderr:\n{}", String::from_utf8_lossy(&out.stderr));
                if out.status.success() {
                    println!("✅ Successfully processed single character data");
                } else {
                    println!("❌ Failed to process single character data");
                }
            }
            Err(e) => {
                println!("❌ Failed to run ingest binary: {}", e);
            }
        }
    }

    // 7. Clean up and print summary
    let elapsed = start.elapsed();
    println!("==================================================");
    println!("✅ HARALD SINGLE CHARACTER TEST COMPLETE (Rust)");
    println!("Total execution time: {:.2?}", elapsed);
    println!("==================================================");
}
