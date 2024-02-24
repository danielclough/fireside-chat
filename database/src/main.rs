use database::server;

#[tokio::main]
pub async fn main() {
    server::db().await;
}
