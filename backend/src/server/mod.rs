pub mod rest;
pub mod types;
pub mod websocket;

use axum::{
    routing::{get, patch},
    Router,
};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

use crate::{
    mistral::types::inference_args::InferenceArgs, server::rest::routes::update_inference,
};
use crate::{
    mistral::types::load_model::ModelTokenizerDevice, server::rest::routes::get_inference,
};

use types::AppState;
use websocket::utils::handler::websocket_handler;

/// Start Server
pub async fn start(
    model_tokenizer_device: Mutex<ModelTokenizerDevice>,
    inference_args: Mutex<InferenceArgs>,
) {
    // Load dotenv
    dotenv::dotenv().ok();

    // Set up application state for use with with_state().
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState {
        user_set,
        tx,
        model_tokenizer_device,
        inference_args,
    });

    // Instantiate new Router and serve.
    let app = Router::new()
        .route("/websocket", get(websocket_handler))
        .route("/inference/:args", patch(update_inference))
        .route("/inference", get(get_inference))
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
