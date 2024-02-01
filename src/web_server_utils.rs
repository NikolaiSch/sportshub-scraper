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

pub async fn run(port: u16) {
    Rocket::custom(rocket::Config { port,
                                    ..Default::default() }).mount("/",
                                                                  routes![get_all_streams,
                                                                          get_active_streams,
                                                                          get_stream_by_id])
                                                           .launch()
                                                           .await
                                                           .unwrap();
}
