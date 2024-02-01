//! This executable is used to scrape the links from reddit.sportshub.fan
//! and save them to database.  It also checks the stream links and saves them
//! to database.

use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
    thread,
};

use anyhow::anyhow;
use db::{models, schema};
use diesel::{ExpressionMethods, RunQueryDsl, SqliteConnection};
use headless_chrome::{Browser, Tab};

use crate::{db, query_selectors};

/// This function scrapes all the games from the home page and saves them to database.
/// It takes roughly 1 second to scrape ~500 games.
/// This function should not panic
///
/// # Arguments
/// *tab* - is the tab that we use to navigate to the page and scrape the games, we use headless_chrome tabs.  
/// *conn* - is the connection to the database, we use diesel to save the games to database.
pub fn today_games(tab: &Tab, conn: &mut SqliteConnection) -> Result<(), anyhow::Error> {
    // we navigate to the page and wait until the table showing links is loaded
    tab.navigate_to("https://reddit.sportshub.fan/")?
        .wait_for_element(".list-events")?;

    // we get the html of the table and remove all the tabs and newlines
    let html = tab
        .find_element(".list-events")?
        .get_content()?
        .replace(['\t', '\n'], "");

    // create the parser using tl
    let dom = tl::parse(&html, tl::ParserOptions::default())?;
    let parser = dom.parser();

    // we get all the games by checking the wrapper class
    let dom_games = dom.get_elements_by_class_name("wrap-events-item");

    // we iterate over all the games and parse them
    for game in dom_games {
        if let Some(x) = game.get(parser) {
            parse_game(conn, &x.inner_html(parser).to_string())?;
        }
    }

    Ok(())
}

/// This function parses a single game and saves it to database.
/// It takes roughly 400µs to parse a single game. (± 100µs)
///
/// This should never panic
pub fn parse_game(conn: &mut SqliteConnection, html: &str) -> Result<(), anyhow::Error> {
    // creating a new parser for each game is not the best idea, but it's not a problem
    // because it takes roughly 400µs to parse a single game
    let dom = tl::parse(html, tl::ParserOptions::default())?;
    let parser = dom.parser();

    // we get the url of the game
    // since there are no other links in the div
    // format is: <a href="https://reddit3.sportshub.stream/event/tоttеnhаm_brеntfоrd_187734226/">

    let url = query_selectors::get_url_from_dom(&dom, parser)?;

    // we get the name of the game
    // format is: <span class="mr-5">HomeTeam - AwayTeam</span>
    let name = query_selectors::get_game_name_from_dom(&dom, parser)?;

    let teams: Vec<&str> = name.split('–').collect();
    let home = teams.first().unwrap_or(&"???").trim().to_string();
    let away = teams.last().unwrap_or(&"???").trim().to_string();

    // we get the info of the game, such as time, league, country
    // format is: <span class="evdesc event-desc">League / Time</span>
    let info = query_selectors::get_info_from_dom(&dom, parser)?;

    // we split the info into league and time
    let mut info_parsed = info.split('/');
    let league = &info_parsed.next().unwrap_or("Unknown").trim().to_string();
    let time = chrono::NaiveDateTime::from_timestamp_opt(
        crate::date_parser::date_string_to_timestamp(info_parsed.next().unwrap_or("Unknown"))?,
        0,
    );

    // we get the country of the game
    // format is: <i class="icon-competitions" style="background-image: url(https://reddit.sportshub.fan/img/competitions/england.svg);"></i>
    let country = query_selectors::get_country_from_dom(&dom, parser)?;

    // we create a new stream and save it to database
    // we leave stream_link empty for now
    let new_stream = models::StreamNew {
        home: &home.trim(),
        away: &away.trim(),
        start_time: time.unwrap(),
        league,
        country: &country.trim(),
        url: &url.trim(),
        stream_link: "",
    };

    db::helpers::create_stream(conn, &new_stream)?;

    Ok(())
}

