/// Mock Browser Framework for Testing
///
/// Provides mock implementations of browser, page, and element types
/// to enable unit testing of browser-dependent code without requiring
/// a real Chrome/Chromium instance.
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// Mock element representing a DOM element in tests
#[derive(Debug, Clone)]
pub struct MockElement {
    /// HTML tag name
    pub tag_name: String,
    /// Element text content
    pub text_content: String,
    /// Element attributes
    pub attributes: HashMap<String, String>,
    /// Whether element is visible
    pub visible: bool,
    /// Whether element is enabled
    pub enabled: bool,
}

impl MockElement {
    /// Create a new mock element
    pub fn new(tag_name: impl Into<String>) -> Self {
        Self {
            tag_name: tag_name.into(),
            text_content: String::new(),
            attributes: HashMap::new(),
            visible: true,
            enabled: true,
        }
    }

    /// Set text content
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text_content = text.into();
        self
    }

    /// Set an attribute
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Set visibility
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get attribute value
    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }
}

/// Mock page representing a browser page in tests
#[derive(Debug, Clone)]
pub struct MockPage {
    /// Page URL
    pub url: String,
    /// Elements on the page indexed by selector
    pub elements: HashMap<String, Vec<MockElement>>,
    /// Whether navigation is complete
    pub navigation_complete: bool,
    /// Screenshot data
    pub screenshot_data: Vec<u8>,
}

impl MockPage {
    /// Create a new mock page
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            elements: HashMap::new(),
            navigation_complete: true,
            screenshot_data: vec![],
        }
    }

    /// Add an element that matches a selector
    pub fn with_element(mut self, selector: impl Into<String>, element: MockElement) -> Self {
        self.elements
            .entry(selector.into())
            .or_default()
            .push(element);
        self
    }

    /// Add multiple elements that match a selector
    pub fn with_elements(
        mut self,
        selector: impl Into<String>,
        elements: Vec<MockElement>,
    ) -> Self {
        self.elements.insert(selector.into(), elements);
        self
    }

    /// Set navigation state
    pub fn with_navigation_complete(mut self, complete: bool) -> Self {
        self.navigation_complete = complete;
        self
    }

    /// Set screenshot data
    pub fn with_screenshot(mut self, data: Vec<u8>) -> Self {
        self.screenshot_data = data;
        self
    }

    /// Find a single element by selector
    pub fn find_element(&self, selector: &str) -> Result<MockElement> {
        self.elements
            .get(selector)
            .and_then(|elements| elements.first())
            .cloned()
            .ok_or_else(|| anyhow!("Element not found for selector: {}", selector))
    }

    /// Find multiple elements by selector
    pub fn find_elements(&self, selector: &str) -> Vec<MockElement> {
        self.elements
            .get(selector)
            .cloned()
            .unwrap_or_else(Vec::new)
    }

    /// Simulate clicking an element
    pub async fn click(&self, selector: &str) -> Result<()> {
        let element = self.find_element(selector)?;
        if !element.visible {
            return Err(anyhow!("Cannot click hidden element"));
        }
        if !element.enabled {
            return Err(anyhow!("Cannot click disabled element"));
        }
        Ok(())
    }

    /// Simulate typing into an element
    pub async fn type_into(&mut self, selector: &str, text: &str) -> Result<()> {
        let element = self.find_element(selector)?;
        if !element.enabled {
            return Err(anyhow!("Cannot type into disabled element"));
        }
        // Update the element's value attribute
        if let Some(elements) = self.elements.get_mut(selector) {
            if let Some(elem) = elements.first_mut() {
                elem.attributes
                    .insert("value".to_string(), text.to_string());
            }
        }
        Ok(())
    }

    /// Wait for an element to appear
    pub async fn wait_for_element(&self, selector: &str, _timeout: Duration) -> Result<()> {
        if self.elements.contains_key(selector) {
            Ok(())
        } else {
            Err(anyhow!("Element not found: {}", selector))
        }
    }

    /// Take a screenshot
    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        Ok(self.screenshot_data.clone())
    }
}

