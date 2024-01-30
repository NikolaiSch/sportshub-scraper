#[macro_use]
extern crate diesel;

use std::error::Error;
use std::ffi::OsStr;

use axum::routing::get;
use axum::routing::post;
use axum::Router;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;
use headless_chrome::Browser;
use headless_chrome::Tab;

#[tokio::main]
async fn main() {
    let app = Router::new();
    // `GET /` goes to `root`

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
}
