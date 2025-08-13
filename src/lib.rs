//! HARALD: A context-aware, emotionally adaptive AI framework
//!
//! This is the main library for the HARALD project, which integrates memory,
//! emotion, and modular execution across a trusted cohort of AI entities.
//!
//! # Modules
//!
//! - `api`: API endpoints and handlers
//! - `core`: Core application logic
//! - `ingest`: Ingestion pipeline
//! - `utils`: Shared utilities and helpers

// Export top-level modules
pub mod api;
pub mod core;
pub mod ingest;
pub mod utils;
// pub use ingest::embed;
// pub use ingest::ingest;
// pub use ingest::query;
