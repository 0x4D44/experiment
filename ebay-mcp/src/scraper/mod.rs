//! Web scraping module for eBay

pub mod ebay;

pub use ebay::{parse_buying_format, parse_price, EbayScraper, ScraperConfig};
