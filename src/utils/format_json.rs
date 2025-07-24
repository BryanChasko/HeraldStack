use anyhow::{Context, Result};
use chrono::Local;
use clap::{Arg, ArgAction, Command};
use colored::*;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as SystemCommand;

/// Registry configuration
#[derive(Debug, Clone)]
struct RegistryConfig {
    file_path: PathBuf,
}

/// Vector store definition from registry
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct VectorStore {
    id: String,
    description: String,
    #[serde(rename = "sourceFiles")]
    source_files: Vec<String>,
    #[serde(rename = "dataLocation")]
    data_location: String,
    #[serde(rename = "ingestScript")]
    ingest_script: String,
    #[serde(rename = "embeddingModel")]
    embedding_model: String,
    #[serde(rename = "chunkingStrategy")]
    chunking_strategy: String,
    #[serde(rename = "maxChunkSize")]
    max_chunk_size: u32,
}

/// Vector store registry structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct VectorRegistry {
    #[serde(rename = "vectorStores")]
    vector_stores: Vec<VectorStore>,
    #[serde(rename = "embeddingModels")]
    embedding_models: Vec<Value>,
    #[serde(rename = "lastUpdated")]
    last_updated: String,
}

/// Processing mode
#[derive(Debug, Clone, PartialEq)]
enum ProcessingMode {
    Format,
    Check,
    Register(String),
    ValidateRegistry,
}

/// Target for processing
#[derive(Debug, Clone)]
enum ProcessingTarget {
    All,
    Store(String),
    File(String),
}

/// Processing statistics
#[derive(Debug, Default)]
struct ProcessingStats {
    total: usize,
    successful: usize,
    failed: usize,
}

impl ProcessingStats {
    fn add_success(&mut self) {
        self.successful += 1;
        self.total += 1;
    }

    fn add_failure(&mut self) {
        self.failed += 1;
        self.total += 1;
    }
}

/// Logging utilities
fn log_info(message: &str) {
    println!("{} {}", "[INFO]".blue().bold(), message);
}

fn log_success(message: &str) {
    println!("{} {}", "[SUCCESS]".green().bold(), message);
}

fn log_warning(message: &str) {
    println!("{} {}", "[WARNING]".yellow().bold(), message);
}

fn log_error(message: &str) {
    eprintln!("{} {}", "[ERROR]".red().bold(), message);
}

/// Check for required dependencies
fn check_dependencies() -> Result<()> {
    let mut missing = false;

    // Check for jq
    if SystemCommand::new("jq").arg("--version").output().is_err() {
        log_error("jq is required but not installed. Please install with: brew install jq");
        missing = true;
    }

    // Check for prettier
    if SystemCommand::new("prettier")
        .arg("--version")
        .output()
        .is_err()
    {
        log_error(
            "prettier is required but not installed. Please install with: npm install -g prettier",
        );
        missing = true;
    }

    if missing {
        return Err(anyhow::anyhow!("Missing required dependencies"));
    }

    Ok(())
}

/// Load and validate the vector store registry
fn load_registry(config: &RegistryConfig) -> Result<VectorRegistry> {
    if !config.file_path.exists() {
        return Err(anyhow::anyhow!(
            "Registry file not found: {:?}",
            config.file_path
        ));
    }

    let content = fs::read_to_string(&config.file_path)
        .with_context(|| format!("Failed to read registry file: {:?}", config.file_path))?;

    let registry: VectorRegistry =
        serde_json::from_str(&content).with_context(|| "Registry file is not valid JSON")?;

    Ok(registry)
}

/// Save the registry back to file
fn save_registry(config: &RegistryConfig, registry: &VectorRegistry) -> Result<()> {
    let content =
        serde_json::to_string_pretty(registry).with_context(|| "Failed to serialize registry")?;

    fs::write(&config.file_path, content)
        .with_context(|| format!("Failed to write registry file: {:?}", config.file_path))?;

    Ok(())
}

