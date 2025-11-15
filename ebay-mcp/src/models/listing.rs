//! eBay listing data models

use serde::{Deserialize, Serialize};

/// eBay listing/item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbayListing {
    /// Item ID
    pub item_id: String,

    /// Title
    pub title: String,

    /// Current price
    pub price: Price,

    /// Shipping cost
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Price>,

    /// Item condition
    pub condition: String,

    /// Buying format
    pub format: BuyingFormat,

    /// Seller information
    pub seller: SellerInfo,

    /// Location
    pub location: String,

    /// Thumbnail URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    /// Listing URL
    pub listing_url: String,

    /// Number of bids (if auction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bids: Option<u32>,

    /// Time left (if auction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_left: Option<String>,

    /// Free shipping flag
    #[serde(default)]
    pub free_shipping: bool,

    /// Returns accepted
    #[serde(default)]
    pub returns_accepted: bool,
}

/// Price information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub amount: f64,
    pub currency: String,
}

impl Price {
    pub fn new(amount: f64, currency: impl Into<String>) -> Self {
        Self {
            amount,
            currency: currency.into(),
        }
    }

    pub fn usd(amount: f64) -> Self {
        Self::new(amount, "USD")
    }
}

/// Buying format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuyingFormat {
    Auction,
    BuyItNow,
    BestOffer,
}

/// Seller information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerInfo {
    pub username: String,
    pub feedback_score: u32,
    pub positive_percentage: f64,
}

impl Default for SellerInfo {
    fn default() -> Self {
        Self {
            username: String::new(),
            feedback_score: 0,
            positive_percentage: 100.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_new() {
        let price = Price::new(99.99, "USD");

        assert_eq!(price.amount, 99.99);
        assert_eq!(price.currency, "USD");
    }

    #[test]
    fn test_price_usd() {
        let price = Price::usd(49.99);

        assert_eq!(price.amount, 49.99);
        assert_eq!(price.currency, "USD");
    }

    #[test]
    fn test_price_serialization() {
        let price = Price::usd(125.50);

        let json = serde_json::to_string(&price).unwrap();
        let deserialized: Price = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.amount, 125.50);
        assert_eq!(deserialized.currency, "USD");
    }

    #[test]
    fn test_price_different_currency() {
        let price = Price::new(75.0, "EUR");

        assert_eq!(price.amount, 75.0);
        assert_eq!(price.currency, "EUR");
    }

    #[test]
    fn test_buying_format_serialization() {
        let formats = vec![
            BuyingFormat::Auction,
            BuyingFormat::BuyItNow,
            BuyingFormat::BestOffer,
        ];

        for format in formats {
            let json = serde_json::to_string(&format).unwrap();
            let _deserialized: BuyingFormat = serde_json::from_str(&json).unwrap();
        }
    }

    #[test]
    fn test_seller_info_default() {
        let seller = SellerInfo::default();

        assert_eq!(seller.username, "");
        assert_eq!(seller.feedback_score, 0);
        assert_eq!(seller.positive_percentage, 100.0);
    }

    #[test]
    fn test_seller_info_serialization() {
        let seller = SellerInfo {
            username: "top_seller".to_string(),
            feedback_score: 5000,
            positive_percentage: 99.8,
        };

        let json = serde_json::to_string(&seller).unwrap();
        let deserialized: SellerInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.username, "top_seller");
        assert_eq!(deserialized.feedback_score, 5000);
        assert_eq!(deserialized.positive_percentage, 99.8);
    }

    #[test]
    fn test_ebay_listing_serialization() {
        let listing = EbayListing {
            item_id: "123456789".to_string(),
            title: "Vintage Camera".to_string(),
            price: Price::usd(199.99),
            shipping: Some(Price::usd(15.0)),
            condition: "Used".to_string(),
            format: BuyingFormat::BuyItNow,
            seller: SellerInfo {
                username: "camera_shop".to_string(),
                feedback_score: 1000,
                positive_percentage: 98.5,
            },
            location: "New York, NY".to_string(),
            thumbnail_url: Some("https://example.com/thumb.jpg".to_string()),
            listing_url: "https://ebay.com/itm/123456789".to_string(),
            bids: None,
            time_left: None,
            free_shipping: false,
            returns_accepted: true,
        };

        let json = serde_json::to_string(&listing).unwrap();
        let deserialized: EbayListing = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.item_id, "123456789");
        assert_eq!(deserialized.title, "Vintage Camera");
        assert_eq!(deserialized.price.amount, 199.99);
        assert_eq!(deserialized.condition, "Used");
        assert!(deserialized.shipping.is_some());
        assert_eq!(deserialized.seller.username, "camera_shop");
    }

    #[test]
    fn test_ebay_listing_auction() {
        let listing = EbayListing {
            item_id: "987654321".to_string(),
            title: "Rare Collectible".to_string(),
            price: Price::usd(50.0),
            shipping: None,
            condition: "New".to_string(),
            format: BuyingFormat::Auction,
            seller: SellerInfo::default(),
            location: "CA".to_string(),
            thumbnail_url: None,
            listing_url: "https://ebay.com/itm/987654321".to_string(),
            bids: Some(15),
            time_left: Some("2d 5h".to_string()),
            free_shipping: true,
            returns_accepted: false,
        };

        assert_eq!(listing.bids, Some(15));
        assert_eq!(listing.time_left, Some("2d 5h".to_string()));
        assert_eq!(listing.free_shipping, true);
    }

    #[test]
    fn test_ebay_listing_minimal_fields() {
        let listing = EbayListing {
            item_id: "MIN123".to_string(),
            title: "Basic Item".to_string(),
            price: Price::usd(10.0),
            shipping: None,
            condition: "New".to_string(),
            format: BuyingFormat::BuyItNow,
            seller: SellerInfo::default(),
            location: "USA".to_string(),
            thumbnail_url: None,
            listing_url: "https://ebay.com/itm/MIN123".to_string(),
            bids: None,
            time_left: None,
            free_shipping: false,
            returns_accepted: false,
        };

        let json = serde_json::to_string(&listing).unwrap();
        let deserialized: EbayListing = serde_json::from_str(&json).unwrap();

        // Optional fields should be None or default
        assert!(deserialized.shipping.is_none());
        assert!(deserialized.thumbnail_url.is_none());
        assert!(deserialized.bids.is_none());
        assert!(deserialized.time_left.is_none());
    }
}
