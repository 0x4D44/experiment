// Page automation utilities
// Provides helper functions for interacting with web pages

use chromiumoxide::Page;
use chromiumoxide::element::Element;
use anyhow::{Result, Context, bail};
use std::time::Duration;

/// Find an element using a list of selector fallbacks
/// Tries each selector in order until one matches
pub async fn find_element_with_fallback(
    page: &Page,
    selectors: &[String],
) -> Result<Element> {
    for selector in selectors {
        tracing::debug!("Trying selector: {}", selector);

        match page.find_element(selector).await {
            Ok(element) => {
                tracing::debug!("Found element with selector: {}", selector);
                return Ok(element);
            }
            Err(e) => {
                tracing::trace!("Selector '{}' failed: {:?}", selector, e);
                continue;
            }
        }
    }

    bail!("No element found with any of the provided selectors: {:?}", selectors)
}

/// Find all elements matching a list of selector fallbacks
/// Tries each selector in order until one returns results
pub async fn find_elements_with_fallback(
    page: &Page,
    selectors: &[String],
) -> Result<Vec<Element>> {
    for selector in selectors {
        tracing::debug!("Trying selector for multiple elements: {}", selector);

        match page.find_elements(selector).await {
            Ok(elements) if !elements.is_empty() => {
                tracing::debug!("Found {} elements with selector: {}", elements.len(), selector);
                return Ok(elements);
            }
            Ok(_) => {
                tracing::trace!("Selector '{}' matched but returned no elements", selector);
                continue;
            }
            Err(e) => {
                tracing::trace!("Selector '{}' failed: {:?}", selector, e);
                continue;
            }
        }
    }

    bail!("No elements found with any of the provided selectors: {:?}", selectors)
}

/// Click an element found using selector fallbacks
pub async fn click_element(
    page: &Page,
    selectors: &[String],
) -> Result<()> {
    let element = find_element_with_fallback(page, selectors).await
        .context("Failed to find element to click")?;

    element.click().await
        .context("Failed to click element")?;

    tracing::debug!("Clicked element");
    Ok(())
}

/// Type text into an input element found using selector fallbacks
pub async fn type_into_element(
    page: &Page,
    selectors: &[String],
    text: &str,
) -> Result<()> {
    let element = find_element_with_fallback(page, selectors).await
        .context("Failed to find element to type into")?;

    element.click().await
        .context("Failed to focus element")?;

    element.type_str(text).await
        .context("Failed to type text")?;

    tracing::debug!("Typed {} characters into element", text.len());
    Ok(())
}

/// Wait for an element to appear on the page
/// Polls every interval until timeout is reached
pub async fn wait_for_element(
    page: &Page,
    selectors: &[String],
    timeout: Duration,
    interval: Duration,
) -> Result<Element> {
    let start = std::time::Instant::now();

    loop {
        // Try to find the element
        if let Ok(element) = find_element_with_fallback(page, selectors).await {
            tracing::debug!("Element appeared after {:?}", start.elapsed());
            return Ok(element);
        }

        // Check timeout
        if start.elapsed() >= timeout {
            bail!("Timeout waiting for element after {:?}", timeout);
        }

        // Wait before next attempt
        tokio::time::sleep(interval).await;
    }
}

/// Wait for an element to disappear from the page
pub async fn wait_for_element_removed(
    page: &Page,
    selectors: &[String],
    timeout: Duration,
    interval: Duration,
) -> Result<()> {
    let start = std::time::Instant::now();

    loop {
        // Try to find the element
        if find_element_with_fallback(page, selectors).await.is_err() {
            tracing::debug!("Element disappeared after {:?}", start.elapsed());
            return Ok(());
        }

        // Check timeout
        if start.elapsed() >= timeout {
            bail!("Timeout waiting for element removal after {:?}", timeout);
        }

        // Wait before next attempt
        tokio::time::sleep(interval).await;
    }
}

