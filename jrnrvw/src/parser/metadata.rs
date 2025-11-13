//! Metadata extraction from parsed journal sections

use std::collections::HashMap;

/// Extractor for metadata from parsed journal sections
pub struct MetadataExtractor {
    sections: HashMap<String, String>,
}

impl MetadataExtractor {
    /// Create a new metadata extractor with the given sections
    ///
    /// # Arguments
    /// * `sections` - HashMap of section names to their content
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use jrnrvw::parser::MetadataExtractor;
    ///
    /// let mut sections = HashMap::new();
    /// sections.insert("Task".to_string(), "Implement parser".to_string());
    /// let extractor = MetadataExtractor::new(sections);
    /// ```
    pub fn new(sections: HashMap<String, String>) -> Self {
        Self { sections }
    }

    /// Extract the task description from the Task section
    ///
    /// # Returns
    /// * `Some(String)` - The task description if the Task section exists
    /// * `None` - If the Task section is missing or empty
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use jrnrvw::parser::MetadataExtractor;
    ///
    /// let mut sections = HashMap::new();
    /// sections.insert("Task".to_string(), "Build parser".to_string());
    /// let extractor = MetadataExtractor::new(sections);
    /// assert_eq!(extractor.extract_task(), Some("Build parser".to_string()));
    /// ```
    pub fn extract_task(&self) -> Option<String> {
        self.sections.get("Task").map(|s| s.trim().to_string())
    }

    /// Extract the repository information from the Repository section
    ///
    /// # Returns
    /// * `Some(String)` - The repository path or URL if the Repository section exists
    /// * `None` - If the Repository section is missing or empty
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use jrnrvw::parser::MetadataExtractor;
    ///
    /// let mut sections = HashMap::new();
    /// sections.insert("Repository".to_string(), "github.com/user/repo".to_string());
    /// let extractor = MetadataExtractor::new(sections);
    /// assert_eq!(extractor.extract_repository(), Some("github.com/user/repo".to_string()));
    /// ```
    pub fn extract_repository(&self) -> Option<String> {
        self.sections
            .get("Repository")
            .map(|s| s.trim().to_string())
    }

