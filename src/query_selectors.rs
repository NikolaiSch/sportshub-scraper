//! This module contains the functions to get the data from the dom of the eventlist
//! and return the data as a string

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

/// Get the event url from the dom of the eventlist
/// # Arguments
/// * `dom` - The dom of the eventlist (tl)
/// * `parser` - The parser of the eventlist (tl)
///
/// # Example
/// ```
/// use scraper::query_selectors::get_url_from_dom;
///
/// let html = r#"
/// <html>
///    <body>
///       <a href="https://sportshub.fan/event/ypiranga_rs_novo_hamburgo_191503337/">
///     </body>
/// </html>
/// "#;
///
/// let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
/// let parser = dom.parser();
///
/// let link = get_url_from_dom(&dom, &parser).unwrap();
/// assert_eq!(link, "https://sportshub.fan/event/ypiranga_rs_novo_hamburgo_191503337/");
/// ```
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

/// Get the game name from the dom of the eventlist
/// # Arguments
/// * `dom` - The dom of the eventlist (tl)
/// * `parser` - The parser of the eventlist (tl)
/// # Example
/// ```
/// use scraper::query_selectors::get_game_name_from_dom;
/// let html = r#"
/// <html>
///   <body>
///    <div class="event-name">
///    <span class="mr-5">Ypiranga RS - Novo Hamburgo</span>
///   </div>
///  </body>
/// </html>
/// "#;
///
/// let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
/// let parser = dom.parser();
///
/// let event_name = get_game_name_from_dom(&dom, &parser).unwrap();
/// assert_eq!(event_name, "Ypiranga RS - Novo Hamburgo");
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

/// Get the event info from the dom of the eventlist
/// # Arguments
/// * `dom` - The dom of the eventlist (tl)
/// * `parser` - The parser of the eventlist (tl)
/// # Example
/// ```
/// use scraper::query_selectors::get_info_from_dom;
/// let html = r#"
/// <html>
///  <body>
///  <div class="event-info">
///  <span class="evdesc event-desc">Brazilian Campeonato Gaucho</span>
/// </div>
/// </body>
/// </html>
/// "#;
///
/// let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
/// let parser = dom.parser();
///
/// let event_info = get_info_from_dom(&dom, &parser).unwrap();
/// assert_eq!(event_info, "Brazilian Campeonato Gaucho");
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

/// Get the event country from the dom of the eventlist
/// # Arguments
/// * `dom` - The dom of the eventlist (tl)
/// * `parser` - The parser of the eventlist (tl)
/// # Example
/// ```
/// use scraper::query_selectors::get_country_from_dom;
/// let html = r#"
/// <html>
///   <body>
///       <i class="icon-competitions" style="background-image: url(https://sportshub.cdn.prismic.io/sportshub/brazil.svg);"></i>
///   </body>
/// </html>
/// "#;
///
/// let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
/// let parser = dom.parser();
///
/// let country = get_country_from_dom(&dom, &parser).unwrap();
/// assert_eq!(country, "brazil");
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
