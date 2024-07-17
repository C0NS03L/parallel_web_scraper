mod formatter;
mod processor;
mod scraper;

use tokio;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://www.lazada.co.th/catalog/?spm=a2o4m.homepage.search.d_go&q=GPU".to_string(),
        "https://www.lazada.co.th/catalog/?spm=a2o4m.homepage.search.d_go&q=GPU&page=2".to_string(),
    ];
    let seq_urls = urls.clone();

    use std::time::Instant;
    let now = Instant::now();
    // Scrape pages
    let raw_data = scraper::scrape_multiple_pages(urls).await;

    // Process data
    let processed_data = processor::process_data(raw_data);

    // Format data
    let formatted_output = formatter::format_data(processed_data);

    // let scrape_seller_score = processor::scrape_review_count("https://www.lazada.co.th/products/vga-vga-amd-hd-6350-512mb-ddr3-low-profile-i356954419-s699390921.html");
    // println!("{}", scrape_seller_score);
    println!("{}", formatted_output);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    //----------------------------------------------------------------------------------------------------------------- SEQUENTIAL SCRAPER!
    let now_seq = Instant::now();
    let raw_data_seq = scraper::seq_scrape_multiple_pages(seq_urls);

    let processed_date_seq = processor::process_data(raw_data_seq);

    let formatted_output_seq = formatter::format_data(processed_date_seq);

    println!("{}", formatted_output_seq);

    let elapsed_seq = now_seq.elapsed();
    println!("Elapsed: {:.2?}", elapsed_seq);
}