/// Validate the registry structure and content
fn validate_registry(config: &RegistryConfig, verbose: bool) -> Result<()> {
    let registry = load_registry(config)?;

    log_success("Registry validation successful");

    if verbose {
        println!();
        println!("Registry Details:");
        println!("=================");
        println!("Total vector stores: {}", registry.vector_stores.len());

        let model_names: Vec<String> = registry
            .embedding_models
            .iter()
            .filter_map(|m| m.get("id"))
            .filter_map(|id| id.as_str())
            .map(|s| s.to_string())
            .collect();
        println!("Embedding models: {}", model_names.join(", "));
        println!();
        println!("Registered stores:");

        for store in &registry.vector_stores {
            println!(
                "- {}: {} ({} files)",
                store.id,
                store.description,
                store.source_files.len()
            );
        }
    }

    Ok(())
}

/// Check if a file is registered in the vector store registry
fn is_file_registered(registry: &VectorRegistry, file_path: &str) -> bool {
    let current_dir = std::env::current_dir().unwrap_or_default();
    let relative_path = Path::new(file_path)
        .strip_prefix(&current_dir)
        .unwrap_or(Path::new(file_path))
        .to_string_lossy()
        .to_string();

    registry.vector_stores.iter().any(|store| {
        store
            .source_files
            .iter()
            .any(|f| f.contains(&relative_path))
    })
}

/// Get the store ID for a given file
fn get_store_for_file(registry: &VectorRegistry, file_path: &str) -> Option<String> {
    let current_dir = std::env::current_dir().unwrap_or_default();
    let relative_path = Path::new(file_path)
        .strip_prefix(&current_dir)
        .unwrap_or(Path::new(file_path))
        .to_string_lossy()
        .to_string();

    registry
        .vector_stores
        .iter()
        .find(|store| {
            store
                .source_files
                .iter()
                .any(|f| f.contains(&relative_path))
        })
        .map(|store| store.id.clone())
}

/// Format a JSON file using prettier
fn format_json_file(file_path: &str, check_mode: bool, show_diff: bool) -> Result<bool> {
    if !Path::new(file_path).exists() {
        return Err(anyhow::anyhow!("File not found: {}", file_path));
    }

    // Validate JSON format first
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let _: Value = serde_json::from_str(&content)
        .with_context(|| format!("File is not valid JSON: {}", file_path))?;

    if check_mode {
        // Create formatted version in temp file
        let temp_file = tempfile::NamedTempFile::new()?;
        let temp_path = temp_file.path().to_string_lossy().to_string();

        let output = SystemCommand::new("prettier")
            .args(&["--parser", "json", file_path])
            .output()
            .with_context(|| "Failed to run prettier")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Prettier failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        fs::write(&temp_path, output.stdout)?;

        // Compare with original
        let original_content = fs::read_to_string(file_path)?;
        let formatted_content = fs::read_to_string(&temp_path)?;

        if original_content == formatted_content {
            log_success(&format!("File already properly formatted: {}", file_path));
            Ok(true)
        } else {
            log_warning(&format!("File needs formatting: {}", file_path));
            if show_diff {
                println!();
                println!("Diff for {}:", file_path);
                // Simple diff - in a real implementation you might use a proper diff library
                println!("(Diff output would show here)");
                println!();
            }
            Ok(false)
        }
    } else {
        // Format the file in place
        let status = SystemCommand::new("prettier")
            .args(&["--parser", "json", "--write", file_path])
            .status()
            .with_context(|| "Failed to run prettier")?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "Prettier failed to format file: {}",
                file_path
            ));
        }

        // Re-validate JSON after formatting
        let formatted_content = fs::read_to_string(file_path)?;
        let _: Value = serde_json::from_str(&formatted_content)
            .with_context(|| "JSON validation failed after formatting!")?;

        log_success(&format!("Formatted file: {}", file_path));
        Ok(true)
    }
}

