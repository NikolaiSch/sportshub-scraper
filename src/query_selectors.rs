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

pub fn get_url_from_dom(dom: &VDom<'_>, parser: &Parser<'_>) -> Result<String, Box<dyn Error>> {
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
