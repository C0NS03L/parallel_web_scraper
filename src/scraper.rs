use anyhow::{anyhow, Result};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::time::Duration;

pub fn scrape_page(url: String) -> Result<String> {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .map_err(|e| anyhow!("Failed to launch browser: {}", e))?,
    )?;

    let tab = browser
        .new_tab()
        .map_err(|e| anyhow!("Failed to open new tab: {}", e))?;
    tab.navigate_to(&url)
        .map_err(|e| anyhow!("Failed to navigate to URL: {}", e))?;
    tab.wait_until_navigated()
        .map_err(|e| anyhow!("Failed to wait for navigation: {}", e))?;

    let content = tab
        .find_element("body")
        .map_err(|e| anyhow!("Failed to find body element: {}", e))?
        .get_inner_text()
        .map_err(|e| anyhow!("Failed to get inner HTML: {}", e))?;

    Ok(content)
}

pub async fn scrape_multiple_pages(urls: Vec<String>) -> Vec<String> {
    let mut results = Vec::new();
    for url in urls {
        let mut retries = 3;
        while retries > 0 {
            let url_clone = url.clone();
            match tokio::task::spawn_blocking(move || scrape_page(url_clone)).await {
                Ok(Ok(content)) => {
                    println!("Scraped content: {}", content);
                    results.push(content);
                    break;
                }
                Ok(Err(e)) => {
                    eprintln!("Error scraping page (retries left: {}): {:?}", retries, e);
                    retries -= 1;
                }
                Err(e) => {
                    eprintln!("Task panicked (retries left: {}): {:?}", retries, e);
                    retries -= 1;
                }
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
    results
}
