//! Test data builders and fixtures

use ebay_mcp_server::models::{
    BuyingFormat, EbayListing, Price, SearchFilters, SearchResults, SearchHistoryEntry,
    ShippingOptions, SortOrder,
};
use chrono::Utc;
use std::time::Duration;

/// Builder for SearchFilters
pub struct SearchFiltersBuilder {
    filters: SearchFilters,
}

impl SearchFiltersBuilder {
    pub fn new() -> Self {
        Self {
            filters: SearchFilters::default(),
        }
    }

    pub fn category(mut self, category: &str) -> Self {
        self.filters.category = Some(category.to_string());
        self
    }

    pub fn price_range(mut self, min: f64, max: f64) -> Self {
        self.filters.price_min = Some(min);
        self.filters.price_max = Some(max);
        self
    }

    pub fn condition(mut self, condition: &str) -> Self {
        self.filters.condition = Some(vec![condition.to_string()]);
        self
    }

    pub fn buying_format(mut self, format: &str) -> Self {
        self.filters.buying_format = Some(vec![format.to_string()]);
        self
    }

    pub fn free_shipping(mut self) -> Self {
        if self.filters.shipping.is_none() {
            self.filters.shipping = Some(ShippingOptions::default());
        }
        if let Some(ref mut shipping) = self.filters.shipping {
            shipping.free_shipping = true;
        }
        self
    }

    pub fn sort_by(mut self, sort: SortOrder) -> Self {
        self.filters.sort_by = Some(sort);
        self
    }

    pub fn build(self) -> SearchFilters {
        self.filters
    }
}

impl Default for SearchFiltersBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for EbayListing
pub struct EbayListingBuilder {
    listing: EbayListing,
}

impl EbayListingBuilder {
    pub fn new() -> Self {
        Self {
            listing: EbayListing {
                item_id: "123456789".to_string(),
                title: "Test Item".to_string(),
                price: Price::usd(100.0),
                shipping_cost: None,
                condition: "New".to_string(),
                seller_name: "test_seller".to_string(),
                seller_rating: None,
                location: "USA".to_string(),
                thumbnail_url: Some("https://example.com/thumb.jpg".to_string()),
                listing_url: "https://ebay.com/itm/123456789".to_string(),
                buying_format: BuyingFormat::BuyItNow,
                bids: None,
                time_left: None,
            },
        }
    }

    pub fn item_id(mut self, id: &str) -> Self {
        self.listing.item_id = id.to_string();
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.listing.title = title.to_string();
        self
    }

    pub fn price(mut self, amount: f64) -> Self {
        self.listing.price = Price::usd(amount);
        self
    }

    pub fn condition(mut self, condition: &str) -> Self {
        self.listing.condition = condition.to_string();
        self
    }

    pub fn buying_format(mut self, format: BuyingFormat) -> Self {
        self.listing.buying_format = format;
        self
    }

    pub fn auction_with_bids(mut self, bids: u32, time_left: &str) -> Self {
        self.listing.buying_format = BuyingFormat::Auction;
        self.listing.bids = Some(bids);
        self.listing.time_left = Some(time_left.to_string());
        self
    }

    pub fn build(self) -> EbayListing {
        self.listing
    }
}

impl Default for EbayListingBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for SearchResults
pub struct SearchResultsBuilder {
    results: SearchResults,
}

impl SearchResultsBuilder {
    pub fn new(query: &str) -> Self {
        Self {
            results: SearchResults {
                query: query.to_string(),
                filters: SearchFilters::default(),
                items: vec![],
                total_count: 0,
                page: 1,
                total_pages: 0,
                searched_at: Utc::now(),
                duration: Duration::from_millis(100),
            },
        }
    }

    pub fn filters(mut self, filters: SearchFilters) -> Self {
        self.results.filters = filters;
        self
    }

    pub fn add_item(mut self, item: EbayListing) -> Self {
        self.results.items.push(item);
        self.results.total_count = self.results.items.len();
        self
    }

    pub fn add_items(mut self, items: Vec<EbayListing>) -> Self {
        self.results.total_count += items.len();
        self.results.items.extend(items);
        self
    }

    pub fn total_count(mut self, count: usize) -> Self {
        self.results.total_count = count;
        self
    }

    pub fn page(mut self, page: usize) -> Self {
        self.results.page = page;
        self
    }

    pub fn total_pages(mut self, pages: usize) -> Self {
        self.results.total_pages = pages;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.results.duration = duration;
        self
    }

    pub fn build(self) -> SearchResults {
        self.results
    }
}

/// Create a sample EbayListing for testing
pub fn sample_listing() -> EbayListing {
    EbayListingBuilder::new().build()
}

/// Create a sample SearchFilters for testing
pub fn sample_filters() -> SearchFilters {
    SearchFiltersBuilder::new()
        .price_range(10.0, 100.0)
        .free_shipping()
        .build()
}

/// Create sample SearchResults for testing
pub fn sample_results(query: &str, count: usize) -> SearchResults {
    let mut builder = SearchResultsBuilder::new(query);

    for i in 0..count {
        let item = EbayListingBuilder::new()
            .item_id(&format!("ITEM{}", i))
            .title(&format!("Test Item {}", i))
            .price(10.0 + (i as f64 * 5.0))
            .build();
        builder = builder.add_item(item);
    }

    builder.total_count(count).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_filters_builder() {
        let filters = SearchFiltersBuilder::new()
            .category("Electronics")
            .price_range(50.0, 200.0)
            .free_shipping()
            .build();

        assert_eq!(filters.category, Some("Electronics".to_string()));
        assert_eq!(filters.price_min, Some(50.0));
        assert_eq!(filters.price_max, Some(200.0));
        assert!(filters.shipping.is_some());
    }

    #[test]
    fn test_listing_builder() {
        let listing = EbayListingBuilder::new()
            .title("Custom Item")
            .price(99.99)
            .condition("Used")
            .build();

        assert_eq!(listing.title, "Custom Item");
        assert_eq!(listing.price.amount, 99.99);
        assert_eq!(listing.condition, "Used");
    }

    #[test]
    fn test_sample_results() {
        let results = sample_results("test query", 5);
        assert_eq!(results.items.len(), 5);
        assert_eq!(results.total_count, 5);
        assert_eq!(results.query, "test query");
    }
}