/// Mock browser for testing
#[derive(Debug, Clone)]
pub struct MockBrowser {
    pages: Arc<RwLock<Vec<MockPage>>>,
    launched: bool,
}

impl MockBrowser {
    /// Create a new mock browser
    pub fn new() -> Self {
        Self {
            pages: Arc::new(RwLock::new(Vec::new())),
            launched: false,
        }
    }

    /// Create a builder for configuring the mock browser
    pub fn builder() -> MockBrowserBuilder {
        MockBrowserBuilder::new()
    }

    /// Add a page to the browser
    pub fn add_page(&self, page: MockPage) {
        self.pages.write().unwrap().push(page);
    }

    /// Get a page by index
    pub fn get_page(&self, index: usize) -> Option<MockPage> {
        self.pages.read().unwrap().get(index).cloned()
    }

    /// Get all pages
    pub fn pages(&self) -> Vec<MockPage> {
        self.pages.read().unwrap().clone()
    }

    /// Check if browser is launched
    pub fn is_launched(&self) -> bool {
        self.launched
    }

    /// Set launched state
    pub fn set_launched(&mut self, launched: bool) {
        self.launched = launched;
    }
}

impl Default for MockBrowser {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for MockBrowser
pub struct MockBrowserBuilder {
    pages: Vec<MockPage>,
    launched: bool,
}

impl MockBrowserBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            launched: true,
        }
    }

    /// Add a page with URL
    pub fn with_page(mut self, url: impl Into<String>) -> Self {
        self.pages.push(MockPage::new(url));
        self
    }

    /// Add a configured page
    pub fn with_configured_page(mut self, page: MockPage) -> Self {
        self.pages.push(page);
        self
    }

    /// Add an element to the last page
    pub fn with_element(mut self, selector: impl Into<String>, element: MockElement) -> Self {
        if let Some(page) = self.pages.last_mut() {
            page.elements
                .entry(selector.into())
                .or_insert_with(Vec::new)
                .push(element);
        }
        self
    }

    /// Set launched state
    pub fn with_launched(mut self, launched: bool) -> Self {
        self.launched = launched;
        self
    }

    /// Build the mock browser
    pub fn build(self) -> MockBrowser {
        MockBrowser {
            pages: Arc::new(RwLock::new(self.pages)),
            launched: self.launched,
        }
    }
}