/// Register a new file in the vector store registry
fn register_new_file(config: &RegistryConfig, file_path: &str) -> Result<()> {
    if !Path::new(file_path).exists() {
        return Err(anyhow::anyhow!("File not found: {}", file_path));
    }

    // Validate JSON format
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let _: Value = serde_json::from_str(&content)
        .with_context(|| format!("File is not valid JSON: {}", file_path))?;

    let mut registry = load_registry(config)?;

    // Convert to relative path for storage
    let current_dir = std::env::current_dir().unwrap_or_default();
    let relative_path = Path::new(file_path)
        .strip_prefix(&current_dir)
        .unwrap_or(Path::new(file_path))
        .to_string_lossy()
        .to_string();

    // Check if already registered
    if is_file_registered(&registry, file_path) {
        log_warning(&format!("File is already registered: {}", relative_path));
        return Ok(());
    }

    println!();
    println!("Registering new file: {}", relative_path);

    // Show existing stores
    println!();
    println!("Existing stores:");
    for store in &registry.vector_stores {
        println!("- {}", store.id);
    }

    println!();
    print!("Enter store ID (existing or new): ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut store_id = String::new();
    std::io::stdin().read_line(&mut store_id)?;
    let store_id = store_id.trim().to_string();

    // Check if store exists
    if let Some(store_index) = registry.vector_stores.iter().position(|s| s.id == store_id) {
        // Add to existing store
        registry.vector_stores[store_index]
            .source_files
            .push(relative_path);
        log_success(&format!("File added to existing store: {}", store_id));
    } else {
        // Create new store
        println!();
        print!("Enter store description: ");
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut description = String::new();
        std::io::stdin().read_line(&mut description)?;
        let description = description.trim().to_string();

        print!("Enter data location (path): ");
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut data_location = String::new();
        std::io::stdin().read_line(&mut data_location)?;
        let data_location = data_location.trim().to_string();

        print!("Enter ingest script (path): ");
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut ingest_script = String::new();
        std::io::stdin().read_line(&mut ingest_script)?;
        let ingest_script = ingest_script.trim().to_string();

        print!("Enter embedding model [ollama]: ");
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut embedding_model = String::new();
        std::io::stdin().read_line(&mut embedding_model)?;
        let embedding_model = if embedding_model.trim().is_empty() {
            "ollama".to_string()
        } else {
            embedding_model.trim().to_string()
        };

        print!("Enter chunking strategy [character-based]: ");
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut chunking_strategy = String::new();
        std::io::stdin().read_line(&mut chunking_strategy)?;
        let chunking_strategy = if chunking_strategy.trim().is_empty() {
            "character-based".to_string()
        } else {
            chunking_strategy.trim().to_string()
        };

        print!("Enter max chunk size [250]: ");
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut max_chunk_size = String::new();
        std::io::stdin().read_line(&mut max_chunk_size)?;
        let max_chunk_size: u32 = if max_chunk_size.trim().is_empty() {
            250
        } else {
            max_chunk_size
                .trim()
                .parse()
                .with_context(|| "Invalid chunk size")?
        };

        // Create new store
        let new_store = VectorStore {
            id: store_id.clone(),
            description,
            source_files: vec![relative_path],
            data_location,
            ingest_script,
            embedding_model,
            chunking_strategy,
            max_chunk_size,
        };

        registry.vector_stores.push(new_store);
        log_success(&format!("New store created: {}", store_id));
    }

    // Update lastUpdated date
    registry.last_updated = Local::now().format("%Y-%m-%d").to_string();

    save_registry(config, &registry)?;
    Ok(())
}

/// Process all registered files
fn process_all_files(
    registry: &VectorRegistry,
    check_mode: bool,
    show_diff: bool,
) -> Result<ProcessingStats> {
    let mut stats = ProcessingStats::default();

    // Get all registered files
    let mut all_files = Vec::new();
    for store in &registry.vector_stores {
        for file in &store.source_files {
            all_files.push(file.clone());
        }
    }

    log_info(&format!(
        "Processing {} registered files...",
        all_files.len()
    ));

    for file in &all_files {
        println!();
        log_info(&format!("Processing: {}", file));

        match format_json_file(file, check_mode, show_diff) {
            Ok(true) => stats.add_success(),
            Ok(false) => stats.add_failure(),
            Err(e) => {
                log_error(&format!("Failed to process {}: {}", file, e));
                stats.add_failure();
            }
        }
    }

    println!();
    log_info("Processing complete:");
    println!("- Total files: {}", stats.total);
    println!("- Successfully processed: {}", stats.successful);
    if stats.failed > 0 {
        log_warning(&format!("- Failed: {}", stats.failed));
    } else {
        println!("- Failed: {}", stats.failed);
    }

    Ok(stats)
}

/// Process files for a specific store
fn process_store_files(
    registry: &VectorRegistry,
    store_id: &str,
    check_mode: bool,
    show_diff: bool,
) -> Result<ProcessingStats> {
    let mut stats = ProcessingStats::default();

    // Find the store
    let store = registry
        .vector_stores
        .iter()
        .find(|s| s.id == store_id)
        .ok_or_else(|| {
            println!("Available stores:");
            for store in &registry.vector_stores {
                println!("- {}", store.id);
            }
            anyhow::anyhow!("Store not found: {}", store_id)
        })?;

    log_info(&format!(
        "Processing {} files for store: {}",
        store.source_files.len(),
        store_id
    ));

    for file in &store.source_files {
        println!();
        log_info(&format!("Processing: {}", file));

        match format_json_file(file, check_mode, show_diff) {
            Ok(true) => stats.add_success(),
            Ok(false) => stats.add_failure(),
            Err(e) => {
                log_error(&format!("Failed to process {}: {}", file, e));
                stats.add_failure();
            }
        }
    }

    println!();
    log_info(&format!("Processing complete for store {}:", store_id));
    println!("- Total files: {}", stats.total);
    println!("- Successfully processed: {}", stats.successful);
    if stats.failed > 0 {
        log_warning(&format!("- Failed: {}", stats.failed));
    } else {
        println!("- Failed: {}", stats.failed);
    }

    Ok(stats)
}

/// Main entry point for the JSON formatting tool
fn main() -> Result<()> {
    let matches = Command::new("format_json")
        .about("JSON formatting utility for vector data files")
        .arg(
            Arg::new("all")
                .long("all")
                .help("Format all registered files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("store")
                .long("store")
                .value_name("STORE_ID")
                .help("Format files for specific store"),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .value_name("FILE_PATH")
                .help("Format specific file (must be registered)"),
        )
        .arg(
            Arg::new("check")
                .long("check")
                .help("Check format without modifying")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("register")
                .long("register")
                .value_name("FILE_PATH")
                .help("Add a new file to registry"),
        )
        .arg(
            Arg::new("validate-registry")
                .long("validate-registry")
                .help("Validate the registry file")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("diff")
                .long("diff")
                .help("Show diff when checking files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .help("Show detailed output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("registry")
                .long("registry")
                .value_name("PATH")
                .help("Path to registry file")
                .default_value("./config/vector-stores-registry.json"),
        )
        .get_matches();

    // Configuration
    let registry_path = matches.get_one::<String>("registry").unwrap();
    let config = RegistryConfig {
        file_path: PathBuf::from(registry_path),
    };

    let verbose = matches.get_flag("verbose");
    let show_diff = matches.get_flag("diff");
    let check_mode = matches.get_flag("check");

    // Determine mode and target
    let mode = if matches.get_flag("validate-registry") {
        ProcessingMode::ValidateRegistry
    } else if let Some(file_path) = matches.get_one::<String>("register") {
        ProcessingMode::Register(file_path.clone())
    } else if check_mode {
        ProcessingMode::Check
    } else {
        ProcessingMode::Format
    };

    let target = if matches.get_flag("all") {
        ProcessingTarget::All
    } else if let Some(store_id) = matches.get_one::<String>("store") {
        ProcessingTarget::Store(store_id.clone())
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        ProcessingTarget::File(file_path.clone())
    } else {
        ProcessingTarget::All
    };

    // Check dependencies
    check_dependencies()?;

    // Process based on mode
    match mode {
        ProcessingMode::ValidateRegistry => {
            validate_registry(&config, verbose)?;
        }
        ProcessingMode::Register(file_path) => {
            register_new_file(&config, &file_path)?;
        }
        ProcessingMode::Check | ProcessingMode::Format => {
            let registry = load_registry(&config)?;
            let check = mode == ProcessingMode::Check;

            match target {
                ProcessingTarget::All => {
                    process_all_files(&registry, check, show_diff)?;
                }
                ProcessingTarget::Store(store_id) => {
                    process_store_files(&registry, &store_id, check, show_diff)?;
                }
                ProcessingTarget::File(file_path) => {
                    // Check if file is registered
                    if !is_file_registered(&registry, &file_path) {
                        log_error("File is not registered in the vector store registry");
                        log_info("Use --register to add it to the registry first");
                        return Err(anyhow::anyhow!("File not registered: {}", file_path));
                    }

                    match format_json_file(&file_path, check, show_diff) {
                        Ok(_) => {}
                        Err(e) => {
                            log_error(&format!("Failed to process file: {}", e));
                            return Err(e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
