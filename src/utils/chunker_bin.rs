//! Standalone binary for the text chunking utility.
//!
//! This provides a command-line interface to the text chunking functionality,
//! matching the features of the original text_chunker.sh script.

use harald::utils::chunking;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    chunking::run_cli()
}
