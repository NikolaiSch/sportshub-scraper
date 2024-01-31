# This is a scraper for sportshub, which also gets active links from the website
## Very much in progress
- Todo
   - Multiple tabs at same time
   - Batch SQLite writes
   - Fix advertisement causing delay
   - Add RustDoc comments
     
- To scrape, use `cargo run --release --bin scraper` in the root directory of the project
  - This will output to 'sports.db' in the root directory
  - This uses chromium with an adblocker to scrape the website

- To run the server, use `cargo run --release --bin web_server` in the root directory of the project
  - This will start a server on port 8080
  - This uses the 'sports.db' file in the root directory
