# Parallel Lazada Scraper with Rust

A parallel webscraper using rust and displaying the data with vuejs. This shows the performance comparisons between Sequential and Parallel.
This project contains 2 sub-folders
-parallel: contains the backend code to do data scraping and data processing
-parallel_scraper_front: a frontend web app to communicate with the backend.

This project also provides a way to browse (even though a slow way), with out any junks on the page. Kind of like a lightweight lazada

## Getting Start

In order to run the backend use `cargo run`
\*Make sure to have chrome installed since this project uses `headless_chrome`

For the frontend make sure `bun` are installed and you can run the project by
`bun dev`

## Results

Here's the runtime for a query `GPU` 1 Page

### Sequential

| Attempt | #1     | #2     | #3     |
| ------- | ------ | ------ | ------ |
| Seconds | 5860ms | 5523ms | 5290ms |

#### Parallel

| Attempt | #1     | #2     | #3     |
| ------- | ------ | ------ | ------ |
| Seconds | 6418ms | 5697ms | 6062ms |

Here's the runtime for a query `GPU` 5 Pages

#### Sequential

| Attempt | #1      | #2      | #3      |
| ------- | ------- | ------- | ------- |
| Seconds | 29474ms | 24598ms | 26847ms |

#### Parallel

| Attempt | #1      | #2      | #3      |
| ------- | ------- | ------- | ------- |
| Seconds | 14398ms | 16541ms | 17151ms |

This shows that parallel are faster at scraping multiple Pages. But for single page, the result are pretty inconclusive!
