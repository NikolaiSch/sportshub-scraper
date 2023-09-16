#[macro_use]
extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;

use std::error::Error;

use headless_chrome::Browser;

#[derive(Debug)]
struct Game {
    url: String,
    name: String,
    time: String,
    league: String,
    country: String,
}

fn today_games() -> Result<(), Box<dyn Error>> {
    let browser = Browser::new({
        headless_chrome::LaunchOptions {
            headless: false,
            sandbox: true,
            ignore_certificate_errors: true,
            ..Default::default()
        }
    })?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://reddit.sportshub.fan/")?;

    let html = tab
        .find_element(".list-events")?
        .get_content()?
        .replace("\t", "")
        .replace("\n", "");

    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    let dom_games = dom.get_elements_by_class_name("wrap-events-item");

    for game in dom_games {
        parse_game(&game.get(parser).unwrap().inner_html(parser).to_string());
    }

    Ok(())
}

fn parse_game(html: &str) -> Game {
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

    let time = info.clone().split("/").nth(1).unwrap().to_string();
    let league = info.clone().split("/").nth(0).unwrap().to_string();

    // println!("{:?}", name);
    // let home = name.split("-").nth(0).unwrap().to_string();
    // let away = name.split("-").nth(1).unwrap().to_string();

    Game {
        url,
        name,
        time,
        league,
        country,
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    dbg!(today_games()?);

    Ok(())
}
