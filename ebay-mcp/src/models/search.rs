//! Search-related data models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::listing::EbayListing;

/// Saved search phrase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearchPhrase {
    /// Unique identifier
    pub id: String,

    /// Display name
    pub name: String,

    /// Search query
    pub query: String,

    /// Search filters
    #[serde(default)]
    pub filters: SearchFilters,

    /// Tags for organization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last used timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<DateTime<Utc>>,

    /// Usage count
    #[serde(default)]
    pub usage_count: u64,
}

/// Search filters for eBay searches
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchFilters {
    /// Category filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Price range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_max: Option<f64>,

    /// Condition (new, used, refurbished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Vec<String>>,

    /// Buying format (auction, buy_it_now, classified_ads)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buying_format: Option<Vec<String>>,

    /// Location/shipping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingOptions>,

    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortOrder>,

    /// Item specifics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_specifics: Option<HashMap<String, String>>,
}

/// Sort order options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortOrder {
    BestMatch,
    PricePlusShippingLowest,
    PricePlusShippingHighest,
    PriceLowest,
    PriceHighest,
    DistanceNearest,
    TimeEndingSoonest,
    TimeNewlyListed,
}

impl SortOrder {
    /// Convert to eBay URL parameter value
    pub fn to_ebay_param(&self) -> &'static str {
        match self {
            SortOrder::BestMatch => "12",
            SortOrder::PricePlusShippingLowest => "15",
            SortOrder::PricePlusShippingHighest => "16",
            SortOrder::PriceLowest => "1",
            SortOrder::PriceHighest => "2",
            SortOrder::DistanceNearest => "7",
            SortOrder::TimeEndingSoonest => "1",
            SortOrder::TimeNewlyListed => "10",
        }
    }
}

/// Shipping options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingOptions {
    #[serde(default)]
    pub free_shipping: bool,

    #[serde(default)]
    pub local_pickup: bool,

    #[serde(default)]
    pub international: bool,
}

/// Search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    /// Search query
    pub query: String,

    /// Applied filters
    pub filters: SearchFilters,

    /// Result listings
    pub items: Vec<EbayListing>,

    /// Total result count
    pub total_count: usize,

    /// Current page
    pub page: usize,

    /// Total pages
    pub total_pages: usize,

    /// Search timestamp
    pub searched_at: DateTime<Utc>,

    /// Time taken to scrape
    #[serde(with = "duration_serde")]
    pub duration: Duration,
}

/// Search history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryEntry {
    pub id: i64,
    pub query: String,
    pub filters_json: Option<String>,
    pub result_count: usize,
    pub searched_at: DateTime<Utc>,
    pub duration_ms: i64,
    pub success: bool,
    pub error_message: Option<String>,
}

// Helper module for Duration serialization
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}
