use backend::server::chat::app;

#[tokio::main]
async fn main() {
    let localhost = "127.0.0.1";
    let backend_url = std::option_env!("FIRESIDE_BACKEND_URL")
        .unwrap_or(localhost)
        .to_string();
    let port = 16981;

    app(backend_url, port).await;
}
