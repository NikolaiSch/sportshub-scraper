use crate::{db, models::Stream};
use rocket::{get, launch, routes, serde::json::Json, Rocket};

#[get("/")]
async fn get_all_streams() -> Json<Vec<Stream>> {
    let mut conn = db::establish_connection();
    let streams = db::get_streams(&mut conn);

    Json(streams)
}

#[get("/active")]
async fn get_active_streams() -> Json<Vec<Stream>> {
    let mut conn = db::establish_connection();
    let streams = db::get_linked_streams(&mut conn);

    Json(streams)
}

#[get("/id/<id>")]
async fn get_stream_by_id(id: i32) -> Json<Vec<Stream>> {
    let mut conn = db::establish_connection();
    let streams = db::get_streams_by_id(&mut conn, id);

    Json(streams)
}

pub async fn run(port: u16) {
    Rocket::custom(rocket::Config {
        port,
        ..Default::default()
    })
    .mount(
        "/",
        routes![get_all_streams, get_active_streams, get_stream_by_id],
    )
    .launch()
    .await
    .unwrap();
}
