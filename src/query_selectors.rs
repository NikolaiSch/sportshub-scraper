use std::error::Error;

use tl::Parser;
use tl::VDom;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomParseError {
    #[error("No tag found")]
    NotFound,
    #[error("No attribute found")]
    NoAttributeFound,
    #[error("unknown Dom Parse error")]
    Unknown,
}

pub fn get_url_from_dom(dom: &VDom<'_>, parser: &Parser<'_>) -> Result<String, anyhow::Error> {
    let q = dom
        .query_selector("a")
        .ok_or(DomParseError::NotFound)?
        .next()
        .ok_or(DomParseError::NotFound)?
        .get(parser)
        .ok_or(DomParseError::Unknown)?
        .as_tag()
        .ok_or(DomParseError::NoAttributeFound)?
        .attributes()
        .get("href")
        .ok_or(DomParseError::NoAttributeFound)?
        .ok_or(DomParseError::NoAttributeFound)?
        .as_utf8_str()
        .to_string();

    Ok(q)
}

pub fn get_game_name_from_dom(
    dom: &VDom<'_>,
    parser: &Parser<'_>,
) -> Result<String, anyhow::Error> {
    let q = dom
        .query_selector("span.mr-5")
        .ok_or(DomParseError::NotFound)?
        .next()
        .ok_or(DomParseError::NotFound)?
        .get(parser)
        .ok_or(DomParseError::Unknown)?
        .inner_text(parser)
        .to_string();

    Ok(q)
}

pub fn get_info_from_dom(dom: &VDom<'_>, parser: &Parser<'_>) -> Result<String, anyhow::Error> {
    let q = dom
        .query_selector("span.evdesc.event-desc")
        .ok_or(DomParseError::NotFound)?
        .next()
        .ok_or(DomParseError::NotFound)?
        .get(parser)
        .ok_or(DomParseError::Unknown)?
        .inner_text(parser)
        .to_string();

    Ok(q)
}

pub fn get_country_from_dom(dom: &VDom<'_>, parser: &Parser<'_>) -> Result<String, anyhow::Error> {
    let q = dom
        .query_selector("i.icon-competitions")
        .ok_or(DomParseError::NotFound)?
        .next()
        .ok_or(DomParseError::NotFound)?
        .get(parser)
        .ok_or(DomParseError::Unknown)?
        .as_tag()
        .ok_or(DomParseError::NoAttributeFound)?
        .attributes()
        .get("style")
        .ok_or(DomParseError::NoAttributeFound)?
        .ok_or(DomParseError::NoAttributeFound)?
        .as_utf8_str()
        .split('/')
        .last()
        .ok_or(DomParseError::NoAttributeFound)?
        .replace(");", "")
        .replace(".svg", "")
        .to_string();

    Ok(q)
}
