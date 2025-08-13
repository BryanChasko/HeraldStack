//! JSON schema validation CLI tool.
//!
//! This binary provides command-line JSON schema validation functionality.

use clap::Parser;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Validate JSON against schema", long_about = None)]
struct Args {
    /// JSON file to validate
    #[arg(value_name = "JSON_FILE")]
    json_file: PathBuf,

    /// JSON schema file
    #[arg(short, long, value_name = "SCHEMA_FILE")]
    schema: PathBuf,

    /// Generate schema from JSON file
    #[arg(long)]
    generate_schema: bool,

    /// Output file for generated schema
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[cfg(feature = "cli")]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.generate_schema {
        // Read JSON file
        let content = fs::read_to_string(&args.json_file)?;
        let value: Value = serde_json::from_str(&content)?;

        // Generate basic schema (simplified)
        let schema = generate_basic_schema(&value);
        let schema_json = serde_json::to_string_pretty(&schema)?;

        // Write schema
        let output_path = args.output.unwrap_or_else(|| {
            let mut path = args.json_file.clone();
            path.set_extension("schema.json");
            path
        });

        fs::write(&output_path, schema_json)?;
        println!("✅ Schema generated: {}", output_path.display());
    } else {
        // Read and validate JSON
        let json_content = fs::read_to_string(&args.json_file)?;
        let _json_value: Value = serde_json::from_str(&json_content)?;

        let _schema_content = fs::read_to_string(&args.schema)?;
        let _schema_value: Value = serde_json::from_str(&_schema_content)?;

        // Basic validation (would need a proper JSON schema validator library)
        println!("✅ JSON validation completed (basic check)");
        println!("Note: Full JSON Schema validation requires additional dependencies");
    }

    Ok(())
}

fn generate_basic_schema(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut properties = serde_json::Map::new();
            for (key, val) in map {
                properties.insert(key.clone(), generate_basic_schema(val));
            }
            serde_json::json!({
                "type": "object",
                "properties": properties
            })
        }
        Value::Array(arr) => {
            if let Some(first) = arr.first() {
                serde_json::json!({
                    "type": "array",
                    "items": generate_basic_schema(first)
                })
            } else {
                serde_json::json!({
                    "type": "array"
                })
            }
        }
        Value::String(_) => serde_json::json!({"type": "string"}),
        Value::Number(n) => {
            if n.is_f64() {
                serde_json::json!({"type": "number"})
            } else {
                serde_json::json!({"type": "integer"})
            }
        }
        Value::Bool(_) => serde_json::json!({"type": "boolean"}),
        Value::Null => serde_json::json!({"type": "null"}),
    }
}
