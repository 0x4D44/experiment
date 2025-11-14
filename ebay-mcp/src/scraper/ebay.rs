//! eBay-specific scraping implementation

use crate::browser::AntiDetection;
use crate::error::{EbayMcpError, Result};
use crate::models::{BuyingFormat, EbayListing, Price, SearchFilters, SearchResults};
#[cfg(test)]
use crate::models::SortOrder;
use chrono::Utc;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Scraper configuration
#[derive(Debug, Clone)]
pub struct ScraperConfig {
    /// Base eBay URL (for different regions)
    pub base_url: String,

    /// Max retries on failure
    pub max_retries: u32,

    /// Screenshot on error
    pub screenshot_on_error: bool,

    /// Screenshot directory
    pub screenshot_dir: Option<std::path::PathBuf>,
}

/// eBay scraper
pub struct EbayScraper {
    /// Scraper configuration
    config: ScraperConfig,

    /// Anti-detection handler
    anti_detection: AntiDetection,
}

impl EbayScraper {
    /// Create new scraper instance
    pub fn new(config: ScraperConfig, anti_detection: AntiDetection) -> Self {
        Self {
            config,
            anti_detection,
        }
    }

    /// Execute search with retry logic
    pub async fn search_with_retry(
        &mut self,
        query: &str,
        filters: &SearchFilters,
        page: usize,
    ) -> Result<SearchResults> {
        let mut attempts = 0;
        let max_retries = self.config.max_retries;

        loop {
            attempts += 1;

            debug!(
                "Search attempt {}/{} for query: '{}'",
                attempts, max_retries, query
            );

            match self.search(query, filters, page).await {
                Ok(results) => {
                    info!(
                        "Search successful: {} results for '{}'",
                        results.items.len(),
                        query
                    );
                    return Ok(results);
                }
                Err(e) => {
                    warn!("Search attempt {} failed: {}", attempts, e);

                    if attempts >= max_retries {
                        return Err(e);
                    }

                    match &e {
                        EbayMcpError::CaptchaDetected => {
                            // Don't retry on CAPTCHA
                            return Err(e);
                        }
                        EbayMcpError::RateLimited => {
                            // Exponential backoff
                            let delay = Duration::from_secs(2u64.pow(attempts));
                            warn!("Rate limited, waiting {:?}", delay);
                            tokio::time::sleep(delay).await;
                        }
                        _ => {
                            // Short delay for other errors
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
            }
        }
    }

    /// Execute search (stub implementation)
    async fn search(
        &mut self,
        query: &str,
        filters: &SearchFilters,
        page: usize,
    ) -> Result<SearchResults> {
        let start = Instant::now();

        debug!("Building search URL");
        let url = self.build_search_url(query, filters, page);

        debug!("Search URL: {}", url);

        // Apply random delay for anti-detection
        self.anti_detection.random_delay().await;

        // TODO: When we can build with headless_chrome:
        // 1. Navigate to URL
        // 2. Wait for results to load
        // 3. Extract listings
        // 4. Handle errors and CAPTCHAs

        // For now, return stub results
        let results = SearchResults {
            query: query.to_string(),
            filters: filters.clone(),
            items: vec![],
            total_count: 0,
            page,
            total_pages: 0,
            searched_at: Utc::now(),
            duration: start.elapsed(),
        };

        Ok(results)
    }

    /// Build eBay search URL
    pub fn build_search_url(&self, query: &str, filters: &SearchFilters, page: usize) -> String {
        let mut url = format!(
            "{}/sch/i.html?_nkw={}",
            self.config.base_url,
            urlencoding::encode(query)
        );

        // Add filters
        if let Some(ref category) = filters.category {
            url.push_str(&format!("&_sacat={}", urlencoding::encode(category)));
        }

        if let Some(price_min) = filters.price_min {
            url.push_str(&format!("&_udlo={}", price_min));
        }

        if let Some(price_max) = filters.price_max {
            url.push_str(&format!("&_udhi={}", price_max));
        }

        if let Some(ref condition) = filters.condition {
            let condition_codes = condition
                .iter()
                .map(|c| condition_to_code(c))
                .collect::<Vec<_>>()
                .join(",");

            if !condition_codes.is_empty() {
                url.push_str(&format!("&LH_ItemCondition={}", condition_codes));
            }
        }

        if let Some(ref buying_format) = filters.buying_format {
            for format in buying_format {
                match format.as_str() {
                    "Auction" => url.push_str("&LH_Auction=1"),
                    "BuyItNow" => url.push_str("&LH_BIN=1"),
                    _ => {}
                }
            }
        }

        if let Some(ref shipping) = filters.shipping {
            if shipping.free_shipping {
                url.push_str("&LH_FS=1");
            }
            if shipping.local_pickup {
                url.push_str("&LH_LPickup=1");
            }
        }

        if let Some(ref sort_order) = filters.sort_by {
            url.push_str(&format!("&_sop={}", sort_order.to_ebay_param()));
        }

        // Add page number
        if page > 1 {
            url.push_str(&format!("&_pgn={}", page));
        }

        url
    }

    /// Extract listings from HTML (stub)
    #[allow(dead_code)]
    async fn extract_listings(&self) -> Result<Vec<EbayListing>> {
        // TODO: Implement when we have browser access
        // This will use JavaScript/CSS selectors to extract:
        // - Item ID
        // - Title
        // - Price
        // - Shipping cost
        // - Condition
        // - Seller info
        // - Location
        // - Thumbnail URL
        // - Listing URL
        // - Bids (if auction)
        // - Time left

        Ok(vec![])
    }

    /// Extract total result count (stub)
    #[allow(dead_code)]
    async fn extract_total_count(&self) -> Result<usize> {
        // TODO: Extract from .srp-controls__count-heading element
        Ok(0)
    }

    /// Detect if CAPTCHA is present (stub)
    #[allow(dead_code)]
    async fn detect_captcha(&self) -> bool {
        // TODO: Look for CAPTCHA elements in the page
        false
    }

    /// Take screenshot for debugging (stub)
    #[allow(dead_code)]
    async fn take_screenshot(&self, _name: &str) -> Result<std::path::PathBuf> {
        // TODO: Implement screenshot capture
        Err(EbayMcpError::NotImplemented(
            "Screenshot capture".to_string(),
        ))
    }
}

/// Convert condition string to eBay condition code
fn condition_to_code(condition: &str) -> &str {
    match condition.to_lowercase().as_str() {
        "new" => "1000",
        "open box" => "1500",
        "refurbished" => "2000",
        "used" => "3000",
        "for parts or not working" => "7000",
        _ => "3000", // Default to used
    }
}

/// Parse price from eBay text
pub fn parse_price(price_text: &str) -> Option<Price> {
    // Remove currency symbols and commas
    let cleaned = price_text
        .replace('$', "")
        .replace(',', "")
        .replace("USD", "")
        .trim()
        .to_string();

    // Extract numeric part
    if let Ok(amount) = cleaned.parse::<f64>() {
        Some(Price::usd(amount))
    } else {
        None
    }
}

/// Parse buying format from eBay listing
pub fn parse_buying_format(format_text: &str) -> BuyingFormat {
    if format_text.to_lowercase().contains("auction") {
        BuyingFormat::Auction
    } else if format_text.to_lowercase().contains("buy it now") {
        BuyingFormat::BuyItNow
    } else {
        BuyingFormat::BestOffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_search_url() {
        let config = ScraperConfig {
            base_url: "https://www.ebay.com".to_string(),
            max_retries: 3,
            screenshot_on_error: false,
            screenshot_dir: None,
        };

        let anti_det = AntiDetection::new(
            vec![],
            false,
            Duration::from_millis(0),
            Duration::from_millis(0),
        );

        let scraper = EbayScraper::new(config, anti_det);

        let mut filters = SearchFilters::default();
        filters.price_min = Some(100.0);
        filters.price_max = Some(500.0);
        filters.sort_by = Some(SortOrder::PriceLowest);

        let url = scraper.build_search_url("vintage camera", &filters, 1);

        assert!(url.contains("_nkw=vintage"));
        assert!(url.contains("_udlo=100"));
        assert!(url.contains("_udhi=500"));
    }

    #[test]
    fn test_condition_to_code() {
        assert_eq!(condition_to_code("New"), "1000");
        assert_eq!(condition_to_code("used"), "3000");
        assert_eq!(condition_to_code("Refurbished"), "2000");
    }

    #[test]
    fn test_parse_price() {
        assert_eq!(parse_price("$123.45").unwrap().amount, 123.45);
        assert_eq!(parse_price("$1,234.56 USD").unwrap().amount, 1234.56);
        assert_eq!(parse_price("99.99").unwrap().amount, 99.99);
        assert!(parse_price("invalid").is_none());
    }

    #[test]
    fn test_parse_buying_format() {
        assert!(matches!(
            parse_buying_format("Auction"),
            BuyingFormat::Auction
        ));
        assert!(matches!(
            parse_buying_format("Buy It Now"),
            BuyingFormat::BuyItNow
        ));
    }
}
