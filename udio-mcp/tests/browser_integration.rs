// Integration tests for browser automation
// These tests use a real headless Chrome instance to test browser operations

use udio_mcp_server::browser::automation;
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;
use std::time::Duration;

// Helper to create a test HTML page
async fn create_test_page(browser: &Browser) -> chromiumoxide::Page {
    let page = browser.new_page("about:blank").await.unwrap();

    // Inject simple test HTML
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Test Page</title></head>
        <body>
            <div class="test-content">
                <h1 id="main-heading">Test Heading</h1>
                <button class="primary-button">Click Me</button>
                <button class="secondary-button">Alternative</button>
                <input type="text" id="text-input" placeholder="Enter text">
                <div class="item" data-id="1">Item 1</div>
                <div class="item" data-id="2">Item 2</div>
                <div class="item" data-id="3">Item 3</div>
            </div>
        </body>
        </html>
    "#;

    page.set_content(html).await.unwrap();
    page
}

#[tokio::test]
async fn test_find_element_with_fallback() {
    let config = BrowserConfig::builder()
        .no_sandbox()
        .build()
        .unwrap();

    let (_browser, mut handler) = Browser::launch(config)
        .await
        .expect("Failed to launch browser");

    // Spawn handler task
    let handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {
            // Process events
        }
    });

    let page = create_test_page(&_browser).await;

    // Test finding element with primary selector
    let selectors = vec![
        "#main-heading".to_string(),
        "h1".to_string(),
    ];

    let result = automation::find_element_with_fallback(&page, &selectors).await;
    assert!(result.is_ok());

    // Test fallback when primary fails
    let selectors = vec![
        "#nonexistent".to_string(),
        ".primary-button".to_string(),
    ];

    let result = automation::find_element_with_fallback(&page, &selectors).await;
    assert!(result.is_ok());

    // Test when no selector matches
    let selectors = vec![
        "#definitely-not-there".to_string(),
        ".also-missing".to_string(),
    ];

    let result = automation::find_element_with_fallback(&page, &selectors).await;
    assert!(result.is_err());

    drop(page);
    drop(_browser);
    handle.abort();
}

#[tokio::test]
async fn test_find_elements_with_fallback() {
    let config = BrowserConfig::builder().no_sandbox().build().unwrap();
    let (_browser, mut handler) = Browser::launch(config).await.expect("Failed to launch browser");

    let handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {
        }
    });

    let page = create_test_page(&_browser).await;

    // Test finding multiple elements
    let selectors = vec![
        ".item".to_string(),
    ];

    let result = automation::find_elements_with_fallback(&page, &selectors).await;
    assert!(result.is_ok());
    let elements = result.unwrap();
    assert_eq!(elements.len(), 3);

    drop(page);
    drop(_browser);
    handle.abort();
}

#[tokio::test]
async fn test_click_element() {
    let config = BrowserConfig::builder().no_sandbox().build().unwrap();
    let (_browser, mut handler) = Browser::launch(config).await.expect("Failed to launch browser");

    let handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {
        }
    });

    let page = create_test_page(&_browser).await;

    let selectors = vec![
        ".primary-button".to_string(),
    ];

    let result = automation::click_element(&page, &selectors).await;
    assert!(result.is_ok());

    drop(page);
    drop(_browser);
    handle.abort();
}

#[tokio::test]
async fn test_type_into_element() {
    let config = BrowserConfig::builder().no_sandbox().build().unwrap();
    let (_browser, mut handler) = Browser::launch(config).await.expect("Failed to launch browser");

    let handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {
        }
    });

    let page = create_test_page(&_browser).await;

    let selectors = vec![
        "#text-input".to_string(),
    ];

    let result = automation::type_into_element(&page, &selectors, "Hello World").await;
    assert!(result.is_ok());

    drop(page);
    drop(_browser);
    handle.abort();
}

#[tokio::test]
async fn test_is_element_visible() {
    let config = BrowserConfig::builder().no_sandbox().build().unwrap();
    let (_browser, mut handler) = Browser::launch(config).await.expect("Failed to launch browser");

    let handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {
        }
    });

    let page = create_test_page(&_browser).await;

    let selectors = vec![
        "#main-heading".to_string(),
    ];

    let visible = automation::is_element_visible(&page, &selectors).await;
    assert!(visible);

    let selectors = vec![
        "#nonexistent".to_string(),
    ];

    let visible = automation::is_element_visible(&page, &selectors).await;
    assert!(!visible);

    drop(page);
    drop(_browser);
    handle.abort();
}

#[tokio::test]
async fn test_wait_for_element() {
    let config = BrowserConfig::builder().no_sandbox().build().unwrap();
    let (_browser, mut handler) = Browser::launch(config).await.expect("Failed to launch browser");

    let handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {
        }
    });

    let page = create_test_page(&_browser).await;

    let selectors = vec![
        "#main-heading".to_string(),
    ];

    let result = automation::wait_for_element(
        &page,
        &selectors,
        Duration::from_secs(5),
        Duration::from_millis(100)
    ).await;
    assert!(result.is_ok());

    drop(page);
    drop(_browser);
    handle.abort();
}

#[tokio::test]
async fn test_wait_for_navigation() {
    let config = BrowserConfig::builder().no_sandbox().build().unwrap();
    let (_browser, mut handler) = Browser::launch(config).await.expect("Failed to launch browser");

    let handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {
        }
    });

    let page = create_test_page(&_browser).await;

    let result = automation::wait_for_navigation(&page, Duration::from_secs(2)).await;
    assert!(result.is_ok());

    drop(page);
    drop(_browser);
    handle.abort();
}