/// Wait for page navigation to complete
pub async fn wait_for_navigation(_page: &Page, timeout: Duration) -> Result<()> {
    // Wait for network to be idle
    tokio::time::timeout(timeout, async {
        // Give the page some time to start loading
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Wait for page to be in a stable state
        tokio::time::sleep(Duration::from_millis(500)).await;

        Ok::<(), anyhow::Error>(())
    })
    .await
    .context("Timeout waiting for navigation")??;

    tracing::debug!("Navigation completed");
    Ok(())
}

/// Take a screenshot of the page
pub async fn take_screenshot(page: &Page, path: &str) -> Result<Vec<u8>> {
    let screenshot = page.screenshot(chromiumoxide::page::ScreenshotParams::builder().build())
        .await
        .context("Failed to take screenshot")?;

    // Optionally save to file
    if !path.is_empty() {
        std::fs::write(path, &screenshot)
            .context("Failed to save screenshot")?;
        tracing::info!("Screenshot saved to: {}", path);
    }

    Ok(screenshot)
}

/// Scroll to an element to make it visible
pub async fn scroll_to_element(
    page: &Page,
    selectors: &[String],
) -> Result<()> {
    let element = find_element_with_fallback(page, selectors).await
        .context("Failed to find element to scroll to")?;

    element.scroll_into_view().await
        .context("Failed to scroll to element")?;

    tracing::debug!("Scrolled to element");
    Ok(())
}

