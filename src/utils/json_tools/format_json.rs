//! JSON formatting and validation CLI tool.
//!
//! This binary provides command-line JSON formatting and validation functionality.

use clap::Parser;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Format and validate JSON files", long_about = None)]
struct Args {
    /// Input JSON file path
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output file path (default: overwrite input)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Pretty print with indentation
    #[arg(short, long, default_value_t = 2)]
    indent: usize,

    /// Validate only, don't format
    #[arg(long)]
    validate_only: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Read input file
    let content = fs::read_to_string(&args.input)?;

    // Parse JSON to validate
    let value: Value = serde_json::from_str(&content)?;

    if args.validate_only {
        println!("✅ JSON is valid");
        return Ok(());
    }

    // Format JSON
    let formatted = if args.indent > 0 {
        serde_json::to_string_pretty(&value)?
    } else {
        serde_json::to_string(&value)?
    };

    // Write output
    let output_path = args.output.unwrap_or(args.input);
    fs::write(&output_path, formatted)?;

    println!("✅ Formatted JSON written to: {}", output_path.display());
    Ok(())
}
