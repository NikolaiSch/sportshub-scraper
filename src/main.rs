//! This executable is used to scrape the links from reddit.sportshub.fan
//! and save them to database.  It also checks the stream links and saves them
//! to database.

const OPEN_TABS: usize = 10;

pub mod db;
pub mod models;
pub mod schema;

extern crate diesel;

use headless_chrome::Browser;
use scraper::scrape;
use std::ffi::OsStr;

fn main() {
    // we use it headful for now, because headless chrome doesn't support extensions
    let browser = Browser::new({
        headless_chrome::LaunchOptions {
            headless: false,
            sandbox: true,
            ignore_certificate_errors: true,
            extensions: vec![OsStr::new("./chrome-ext/adblock")],
            ..Default::default()
        }
    })
    .unwrap();

    let mut conn = db::establish_connection();

    let tab = browser.new_tab().unwrap();

    // we get to the page with all the links for upcoming games
    // this will scrape ~500 games and save them to database
    // takes roughly 1 second, but it's not a problem, because
    // we do it only once a day
    scrape::today_games(&tab, &mut conn).unwrap();

    // we get all the links from database that don't have stream links
    // and we check them in parallel
    // my 8gb ram m1 macbook air can handle 10 tabs relatively easily
    // takes ~27 seconds to scan everything
    // however can improve by using a shared queue instead of splitting it
    // so... TODO!
    scrape::check_all_links(&browser, &mut conn, OPEN_TABS).unwrap();

    // we close all the tabs because otherwise it shows an error when program
    // finishes
    for t in (*browser.get_tabs().as_ref().lock().unwrap()).iter() {
        t.close(true).unwrap();
    }
}
