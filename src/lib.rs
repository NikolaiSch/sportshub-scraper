pub mod constants;
pub mod date_parser;
pub mod db;
pub mod query_selectors;
pub mod scrape_utils;
pub mod web_server_utils;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
