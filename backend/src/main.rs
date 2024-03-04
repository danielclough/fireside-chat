use backend::server::chat::app;

#[shuttle_runtime::main]
pub async fn main() -> shuttle_axum::ShuttleAxum {
    let ipv4 = "127.0.0.1".to_string();
    let port = 16981;

    Ok(app(ipv4, port).await.into())
}

