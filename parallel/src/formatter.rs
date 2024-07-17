use crate::processor::Item;
use serde_json::json;

#[allow(dead_code)]
pub fn format_data(items: Vec<Item>) -> String {
    let json_items: Vec<serde_json::Value> = items
        .iter()
        .map(|item| {
            json!({
                "name": item.name,
                "link": item.link.to_owned().split_off(2), // remove beginning "//"
                "price": item.price,
                "review_count": item.review_count,
                "sale_count": item.sale_count,
            })
        })
        .collect();

    let json_output = json!({ "items": json_items });
    serde_json::to_string_pretty(&json_output).unwrap_or_else(|_| "[]".to_string())
}