    /// Extract activities from the Activities section
    ///
    /// This method parses the Activities section and extracts individual activity items.
    /// Activities are expected to be formatted as a bulleted list.
    ///
    /// # Returns
    /// * `Vec<String>` - List of activities, empty if the Activities section is missing
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use jrnrvw::parser::MetadataExtractor;
    ///
    /// let mut sections = HashMap::new();
    /// sections.insert("Activities".to_string(), "- Wrote tests\n- Implemented parser".to_string());
    /// let extractor = MetadataExtractor::new(sections);
    /// let activities = extractor.extract_activities();
    /// assert_eq!(activities.len(), 2);
    /// ```
    pub fn extract_activities(&self) -> Vec<String> {
        self.sections
            .get("Activities")
            .map(|content| {
                content
                    .lines()
                    .filter_map(|line| {
                        let trimmed = line.trim();
                        // Handle bullet points (-, *, +) and numbered lists
                        if trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with('+') {
                            Some(trimmed[1..].trim().to_string())
                        } else if trimmed.chars().next().map_or(false, |c| c.is_numeric()) {
                            // Handle numbered lists like "1. Activity"
                            trimmed
                                .find(|c: char| c == '.' || c == ')')
                                .map(|idx| trimmed[idx + 1..].trim().to_string())
                        } else if !trimmed.is_empty() {
                            // Plain line without bullet
                            Some(trimmed.to_string())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Extract notes from the Notes section
    ///
    /// # Returns
    /// * `Some(String)` - The notes content if the Notes section exists
    /// * `None` - If the Notes section is missing or empty
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use jrnrvw::parser::MetadataExtractor;
    ///
    /// let mut sections = HashMap::new();
    /// sections.insert("Notes".to_string(), "Remember to test".to_string());
    /// let extractor = MetadataExtractor::new(sections);
    /// assert_eq!(extractor.extract_notes(), Some("Remember to test".to_string()));
    /// ```
    pub fn extract_notes(&self) -> Option<String> {
        self.sections.get("Notes").map(|s| s.trim().to_string())
    }

    /// Extract time spent from the Time Spent section
    ///
    /// # Returns
    /// * `Some(String)` - The time spent information if the Time Spent section exists
    /// * `None` - If the Time Spent section is missing or empty
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use jrnrvw::parser::MetadataExtractor;
    ///
    /// let mut sections = HashMap::new();
    /// sections.insert("Time Spent".to_string(), "2 hours".to_string());
    /// let extractor = MetadataExtractor::new(sections);
    /// assert_eq!(extractor.extract_time_spent(), Some("2 hours".to_string()));
    /// ```
    pub fn extract_time_spent(&self) -> Option<String> {
        self.sections
            .get("Time Spent")
            .or_else(|| self.sections.get("TimeSpent"))
            .or_else(|| self.sections.get("Duration"))
            .map(|s| s.trim().to_string())
    }

    /// Get all available section names
    ///
    /// # Returns
    /// * `Vec<String>` - List of all section names found in the journal
    pub fn sections(&self) -> Vec<String> {
        self.sections.keys().cloned().collect()
    }

    /// Get raw content of a specific section
    ///
    /// # Arguments
    /// * `section_name` - Name of the section to retrieve
    ///
    /// # Returns
    /// * `Some(String)` - The raw content of the section if it exists
    /// * `None` - If the section doesn't exist
    pub fn get_section(&self, section_name: &str) -> Option<String> {
        self.sections.get(section_name).map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::journal::ParsedContent;

    #[test]
    fn test_extract_task() {
        let mut sections = HashMap::new();
        sections.insert("Task".to_string(), "Implement parser".to_string());

        let extractor = MetadataExtractor::new(sections);
        assert_eq!(extractor.extract_task(), Some("Implement parser".to_string()));
    }

    #[test]
    fn test_extract_task_missing() {
        let sections = HashMap::new();
        let extractor = MetadataExtractor::new(sections);
        assert_eq!(extractor.extract_task(), None);
    }

    #[test]
    fn test_extract_repository() {
        let mut sections = HashMap::new();
        sections.insert("Repository".to_string(), "github.com/user/repo".to_string());

        let extractor = MetadataExtractor::new(sections);
        assert_eq!(
            extractor.extract_repository(),
            Some("github.com/user/repo".to_string())
        );
    }

    #[test]
    fn test_extract_activities() {
        let mut sections = HashMap::new();
        sections.insert(
            "Activities".to_string(),
            "- Wrote tests\n- Implemented parser\n- Updated docs".to_string(),
        );

        let extractor = MetadataExtractor::new(sections);
        let activities = extractor.extract_activities();

        assert_eq!(activities.len(), 3);
        assert_eq!(activities[0], "Wrote tests");
        assert_eq!(activities[1], "Implemented parser");
        assert_eq!(activities[2], "Updated docs");
    }

    #[test]
    fn test_extract_activities_empty() {
        let sections = HashMap::new();
        let extractor = MetadataExtractor::new(sections);
        let activities = extractor.extract_activities();

        assert_eq!(activities.len(), 0);
    }

    #[test]
    fn test_extract_activities_numbered() {
        let mut sections = HashMap::new();
        sections.insert(
            "Activities".to_string(),
            "1. First activity\n2. Second activity".to_string(),
        );

        let extractor = MetadataExtractor::new(sections);
        let activities = extractor.extract_activities();

        assert_eq!(activities.len(), 2);
        assert_eq!(activities[0], "First activity");
        assert_eq!(activities[1], "Second activity");
    }

    #[test]
    fn test_extract_notes() {
        let mut sections = HashMap::new();
        sections.insert("Notes".to_string(), "Important note".to_string());

        let extractor = MetadataExtractor::new(sections);
        assert_eq!(extractor.extract_notes(), Some("Important note".to_string()));
    }

    #[test]
    fn test_extract_time_spent() {
        let mut sections = HashMap::new();
        sections.insert("Time Spent".to_string(), "2 hours".to_string());

        let extractor = MetadataExtractor::new(sections);
        assert_eq!(extractor.extract_time_spent(), Some("2 hours".to_string()));
    }

    #[test]
    fn test_extract_time_spent_alternative_names() {
        let mut sections = HashMap::new();
        sections.insert("Duration".to_string(), "3h".to_string());

        let extractor = MetadataExtractor::new(sections);
        assert_eq!(extractor.extract_time_spent(), Some("3h".to_string()));
    }

    #[test]
    fn test_sections() {
        let mut sections = HashMap::new();
        sections.insert("Task".to_string(), "Test".to_string());
        sections.insert("Notes".to_string(), "Notes here".to_string());

        let extractor = MetadataExtractor::new(sections);
        let section_names = extractor.sections();

        assert_eq!(section_names.len(), 2);
        assert!(section_names.contains(&"Task".to_string()));
        assert!(section_names.contains(&"Notes".to_string()));
    }

    #[test]
    fn test_get_section() {
        let mut sections = HashMap::new();
        sections.insert("Task".to_string(), "My task".to_string());

        let extractor = MetadataExtractor::new(sections);
        assert_eq!(extractor.get_section("Task"), Some("My task".to_string()));
        assert_eq!(extractor.get_section("Missing"), None);
    }

    #[test]
    fn test_extract_activities_with_blank_lines() {
        let mut sections = HashMap::new();
        sections.insert(
            "Activities".to_string(),
            "- First item\n\n- Second item\n\n- Third item".to_string(),
        );

        let parsed = ParsedContent { sections };
        let extractor = MetadataExtractor::new(parsed.sections);
        let activities = extractor.extract_activities();

        assert_eq!(activities.len(), 3);
        assert!(activities.contains(&"First item".to_string()));
        assert!(activities.contains(&"Second item".to_string()));
        assert!(activities.contains(&"Third item".to_string()));
    }

    #[test]
    fn test_extract_time_spent_case_variations() {
        let mut sections = HashMap::new();
        sections.insert("TimeSpent".to_string(), "3h 30m".to_string());

        let extractor = MetadataExtractor::new(sections);
        let time = extractor.extract_time_spent();

        assert_eq!(time, Some("3h 30m".to_string()));
    }
}
