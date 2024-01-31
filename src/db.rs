//! Database operation helpers for sqlite, using diesel

use diesel::prelude::*;

use crate::models::*;
use crate::schema::{
    self,
    stream::{self, dsl::*},
};
use diesel::RunQueryDsl;

pub fn establish_connection() -> Result<SqliteConnection, anyhow::Error> {
    let database_url = format!("{}/sports.db", std::env::temp_dir().display());

    Ok(SqliteConnection::establish(&database_url)?)
}

pub fn create_stream(
    conn: &mut SqliteConnection,
    new_stream: &StreamNew,
) -> Result<usize, anyhow::Error> {
    Ok(diesel::insert_or_ignore_into(stream::table)
        .values(new_stream)
        .execute(conn)?)
}

pub fn get_streams(conn: &mut SqliteConnection) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream.load::<Stream>(conn)?)
}

pub fn get_empty_streams(conn: &mut SqliteConnection) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream
        .filter(schema::stream::stream_link.eq(""))
        .load::<Stream>(conn)?)
}

pub fn get_linked_streams(conn: &mut SqliteConnection) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream
        .filter(schema::stream::stream_link.ne(""))
        .load::<Stream>(conn)?)
}

pub fn get_streams_by_id(
    conn: &mut SqliteConnection,
    search_id: i32,
) -> Result<Vec<Stream>, anyhow::Error> {
    Ok(stream
        .filter(schema::stream::id.eq(search_id))
        .load::<Stream>(conn)?)
}
