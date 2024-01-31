#[tokio::main]
async fn main() {
    scraper::web_server_utils::run().await;
}
