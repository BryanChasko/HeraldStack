//! Library component of rust_ingest.
//!
//! This file exposes the core functionality as a library,
//! which allows it to be imported by other crates and
//! enables proper testing of the components.

pub mod embed;
pub mod ingest;
pub mod query;

// Re-export key functionality for convenience
pub use crate::embed::embed;
pub use crate::ingest::run as ingest_run;
pub use crate::query::run as query_run;
