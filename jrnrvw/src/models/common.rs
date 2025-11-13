//! Common types and enums

use std::str::FromStr;
use crate::error::{JrnrvwError, Result};

/// How to group journal entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupBy {
    Repository,
    Task,
    Date,
    Week,
    Month,
}

impl FromStr for GroupBy {
    type Err = JrnrvwError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "repo" | "repository" => Ok(GroupBy::Repository),
            "task" => Ok(GroupBy::Task),
            "date" => Ok(GroupBy::Date),
            "week" => Ok(GroupBy::Week),
            "month" => Ok(GroupBy::Month),
            _ => Err(JrnrvwError::InvalidArgument(
                format!("Invalid group-by value: {}", s)
            )),
        }
    }
}

/// How to sort entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Date,
    Repository,
    Task,
}

impl FromStr for SortBy {
    type Err = JrnrvwError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "date" => Ok(SortBy::Date),
            "repo" | "repository" => Ok(SortBy::Repository),
            "task" => Ok(SortBy::Task),
            _ => Err(JrnrvwError::InvalidArgument(
                format!("Invalid sort-by value: {}", s)
            )),
        }
    }
}

/// Output format for reports
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Markdown,
    Json,
    Html,
    Csv,
}

impl FromStr for OutputFormat {
    type Err = JrnrvwError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "markdown" | "md" => Ok(OutputFormat::Markdown),
            "json" => Ok(OutputFormat::Json),
            "html" => Ok(OutputFormat::Html),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(JrnrvwError::InvalidArgument(
                format!("Invalid output format: {}", s)
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_from_str() {
        assert_eq!("repo".parse::<GroupBy>().unwrap(), GroupBy::Repository);
        assert_eq!("task".parse::<GroupBy>().unwrap(), GroupBy::Task);
        assert_eq!("date".parse::<GroupBy>().unwrap(), GroupBy::Date);
        assert!(GroupBy::from_str("invalid").is_err());
    }

    #[test]
    fn test_sort_by_from_str() {
        assert_eq!("date".parse::<SortBy>().unwrap(), SortBy::Date);
        assert_eq!("repo".parse::<SortBy>().unwrap(), SortBy::Repository);
        assert!(SortBy::from_str("invalid").is_err());
    }

    #[test]
    fn test_output_format_from_str() {
        assert_eq!("text".parse::<OutputFormat>().unwrap(), OutputFormat::Text);
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("md".parse::<OutputFormat>().unwrap(), OutputFormat::Markdown);
        assert!(OutputFormat::from_str("invalid").is_err());
    }
}
