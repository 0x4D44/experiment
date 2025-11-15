// Data extraction utilities
// Helper functions for extracting data from web pages

use anyhow::{Context, Result};
use chromiumoxide::element::Element;
use chromiumoxide::Page;

use super::automation::find_element_with_fallback;
use super::automation::find_elements_with_fallback;

/// Extract text content from an element using selector fallbacks
pub async fn extract_text(page: &Page, selectors: &[String]) -> Result<String> {
    let element = find_element_with_fallback(page, selectors)
        .await
        .context("Failed to find element for text extraction")?;

    let text = element
        .inner_text()
        .await
        .context("Failed to extract text from element")?
        .unwrap_or_default();

    tracing::debug!("Extracted text: {} characters", text.len());
    Ok(text.trim().to_string())
}

/// Extract text from all matching elements
pub async fn extract_text_from_all(page: &Page, selectors: &[String]) -> Result<Vec<String>> {
    let elements = find_elements_with_fallback(page, selectors)
        .await
        .context("Failed to find elements for text extraction")?;

    let mut texts = Vec::new();

    for element in elements {
        if let Ok(Some(text)) = element.inner_text().await {
            texts.push(text.trim().to_string());
        }
    }

    tracing::debug!("Extracted text from {} elements", texts.len());
    Ok(texts)
}

/// Extract attribute value from an element
pub async fn extract_attribute(
    page: &Page,
    selectors: &[String],
    attribute_name: &str,
) -> Result<String> {
    let element = find_element_with_fallback(page, selectors)
        .await
        .context("Failed to find element for attribute extraction")?;

    let value = element
        .attribute(attribute_name)
        .await
        .context("Failed to extract attribute")?
        .unwrap_or_default();

    tracing::debug!("Extracted attribute '{}': {}", attribute_name, value);
    Ok(value)
}

/// Extract attributes from all matching elements
pub async fn extract_attributes_from_all(
    page: &Page,
    selectors: &[String],
    attribute_name: &str,
) -> Result<Vec<String>> {
    let elements = find_elements_with_fallback(page, selectors)
        .await
        .context("Failed to find elements for attribute extraction")?;

    let mut values = Vec::new();

    for element in elements {
        if let Ok(Some(value)) = element.attribute(attribute_name).await {
            values.push(value);
        }
    }

    tracing::debug!(
        "Extracted attribute '{}' from {} elements",
        attribute_name,
        values.len()
    );
    Ok(values)
}

/// Extract data-* attribute from an element
pub async fn extract_data_attribute(element: &Element, data_name: &str) -> Result<String> {
    let attr_name = format!("data-{}", data_name);
    let value = element
        .attribute(&attr_name)
        .await
        .context("Failed to extract data attribute")?
        .unwrap_or_default();

    Ok(value)
}

/// Extract multiple attributes from a single element
pub async fn extract_multiple_attributes(
    element: &Element,
    attribute_names: &[&str],
) -> Result<Vec<(String, String)>> {
    let mut results = Vec::new();

    for &attr_name in attribute_names {
        if let Ok(Some(value)) = element.attribute(attr_name).await {
            results.push((attr_name.to_string(), value));
        }
    }

    Ok(results)
}

/// Extract structured data from a list of items
/// Returns a vector of key-value pairs for each item
pub async fn extract_list_data(
    page: &Page,
    item_selectors: &[String],
    field_extractors: Vec<(&str, Vec<String>)>, // (field_name, selectors)
) -> Result<Vec<std::collections::HashMap<String, String>>> {
    let items = find_elements_with_fallback(page, item_selectors)
        .await
        .context("Failed to find list items")?;

    let mut results = Vec::new();

    for item in items {
        let mut item_data = std::collections::HashMap::new();

        for (field_name, field_selectors) in &field_extractors {
            // Try to find element within the item context
            for selector in field_selectors {
                if let Ok(element) = item.find_element(selector).await {
                    if let Ok(Some(text)) = element.inner_text().await {
                        item_data.insert(field_name.to_string(), text.trim().to_string());
                        break;
                    }
                }
            }
        }

        if !item_data.is_empty() {
            results.push(item_data);
        }
    }

    tracing::debug!("Extracted structured data from {} items", results.len());
    Ok(results)
}

/// Extract table data from the page
/// Returns rows as vectors of cell text
pub async fn extract_table_data(
    page: &Page,
    table_selectors: &[String],
    row_selector: &str,
    cell_selector: &str,
) -> Result<Vec<Vec<String>>> {
    let table = find_element_with_fallback(page, table_selectors)
        .await
        .context("Failed to find table")?;

    let rows = table
        .find_elements(row_selector)
        .await
        .context("Failed to find table rows")?;

    let mut table_data = Vec::new();

    for row in rows {
        let cells = row
            .find_elements(cell_selector)
            .await
            .context("Failed to find cells in row")?;

        let mut row_data = Vec::new();

        for cell in cells {
            if let Ok(Some(text)) = cell.inner_text().await {
                row_data.push(text.trim().to_string());
            }
        }

        if !row_data.is_empty() {
            table_data.push(row_data);
        }
    }

    tracing::debug!("Extracted table with {} rows", table_data.len());
    Ok(table_data)
}

