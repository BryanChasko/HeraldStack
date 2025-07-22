use anyhow::{Context, Result};
use clap::{ArgAction, Command};
use colored::*;
use std::path::PathBuf;
use std::process::Command as SystemCommand;
use std::process::ExitStatus;

/// Run a command and return its exit status
fn run_command(command: &str, args: &[&str]) -> Result<ExitStatus> {
    let status = SystemCommand::new(command)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute command: {} {:?}", command, args))?;

    Ok(status)
}

/// Log utilities for consistent output
fn log_info(message: &str) {
    println!("{} {}", "[INFO]".blue().bold(), message);
}

fn log_success(message: &str) {
    println!("{} {}", "[SUCCESS]".green().bold(), message);
}

fn log_error(message: &str) {
    eprintln!("{} {}", "[ERROR]".red().bold(), message);
}

fn main() -> Result<()> {
    let matches = Command::new("check_json")
        .version("0.1.0")
        .author("Bryan Chasko")
        .about("Validate and format JSON files")
        .arg(
            clap::Arg::new("build")
                .short('b')
                .long("build")
                .help("Build the Rust JSON tools before running")
                .action(ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("fix")
                .short('f')
                .long("fix")
                .help("Automatically fix formatting issues")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Get the path to the format_json binary
    let format_json_path = get_format_json_path()?;

    // Build Rust tools if requested
    if matches.get_flag("build") {
        log_info("Building Rust JSON tools...");
        let status = run_command("cargo", &["build", "--release", "--features", "cli"])?;
        if !status.success() {
            log_error("Failed to build Rust tools");
            std::process::exit(1);
        }
        log_success("Build successful");
    }

    log_info("Validating and formatting JSON files using Rust tools...");

    // Get the absolute path to the registry file
    let current_dir = std::env::current_dir()?;
    log_info(&format!("Current directory: {}", current_dir.display()));

    // Try to find the registry file
    log_info("Searching for registry file...");

    // Try direct path
    let registry_path_direct = current_dir.join("data").join("vector-stores-registry.json");
    log_info(&format!("Checking: {}", registry_path_direct.display()));

    // Try project root
    let registry_path_root = if current_dir.ends_with("src") {
        current_dir
            .parent()
            .unwrap_or(&current_dir)
            .join("data")
            .join("vector-stores-registry.json")
    } else {
        current_dir.join("data").join("vector-stores-registry.json")
    };
    log_info(&format!("Checking: {}", registry_path_root.display()));

    // Use whatever exists
    let registry_path = if registry_path_direct.exists() {
        log_info(&format!(
            "Found registry at: {}",
            registry_path_direct.display()
        ));
        registry_path_direct
    } else if registry_path_root.exists() {
        log_info(&format!(
            "Found registry at: {}",
            registry_path_root.display()
        ));
        registry_path_root
    } else {
        log_error("Registry file not found!");
        log_info("Using default registry path as fallback...");
        registry_path_root
    };

    // First, check if all registered JSON files are valid
    let mut check_args = vec!["--check", "--all"];

    // Always provide the explicit registry path
    check_args.push("--registry");
    check_args.push(registry_path.to_str().unwrap());

    log_info(&format!(
        "Running: {} {}",
        format_json_path.display(),
        check_args.join(" ")
    ));

    let check_status = run_command(format_json_path.to_str().unwrap(), &check_args)?;

    if check_status.success() {
        log_success("All registered JSON files are valid");
    } else if matches.get_flag("fix") {
        log_info("Some JSON files need formatting - applying fixes...");

        let mut fix_args = vec!["--all"];

        // Always provide the explicit registry path
        fix_args.push("--registry");
        fix_args.push(registry_path.to_str().unwrap());

        let fix_status = run_command(format_json_path.to_str().unwrap(), &fix_args)?;

        if fix_status.success() {
            log_success("Applied formatting fixes");
        } else {
            log_error("Failed to apply formatting fixes");
            std::process::exit(1);
        }
    } else {
        log_error("JSON formatting issues detected. Run with --fix to automatically resolve them.");
        std::process::exit(1);
    }

    // Now validate registry
    log_info("Validating registry...");

    let mut validate_args = vec!["--validate-registry"];

    // Always provide the explicit registry path
    validate_args.push("--registry");
    validate_args.push(registry_path.to_str().unwrap());

    let validate_status = run_command(format_json_path.to_str().unwrap(), &validate_args)?;

    if validate_status.success() {
        log_success("Registry validation passed");
    } else {
        log_error("Registry validation failed");
        std::process::exit(1);
    }

    log_success("JSON validation and formatting complete!");
    Ok(())
}

/// Get the path to the format_json binary
fn get_format_json_path() -> Result<PathBuf> {
    // First check if we're in the src directory or project root
    let mut path = std::env::current_dir()?;

    // Check if we need to navigate to project root
    if path.ends_with("src") {
        path.pop();
    }

    // Check if target/release exists
    let release_path = path.join("target").join("release").join("format_json");
    if release_path.exists() {
        return Ok(release_path);
    }

    // Check if src/target/release exists (older structure)
    let src_release_path = path
        .join("src")
        .join("target")
        .join("release")
        .join("format_json");
    if src_release_path.exists() {
        return Ok(src_release_path);
    }

    // If we're in another directory structure, try to find format_json in PATH
    if let Ok(output) = SystemCommand::new("which").arg("format_json").output() {
        if output.status.success() {
            if let Ok(path_str) = String::from_utf8(output.stdout) {
                let path_str = path_str.trim();
                if !path_str.is_empty() {
                    return Ok(PathBuf::from(path_str));
                }
            }
        }
    }

    // If neither exists, return the expected path and let the command fail naturally
    Ok(release_path)
}
