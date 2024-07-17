use serde::{Deserialize, Serialize};
use warp::Filter;
use warp::cors::Cors;

mod formatter;
mod processor;
mod scraper;

#[derive(Deserialize)]
struct ScrapeRequest {
    urls: Vec<String>,
}

#[derive(Serialize)]
struct ScrapeResponse {
    items: Vec<Item>,
}

#[derive(Serialize)]
struct Item {
    link: String,
    name: String,
    price: String,
    review_count: u32,
    sale_count: u32,
}

#[tokio::main]
async fn main() {
    let scraper_route = warp::path("scrape")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_scrape_request);

    let seq_scraper_route = warp::path("seq-scrape")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_seq_scrape_request);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Content-Type"])
        .allow_methods(vec!["GET", "POST"]);

    let routes = scraper_route.or(seq_scraper_route).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_scrape_request(req: ScrapeRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let raw_data = scraper::scrape_multiple_pages(req.urls).await;
    let processed_data = processor::process_data(raw_data);

    let items: Vec<Item> = processed_data
        .into_iter()
        .map(|data| Item {
            link: data.link,
            name: data.name,
            price: data.price,
            review_count: data.review_count,
            sale_count: data.sale_count,
        })
        .collect();

    let response = ScrapeResponse { items };
    Ok(warp::reply::json(&response))
}

async fn handle_seq_scrape_request(req: ScrapeRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let raw_data = scraper::seq_scrape_multiple_pages(req.urls);
    let processed_data = processor::process_data(raw_data);

    let items: Vec<Item> = processed_data
        .into_iter()
        .map(|data| Item {
            link: data.link,
            name: data.name,
            price: data.price,
            review_count: data.review_count,
            sale_count: data.sale_count,
        })
        .collect();

    let response = ScrapeResponse { items };
    Ok(warp::reply::json(&response))
}
