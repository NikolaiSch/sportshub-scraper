use axum::{routing::get, Json, Router};
use scraper::{db, models::Stream};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_all_streams))
        .route("/active", get(get_active_streams));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
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
