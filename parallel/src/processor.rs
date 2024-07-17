use anyhow::Result;
use rayon::prelude::*;
use scraper::{Html, Selector};
pub struct Item {
    pub name: String,
    pub link: String,
    pub price: String,
    pub review_count: u32,
    pub sale_count: u32,
}

pub fn process_data(data: Vec<String>) -> Vec<Item> {
    let items: Vec<Item> = data
        .par_iter()
        .filter_map(|html| process_html(html).ok())
        .flatten()
        .collect();
    items
}

fn process_html(html: &str) -> Result<Vec<Item>> {
    let document = Html::parse_document(html);

    let item_selector = Selector::parse("div.buTCk").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let name_selector = Selector::parse("a").unwrap();
    let price_selector = Selector::parse("span.ooOxS").unwrap();
    let review_selector = Selector::parse("span.qzqFw").unwrap();
    let sale_count = Selector::parse("span._1cEkb").unwrap();

    let items: Vec<Item> = document
        .select(&item_selector)
        .filter_map(|item| {
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

            let review_count = item
                .select(&review_selector)
                .next()
                .and_then(|r| r.text().next())
                .unwrap_or("")
                .trim_matches(|c| c == '(' || c == ')')
                .parse::<u32>()
                .unwrap_or(0);

            let sale_count = item
                .select(&sale_count)
                .next()
                .and_then(|s| s.text().next())
                .unwrap_or("")
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);

            if !name.is_empty() && !link.is_empty() && !price.is_empty() {
                Some(Item {
                    name,
                    link,
                    price,
                    review_count,
                    sale_count,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(items)
}

// fn scrape_review_counts_parallel(mut items: Vec<Item>) -> Vec<Item> {
//     items.par_iter_mut().for_each(|item| {
//         item.review_count = scrape_review_count(&item.link).unwrap_or(None);
//     });
//     items
// }

// pub(crate) fn scrape_review_count(product_url: &str) -> Result<Option<String>> {
//     let review_scraper = scrape_page(product_url.to_owned())?;
//     let review_document = Html::parse_document(&review_scraper);

//     let review_selector =
//         Selector::parse("div.pdp-review-summary a.pdp-review-summary__link").unwrap();

//     if let Some(review_element) = review_document.select(&review_selector).next() {
//         let review_count = review_element
//             .text()
//             .next()
//             .map(|s| s.trim().to_string())
//             .filter(|s| !s.is_empty());

//         Ok(review_count)
//     } else {
//         println!("Review count information not found");
//         Ok(None)
//     }
// }
