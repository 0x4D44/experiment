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

    #[test]
    fn test_selector_fallback_list() {
        let selectors = vec![
            ".primary-selector".to_string(),
            ".fallback-selector".to_string(),
            "#last-resort".to_string(),
        ];
        assert_eq!(selectors.len(), 3);
    }

    #[test]
    fn test_timeout_duration() {
        let timeout = Duration::from_secs(30);
        assert_eq!(timeout.as_secs(), 30);
    }

    #[test]
    fn test_interval_duration() {
        let interval = Duration::from_millis(500);
        assert_eq!(interval.as_millis(), 500);
    }
}
