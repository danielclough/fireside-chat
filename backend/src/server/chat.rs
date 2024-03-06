use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::{get, post},
    Router,
};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    llm::{
        inference_args::InferenceArgs,
        load_model::{LoadModel, ModelTokenizerDevice},
    },
    server::rest::{
        inference::{get_inference, update_inference},
        model::{get_model_args, update_model_args},
        model_download::download_model,
        model_list::{get_model_list, update_model_list},
        role_list::{get_role_list, role_select},
    },
};

use crate::server::{types::AppState, websocket::handler::websocket_handler};

pub async fn app(backend_url: String, port: u16) {
    // Load Mistral
    // Instantiate args to get current repo_id to load model
    let model_args_for_loading_model = LoadModel::load_current_args();
    let no_model = model_args_for_loading_model
        .clone()
        .template
        .unwrap_or_default()
        == *"NoModel";
    println!("\nNoModel: {}\n", no_model);
    let model_tokenizer_device: Mutex<ModelTokenizerDevice> = Mutex::new(
        LoadModel::load(model_args_for_loading_model.clone(), no_model)
            .expect("*** load_model should work."),
    );
    let inference_args = Mutex::new(InferenceArgs::load_current_args());

    println!("Model Loaded!\n Starting Server!\n");

    // Start chat server

    // Instantiate args for passing into AppState
    let model_args = Mutex::new(model_args_for_loading_model);

    start(
        model_args,
        model_tokenizer_device,
        inference_args,
        backend_url,
        port,
    )
    .await;
}

/// Start Server
pub async fn start(
    model_args: Mutex<LoadModel>,
    model_tokenizer_device: Mutex<ModelTokenizerDevice>,
    inference_args: Mutex<InferenceArgs>,
    backend_url: String,
    port: u16,
) {
    // allow CORS from any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::PATCH,
            Method::POST,
            Method::HEAD,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE]);

    // Set up application state for use with with_state().
    let user_set = Mutex::new(HashSet::new());
    let (broadcast_sender, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState {
        model_args,
        user_set,
        broadcast_sender,
        model_tokenizer_device,
        inference_args,
    });

    // Instantiate new Router and serve.
    let app = Router::new()
        .route("/websocket", get(websocket_handler))
        .route("/model", get(get_model_args).patch(update_model_args))
        .route(
            "/model-list/:q_lvl",
            get(get_model_list).patch(update_model_list),
        )
        .route("/model-download", post(download_model))
        .route("/model-download/:repo_id", post(download_model))
        .route("/inference", get(get_inference).patch(update_inference))
        .route("/role-list", get(get_role_list).patch(role_select))
        .layer(cors)
        .with_state(app_state);

    // Serve
    let tcp_string = format!("{}:{}", backend_url, port);
    let listener = TcpListener::bind(tcp_string).await.unwrap();
    println!("listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
}
