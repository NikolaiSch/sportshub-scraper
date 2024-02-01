use chrono::NaiveDateTime;
use diesel::SqliteConnection;
use scraper::db::{
    self,
    helpers::{self, create_stream},
    models::StreamNew,
};

mod common;


fn test_create_stream() -> Result<SqliteConnection, anyhow::Error> {
    common::create_db()
}


#[test]
fn test_get_home_team() -> Result<(), anyhow::Error> {
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

    create_stream(&mut test_db, &new_stream)?;

    let streams = dbg!(helpers::get_streams_by_home_team(&mut test_db, "Arsenal".to_string())?);

    assert_eq!(streams.len(), 1);

    assert_eq!(streams[0].home, "Arsenal");

    helpers::delete_all_streams(&mut test_db)?;

    assert_eq!(helpers::get_streams(&mut test_db)?.len(), 0);

    Ok(())
}

// #[test]
// fn test_get_away_team() -> Result<(), anyhow::Error> {
//     let mut test_db = test_create_stream()?;

//     let new_stream = StreamNew {
//         home: "Arsenal",
//         away: "Chelsea",
//         sport: "Football",
//         start_time: NaiveDateTime::from_timestamp_opt(1612128000, 0).unwrap(),
//         stream_link: "https://www.youtube.com/watch?v=dQw4w9WgXcQ,https://www.youtube.com/watch?v=dQw4w9WgXcQ",
//         country: "England",
//         league: "Premier League",
//         url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
//     };

//     create_stream(&mut test_db, &new_stream)?;

//     let streams = helpers::get_streams_by_away_team(&mut test_db, "Chelsea".to_string())?;

//     assert_eq!(streams.len(), 1);


//     assert_eq!(streams[0].away, "Chelsea");

//     helpers::delete_all_streams(&mut test_db)?;

//     assert_eq!(helpers::get_streams(&mut test_db)?.len(), 0);

//     Ok(())
// }

// #[test]
// fn test_get_sport() -> Result<(), anyhow::Error> {
//     let mut test_db = test_create_stream()?;

//     let new_stream = StreamNew {
//         home: "Arsenal",
//         away: "Chelsea",
//         sport: "Football",
//         start_time: NaiveDateTime::from_timestamp_opt(1612128000, 0).unwrap(),
//         stream_link: "https://www.youtube.com/watch?v=dQw4w9WgXcQ,https://www.youtube.com/watch?v=dQw4w9WgXcQ",
//         country: "England",
//         league: "Premier League",
//         url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
//     };

//     create_stream(&mut test_db, &new_stream)?;

//     let streams = helpers::get_streams_by_sport(&mut test_db, "Football".to_string())?;

//     assert_eq!(streams.len(), 1);

//     assert_eq!(streams[0].sport, "Football");

//     helpers::delete_all_streams(&mut test_db)?;

//     assert_eq!(helpers::get_streams(&mut test_db)?.len(), 0);

//     Ok(())
// }

// #[test]
// fn test_get_streams() -> Result<(), anyhow::Error> {
//     let mut test_db = test_create_stream()?;

//     let new_stream = StreamNew {
//         home: "Arsenal",
//         away: "Chelsea",
//         sport: "Football",
//         start_time: NaiveDateTime::from_timestamp_opt(1612128000, 0).unwrap(),
//         stream_link: "https://www.youtube.com/watch?v=dQw4w9WgXcQ,https://www.youtube.com/watch?v=dQw4w9WgXcQ",
//         country: "England",
//         league: "Premier League",
//         url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
//     };

//     create_stream(&mut test_db, &new_stream)?;

//     let streams = helpers::get_streams(&mut test_db)?;

//     assert_eq!(streams.len(), 1);

//     assert_eq!(streams[0].sport, "Football");

//     helpers::delete_all_streams(&mut test_db)?;

//     assert_eq!(helpers::get_streams(&mut test_db)?.len(), 0);

//     Ok(())
// }

// #[test]
// fn test_get_streams_by_id() -> Result<(), anyhow::Error> {
//     let mut test_db = test_create_stream()?;

//     let new_stream = StreamNew {
//         home: "Arsenal",
//         away: "Chelsea",
//         sport: "Football",
//         start_time: NaiveDateTime::from_timestamp_opt(1612128000, 0).unwrap(),
//         stream_link: "https://www.youtube.com/watch?v=dQw4w9WgXcQ,https://www.youtube.com/watch?v=dQw4w9WgXcQ",
//         country: "England",
//         league: "Premier League",
//         url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
//     };

//     create_stream(&mut test_db, &new_stream)?;

//     let streams = helpers::get_streams_by_id(&mut test_db, 1)?;

//     assert_eq!(streams.len(), 1);

//     assert_eq!(streams[0].sport, "Football");

//     helpers::delete_all_streams(&mut test_db)?;

//     assert_eq!(helpers::get_streams(&mut test_db)?.len(), 0);

//     Ok(())
// }
