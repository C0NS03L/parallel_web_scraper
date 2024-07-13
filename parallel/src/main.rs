mod formatter;
mod processor;
mod scraper;

use tokio;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://www.lazada.co.th/catalog/?spm=a2o4m.homepage.search.d_go&q=GPU".to_string(),
        "https://www.lazada.co.th/catalog/?spm=a2o4m.homepage.search.d_go&q=GPU&page=2".to_string(),
        "https://www.lazada.co.th/catalog/?spm=a2o4m.homepage.search.d_go&q=GPU&page=3".to_string(),
        "https://www.lazada.co.th/catalog/?spm=a2o4m.homepage.search.d_go&q=GPU&page=4".to_string(),
        "https://www.lazada.co.th/catalog/?spm=a2o4m.homepage.search.d_go&q=GPU&page=5".to_string(),
    ];

    use std::time::Instant;
    let now = Instant::now();
    // Scrape pages
    let raw_data = scraper::scrape_multiple_pages(urls).await;

    // Process data
    let processed_data = processor::process_data(raw_data);

    // Format data
    let formatted_output = formatter::format_data(processed_data);

    println!("{}", formatted_output);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
