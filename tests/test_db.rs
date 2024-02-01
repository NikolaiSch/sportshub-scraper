use chrono::NaiveDateTime;
use diesel::SqliteConnection;
use scraper::db::{helpers::create_stream, models::StreamNew};

mod common;


fn test_create_stream() -> Result<SqliteConnection, anyhow::Error> {
    common::create_db()
}

#[test]
fn test_insert_stream() -> Result<(), anyhow::Error> {
    let mut test_db = test_create_stream()?;

    let new_stream = StreamNew {
        home: "Arsenal",
        away: "Chelsea",
        sport: "Football",
        start_time: NaiveDateTime::from_timestamp_opt(1612128000, 0).unwrap(),
        stream_link: "https://www.youtube.com/watch?v=dQw4w9WgXcQ,https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        country: "England",
        league: "Premier League",
        url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    };

    let result = create_stream(&mut test_db, &new_stream)?;

    assert_eq!(result, 1);

    Ok(())
}
