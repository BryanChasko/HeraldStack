use crate::ingest::{run_with_config, IngestConfig};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "MarvelAI Marvel Ingest Wrapper", long_about = None)]
struct Args {
    /// Path to the root directory for ingestion
    #[arg(short, long, default_value = ".")]
    root_dir: PathBuf,
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
async fn main() {
    let args = Args::parse();
    let config = IngestConfig {
        root_dir: args.root_dir,
        max_chars: args.max_chars,
        max_tokens: args.max_tokens,
        max_concurrent_files: args.max_concurrent_files,
    };
    match run_with_config(config).await {
        Ok(_) => println!("MarvelAI ingest completed successfully."),
        Err(e) => eprintln!("MarvelAI ingest failed: {}", e),
    }
}
