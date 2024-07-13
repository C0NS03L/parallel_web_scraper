use anyhow::Result;
use scraper::{Html, Selector};

pub struct Item {
    pub name: String,
    pub link: String,
    pub price: String,
    pub reviews: Vec<String>,
}

pub fn process_data(data: Vec<String>) -> Vec<Item> {
    data.iter()
        .filter_map(|html| process_html(html).ok())
        .flatten()
        .collect()
}

fn process_html(html: &str) -> Result<Vec<Item>> {
    let document = Html::parse_document(html);

    // Adjust these selectors based on the actual HTML structure of the Shopee site
    let item_selector = Selector::parse("div.buTCk").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let name_selector = Selector::parse("a").unwrap();
    let price_selector = Selector::parse("span.ooOxS").unwrap();

    let mut items = Vec::new();

    for item in document.select(&item_selector) {
        let name = item
            .select(&name_selector)
            .next()
            .and_then(|n| n.value().attr("title"))
            .unwrap_or("")
            .to_string();

        let link = item
            .select(&link_selector)
            .next()
            .and_then(|l| l.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let price = item
            .select(&price_selector)
            .next()
            .and_then(|p| p.text().next())
            .unwrap_or("")
            .to_string();

        if !name.is_empty() && !link.is_empty() && !price.is_empty() {
            let reviews = scrape_reviews(&link)?;
            items.push(Item {
                name,
                link,
                price,
                reviews,
            });
        }
    }

    Ok(items)
}

fn scrape_reviews(product_url: &str) -> Result<Vec<String>> {
    //TODO: Actual reviews :C
    Ok(vec![
        "Review 1".to_string(),
        "Review 2".to_string(),
        "Review 3".to_string(),
    ])
}
