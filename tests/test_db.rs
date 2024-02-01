use anyhow::Error;
use chrono::NaiveDateTime;
use diesel::{sqlite::Sqlite, SqliteConnection};
use scraper::db::{
    self,
    helpers::{self, create_stream},
    models::StreamNew,
};

mod common;


fn test_create_stream() -> Result<SqliteConnection, anyhow::Error> {
    common::create_db()
}

fn create_new_stream(conn: &mut SqliteConnection) -> Result<(), Error> {
    let stream = StreamNew {
        home: "Arsenal",
        away: "Chelsea",
        sport: "Football",
        start_time: NaiveDateTime::from_timestamp_opt(1612128000, 0).unwrap(),
        stream_link: "https://www.youtube.com/watch?v=dQw4w9WgXcQ,https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        country: "England",
        league: "Premier League",
        url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    };

    create_stream(conn, &stream)?;

    Ok(())
}

#[test]
fn test_get_home_team() -> Result<(), anyhow::Error> {
    let mut test_db = test_create_stream()?;

    create_new_stream(&mut test_db)?;

    let streams = dbg!(helpers::get_streams_by_home_team(&mut test_db, "Arsenal".to_string())?);

    assert_eq!(streams.len(), 1);

    assert_eq!(streams[0].home, "Arsenal");

    helpers::delete_all_streams(&mut test_db)?;

    assert_eq!(helpers::get_streams(&mut test_db)?.len(), 0);

    Ok(())
}
