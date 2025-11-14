// Data extraction utilities
// Helper functions for extracting data from web pages

use chromiumoxide::Page;
use chromiumoxide::element::Element;
use anyhow::{Result, Context};

use super::automation::find_element_with_fallback;
use super::automation::find_elements_with_fallback;

/// Extract text content from an element using selector fallbacks
pub async fn extract_text(
    page: &Page,
    selectors: &[String],
) -> Result<String> {
    let element = find_element_with_fallback(page, selectors).await
        .context("Failed to find element for text extraction")?;

    let text = element.inner_text().await
        .context("Failed to extract text from element")?
        .unwrap_or_default();

    tracing::debug!("Extracted text: {} characters", text.len());
    Ok(text.trim().to_string())
}

/// Extract text from all matching elements
pub async fn extract_text_from_all(
    page: &Page,
    selectors: &[String],
) -> Result<Vec<String>> {
    let elements = find_elements_with_fallback(page, selectors).await
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
    let element = find_element_with_fallback(page, selectors).await
        .context("Failed to find element for attribute extraction")?;

    let value = element.attribute(attribute_name).await
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
    let elements = find_elements_with_fallback(page, selectors).await
        .context("Failed to find elements for attribute extraction")?;

    let mut values = Vec::new();

    for element in elements {
        if let Ok(Some(value)) = element.attribute(attribute_name).await {
            values.push(value);
        }
    }

    tracing::debug!("Extracted attribute '{}' from {} elements", attribute_name, values.len());
    Ok(values)
}

/// Extract data-* attribute from an element
pub async fn extract_data_attribute(
    element: &Element,
    data_name: &str,
) -> Result<String> {
    let attr_name = format!("data-{}", data_name);
    let value = element.attribute(&attr_name).await
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
    let items = find_elements_with_fallback(page, item_selectors).await
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
    let table = find_element_with_fallback(page, table_selectors).await
        .context("Failed to find table")?;

    let rows = table.find_elements(row_selector).await
        .context("Failed to find table rows")?;

    let mut table_data = Vec::new();

    for row in rows {
        let cells = row.find_elements(cell_selector).await
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
pub async fn extract_links(
    page: &Page,
    selectors: &[String],
) -> Result<Vec<String>> {
    extract_attributes_from_all(page, selectors, "href").await
}

/// Extract image sources from all matching elements
pub async fn extract_image_sources(
    page: &Page,
    selectors: &[String],
) -> Result<Vec<String>> {
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
pub async fn extract_style_property(
    element: &Element,
    property_name: &str,
) -> Result<String> {
    let style = element.attribute("style").await
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
}
