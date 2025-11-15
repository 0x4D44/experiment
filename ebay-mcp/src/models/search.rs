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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_filters_default() {
        let filters = SearchFilters::default();

        assert!(filters.category.is_none());
        assert!(filters.price_min.is_none());
        assert!(filters.price_max.is_none());
        assert!(filters.condition.is_none());
        assert!(filters.buying_format.is_none());
        assert!(filters.location.is_none());
        assert!(filters.shipping.is_none());
        assert!(filters.sort_by.is_none());
        assert!(filters.item_specifics.is_none());
    }

    #[test]
    fn test_search_filters_serialization() {
        let mut filters = SearchFilters::default();
        filters.category = Some("Electronics".to_string());
        filters.price_min = Some(10.0);
        filters.price_max = Some(100.0);

        let json = serde_json::to_string(&filters).unwrap();
        let deserialized: SearchFilters = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.category, Some("Electronics".to_string()));
        assert_eq!(deserialized.price_min, Some(10.0));
        assert_eq!(deserialized.price_max, Some(100.0));
    }

    #[test]
    fn test_search_filters_with_condition() {
        let mut filters = SearchFilters::default();
        filters.condition = Some(vec!["New".to_string(), "Used".to_string()]);

        let json = serde_json::to_string(&filters).unwrap();
        let deserialized: SearchFilters = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.condition.unwrap().len(), 2);
    }

    #[test]
    fn test_sort_order_to_ebay_param() {
        assert_eq!(SortOrder::BestMatch.to_ebay_param(), "12");
        assert_eq!(SortOrder::PricePlusShippingLowest.to_ebay_param(), "15");
        assert_eq!(SortOrder::PricePlusShippingHighest.to_ebay_param(), "16");
        assert_eq!(SortOrder::PriceLowest.to_ebay_param(), "1");
        assert_eq!(SortOrder::PriceHighest.to_ebay_param(), "2");
        assert_eq!(SortOrder::DistanceNearest.to_ebay_param(), "7");
        assert_eq!(SortOrder::TimeEndingSoonest.to_ebay_param(), "1");
        assert_eq!(SortOrder::TimeNewlyListed.to_ebay_param(), "10");
    }

    #[test]
    fn test_sort_order_serialization() {
        let order = SortOrder::PriceLowest;
        let json = serde_json::to_string(&order).unwrap();
        let deserialized: SortOrder = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.to_ebay_param(), "1");
    }

    #[test]
    fn test_shipping_options_default() {
        let shipping = ShippingOptions::default();

        assert_eq!(shipping.free_shipping, false);
        assert_eq!(shipping.local_pickup, false);
        assert_eq!(shipping.international, false);
    }

    #[test]
    fn test_shipping_options_serialization() {
        let mut shipping = ShippingOptions::default();
        shipping.free_shipping = true;
        shipping.international = true;

        let json = serde_json::to_string(&shipping).unwrap();
        let deserialized: ShippingOptions = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.free_shipping, true);
        assert_eq!(deserialized.local_pickup, false);
        assert_eq!(deserialized.international, true);
    }

    #[test]
    fn test_saved_search_phrase_serialization() {
        use uuid::Uuid;

        let phrase = SavedSearchPhrase {
            id: Uuid::new_v4().to_string(),
            name: "Vintage Cameras".to_string(),
            query: "vintage camera".to_string(),
            filters: SearchFilters::default(),
            tags: vec!["cameras".to_string(), "vintage".to_string()],
            created_at: Utc::now(),
            last_used: None,
            usage_count: 5,
        };

        let json = serde_json::to_string(&phrase).unwrap();
        let deserialized: SavedSearchPhrase = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Vintage Cameras");
        assert_eq!(deserialized.query, "vintage camera");
        assert_eq!(deserialized.tags.len(), 2);
        assert_eq!(deserialized.usage_count, 5);
        assert!(deserialized.last_used.is_none());
    }

    #[test]
    fn test_saved_search_phrase_with_last_used() {
        use uuid::Uuid;

        let now = Utc::now();
        let phrase = SavedSearchPhrase {
            id: Uuid::new_v4().to_string(),
            name: "Test".to_string(),
            query: "test query".to_string(),
            filters: SearchFilters::default(),
            tags: vec![],
            created_at: now,
            last_used: Some(now),
            usage_count: 10,
        };

        let json = serde_json::to_string(&phrase).unwrap();
        let deserialized: SavedSearchPhrase = serde_json::from_str(&json).unwrap();

        assert!(deserialized.last_used.is_some());
        assert_eq!(deserialized.usage_count, 10);
    }

    #[test]
    fn test_search_history_entry_serialization() {
        let entry = SearchHistoryEntry {
            id: 1,
            query: "test search".to_string(),
            filters_json: Some(r#"{"price_min":10.0}"#.to_string()),
            result_count: 42,
            searched_at: Utc::now(),
            duration_ms: 1500,
            success: true,
            error_message: None,
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: SearchHistoryEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, 1);
        assert_eq!(deserialized.query, "test search");
        assert_eq!(deserialized.result_count, 42);
        assert_eq!(deserialized.duration_ms, 1500);
        assert_eq!(deserialized.success, true);
        assert!(deserialized.error_message.is_none());
    }

    #[test]
    fn test_search_history_entry_with_error() {
        let entry = SearchHistoryEntry {
            id: 2,
            query: "failed search".to_string(),
            filters_json: None,
            result_count: 0,
            searched_at: Utc::now(),
            duration_ms: 500,
            success: false,
            error_message: Some("Network timeout".to_string()),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: SearchHistoryEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.success, false);
        assert_eq!(deserialized.error_message, Some("Network timeout".to_string()));
    }

    #[test]
    fn test_search_filters_with_item_specifics() {
        let mut filters = SearchFilters::default();
        let mut specifics = HashMap::new();
        specifics.insert("Brand".to_string(), "Canon".to_string());
        specifics.insert("Condition".to_string(), "New".to_string());
        filters.item_specifics = Some(specifics);

        let json = serde_json::to_string(&filters).unwrap();
        let deserialized: SearchFilters = serde_json::from_str(&json).unwrap();

        let item_specifics = deserialized.item_specifics.unwrap();
        assert_eq!(item_specifics.get("Brand"), Some(&"Canon".to_string()));
        assert_eq!(item_specifics.get("Condition"), Some(&"New".to_string()));
    }

    #[test]
    fn test_search_filters_complete() {
        let mut filters = SearchFilters::default();
        filters.category = Some("Cameras".to_string());
        filters.price_min = Some(50.0);
        filters.price_max = Some(500.0);
        filters.condition = Some(vec!["New".to_string()]);
        filters.buying_format = Some(vec!["BuyItNow".to_string()]);
        filters.location = Some("US".to_string());
        filters.shipping = Some(ShippingOptions {
            free_shipping: true,
            local_pickup: false,
            international: false,
        });
        filters.sort_by = Some(SortOrder::PriceLowest);

        let json = serde_json::to_string(&filters).unwrap();
        let deserialized: SearchFilters = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.category, Some("Cameras".to_string()));
        assert_eq!(deserialized.price_min, Some(50.0));
        assert_eq!(deserialized.price_max, Some(500.0));
        assert!(deserialized.shipping.unwrap().free_shipping);
    }

    #[test]
    fn test_search_results_serialization() {
        let results = SearchResults {
            query: "vintage camera".to_string(),
            filters: SearchFilters::default(),
            items: vec![],
            total_count: 42,
            page: 1,
            total_pages: 3,
            searched_at: Utc::now(),
            duration: Duration::from_millis(1500),
        };

        let json = serde_json::to_string(&results).unwrap();
        let deserialized: SearchResults = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.query, "vintage camera");
        assert_eq!(deserialized.total_count, 42);
        assert_eq!(deserialized.page, 1);
        assert_eq!(deserialized.total_pages, 3);
        assert_eq!(deserialized.duration.as_millis(), 1500);
    }

    #[test]
    fn test_search_results_with_items() {
        use super::super::listing::{BuyingFormat, EbayListing, Price, SellerInfo};

        let listing = EbayListing {
            item_id: "123456".to_string(),
            title: "Test Item".to_string(),
            price: Price::usd(99.99),
            shipping: Some(Price::usd(5.00)),
            condition: "New".to_string(),
            format: BuyingFormat::BuyItNow,
            seller: SellerInfo {
                username: "testseller".to_string(),
                feedback_score: 1000,
                positive_percentage: 99.5,
            },
            location: "US".to_string(),
            thumbnail_url: Some("https://example.com/thumb.jpg".to_string()),
            listing_url: "https://ebay.com/itm/123456".to_string(),
            bids: None,
            time_left: None,
            free_shipping: false,
            returns_accepted: true,
        };

        let results = SearchResults {
            query: "test".to_string(),
            filters: SearchFilters::default(),
            items: vec![listing],
            total_count: 1,
            page: 1,
            total_pages: 1,
            searched_at: Utc::now(),
            duration: Duration::from_millis(500),
        };

        let json = serde_json::to_string(&results).unwrap();
        let deserialized: SearchResults = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.items.len(), 1);
        assert_eq!(deserialized.items[0].item_id, "123456");
    }

    #[test]
    fn test_duration_serialization() {
        let results = SearchResults {
            query: "test".to_string(),
            filters: SearchFilters::default(),
            items: vec![],
            total_count: 0,
            page: 1,
            total_pages: 0,
            searched_at: Utc::now(),
            duration: Duration::from_millis(2500),
        };

        let json = serde_json::to_string(&results).unwrap();
        assert!(json.contains("2500")); // Duration should be serialized as milliseconds

        let deserialized: SearchResults = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.duration.as_millis(), 2500);
    }

    #[test]
    fn test_sort_order_clone() {
        let order = SortOrder::PriceLowest;
        let cloned = order;
        assert_eq!(cloned.to_ebay_param(), "1");
    }

    #[test]
    fn test_shipping_options_clone() {
        let shipping = ShippingOptions {
            free_shipping: true,
            local_pickup: false,
            international: true,
        };
        let cloned = shipping.clone();
        assert_eq!(cloned.free_shipping, true);
        assert_eq!(cloned.international, true);
    }
}