/// This function scrapes the stream links from the game page and saves them to database.
/// It takes roughly 1 second to scrape a single page.
///
/// # Arguments
/// *tab* - is the tab that we use to navigate to the page and scrape the links, we use headless_chrome tabs.  
/// *conn* - is the connection to the database, we use diesel to save the links to database.  
/// *url* - is the url of the game page that we get from database.
pub fn url_to_links(
    tab: &Tab,
    conn: &mut SqliteConnection,
    url: &str,
) -> Result<(), anyhow::Error> {
    tab.navigate_to(url)?.wait_for_element("#content-event")?;

    // they encode url, so we need to decode it
    let u = urlencoding::decode(url).unwrap();

    // we wait until the table showing links is loaded
    // then we get all link elements in the table
    // xpath seems slower but they changed the classname overnight
    // so xpath is more reliable
    let elements = tab.find_elements_by_xpath("//*[@class=\"lnktbj\"]/tbody/tr/td[6]/a");

    // if there are no links, we return
    if elements.is_err() {
        // println!("No links for {}", u);
        return Ok(());
    };

    // we get the links from the elements
    // checking if they have "//" in them because some of them are just text
    let stream_links: Vec<String> = elements
        .unwrap()
        .into_iter()
        .map(|e| e.get_attributes().unwrap().unwrap().get(1).unwrap().clone())
        .collect();

    let joined_links = stream_links.join(",");

    // we save the links to database
    diesel::update(schema::stream::table)
        .set(schema::stream::stream_link.eq(joined_links))
        .filter(schema::stream::url.eq(u))
        .execute(conn)?;

    Ok(())
}

/// This function checks all the links in database and saves them to database.
/// It takes roughly 27 seconds to check all the links.
/// (My 8gb ram m1 macbook air with a 90mbps internet connection can handle 10 tabs relatively easily)
/// It can be improved by using a shared queue instead of splitting it.
pub fn check_all_links(
    browser: &Browser,
    conn: &mut SqliteConnection,
    tabs_count: usize,
) -> Result<(), anyhow::Error> {
    // we get all the streams from database that have no links
    // wrap it in an arc to share it between threads
    let all_streams = Arc::new(db::helpers::get_empty_streams(conn)?);

    // we split the streams into chunks and create a thread for each chunk
    let chunked_streams: Vec<&[models::Stream]> =
        all_streams.chunks(all_streams.len() / tabs_count).collect();

    let length = all_streams.len();

    let mut tabs: Vec<Arc<Tab>> = vec![];
    let mut threads = vec![];
    let completed_mutex = Arc::new(Mutex::new(0));

    let time_start = std::time::Instant::now();

    // for each tab count we create a new tab and a new thread
    for tab_num in 0..tabs_count {
        // we create a new tab and push it to the tabs vector
        let tab = browser.new_tab().unwrap();
        tabs.push(tab.clone());

        // we get the streams from the chunked streams and turn it to a vec
        let mut streams = chunked_streams
            .get(tab_num)
            .ok_or(anyhow!("invalid chunked_stream index"))?
            .to_vec()
            .clone();

        let completed = completed_mutex.clone();

        threads.push(thread::spawn(move || {
            // sqlite should be able to handle 10 connections at once
            let mut conn = db::helpers::establish_connection().unwrap();

            // we iterate over all the streams and check them
            while let Some(stream) = streams.pop() {
                check_link(tab.clone().borrow_mut(), &mut conn, &stream.url).unwrap();
                // we print the progress
                let mut completed_count = completed
                    .lock()
                    .expect("mutex is already opened by current thread");
                *completed_count += 1;
                println!("{} / {}", completed_count, length);
            }
        }));
    }

    // we wait for all the threads to finish
    for t in threads {
        if t.join().is_err() {
            return Err(anyhow!("thread panicked"));
        }
    }

    let time_end = std::time::Instant::now();

    println!(
        "Time elapsed to scan all games: {:?}",
        time_end - time_start
    );

    Ok(())
}

pub fn check_link(
    tab: &mut Arc<Tab>,
    conn: &mut SqliteConnection,
    link: &str,
) -> Result<(), anyhow::Error> {
    url_to_links(tab.borrow_mut(), conn.borrow_mut(), link).unwrap();

    Ok(())
}

pub fn start_scraping(open_tabs: usize) -> Result<(), anyhow::Error> {
    // realised we didnt need adblocker when headless
    let browser = Browser::new({
        headless_chrome::LaunchOptions {
            headless: true,
            sandbox: true,
            ignore_certificate_errors: true,
            ..Default::default()
        }
    })?;

    let mut conn = db::helpers::establish_connection()?;

    let tab = browser.new_tab()?;

    // we get to the page with all the links for upcoming games
    // this will scrape ~500 games and save them to database
    // takes roughly 1 second, but it's not a problem, because
    // we do it only once a day
    today_games(&tab, &mut conn)?;

    // we get all the links from database that don't have stream links
    // and we check them in parallel
    // my 8gb ram m1 macbook air can handle 10 tabs relatively easily
    // takes ~27 seconds to scan everything
    // however can improve by using a shared queue instead of splitting it
    // so... TODO!
    check_all_links(&browser, &mut conn, open_tabs)?;

    // we close all the tabs because otherwise it shows an error when program
    // finishes
    for t in (*browser.get_tabs().as_ref().lock().unwrap()).iter() {
        t.close(true)?;
    }

    Ok(())
}
