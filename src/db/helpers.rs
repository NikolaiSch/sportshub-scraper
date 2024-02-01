//! Database operation helpers for sqlite, using diesel

use std::time::{Duration, Instant};

use diesel::{dsl::*, prelude::*, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::{
    models::{Stream, StreamNew},
    schema,
    schema::{stream, stream::dsl::*},
};

pub fn establish_connection() -> Result<SqliteConnection, anyhow::Error> {
    let database_url = format!("{}/sports.db", std::env::temp_dir().display());

    Ok(SqliteConnection::establish(&database_url)?)
}

pub fn create_stream(conn: &mut SqliteConnection, new_stream: &StreamNew) -> Result<usize, anyhow::Error> {
    Ok(diesel::insert_or_ignore_into(stream::table)
        .values(new_stream)
        .execute(conn)?)
}

pub fn get_streams(conn: &mut SqliteConnection) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream.load::<Stream>(conn)?)
}

pub fn get_empty_streams(conn: &mut SqliteConnection) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream.filter(schema::stream::stream_link.eq("")).load::<Stream>(conn)?)
}

pub fn get_linked_streams(conn: &mut SqliteConnection) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream.filter(schema::stream::stream_link.ne("")).load::<Stream>(conn)?)
}

pub fn get_streams_by_id(conn: &mut SqliteConnection, search_id: i32) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream.filter(schema::stream::id.eq(search_id)).load::<Stream>(conn)?)
}

pub fn delete_all_past_streams(conn: &mut SqliteConnection) -> Result<usize, anyhow::Error> {
    Ok(
        diesel::delete(stream.filter(start_time.le(chrono::Utc::now().naive_utc() - Duration::from_secs(3 * 60 * 60))))
            .execute(conn)?,
    )
}

pub fn get_streams_by_sport(conn: &mut SqliteConnection, search_sport: String) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream
        .filter(schema::stream::sport.eq(search_sport))
        .load::<Stream>(conn)?)
}

pub fn get_streams_by_home_team(
    conn: &mut SqliteConnection,
    search_home_team: String,
) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream
        .filter(schema::stream::home.eq(search_home_team))
        .load::<Stream>(conn)?)
}

pub fn get_streams_by_away_team(
    conn: &mut SqliteConnection,
    search_away_team: String,
) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream
        .filter(schema::stream::away.eq(search_away_team))
        .load::<Stream>(conn)?)
}

pub fn get_streams_by_either_team(
    conn: &mut SqliteConnection,
    search_team: String,
) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream
        .filter(
            schema::stream::home
                .eq(search_team.clone())
                .or(schema::stream::away.eq(search_team)),
        )
        .load::<Stream>(conn)?)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct LeagueWithCountry {
    pub league: String,
    pub country: String,
}

pub fn get_unique_leagues_with_country(conn: &mut SqliteConnection) -> Result<Vec<LeagueWithCountry>, anyhow::Error> {
    let mut leagues = Vec::new();

    let mut results = stream
        .select((schema::stream::league, schema::stream::country))
        .distinct()
        .load::<(String, String)>(conn)?;

    results.sort_by(|a, b| a.0.cmp(&b.0));

    for (i_league, i_country) in results {
        if !leagues.contains(&LeagueWithCountry {
            league: i_league.clone(),
            country: i_country.clone(),
        }) {
            leagues.push(LeagueWithCountry {
                league: i_league,
                country: i_country,
            });
        }
    }

    Ok(leagues)
}

pub fn get_active_games(conn: &mut SqliteConnection) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream.filter(schema::stream::stream_link.ne("")).load::<Stream>(conn)?)
}

pub fn delete_all_streams(conn: &mut SqliteConnection) -> Result<usize, anyhow::Error> {
    Ok(diesel::delete(stream).execute(conn)?)
}

pub fn get_streams_by_league(conn: &mut SqliteConnection, search_league: String) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream
        .filter(schema::stream::league.eq(search_league))
        .load::<Stream>(conn)?)
}
