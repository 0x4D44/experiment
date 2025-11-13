//! Command-line interface definitions

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use chrono::NaiveDate;

#[derive(Parser, Debug)]
#[command(name = "jrnrvw")]
#[command(about = "Journal Review Tool - Analyze task journal files", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Root directory to search (default: current directory)
    #[arg(value_name = "PATH")]
    pub path: Option<PathBuf>,

    // Time range options
    /// Last 7 calendar days
    #[arg(long, conflicts_with_all = ["last_month", "this_week", "this_month"])]
    pub last_week: bool,

    /// Last 30 calendar days
    #[arg(long, conflicts_with_all = ["last_week", "this_week", "this_month"])]
    pub last_month: bool,

    /// Current calendar week (Monday-Sunday)
    #[arg(long, conflicts_with_all = ["last_week", "last_month", "this_month"])]
    pub this_week: bool,

    /// Current calendar month
    #[arg(long, conflicts_with_all = ["last_week", "last_month", "this_week"])]
    pub this_month: bool,

    /// Last N days with journal entries
    #[arg(long, value_name = "N")]
    pub activity_days: Option<usize>,

    /// Alias for --activity-days
    #[arg(long, value_name = "N")]
    pub activity_window: Option<usize>,

    /// Start date (yyyy-mm-dd)
    #[arg(long, value_name = "DATE", value_parser = parse_date)]
    pub from: Option<NaiveDate>,

    /// End date (yyyy-mm-dd)
    #[arg(long, value_name = "DATE", value_parser = parse_date)]
    pub to: Option<NaiveDate>,

    /// All entries since date (inclusive)
    #[arg(long, value_name = "DATE", value_parser = parse_date)]
    pub since: Option<NaiveDate>,

    /// All entries before date (exclusive)
    #[arg(long, value_name = "DATE", value_parser = parse_date)]
    pub before: Option<NaiveDate>,

    // Filtering
    /// Filter by repository name (regex)
    #[arg(long, value_name = "PATTERN")]
    pub repo: Option<String>,

    /// Filter by task name (regex)
    #[arg(long, value_name = "PATTERN")]
    pub task: Option<String>,

    /// Custom filename pattern
    #[arg(long, value_name = "PATTERN")]
    pub pattern: Option<String>,

    // Grouping and sorting
    /// Group by: repo, task, date, week, month
    #[arg(long, value_enum, default_value = "repo")]
    pub group_by: GroupByArg,

    /// Sort by: date, repo, task
    #[arg(long, value_enum, default_value = "date")]
    pub sort_by: SortByArg,

    /// Reverse sort order
    #[arg(long)]
    pub reverse: bool,

    // Output
    /// Output file (default: stdout)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Output format: text, markdown, json, html, csv
    #[arg(short = 'f', long, value_enum, default_value = "text")]
    pub format: FormatArg,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Minimal output
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,

    // Display options
    /// Show only summary statistics
    #[arg(long)]
    pub summary: bool,

    /// Show detailed entries (default)
    #[arg(long)]
    pub detailed: bool,

    /// Include activity lists
    #[arg(long)]
    pub with_activities: bool,

    /// Include notes sections
    #[arg(long)]
    pub with_notes: bool,

    /// Include statistics
    #[arg(long)]
    pub stats: bool,

    // Config
    /// Load configuration from file
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GroupByArg {
    Repo,
    Task,
    Date,
    Week,
    Month,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortByArg {
    Date,
    Repo,
    Task,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FormatArg {
    Text,
    Markdown,
    Json,
    Html,
    Csv,
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format '{}': {}. Expected yyyy-mm-dd", s, e))
}
