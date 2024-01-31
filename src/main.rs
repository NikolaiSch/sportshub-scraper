//! This executable is used to scrape the links from reddit.sportshub.fan
//! and save them to database.  It also checks the stream links and saves them
//! to database.

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

const OPEN_TABS: usize = 10;

pub mod db;
pub mod models;
pub mod schema;

use scraper::scrape_utils::start_scraping;

fn main() {
    // This should be the only place it can panic
    start_scraping(OPEN_TABS).unwrap();
}
