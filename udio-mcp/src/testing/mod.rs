/// Testing utilities and mocks for Udio MCP Server
///
/// This module provides mock implementations and test helpers
/// to facilitate testing of browser-dependent and OS-dependent code.
/// Mock browser implementation for testing
pub mod mock_browser;
/// Mock keychain implementation for testing
pub mod mock_keychain;

pub use mock_browser::{create_test_page, MockBrowser, MockBrowserBuilder, MockElement, MockPage};
pub use mock_keychain::{MockKeychain, MockKeychainBuilder};

/// Common test fixtures and helpers
pub mod fixtures {
    use super::*;

    /// Create a test browser with a single page
    pub fn create_simple_browser(url: &str) -> MockBrowser {
        MockBrowser::builder().with_page(url).build()
    }

    /// Create a test browser with Udio-like page structure
    pub fn create_udio_browser() -> MockBrowser {
        MockBrowser::builder()
            .with_page("https://udio.com")
            .with_element(
                "#login-email",
                MockElement::new("input")
                    .with_attribute("type", "email")
                    .with_enabled(true),
            )
            .with_element(
                "#login-password",
                MockElement::new("input")
                    .with_attribute("type", "password")
                    .with_enabled(true),
            )
            .with_element(
                "#login-button",
                MockElement::new("button")
                    .with_text("Log In")
                    .with_visible(true),
            )
            .build()
    }

    /// Create a test keychain with common credentials
    pub fn create_test_keychain() -> MockKeychain {
        MockKeychain::builder()
            .with_password("udio", "test@example.com", "test_password")
            .build()
    }
}

/// Assertion helpers for tests
pub mod assertions {
    use std::fmt::Debug;

    /// Assert that a result contains a specific error message
    pub fn assert_error_contains<T: Debug>(result: anyhow::Result<T>, expected: &str) {
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(
            error.to_string().contains(expected),
            "Expected error containing '{}', got: {}",
            expected,
            error
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fixtures::*;

    #[test]
    fn test_create_simple_browser() {
        let browser = create_simple_browser("https://test.com");
        assert!(browser.is_launched());
        assert_eq!(browser.pages().len(), 1);
        assert_eq!(browser.get_page(0).unwrap().url, "https://test.com");
    }

    #[test]
    fn test_create_udio_browser() {
        let browser = create_udio_browser();
        let page = browser.get_page(0).unwrap();

        assert_eq!(page.url, "https://udio.com");
        assert!(page.find_element("#login-email").is_ok());
        assert!(page.find_element("#login-password").is_ok());
        assert!(page.find_element("#login-button").is_ok());
    }

    #[test]
    fn test_create_test_keychain() {
        let keychain = create_test_keychain();
        assert!(keychain.has_password("udio", "test@example.com"));
        assert_eq!(
            keychain.get_password("udio", "test@example.com").unwrap(),
            "test_password"
        );
    }
}
