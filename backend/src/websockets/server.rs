use axum::{routing::get, Router};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

use crate::mistral::types::config::{InferenceArgs, ModelTokenizerDevice};
use crate::websockets::{types::AppState, utils::websocket_handler};

/// Start Server w/ model_args
pub async fn server(model_args: Mutex<ModelTokenizerDevice>, inference_args: Mutex<InferenceArgs>) {
    // Load dotenv
    dotenv::dotenv().ok();

    // Set up application state for use with with_state().
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState {
        user_set,
        tx,
        model_args,
        inference_args,
    });

    // Instantiate new Router and serve.
    let app = Router::new()
        .route("/websocket", get(websocket_handler))
        .with_state(app_state);

    // Instantiate addr websocket_server_address with .env or default values.
    let ipv4 = std::env::var("IPV4").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let websocket_server_address = format!("{}:{}", ipv4, port).parse::<SocketAddr>().unwrap();

    // Serve
    tracing::debug!("listening on {}", websocket_server_address);
    axum::Server::bind(&websocket_server_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
