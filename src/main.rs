pub mod db;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
use std::error::Error;
use std::ffi::OsStr;

use axum::routing::get;
use axum::routing::post;
use axum::Router;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;
use headless_chrome::Browser;
use headless_chrome::Tab;

fn main() {
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

    today_games(&tab, &mut conn).unwrap();
    url_to_links(&tab,"https://reddit3.sportshub.stream/event/s%D0%B0ud%D1%96_%D0%B0r%D0%B0b%D1%96%D0%B0_s%D0%B0u_s%D0%BEuth_k%D0%BEr%D0%B5%D0%B0_k%D0%BEr_189842032/", &mut conn).unwrap();

    check_links(&tab, &mut conn).unwrap();

    for t in (*browser.get_tabs().as_ref().lock().unwrap()).iter() {
        t.close(true).unwrap();
    }
}

#[derive(Debug)]
struct Game {
    url: String,
    name: String,
    time: String,
    league: String,
    country: String,
}

fn today_games(tab: &Tab, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    tab.navigate_to("https://reddit.sportshub.fan/")?
        .wait_for_element(".list-events")?;

    let html = tab
        .find_element(".list-events")?
        .get_content()?
        .replace("\t", "")
        .replace("\n", "");

    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    let dom_games = dom.get_elements_by_class_name("wrap-events-item");

    for game in dom_games {
        parse_game(
            conn,
            &game.get(parser).unwrap().inner_html(parser).to_string(),
        );
    }

    Ok(())
}

fn parse_game(conn: &mut SqliteConnection, html: &str) -> Game {
    let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    let url = dom
        .query_selector("a")
        .unwrap()
        .next()
        .unwrap()
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap()
        .attributes()
        .get("href")
        .unwrap()
        .unwrap()
        .as_utf8_str()
        .to_string();

    let name = dom
        .query_selector("span.mr-5")
        .unwrap()
        .next()
        .unwrap()
        .get(parser)
        .unwrap()
        .inner_text(parser)
        .to_string();

    let info = dom
        .query_selector("span.evdesc.event-desc")
        .unwrap()
        .next()
        .unwrap()
        .get(parser)
        .unwrap()
        .inner_text(parser)
        .to_string();

    let country = dom
        .query_selector("i.icon-competitions")
        .unwrap()
        .next()
        .unwrap()
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap()
        .attributes()
        .get("style")
        .unwrap()
        .unwrap()
        .as_utf8_str()
        .split("/")
        .last()
        .unwrap()
        .replace(");", "")
        .replace(".svg", "");

    let mut info_parsed = info.split("/");

    let league = &info_parsed.next().unwrap().to_string();
    let time = info_parsed.next().unwrap().to_string();

    let teams: Vec<&str> = name.split("–").collect();
    let home = teams.first().unwrap().trim().to_string();
    let away = teams.last().unwrap().trim().to_string();

    let new_stream = models::StreamNew {
        home: &home,
        away: &away,
        start_time: &time,
        league: &league,
        country: &country,
        url: &url,
        stream_link: "",
    };

    db::create_stream(conn, &new_stream);

    Game {
        url,
        name,
        time,
        league: league.clone(),
        country,
    }
}

fn url_to_links(tab: &Tab, url: &str, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    tab.navigate_to(url)?.wait_for_element("div")?;

    let u = urlencoding::decode(url).unwrap();

    let elements = tab.find_elements("#links_block table a");

    if elements.is_err() {
        return Ok(());
    };
    let stream_links: Vec<String> = elements
        .unwrap()
        .into_iter()
        .map(|e| e.get_attributes().unwrap().unwrap().get(1).unwrap().clone())
        .filter(|e| e.contains("//"))
        .collect();

    diesel::update(schema::stream::table)
        .set(schema::stream::stream_link.eq(stream_links.join(",")))
        .filter(schema::stream::url.eq(u))
        .execute(conn)
        .unwrap();
    Ok(())
}

fn check_links(tab: &Tab, conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    let streams = db::get_empty_streams(conn);

    for stream in streams {
        if stream.stream_link == "" {
            url_to_links(tab, &stream.url, conn)?;
        }
    }

    Ok(())
}
