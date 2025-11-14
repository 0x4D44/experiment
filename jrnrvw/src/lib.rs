//! jrnrvw - Journal Review Tool
//!
//! A command-line tool for finding and analyzing task journal files.

pub mod cli;
pub mod config;
pub mod error;
pub mod models;
pub mod discovery;
pub mod parser;
pub mod analyzer;
pub mod output;
pub mod llm;

// Re-export commonly used types
pub use error::{JrnrvwError, Result};
pub use models::{JournalEntry, Repository, Task, Report};
