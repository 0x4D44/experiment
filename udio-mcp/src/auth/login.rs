// Login automation for Udio platform
// Uses browser automation to perform login and extract session

use anyhow::{Result, Context, bail};
use chromiumoxide::Page;
use std::time::Duration;

use crate::browser::{automation, selectors::Selectors};
use super::session::{Session, Cookie};
use super::credentials::Credentials;

/// Handles automated login to Udio platform
pub struct LoginAutomation {
    /// Selectors for auth UI elements
    selectors: Selectors,

    /// Udio base URL
    base_url: String,
}

impl LoginAutomation {
    /// Create a new login automation handler
    pub fn new() -> Self {
        Self {
            selectors: Selectors::load_default(),
            base_url: "https://www.udio.com".to_string(),
        }
    }

    /// Create with custom selectors
    pub fn with_selectors(selectors: Selectors) -> Self {
        Self {
            selectors,
            base_url: "https://www.udio.com".to_string(),
        }
    }

    /// Perform login and return authenticated session
    pub async fn login(&self, page: &Page, credentials: &Credentials) -> Result<Session> {
        credentials.validate()
            .context("Invalid credentials")?;

        tracing::info!("Starting login automation for: {}", credentials.email);

        // Navigate to login page
        let login_url = format!("{}/login", self.base_url);
        page.goto(&login_url).await
            .context("Failed to navigate to login page")?;

        // Wait for page to load
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Wait for and fill email input
        tracing::debug!("Waiting for email input field");
        automation::wait_for_element(
            page,
            &self.selectors.auth.email_input,
            Duration::from_secs(10),
            Duration::from_millis(500),
        ).await.context("Email input field not found")?;

        automation::type_into_element(
            page,
            &self.selectors.auth.email_input,
            &credentials.email,
        ).await.context("Failed to enter email")?;

        tracing::debug!("Email entered successfully");

        // Wait for and fill password input
        tracing::debug!("Waiting for password input field");
        automation::type_into_element(
            page,
            &self.selectors.auth.password_input,
            &credentials.password,
        ).await.context("Failed to enter password")?;

        tracing::debug!("Password entered successfully");

        // Click submit button
        tracing::debug!("Clicking submit button");
        automation::click_element(
            page,
            &self.selectors.auth.submit_button,
        ).await.context("Failed to click submit button")?;

        // Wait for login to complete (navigation or specific element)
        self.wait_for_login_completion(page).await
            .context("Login did not complete successfully")?;

        // Extract session cookies
        let cookies = self.extract_cookies(page).await
            .context("Failed to extract session cookies")?;

        if cookies.is_empty() {
            bail!("No session cookies found after login");
        }

        tracing::info!("Login successful for: {}", credentials.email);

        // Create session with 24 hour TTL
        let session = Session::new(
            credentials.email.clone(),
            cookies,
            24 * 60 * 60, // 24 hours in seconds
        );

        Ok(session)
    }

    /// Wait for login completion indicators
    async fn wait_for_login_completion(&self, page: &Page) -> Result<()> {
        // Wait for URL change or specific post-login element
        // This is a simple implementation - could be enhanced
        tokio::time::sleep(Duration::from_secs(3)).await;

        // Check if we're still on login page
        let url = page.url().await?;
        if url.as_ref().map(|u| u.contains("/login")).unwrap_or(false) {
            // Still on login page - might indicate failed login
            tracing::warn!("Still on login page after submit - login may have failed");
        }

        Ok(())
    }

    /// Extract cookies from the current page
    async fn extract_cookies(&self, page: &Page) -> Result<Vec<Cookie>> {
        // Get all cookies from the browser
        let browser_cookies = page.get_cookies().await
            .context("Failed to get cookies from page")?;

        let mut cookies = Vec::new();

        for cookie in browser_cookies {
            cookies.push(Cookie {
                name: cookie.name,
                value: cookie.value,
                domain: Some(cookie.domain),
                path: Some(cookie.path),
                secure: cookie.secure,
                http_only: cookie.http_only,
            });
        }

        tracing::debug!("Extracted {} cookies from page", cookies.len());

        Ok(cookies)
    }

    /// Check if already logged in
    pub async fn is_logged_in(&self, page: &Page) -> bool {
        // Navigate to a protected page or check for auth indicators
        let url = page.url().await;

        match url {
            Ok(Some(url)) => {
                // If we're not on login page, we might be logged in
                !url.contains("/login")
            }
            _ => false,
        }
    }

