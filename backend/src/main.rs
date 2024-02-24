use backend::server::chat::app;

#[tokio::main]
async fn main() {
    let ipv4 = "127.0.0.1".to_string();
    let port = 16981;

    app(ipv4, port).await;
}