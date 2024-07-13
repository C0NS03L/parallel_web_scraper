mod formatter;
mod processor;
mod scraper;

use tokio;

#[tokio::main]
async fn main() {
    let urls = vec!["https://shopee.co.th/".to_string()];
    // Scrape pages
    let raw_data = scraper::scrape_multiple_pages(urls).await;

    // Process data
    let processed_data = processor::process_data(raw_data);

    // Format data
    let formatted_output = formatter::format_data(processed_data);

    println!("{}", formatted_output);
}