/// Extract href links from all matching elements
pub async fn extract_links(page: &Page, selectors: &[String]) -> Result<Vec<String>> {
    extract_attributes_from_all(page, selectors, "href").await
}

/// Extract image sources from all matching elements
pub async fn extract_image_sources(page: &Page, selectors: &[String]) -> Result<Vec<String>> {
    extract_attributes_from_all(page, selectors, "src").await
}

/// Check if element has a specific class
pub async fn has_class(element: &Element, class_name: &str) -> bool {
    if let Ok(Some(classes)) = element.attribute("class").await {
        classes.split_whitespace().any(|c| c == class_name)
    } else {
        false
    }
}

/// Extract computed style property from an element
pub async fn extract_style_property(element: &Element, property_name: &str) -> Result<String> {
    let style = element
        .attribute("style")
        .await
        .context("Failed to get style attribute")?
        .unwrap_or_default();

    // Simple parsing of inline styles
    for declaration in style.split(';') {
        let parts: Vec<&str> = declaration.split(':').collect();
        if parts.len() == 2 && parts[0].trim() == property_name {
            return Ok(parts[1].trim().to_string());
        }
    }

    Ok(String::new())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_attribute_name_format() {
        let data_name = "song-id";
        let attr_name = format!("data-{}", data_name);
        assert_eq!(attr_name, "data-song-id");
    }

    #[test]
    fn test_field_extractors_structure() {
        let extractors: Vec<(&str, Vec<String>)> = vec![
            ("title", vec![".title".to_string(), "h1".to_string()]),
            ("artist", vec![".artist".to_string()]),
        ];
        assert_eq!(extractors.len(), 2);
        assert_eq!(extractors[0].0, "title");
    }

    #[test]
    fn test_empty_style_parsing() {
        let style = "";
        let parts: Vec<&str> = style.split(';').collect();
        assert!(parts.len() <= 1);
    }

    #[test]
    fn test_class_checking() {
        let classes = "btn btn-primary active";
        let has_active = classes.split_whitespace().any(|c| c == "active");
        assert!(has_active);
    }

    #[test]
    fn test_data_attribute_name_with_dashes() {
        let data_name = "song-duration-ms";
        let attr_name = format!("data-{}", data_name);
        assert_eq!(attr_name, "data-song-duration-ms");
    }

    #[test]
    fn test_data_attribute_name_simple() {
        let data_name = "id";
        let attr_name = format!("data-{}", data_name);
        assert_eq!(attr_name, "data-id");
    }

    #[test]
    fn test_style_parsing_single_property() {
        let style = "color: red";
        let parts: Vec<&str> = style.split(';').collect();
        assert_eq!(parts.len(), 1);

        let declaration: Vec<&str> = parts[0].split(':').collect();
        assert_eq!(declaration.len(), 2);
        assert_eq!(declaration[0].trim(), "color");
        assert_eq!(declaration[1].trim(), "red");
    }

    #[test]
    fn test_style_parsing_multiple_properties() {
        let style = "color: red; background: blue; font-size: 14px";
        let properties: Vec<&str> = style.split(';').collect();
        assert_eq!(properties.len(), 3);

        // Check first property
        let first: Vec<&str> = properties[0].split(':').collect();
        assert_eq!(first[0].trim(), "color");
        assert_eq!(first[1].trim(), "red");
    }

    #[test]
    fn test_style_parsing_with_spaces() {
        let style = "  color  :  red  ;  background  :  blue  ";
        let properties: Vec<&str> = style.split(';').collect();

        for prop in properties {
            if !prop.trim().is_empty() {
                let parts: Vec<&str> = prop.split(':').collect();
                if parts.len() == 2 {
                    assert!(!parts[0].trim().is_empty());
                    assert!(!parts[1].trim().is_empty());
                }
            }
        }
    }

    #[test]
    fn test_style_parsing_trailing_semicolon() {
        let style = "color: red;";
        let properties: Vec<&str> = style.split(';').collect();
        // Will have empty string after trailing semicolon
        assert!(!properties.is_empty());
    }

    #[test]
    fn test_class_checking_single_class() {
        let classes = "active";
        let has_active = classes.split_whitespace().any(|c| c == "active");
        assert!(has_active);
    }

    #[test]
    fn test_class_checking_not_found() {
        let classes = "btn btn-primary";
        let has_active = classes.split_whitespace().any(|c| c == "active");
        assert!(!has_active);
    }

    #[test]
    fn test_class_checking_empty() {
        let classes = "";
        let has_active = classes.split_whitespace().any(|c| c == "active");
        assert!(!has_active);
    }

    #[test]
    fn test_class_checking_with_extra_whitespace() {
        let classes = "  btn   btn-primary    active  ";
        let has_active = classes.split_whitespace().any(|c| c == "active");
        assert!(has_active);
    }

    #[test]
    fn test_field_extractors_empty() {
        let extractors: Vec<(&str, Vec<String>)> = vec![];
        assert_eq!(extractors.len(), 0);
    }

    #[test]
    fn test_field_extractors_single_selector() {
        let extractors: Vec<(&str, Vec<String>)> = vec![("title", vec![".title".to_string()])];
        assert_eq!(extractors.len(), 1);
        assert_eq!(extractors[0].1.len(), 1);
    }

    #[test]
    fn test_field_extractors_multiple_selectors() {
        let extractors: Vec<(&str, Vec<String>)> = vec![(
            "title",
            vec![
                ".title".to_string(),
                "h1".to_string(),
                "#song-title".to_string(),
                "[data-title]".to_string(),
            ],
        )];
        assert_eq!(extractors[0].1.len(), 4);
    }

    #[test]
    fn test_text_trimming_whitespace() {
        let text = "  Hello World  ";
        let trimmed = text.trim().to_string();
        assert_eq!(trimmed, "Hello World");
    }

    #[test]
    fn test_text_trimming_newlines() {
        let text = "\n\nHello World\n\n";
        let trimmed = text.trim().to_string();
        assert_eq!(trimmed, "Hello World");
    }

    #[test]
    fn test_text_trimming_tabs() {
        let text = "\t\tHello World\t\t";
        let trimmed = text.trim().to_string();
        assert_eq!(trimmed, "Hello World");
    }

    #[test]
    fn test_text_trimming_mixed() {
        let text = " \n\t Hello World \t\n ";
        let trimmed = text.trim().to_string();
        assert_eq!(trimmed, "Hello World");
    }

    #[test]
    fn test_empty_hashmap_creation() {
        let data = std::collections::HashMap::<String, String>::new();
        assert!(data.is_empty());
    }

    #[test]
    fn test_hashmap_with_data() {
        let mut data = std::collections::HashMap::new();
        data.insert("title".to_string(), "Song Title".to_string());
        data.insert("artist".to_string(), "Artist Name".to_string());
        assert_eq!(data.len(), 2);
        assert!(data.contains_key("title"));
    }

    #[test]
    fn test_attribute_tuple_vector() {
        let attributes = [
            ("href".to_string(), "https://example.com".to_string()),
            ("title".to_string(), "Example".to_string()),
        ];
        assert_eq!(attributes.len(), 2);
        assert_eq!(attributes[0].0, "href");
        assert_eq!(attributes[1].0, "title");
    }

    #[test]
    fn test_table_row_data_structure() {
        let row: Vec<String> = vec![
            "Cell 1".to_string(),
            "Cell 2".to_string(),
            "Cell 3".to_string(),
        ];
        assert_eq!(row.len(), 3);
    }

    #[test]
    fn test_table_data_structure() {
        let table: Vec<Vec<String>> = vec![
            vec!["Header 1".to_string(), "Header 2".to_string()],
            vec!["Data 1".to_string(), "Data 2".to_string()],
        ];
        assert_eq!(table.len(), 2);
        assert_eq!(table[0].len(), 2);
    }

    #[test]
    fn test_empty_table_data() {
        let table: Vec<Vec<String>> = vec![];
        assert!(table.is_empty());
    }

    #[test]
    fn test_style_property_extraction_logic() {
        let style = "color: red; background: blue";
        let property_name = "color";

        let mut found = String::new();
        for declaration in style.split(';') {
            let parts: Vec<&str> = declaration.split(':').collect();
            if parts.len() == 2 && parts[0].trim() == property_name {
                found = parts[1].trim().to_string();
                break;
            }
        }

        assert_eq!(found, "red");
    }

    #[test]
    fn test_style_property_not_found() {
        let style = "color: red; background: blue";
        let property_name = "font-size";

        let mut found = String::new();
        for declaration in style.split(';') {
            let parts: Vec<&str> = declaration.split(':').collect();
            if parts.len() == 2 && parts[0].trim() == property_name {
                found = parts[1].trim().to_string();
                break;
            }
        }

        assert_eq!(found, "");
    }

    #[test]
    fn test_style_property_with_url_value() {
        let style = "background: url(https://example.com/image.jpg)";
        let parts: Vec<&str> = style.split(':').collect();
        assert_eq!(parts.len(), 3); // Split by all colons including in URL
    }
}
