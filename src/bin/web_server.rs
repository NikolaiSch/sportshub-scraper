use axum::{routing::get, Json, Router};
use scraper::{db, models::Stream};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(get_all_streams));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_all_streams() -> Json<Vec<Stream>> {
    let mut conn = db::establish_connection();
    let streams = db::get_streams(&mut conn);

    Json(streams)
}