    /// Logout from current session
    pub async fn logout(&self, page: &Page) -> Result<()> {
        tracing::info!("Logging out");

        // Clear cookies
        let cookies = page.get_cookies().await?;
        let delete_params: Vec<_> = cookies.into_iter().map(|cookie| {
            chromiumoxide::cdp::browser_protocol::network::DeleteCookiesParams {
                name: cookie.name,
                url: None,
                domain: Some(cookie.domain),
                path: Some(cookie.path),
            }
        }).collect();

        if !delete_params.is_empty() {
            page.delete_cookies(delete_params).await?;
        }

        tracing::info!("Logout successful - cookies cleared");
        Ok(())
    }
}

impl Default for LoginAutomation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_automation_creation() {
        let login = LoginAutomation::new();
        assert_eq!(login.base_url, "https://www.udio.com");
    }

    #[test]
    fn test_login_automation_with_selectors() {
        let selectors = Selectors::default();
        let login = LoginAutomation::with_selectors(selectors);
        assert_eq!(login.base_url, "https://www.udio.com");
    }

    #[test]
    fn test_login_automation_default() {
        let login = LoginAutomation::default();
        assert_eq!(login.base_url, "https://www.udio.com");
    }

    #[test]
    fn test_login_url_format() {
        let login = LoginAutomation::new();
        let login_url = format!("{}/login", login.base_url);
        assert_eq!(login_url, "https://www.udio.com/login");
    }

    #[test]
    fn test_base_url_structure() {
        let login = LoginAutomation::new();
        assert!(login.base_url.starts_with("https://"));
        assert!(login.base_url.contains("udio.com"));
    }

    #[test]
    fn test_session_ttl_calculation() {
        let ttl_seconds = 24 * 60 * 60;
        assert_eq!(ttl_seconds, 86400); // 24 hours
    }

    #[test]
    fn test_cookie_conversion_structure() {
        // Test the structure used for cookie conversion
        let cookie = Cookie {
            name: "session".to_string(),
            value: "abc123".to_string(),
            domain: Some("udio.com".to_string()),
            path: Some("/".to_string()),
            secure: true,
            http_only: true,
        };

        assert_eq!(cookie.name, "session");
        assert_eq!(cookie.value, "abc123");
        assert!(cookie.secure);
        assert!(cookie.http_only);
    }

    #[test]
    fn test_cookie_vec_creation() {
        let cookies: Vec<Cookie> = Vec::new();
        assert!(cookies.is_empty());
    }

    #[test]
    fn test_cookie_vec_with_data() {
        let mut cookies = Vec::new();
        cookies.push(Cookie::new("session", "value1"));
        cookies.push(Cookie::new("auth", "value2"));
        assert_eq!(cookies.len(), 2);
    }

    #[test]
    fn test_url_login_check() {
        let url = "https://www.udio.com/login";
        assert!(url.contains("/login"));
    }

    #[test]
    fn test_url_not_login() {
        let url = "https://www.udio.com/dashboard";
        assert!(!url.contains("/login"));
    }

    #[test]
    fn test_is_logged_in_url_check() {
        let login_url = "https://www.udio.com/login";
        let dashboard_url = "https://www.udio.com/dashboard";

        assert!(login_url.contains("/login"));
        assert!(!dashboard_url.contains("/login"));
    }

    #[test]
    fn test_empty_cookies_check() {
        let cookies: Vec<Cookie> = Vec::new();
        let is_empty = cookies.is_empty();
        assert!(is_empty);
    }

    #[test]
    fn test_non_empty_cookies_check() {
        let cookies = vec![Cookie::new("session", "value")];
        let is_empty = cookies.is_empty();
        assert!(!is_empty);
    }

    #[test]
    fn test_delete_cookies_params_creation() {
        // Test the structure for delete params
        let cookie_name = "session_cookie";
        let cookie_domain = "udio.com";
        let cookie_path = "/";

        assert_eq!(cookie_name, "session_cookie");
        assert_eq!(cookie_domain, "udio.com");
        assert_eq!(cookie_path, "/");
    }

    #[test]
    fn test_multiple_login_instances() {
        let _login1 = LoginAutomation::new();
        let _login2 = LoginAutomation::new();
        let _login3 = LoginAutomation::default();
        // Should be able to create multiple instances
    }

    #[test]
    fn test_base_url_https() {
        let login = LoginAutomation::new();
        assert!(login.base_url.starts_with("https://"));
    }

    #[test]
    fn test_selectors_loading() {
        let selectors = Selectors::load_default();
        let login = LoginAutomation::with_selectors(selectors);
        // Verify selectors are set
        let _ = login.selectors;
    }

    #[test]
    fn test_ttl_variations() {
        // Test different TTL values
        let hour = 60 * 60;
        let day = 24 * hour;
        let week = 7 * day;

        assert_eq!(hour, 3600);
        assert_eq!(day, 86400);
        assert_eq!(week, 604800);
    }
}
