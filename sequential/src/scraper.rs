use anyhow::{anyhow, Result};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use serde_json::Value;

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

    let body = tab
        .find_element("body")
        .map_err(|e| anyhow!("Failed to find body element: {}", e))?;

    let js_script = r#"
        function() {
            return document.body.innerHTML;
        }
    "#;

    let result = body
        .call_js_fn(js_script, vec![], false)
        .map_err(|e| anyhow!("Failed to execute JS script: {}", e))?
        .value;

    let inner_html = match result {
        Some(Value::String(s)) => s,
        _ => return Err(anyhow!("Failed to convert result to string")),
    };

    Ok(inner_html)
}

pub fn scrape_multiple_pages(urls: Vec<String>) -> Vec<String> {
    let mut results = Vec::new();
    for url in urls {
        match scrape_page(url.clone()) {
            Ok(content) => results.push(content),
            Err(e) => eprintln!("Error scraping page: {:?}", e),
        }
    }
    results
}
