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
