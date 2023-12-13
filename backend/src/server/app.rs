use axum::{routing::get, Router};
use http::{header::CONTENT_TYPE, Method};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    mistral::types::inference_args::InferenceArgs,
    server::rest::routes::{get_model_args, update_inference, update_model_args},
};
use crate::{
    mistral::types::load_model::ModelTokenizerDevice, server::rest::routes::get_inference,
};

use crate::server::types::AppState;
use crate::server::websocket::utils::handler::websocket_handler;

/// Start Server
pub async fn start(
    model_tokenizer_device: Mutex<ModelTokenizerDevice>,
    inference_args: Mutex<InferenceArgs>,
) -> Router {
    // Load dotenv
    dotenv::dotenv().ok();

    // allow CORS from any origin
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::PATCH, Method::HEAD])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

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
    Router::new()
        .route("/websocket", get(websocket_handler))
        .route("/inference", get(get_inference).patch(update_inference))
        .route("/model", get(get_model_args).patch(update_model_args))
        .layer(cors)
        .with_state(app_state)
}
