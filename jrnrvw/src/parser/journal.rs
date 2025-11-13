//! Journal file parser using pulldown-cmark for Markdown parsing

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use std::collections::HashMap;

use crate::error::Result;

/// Parsed content from a journal file
#[derive(Debug, Clone)]
pub struct ParsedContent {
    /// Map of section headers to their content
    pub sections: HashMap<String, String>,
}

/// Parser for journal markdown files
pub struct JournalParser {
    content: String,
}

impl JournalParser {
    /// Create a new journal parser with the given markdown content
    ///
    /// # Arguments
    /// * `content` - The markdown content to parse
    ///
    /// # Example
    /// ```
    /// use jrnrvw::parser::JournalParser;
    ///
    /// let content = "# Journal\n## Task\nMy task".to_string();
    /// let parser = JournalParser::new(content);
    /// ```
    pub fn new(content: String) -> Self {
        Self { content }
    }

    /// Parse the journal content and extract sections
    ///
    /// This method uses pulldown-cmark to parse the markdown content and extracts
    /// sections based on level 2 headers (##). Each section contains all content
    /// until the next header of the same or higher level.
    ///
    /// # Returns
    /// * `Ok(ParsedContent)` - Successfully parsed content with sections
    /// * `Err(JrnrvwError)` - If parsing fails
    ///
    /// # Example
    /// ```
    /// use jrnrvw::parser::JournalParser;
    ///
    /// let content = "## Task\nImplement parser\n## Notes\nUse pulldown-cmark".to_string();
    /// let parser = JournalParser::new(content);
    /// let parsed = parser.parse().unwrap();
    /// ```
    pub fn parse(&self) -> Result<ParsedContent> {
        let mut sections = HashMap::new();
        let mut current_section: Option<String> = None;
        let mut current_content = String::new();
        let mut in_heading = false;
        let mut heading_level = 0;

        let parser = Parser::new(&self.content);

        for event in parser {
            match event {
                Event::Start(Tag::Heading(level, _, _)) => {
                    // Save previous section if it exists
                    if let Some(ref section_name) = current_section {
                        if !current_content.trim().is_empty() {
                            sections.insert(section_name.clone(), current_content.trim().to_string());
                        }
                        current_content.clear();
                    }

                    in_heading = true;
                    heading_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                }
                Event::End(Tag::Heading(_, _, _)) => {
                    in_heading = false;
                }
                Event::Text(text) => {
                    if in_heading && heading_level == 2 {
                        // Level 2 heading - this is a section header
                        current_section = Some(text.to_string());
                    } else if current_section.is_some() {
                        // Content within a section
                        current_content.push_str(&text);
                    }
                }
                Event::Code(code) => {
                    if current_section.is_some() {
                        current_content.push('`');
                        current_content.push_str(&code);
                        current_content.push('`');
                    }
                }
                Event::SoftBreak | Event::HardBreak => {
                    if current_section.is_some() && !in_heading {
                        current_content.push('\n');
                    }
                }
                Event::Start(Tag::List(_)) => {
                    if current_section.is_some() {
                        current_content.push('\n');
                    }
                }
                Event::Start(Tag::Item) => {
                    if current_section.is_some() {
                        current_content.push_str("- ");
                    }
                }
                Event::End(Tag::Item) => {
                    if current_section.is_some() {
                        current_content.push('\n');
                    }
                }
                Event::Start(Tag::Paragraph) => {
                    if current_section.is_some() && !current_content.is_empty() {
                        current_content.push('\n');
                    }
                }
                Event::End(Tag::Paragraph) => {
                    if current_section.is_some() {
                        current_content.push('\n');
                    }
                }
                _ => {}
            }
        }

        // Save the last section
        if let Some(section_name) = current_section {
            if !current_content.trim().is_empty() {
                sections.insert(section_name, current_content.trim().to_string());
            }
        }

        Ok(ParsedContent { sections })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_sections() {
        let content = r#"
## Task
Implement parser

## Notes
Use pulldown-cmark
"#
        .to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        assert_eq!(result.sections.get("Task").unwrap(), "Implement parser");
        assert_eq!(
            result.sections.get("Notes").unwrap(),
            "Use pulldown-cmark"
        );
    }

    #[test]
    fn test_parse_multiple_sections() {
        let content = r#"
## Task
Build a parser

## Activities
- Wrote tests
- Implemented parsing

## Notes
All done
"#
        .to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        assert_eq!(result.sections.len(), 3);
        assert!(result.sections.contains_key("Task"));
        assert!(result.sections.contains_key("Activities"));
        assert!(result.sections.contains_key("Notes"));
    }

    #[test]
    fn test_parse_empty_sections() {
        let content = "## Task\n\n## Notes\n".to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        // Empty sections should not be included
        assert_eq!(result.sections.len(), 0);
    }

    #[test]
    fn test_parse_with_code_blocks() {
        let content = r#"
## Implementation
Here is some `inline code` for testing
"#
        .to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        assert_eq!(result.sections.len(), 1);
        assert!(result.sections.get("Implementation").unwrap().contains("`inline code`"));
    }

    #[test]
    fn test_parse_with_lists() {
        let content = r#"
## Activities
- First item
- Second item
- Third item
"#
        .to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        assert_eq!(result.sections.len(), 1);
        let activities = result.sections.get("Activities").unwrap();
        assert!(activities.contains("- First item"));
        assert!(activities.contains("- Second item"));
        assert!(activities.contains("- Third item"));
    }

    #[test]
    fn test_parse_with_paragraphs() {
        let content = r#"
## Notes
First paragraph with some text.

Second paragraph with more text.
"#
        .to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        assert_eq!(result.sections.len(), 1);
        let notes = result.sections.get("Notes").unwrap();
        assert!(notes.contains("First paragraph"));
        assert!(notes.contains("Second paragraph"));
    }

    #[test]
    fn test_parse_multiple_heading_levels() {
        let content = r#"
# Title
Some intro text

## Section 1
Content for section 1

### Subsection
This should be part of section 1

## Section 2
Content for section 2
"#
        .to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        assert_eq!(result.sections.len(), 2);
        assert!(result.sections.contains_key("Section 1"));
        assert!(result.sections.contains_key("Section 2"));
    }

    #[test]
    fn test_parse_empty_content() {
        let content = "".to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        assert_eq!(result.sections.len(), 0);
    }

    #[test]
    fn test_parse_whitespace_only_sections() {
        let content = r#"
## Task


## Notes
Some actual content
"#
        .to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        assert_eq!(result.sections.len(), 1);
        assert!(result.sections.contains_key("Notes"));
        assert!(!result.sections.contains_key("Task"));
    }

    #[test]
    fn test_parse_with_soft_and_hard_breaks() {
        let content = r#"
## Notes
Line one
Line two

New paragraph
"#
        .to_string();

        let parser = JournalParser::new(content);
        let result = parser.parse().unwrap();

        assert_eq!(result.sections.len(), 1);
        let notes = result.sections.get("Notes").unwrap();
        assert!(notes.contains("Line one"));
        assert!(notes.contains("Line two"));
    }
}
