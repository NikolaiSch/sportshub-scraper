use anyhow::Error;
use diesel::SqliteConnection;
use scraper::db::helpers::establish_test_connection;


pub fn create_db() -> Result<SqliteConnection, Error> {
    let test_db = establish_test_connection()?;


    Ok(test_db)
}