impl Default for MockBrowserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a simple mock page with elements
pub fn create_test_page(url: &str, selectors: Vec<(&str, MockElement)>) -> MockPage {
    let mut page = MockPage::new(url);
    for (selector, element) in selectors {
        page = page.with_element(selector, element);
    }
    page
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_element_creation() {
        let element = MockElement::new("div")
            .with_text("Hello")
            .with_attribute("class", "test")
            .with_visible(true);

        assert_eq!(element.tag_name, "div");
        assert_eq!(element.text_content, "Hello");
        assert_eq!(element.get_attribute("class"), Some(&"test".to_string()));
        assert!(element.visible);
    }

    #[test]
    fn test_mock_element_attributes() {
        let element = MockElement::new("input")
            .with_attribute("type", "text")
            .with_attribute("placeholder", "Enter text")
            .with_enabled(true);

        assert_eq!(element.get_attribute("type"), Some(&"text".to_string()));
        assert_eq!(
            element.get_attribute("placeholder"),
            Some(&"Enter text".to_string())
        );
        assert!(element.enabled);
    }

    #[test]
    fn test_mock_page_creation() {
        let page = MockPage::new("https://example.com")
            .with_element("#button", MockElement::new("button").with_text("Click me"));

        assert_eq!(page.url, "https://example.com");
        assert!(page.elements.contains_key("#button"));
    }

    #[test]
    fn test_mock_page_find_element() {
        let element = MockElement::new("div").with_text("Test");
        let page = MockPage::new("https://test.com").with_element("#test", element);

        let found = page.find_element("#test");
        assert!(found.is_ok());
        assert_eq!(found.unwrap().text_content, "Test");
    }

    #[test]
    fn test_mock_page_find_element_not_found() {
        let page = MockPage::new("https://test.com");
        let result = page.find_element("#nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_page_find_elements_multiple() {
        let page = MockPage::new("https://test.com").with_elements(
            ".item",
            vec![
                MockElement::new("li").with_text("Item 1"),
                MockElement::new("li").with_text("Item 2"),
                MockElement::new("li").with_text("Item 3"),
            ],
        );

        let elements = page.find_elements(".item");
        assert_eq!(elements.len(), 3);
        assert_eq!(elements[0].text_content, "Item 1");
        assert_eq!(elements[2].text_content, "Item 3");
    }

    #[tokio::test]
    async fn test_mock_page_click_visible_element() {
        let page = MockPage::new("https://test.com")
            .with_element("#button", MockElement::new("button").with_visible(true));

        let result = page.click("#button").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_page_click_hidden_element() {
        let page = MockPage::new("https://test.com")
            .with_element("#button", MockElement::new("button").with_visible(false));

        let result = page.click("#button").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_page_type_into_enabled_element() {
        let mut page = MockPage::new("https://test.com")
            .with_element("#input", MockElement::new("input").with_enabled(true));

        let result = page.type_into("#input", "Hello World").await;
        assert!(result.is_ok());

        let element = page.find_element("#input").unwrap();
        assert_eq!(
            element.get_attribute("value"),
            Some(&"Hello World".to_string())
        );
    }

    #[tokio::test]
    async fn test_mock_page_type_into_disabled_element() {
        let mut page = MockPage::new("https://test.com")
            .with_element("#input", MockElement::new("input").with_enabled(false));

        let result = page.type_into("#input", "Hello").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_browser_builder() {
        let browser = MockBrowser::builder()
            .with_page("https://example.com")
            .with_element("#test", MockElement::new("div").with_text("Test"))
            .build();

        assert!(browser.is_launched());
        assert_eq!(browser.pages().len(), 1);

        let page = browser.get_page(0).unwrap();
        assert_eq!(page.url, "https://example.com");
        assert!(page.elements.contains_key("#test"));
    }

    #[test]
    fn test_mock_browser_add_page() {
        let browser = MockBrowser::new();
        let page = MockPage::new("https://test.com");

        browser.add_page(page);

        assert_eq!(browser.pages().len(), 1);
        assert_eq!(browser.get_page(0).unwrap().url, "https://test.com");
    }

    #[test]
    fn test_create_test_page_helper() {
        let page = create_test_page(
            "https://test.com",
            vec![
                ("#id1", MockElement::new("div").with_text("Text 1")),
                ("#id2", MockElement::new("span").with_text("Text 2")),
            ],
        );

        assert_eq!(page.url, "https://test.com");
        assert_eq!(page.find_element("#id1").unwrap().text_content, "Text 1");
        assert_eq!(page.find_element("#id2").unwrap().text_content, "Text 2");
    }

    #[tokio::test]
    async fn test_mock_page_screenshot() {
        let page = MockPage::new("https://test.com").with_screenshot(vec![1, 2, 3, 4]);

        let screenshot = page.screenshot().await.unwrap();
        assert_eq!(screenshot, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_mock_page_wait_for_element_success() {
        let page = MockPage::new("https://test.com").with_element("#test", MockElement::new("div"));

        let result = page.wait_for_element("#test", Duration::from_secs(1)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_page_wait_for_element_not_found() {
        let page = MockPage::new("https://test.com");

        let result = page
            .wait_for_element("#nonexistent", Duration::from_secs(1))
            .await;
        assert!(result.is_err());
    }
}
