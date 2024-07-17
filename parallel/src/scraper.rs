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

    // std::thread::sleep(Duration::from_secs(10));

    let content = tab
        .find_element("body")
        .map_err(|e| anyhow!("Failed to find body element: {}", e))?
        .get_content()
        .map_err(|e| anyhow!("Failed to get body content: {}", e))?;

    Ok(content)
}

use std::sync::{Arc, Mutex};

pub async fn scrape_multiple_pages(urls: Vec<String>) -> Vec<String> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut tasks = Vec::new();

    for url in urls {
        let results_clone = Arc::clone(&results);
        tasks.push(tokio::spawn(async move {
            let mut retries = 3;
            while retries > 0 {
                match scrape_page(url.clone()) {
                    Ok(content) => {
                        // println!("Scraped content: {}", content);
                        results_clone.lock().unwrap().push(content);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Error scraping page (retries left: {}): {:?}", retries, e);
                        retries -= 1;
                    }
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }));
    }

    // Wait for all tasks to complete
    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("Task panicked: {:?}", e);
        }
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

//allow dead code
#[allow(dead_code)]
pub fn seq_scrape_multiple_pages(urls: Vec<String>) -> Vec<String> {
    let mut results = Vec::new();
    for url in urls {
        match scrape_page(url.clone()) {
            Ok(content) => results.push(content),
            Err(e) => eprintln!("Error scraping page: {:?}", e),
        }
    }
    results
}
