//! Filename pattern matching and parsing for journal files

use crate::error::{JrnrvwError, Result};
use chrono::NaiveDate;
use regex::Regex;

/// Parser for extracting information from journal filenames
///
/// Matches filenames in the format: `YYYY.MM.DD - JRN - description.md`
pub struct FilenameParser {
    /// Compiled regex pattern for matching journal filenames
    pattern: Regex,
}

impl FilenameParser {
    /// Create a new filename parser
    ///
    /// # Returns
    ///
    /// A `Result` containing the parser if the regex compiles successfully
    ///
    /// # Errors
    ///
    /// Returns an error if the regex pattern fails to compile
    ///
    /// # Examples
    ///
    /// ```
    /// use jrnrvw::discovery::FilenameParser;
    ///
    /// let parser = FilenameParser::new().unwrap();
    /// ```
    pub fn new() -> Result<Self> {
        // Pattern: YYYY.MM.DD - JRN - description.md
        // Captures: (year)(month)(day)(description)
        let pattern = Regex::new(r"^(\d{4})\.(\d{2})\.(\d{2})\s*-\s*JRN\s*-\s*(.+)\.md$")?;
        Ok(Self { pattern })
    }

    /// Check if a filename matches the journal filename pattern
    ///
    /// # Arguments
    ///
    /// * `filename` - The filename to check
    ///
    /// # Returns
    ///
    /// `true` if the filename matches the pattern, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use jrnrvw::discovery::FilenameParser;
    ///
    /// let parser = FilenameParser::new().unwrap();
    /// assert!(parser.matches("2024.01.15 - JRN - Daily standup.md"));
    /// assert!(!parser.matches("random-file.md"));
    /// ```
    pub fn matches(&self, filename: &str) -> bool {
        self.pattern.is_match(filename)
    }

    /// Parse and extract the date from a journal filename
    ///
    /// # Arguments
    ///
    /// * `filename` - The filename to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed date
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The filename doesn't match the expected pattern
    /// - The date components are invalid (e.g., month 13)
    ///
    /// # Examples
    ///
    /// ```
    /// use jrnrvw::discovery::FilenameParser;
    /// use chrono::NaiveDate;
    ///
    /// let parser = FilenameParser::new().unwrap();
    /// let date = parser.parse_date("2024.01.15 - JRN - Daily standup.md").unwrap();
    /// assert_eq!(date, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
    /// ```
    pub fn parse_date(&self, filename: &str) -> Result<NaiveDate> {
        let captures = self
            .pattern
            .captures(filename)
            .ok_or_else(|| JrnrvwError::InvalidDateFormat(filename.to_string()))?;

        // Extract year, month, day from captures
        let year: i32 = captures[1]
            .parse()
            .map_err(|_| JrnrvwError::InvalidDateFormat(filename.to_string()))?;

        let month: u32 = captures[2]
            .parse()
            .map_err(|_| JrnrvwError::InvalidDateFormat(filename.to_string()))?;

        let day: u32 = captures[3]
            .parse()
            .map_err(|_| JrnrvwError::InvalidDateFormat(filename.to_string()))?;

        // Validate and create date
        NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| JrnrvwError::InvalidDateFormat(filename.to_string()))
    }

    /// Parse and extract the description from a journal filename
    ///
    /// # Arguments
    ///
    /// * `filename` - The filename to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the description string
    ///
    /// # Errors
    ///
    /// Returns an error if the filename doesn't match the expected pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use jrnrvw::discovery::FilenameParser;
    ///
    /// let parser = FilenameParser::new().unwrap();
    /// let desc = parser.parse_description("2024.01.15 - JRN - Daily standup.md").unwrap();
    /// assert_eq!(desc, "Daily standup");
    /// ```
    pub fn parse_description(&self, filename: &str) -> Result<String> {
        let captures = self
            .pattern
            .captures(filename)
            .ok_or_else(|| JrnrvwError::InvalidDateFormat(filename.to_string()))?;

        // Extract description (4th capture group)
        Ok(captures[4].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_new() {
        let parser = FilenameParser::new();
        assert!(parser.is_ok());
    }

    #[test]
    fn test_matches_valid_filename() {
        let parser = FilenameParser::new().unwrap();
        assert!(parser.matches("2024.01.15 - JRN - Daily standup.md"));
        assert!(parser.matches("2023.12.31 - JRN - Year end review.md"));
        assert!(parser.matches("2024.01.01-JRN-New year planning.md"));
    }

    #[test]
    fn test_matches_invalid_filename() {
        let parser = FilenameParser::new().unwrap();
        assert!(!parser.matches("random-file.md"));
        assert!(!parser.matches("2024-01-15 - JRN - Daily standup.md")); // wrong separator
        assert!(!parser.matches("24.01.15 - JRN - Daily standup.md")); // 2-digit year
        assert!(!parser.matches("2024.01.15 - Daily standup.md")); // missing JRN
        assert!(!parser.matches("2024.01.15 - JRN - Daily standup.txt")); // wrong extension
    }

    #[test]
    fn test_parse_date_valid() {
        let parser = FilenameParser::new().unwrap();
        let date = parser.parse_date("2024.01.15 - JRN - Daily standup.md").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
    }

    #[test]
    fn test_parse_date_invalid_format() {
        let parser = FilenameParser::new().unwrap();
        let result = parser.parse_date("random-file.md");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_date_invalid_date() {
        let parser = FilenameParser::new().unwrap();
        let result = parser.parse_date("2024.13.01 - JRN - Invalid month.md");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_description() {
        let parser = FilenameParser::new().unwrap();
        let desc = parser
            .parse_description("2024.01.15 - JRN - Daily standup.md")
            .unwrap();
        assert_eq!(desc, "Daily standup");
    }

    #[test]
    fn test_parse_description_with_special_chars() {
        let parser = FilenameParser::new().unwrap();
        let desc = parser
            .parse_description("2024.01.15 - JRN - Bug fix: memory leak in parser.md")
            .unwrap();
        assert_eq!(desc, "Bug fix: memory leak in parser");
    }

    #[test]
    fn test_parse_description_invalid() {
        let parser = FilenameParser::new().unwrap();
        let result = parser.parse_description("random-file.md");
        assert!(result.is_err());
    }
}
