//! This module contains the web server for the API.
//! It uses the rocket framework.

use db::models::Stream;
use rocket::{get, routes, serde::json::Json, Rocket};

use crate::db;

#[get("/")]
async fn get_all_streams() -> Json<Vec<Stream>> {
    let mut conn = db::helpers::establish_connection().unwrap();
    let streams = db::helpers::get_streams(&mut conn).unwrap();

    Json(streams)
}

#[get("/active")]
async fn get_active_streams() -> Json<Vec<Stream>> {
    let mut conn = db::helpers::establish_connection().unwrap();
    let streams = db::helpers::get_linked_streams(&mut conn).unwrap();

    Json(streams)
}

#[get("/id/<id>")]
async fn get_stream_by_id(id: i32) -> Json<Vec<Stream>> {
    let mut conn = db::helpers::establish_connection().unwrap();
    let streams = db::helpers::get_streams_by_id(&mut conn, id).unwrap();

    Json(streams)
}

#[get("/sport/<sport>")]
async fn get_streams_by_sport(sport: &str) -> Json<Vec<Stream>> {
    let mut conn = db::helpers::establish_connection().unwrap();
    let streams = db::helpers::get_streams_by_sport(&mut conn, sport.to_owned()).unwrap();

    Json(streams)
}

#[get("/team/home/<team>")]
async fn get_streams_by_home_team(team: &str) -> Json<Vec<Stream>> {
    let mut conn = db::helpers::establish_connection().unwrap();
    let streams = db::helpers::get_streams_by_home_team(&mut conn, team.to_owned()).unwrap();

    Json(streams)
}

#[get("/team/away/<team>")]
async fn get_streams_by_away_team(team: &str) -> Json<Vec<Stream>> {
    let mut conn = db::helpers::establish_connection().unwrap();
    let streams = db::helpers::get_streams_by_away_team(&mut conn, team.to_owned()).unwrap();

    Json(streams)
}

#[get("/team/<team>")]
async fn get_streams_by_either_team(team: &str) -> Json<Vec<Stream>> {
    let mut conn = db::helpers::establish_connection().unwrap();
    let streams = db::helpers::get_streams_by_either_team(&mut conn, team.to_owned()).unwrap();

    Json(streams)
}

pub async fn run(port: u16) -> anyhow::Result<()> {
    Rocket::custom(rocket::Config {
        port,
        ..Default::default()
    })
    .mount(
        "/",
        routes![
            get_all_streams,
            get_active_streams,
            get_stream_by_id,
            get_streams_by_sport,
            get_streams_by_home_team,
            get_streams_by_away_team,
            get_streams_by_either_team,
        ],
    )
    .launch()
    .await?;

    Ok(())
}
