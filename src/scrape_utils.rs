use anyhow::Error;
use headless_chrome::Browser;

/// Create a new browser for use in scraping
///
/// # Arguments
///  * `headless` - Whether to run the browser in headless mode
///
/// # Returns
///  * `Result<Browser, Error>` - The browser to use for scraping
///
/// # Examples
/// ```
/// use scraper::scrape_utils::create_browser;
///
/// let browser = create_browser(true).unwrap();
/// ```
pub fn create_browser(headless: bool) -> Result<Browser, Error> {
    let browser = Browser::new({
        headless_chrome::LaunchOptions {
            headless,
            sandbox: true,
            ..Default::default()
        }
    })?;

    Ok(browser)
}
