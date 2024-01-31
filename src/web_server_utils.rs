use crate::{db, models::Stream};
use axum::{routing::get, Json, Router};

pub async fn run(port: u16) {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_all_streams))
        .route("/active", get(get_active_streams));

    let host = format!("0.0.0.0:{}", port);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    println!("Listening on http://{}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn get_all_streams() -> Json<Vec<Stream>> {
    println!("get_all_streams");
    let mut conn = db::establish_connection();
    let streams = db::get_streams(&mut conn);

    Json(streams)
}

async fn get_active_streams() -> Json<Vec<Stream>> {
    println!("get_active_streams");
    let mut conn = db::establish_connection();
    let streams = db::get_linked_streams(&mut conn);

    Json(streams)
}
