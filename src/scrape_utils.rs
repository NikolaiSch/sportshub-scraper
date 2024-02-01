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


/// Close all tabs from browser, so we exit cleanly
///
/// # Arguments
/// * `browser` - The browser to close tabs from
///
/// # Returns
/// * `Result<(), Error>` - Whether the tabs were closed successfully
///
/// # Examples
/// ```
/// use scraper::scrape_utils::{close_tabs, create_browser};
///
/// let browser = create_browser(true).unwrap();
/// close_tabs(&browser).unwrap();
/// ```
///
/// # Panics
///
/// - If the browser is not running, this function will panic
/// - If the mutex is poisoned, this function will panic
pub fn close_tabs(browser: &Browser) -> Result<(), Error> {
    for t in (*browser.get_tabs().as_ref().lock().unwrap()).iter() {
        t.close(true)?;
    }

    Ok(())
}
