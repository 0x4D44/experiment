//! Output formatters for different formats

pub mod text;
pub mod markdown;
pub mod json;
pub mod html;
pub mod csv;

use crate::{Report, Result};

/// Options for formatting output
#[derive(Debug, Clone)]
pub struct OutputOptions {
    pub colored: bool,
    pub verbose: bool,
    pub include_activities: bool,
    pub include_notes: bool,
    pub include_stats: bool,
    pub summary_only: bool,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            colored: true,
            verbose: false,
            include_activities: true,
            include_notes: false,
            include_stats: true,
            summary_only: false,
        }
    }
}

/// Trait for output formatters
pub trait Formatter {
    fn format(&self, report: &Report, options: &OutputOptions) -> Result<String>;
}
