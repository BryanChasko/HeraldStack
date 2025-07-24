use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "validate_json_schema",
    about = "JSON schema validation utility for vector data stores",
    long_about = "This tool validates JSON files against predefined schemas for vector data stores.\nIt works with the vector_stores_registry.json to ensure consistency."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Show detailed output
    #[arg(long)]
    verbose: bool,

    /// Path to registry file
    #[arg(long, default_value = "./config/vector-stores-registry.json")]
    registry: PathBuf,

    /// Path to schema directory
    #[arg(long, default_value = "./data/schemas")]
    schema_dir: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate all files for a specific store
    Store {
        /// Store ID to validate
        store_id: String,
    },
    /// Validate a specific file
    File {
        /// File path to validate
        file_path: PathBuf,
        /// Store ID (optional, will be auto-detected if not provided)
        #[arg(long)]
        store_id: Option<String>,
    },
    /// Generate schema from existing files
    Generate {
        /// Store ID to generate schema for
        store_id: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct VectorRegistry {
    #[serde(rename = "vectorStores")]
    vector_stores: Vec<VectorStore>,
}

#[derive(Debug, Deserialize, Serialize)]
struct VectorStore {
    id: String,
    name: String,
    description: String,
    #[serde(rename = "sourceFiles")]
    source_files: Vec<String>,
    #[serde(rename = "embeddingModel")]
    embedding_model: String,
    #[serde(rename = "chunkSize")]
    chunk_size: u32,
    #[serde(rename = "vectorDimensions")]
    vector_dimensions: u32,
}

#[derive(Debug, Serialize)]
struct JsonSchema {
    #[serde(rename = "$schema")]
    schema: String,
    title: String,
    description: String,
    #[serde(rename = "type")]
    schema_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Box<JsonSchema>>,
}

struct ValidationResult {
    total: usize,
    successful: usize,
    failed: usize,
    errors: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    check_dependencies()?;

    match cli.command {
        Commands::Store { store_id } => {
            validate_store_files(&store_id, &cli.registry, &cli.schema_dir, cli.verbose)?;
        }
        Commands::File {
            file_path,
            store_id,
        } => {
            validate_file(
                &file_path,
                store_id.as_deref(),
                &cli.registry,
                &cli.schema_dir,
                cli.verbose,
            )?;
        }
        Commands::Generate { store_id } => {
            generate_schema(&store_id, &cli.registry, &cli.schema_dir, cli.verbose)?;
        }
    }

    Ok(())
}

fn check_dependencies() -> Result<()> {
    let mut missing = false;

    // Check for jq
    if Command::new("jq").arg("--version").output().is_err() {
        log_error("jq is required but not installed. Please install with: brew install jq");
        missing = true;
    }

    // Check for ajv (JSON Schema validator) - optional but recommended
    if Command::new("ajv").arg("--version").output().is_err() {
        log_warning("ajv is recommended but not installed. For full schema validation, install with: npm install -g ajv-cli");
    }

    if missing {
        return Err(anyhow!("Missing required dependencies"));
    }

    Ok(())
}

fn load_registry(registry_path: &Path) -> Result<VectorRegistry> {
    let content = fs::read_to_string(registry_path)
        .with_context(|| format!("Failed to read registry file: {}", registry_path.display()))?;

    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse registry file: {}", registry_path.display()))
}

fn ensure_schema_dir(schema_dir: &Path) -> Result<()> {
    if !schema_dir.exists() {
        fs::create_dir_all(schema_dir).with_context(|| {
            format!(
                "Failed to create schema directory: {}",
                schema_dir.display()
            )
        })?;
        log_info(&format!(
            "Created schema directory: {}",
            schema_dir.display()
        ));
    }
    Ok(())
}

fn get_store_for_file(registry: &VectorRegistry, file_path: &str) -> Option<String> {
    // Convert to relative path for comparison
    let relative_path = file_path.strip_prefix("./").unwrap_or(file_path);

    for store in &registry.vector_stores {
        for source_file in &store.source_files {
            if source_file.contains(relative_path) || relative_path.contains(source_file) {
                return Some(store.id.clone());
            }
        }
    }
    None
}

fn validate_file(
    file_path: &Path,
    store_id: Option<&str>,
    registry_path: &Path,
    schema_dir: &Path,
    verbose: bool,
) -> Result<()> {
    if !file_path.exists() {
        return Err(anyhow!("File not found: {}", file_path.display()));
    }

    let registry = load_registry(registry_path)?;

    // Determine store_id if not provided
    let store_id = match store_id {
        Some(id) => id.to_string(),
        None => {
            let file_str = file_path.to_string_lossy().to_string();
            get_store_for_file(&registry, &file_str)
                .ok_or_else(|| anyhow!("Could not determine store ID for file: {}. Please specify store ID with --store-id option", file_path.display()))?
        }
    };

    let schema_file = schema_dir.join(format!("{}_schema.json", store_id));

    // Check if schema exists
    if !schema_file.exists() {
        log_warning(&format!("No schema found for store: {}", store_id));
        log_info(&format!("Use 'generate {}' to create schema", store_id));
        return Ok(());
    }

    log_info(&format!(
        "Validating {} against schema for {}",
        file_path.display(),
        store_id
    ));

    // Basic JSON validation
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    let json_value: Value = serde_json::from_str(&content)
        .with_context(|| format!("File is not valid JSON: {}", file_path.display()))?;

    // Advanced validation with ajv if available
    if Command::new("ajv").arg("--version").output().is_ok() {
        let result = Command::new("ajv")
            .args(&[
                "validate",
                "-s",
                &schema_file.to_string_lossy(),
                "-d",
                &file_path.to_string_lossy(),
                "--errors=text",
            ])
            .output()
            .context("Failed to run ajv validation")?;

        if !result.status.success() {
            log_error(&format!(
                "Schema validation failed for: {}",
                file_path.display()
            ));
            if verbose {
                let error_output = String::from_utf8_lossy(&result.stderr);
                println!("{}", error_output);
            }
            return Err(anyhow!("Schema validation failed"));
        } else {
            log_success(&format!(
                "Schema validation passed for: {}",
                file_path.display()
            ));
        }
    } else {
        // Fallback to basic structure validation
        let schema_content = fs::read_to_string(&schema_file)
            .with_context(|| format!("Failed to read schema file: {}", schema_file.display()))?;

        let schema: Value = serde_json::from_str(&schema_content)
            .with_context(|| format!("Invalid schema file: {}", schema_file.display()))?;

        validate_structure(&json_value, &schema, file_path)?;
        log_success(&format!(
            "Basic structure validation passed for: {}",
            file_path.display()
        ));
    }

    // Store-specific validations
    validate_store_specific(&json_value, &store_id, file_path)?;

    Ok(())
}

fn validate_structure(json_value: &Value, schema: &Value, file_path: &Path) -> Result<()> {
    if let Some(required_keys) = schema.get("required").and_then(|r| r.as_array()) {
        for key in required_keys {
            if let Some(key_str) = key.as_str() {
                if json_value.get(key_str).is_none() {
                    return Err(anyhow!(
                        "Missing required key in {}: {}",
                        file_path.display(),
                        key_str
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_store_specific(json_value: &Value, store_id: &str, _file_path: &Path) -> Result<()> {
    match store_id {
        "marvel_characters" => {
            // Specific validation for Marvel characters
            if json_value.is_array() {
                if let Some(first_item) = json_value.as_array().and_then(|arr| arr.first()) {
                    if first_item.get("character_name").is_none() {
                        return Err(anyhow!("Marvel character file missing character_name"));
                    }
                }
            }
        }
        "heralds" => {
            // Specific validation for Heralds
            if json_value.get("personality").is_none() {
                log_warning("Heralds file may be missing personality attributes");
            }
        }
        _ => {
            // Default validation
        }
    }
    Ok(())
}

fn validate_store_files(
    store_id: &str,
    registry_path: &Path,
    schema_dir: &Path,
    verbose: bool,
) -> Result<()> {
    let registry = load_registry(registry_path)?;

    // Check if store exists
    let store = registry
        .vector_stores
        .iter()
        .find(|s| s.id == store_id)
        .ok_or_else(|| {
            let available_stores: Vec<String> = registry
                .vector_stores
                .iter()
                .map(|s| s.id.clone())
                .collect();
            anyhow!(
                "Store not found: {}. Available stores: {}",
                store_id,
                available_stores.join(", ")
            )
        })?;

    let mut result = ValidationResult {
        total: store.source_files.len(),
        successful: 0,
        failed: 0,
        errors: Vec::new(),
    };

    log_info(&format!(
        "Validating {} files for store: {}",
        result.total, store_id
    ));

    for file in &store.source_files {
        println!();
        log_info(&format!("Validating: {}", file));

        let file_path = Path::new(file);
        match validate_file(
            file_path,
            Some(store_id),
            registry_path,
            schema_dir,
            verbose,
        ) {
            Ok(_) => result.successful += 1,
            Err(e) => {
                result.failed += 1;
                result.errors.push(format!("{}: {}", file, e));
            }
        }
    }

    println!();
    log_info(&format!("Validation complete for store {}:", store_id));
    println!("- Total files: {}", result.total);
    println!("- Successfully validated: {}", result.successful);
    if result.failed > 0 {
        log_warning(&format!("- Failed: {}", result.failed));
        if verbose {
            for error in &result.errors {
                log_error(error);
            }
        }
    } else {
        println!("- Failed: {}", result.failed);
    }

    if result.failed > 0 {
        return Err(anyhow!("Some validations failed"));
    }

    Ok(())
}

fn generate_schema(
    store_id: &str,
    registry_path: &Path,
    schema_dir: &Path,
    verbose: bool,
) -> Result<()> {
    let registry = load_registry(registry_path)?;

    // Check if store exists
    let store = registry
        .vector_stores
        .iter()
        .find(|s| s.id == store_id)
        .ok_or_else(|| {
            let available_stores: Vec<String> = registry
                .vector_stores
                .iter()
                .map(|s| s.id.clone())
                .collect();
            anyhow!(
                "Store not found: {}. Available stores: {}",
                store_id,
                available_stores.join(", ")
            )
        })?;

    if store.source_files.is_empty() {
        return Err(anyhow!("No files found for store: {}", store_id));
    }

    ensure_schema_dir(schema_dir)?;

    let schema_file = schema_dir.join(format!("{}_schema.json", store_id));
    let first_file = Path::new(&store.source_files[0]);

    log_info(&format!(
        "Generating schema for {} based on: {}",
        store_id, store.source_files[0]
    ));

    // Read and parse the first file
    let content = fs::read_to_string(first_file)
        .with_context(|| format!("Failed to read file: {}", first_file.display()))?;

    let json_value: Value = serde_json::from_str(&content)
        .with_context(|| format!("File is not valid JSON: {}", first_file.display()))?;

    // Generate schema based on file structure
    let schema = if json_value.is_array() {
        // Handle array structure
        if let Some(first_item) = json_value.as_array().and_then(|arr| arr.first()) {
            let properties = generate_properties_from_value(first_item);
            let required = extract_required_keys(first_item);

            JsonSchema {
                schema: "http://json-schema.org/draft-07/schema#".to_string(),
                title: format!("{} Schema", store_id),
                description: format!("Schema for {} JSON files", store_id),
                schema_type: "array".to_string(),
                required: None,
                properties: None,
                items: Some(Box::new(JsonSchema {
                    schema: "http://json-schema.org/draft-07/schema#".to_string(),
                    title: "Item".to_string(),
                    description: "Array item".to_string(),
                    schema_type: "object".to_string(),
                    required: Some(required),
                    properties: Some(properties),
                    items: None,
                })),
            }
        } else {
            return Err(anyhow!("Empty array in source file"));
        }
    } else {
        // Handle object structure
        let properties = generate_properties_from_value(&json_value);
        let required = extract_required_keys(&json_value);

        JsonSchema {
            schema: "http://json-schema.org/draft-07/schema#".to_string(),
            title: format!("{} Schema", store_id),
            description: format!("Schema for {} JSON files", store_id),
            schema_type: "object".to_string(),
            required: Some(required),
            properties: Some(properties),
            items: None,
        }
    };

    // Write schema to file
    let schema_json =
        serde_json::to_string_pretty(&schema).context("Failed to serialize schema")?;

    fs::write(&schema_file, schema_json)
        .with_context(|| format!("Failed to write schema file: {}", schema_file.display()))?;

    // Add store-specific customizations
    customize_schema_for_store(&schema_file, store_id)?;

    log_success(&format!("Generated schema: {}", schema_file.display()));

    if verbose {
        println!();
        println!("Schema contents:");
        println!("================");
        let final_content = fs::read_to_string(&schema_file)?;
        println!("{}", final_content);
    }

    Ok(())
}

fn generate_properties_from_value(value: &Value) -> Map<String, Value> {
    let mut properties = Map::new();

    if let Some(obj) = value.as_object() {
        for (key, val) in obj {
            let property_type = match val {
                Value::String(_) => "string",
                Value::Number(_) => "number",
                Value::Bool(_) => "boolean",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
                Value::Null => "null",
            };

            let mut property = Map::new();
            property.insert("type".to_string(), Value::String(property_type.to_string()));

            properties.insert(key.clone(), Value::Object(property));
        }
    }

    properties
}

fn extract_required_keys(value: &Value) -> Vec<String> {
    if let Some(obj) = value.as_object() {
        obj.keys().cloned().collect()
    } else {
        Vec::new()
    }
}

fn customize_schema_for_store(schema_file: &Path, store_id: &str) -> Result<()> {
    let content = fs::read_to_string(schema_file)?;
    let mut schema: Value = serde_json::from_str(&content)?;

    match store_id {
        "marvel_characters" => {
            // Add specific schema rules for Marvel characters
            if let Some(items) = schema.get_mut("items") {
                if let Some(properties) = items.get_mut("properties") {
                    if let Some(char_name) = properties.get_mut("character_name") {
                        if let Some(obj) = char_name.as_object_mut() {
                            obj.insert(
                                "description".to_string(),
                                Value::String("The name of the Marvel character".to_string()),
                            );
                        }
                    }
                }
            }
        }
        _ => {
            // Default customization
        }
    }

    let updated_content = serde_json::to_string_pretty(&schema)?;
    fs::write(schema_file, updated_content)?;

    Ok(())
}

fn log_info(msg: &str) {
    println!("{} {}", "[INFO]".blue().bold(), msg);
}

fn log_success(msg: &str) {
    println!("{} {}", "[SUCCESS]".green().bold(), msg);
}

fn log_warning(msg: &str) {
    println!("{} {}", "[WARNING]".yellow().bold(), msg);
}

fn log_error(msg: &str) {
    eprintln!("{} {}", "[ERROR]".red().bold(), msg);
}
