use diesel::prelude::*;

use crate::models::*;
use crate::schema::stream;
use crate::schema::stream::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "sports.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_stream(conn: &mut SqliteConnection, new_stream: &StreamNew) -> () {
    diesel::insert_into(stream::table)
        .values(new_stream)
        .execute(conn)
        .unwrap();
}

pub fn get_streams(conn: &mut SqliteConnection) -> Vec<Stream> {
    stream.load::<Stream>(conn).expect("Error loading streams")
}