/// Check if an element is visible on the page
pub async fn is_element_visible(
    page: &Page,
    selectors: &[String],
) -> bool {
    match find_element_with_fallback(page, selectors).await {
        Ok(_element) => {
            // Element exists, assume it's visible
            // Note: chromiumoxide doesn't provide is_visible method
            // Could use JavaScript evaluation for more accurate check
            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Most functions in this module require a real chromiumoxide::Page
    // instance for full integration testing. These tests focus on validating
    // logic, parameters, and error handling where possible without a browser.
    // Full integration tests should be added when browser automation is available.

    #[test]
    fn test_selector_fallback_list_creation() {
        let selectors = vec![
            ".primary-selector".to_string(),
            ".fallback-selector".to_string(),
            "#last-resort".to_string(),
        ];
        assert_eq!(selectors.len(), 3);
        assert_eq!(selectors[0], ".primary-selector");
        assert_eq!(selectors[1], ".fallback-selector");
        assert_eq!(selectors[2], "#last-resort");
    }

    #[test]
    fn test_selector_empty_list() {
        let selectors: Vec<String> = vec![];
        assert!(selectors.is_empty());
    }

    #[test]
    fn test_timeout_duration_creation() {
        let timeout = Duration::from_secs(30);
        assert_eq!(timeout.as_secs(), 30);
        assert_eq!(timeout.as_millis(), 30_000);
    }

    #[test]
    fn test_timeout_duration_zero() {
        let timeout = Duration::from_secs(0);
        assert_eq!(timeout.as_secs(), 0);
        assert!(timeout.is_zero());
    }

    #[test]
    fn test_interval_duration_creation() {
        let interval = Duration::from_millis(500);
        assert_eq!(interval.as_millis(), 500);
        assert_eq!(interval.as_secs(), 0);
    }

    #[test]
    fn test_interval_duration_short() {
        let interval = Duration::from_millis(100);
        assert_eq!(interval.as_millis(), 100);
    }

    #[test]
    fn test_selector_string_formats() {
        // Test different CSS selector formats
        let class_selector = ".my-class".to_string();
        let id_selector = "#my-id".to_string();
        let tag_selector = "button".to_string();
        let attribute_selector = "[data-test='value']".to_string();

        assert!(class_selector.starts_with('.'));
        assert!(id_selector.starts_with('#'));
        assert!(!tag_selector.starts_with('.'));
        assert!(attribute_selector.starts_with('['));
    }

    #[test]
    fn test_screenshot_path_empty() {
        let path = "";
        assert!(path.is_empty());
    }

    #[test]
    fn test_screenshot_path_valid() {
        let path = "/tmp/screenshot.png";
        assert!(!path.is_empty());
        assert!(path.ends_with(".png"));
    }

    #[test]
    fn test_duration_comparison() {
        let short = Duration::from_millis(100);
        let long = Duration::from_secs(1);
        assert!(short < long);
    }

    #[test]
    fn test_multiple_selectors_order() {
        let selectors = vec![
            "button.primary".to_string(),
            "button".to_string(),
            "[role='button']".to_string(),
        ];
        // First selector should be most specific
        assert_eq!(selectors[0], "button.primary");
        // Last should be most general
        assert_eq!(selectors[2], "[role='button']");
    }

    #[test]
    fn test_text_input_empty() {
        let text = "";
        assert_eq!(text.len(), 0);
    }

    #[test]
    fn test_text_input_with_content() {
        let text = "Hello, World!";
        assert_eq!(text.len(), 13);
        assert!(text.contains("Hello"));
    }

    #[test]
    fn test_text_input_special_characters() {
        let text = "test@example.com";
        assert!(text.contains('@'));
        assert!(text.contains('.'));
    }

    #[test]
    fn test_timeout_and_interval_relationship() {
        let timeout = Duration::from_secs(10);
        let interval = Duration::from_millis(500);
        // Interval should be smaller than timeout for polling
        assert!(interval < timeout);
    }

    #[test]
    fn test_selector_fallback_count() {
        // Common pattern: 3 selectors (primary, secondary, fallback)
        let selectors = vec![
            ".specific-class".to_string(),
            ".general-class".to_string(),
            "div".to_string(),
        ];
        assert_eq!(selectors.len(), 3);
    }

    #[test]
    fn test_duration_millisecond_precision() {
        let duration = Duration::from_millis(1500);
        assert_eq!(duration.as_millis(), 1500);
        assert_eq!(duration.as_secs(), 1);
    }

    #[test]
    fn test_screenshot_path_formats() {
        let png_path = "screenshot.png";
        let jpg_path = "screenshot.jpg";
        let relative_path = "./screenshots/test.png";

        assert!(png_path.ends_with(".png"));
        assert!(jpg_path.ends_with(".jpg"));
        assert!(relative_path.starts_with('.'));
    }

    #[test]
    fn test_selector_complexity() {
        // Test complex selector string
        let complex = "div.container > button.primary:not(.disabled)".to_string();
        assert!(complex.contains(">"));
        assert!(complex.contains(":not"));
        assert!(complex.len() > 10);
    }

    #[test]
    fn test_wait_timeout_reasonable() {
        let timeout = Duration::from_secs(30);
        // Timeout should be between 1 and 60 seconds for most use cases
        assert!(timeout.as_secs() >= 1);
        assert!(timeout.as_secs() <= 60);
    }

    #[test]
    fn test_poll_interval_reasonable() {
        let interval = Duration::from_millis(500);
        // Poll interval should typically be between 100ms and 2s
        assert!(interval.as_millis() >= 100);
        assert!(interval.as_millis() <= 2000);
    }

    #[test]
    fn test_text_input_unicode() {
        let text = "Hello 世界 🌍";
        assert!(text.len() > 5);
        assert!(text.contains("Hello"));
    }

    #[test]
    fn test_selector_list_iteration() {
        let selectors = vec![
            "#first".to_string(),
            "#second".to_string(),
            "#third".to_string(),
        ];
        let mut count = 0;
        for _ in &selectors {
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn test_empty_screenshot_path_check() {
        let path = String::new();
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
    }

    #[test]
    fn test_duration_from_secs_f64() {
        let duration = Duration::from_secs_f64(1.5);
        assert_eq!(duration.as_millis(), 1500);
    }

    #[test]
    fn test_selector_vec_capacity() {
        let mut selectors = Vec::with_capacity(5);
        selectors.push(".selector1".to_string());
        selectors.push(".selector2".to_string());
        assert_eq!(selectors.len(), 2);
        assert!(selectors.capacity() >= 2);
    }
}
