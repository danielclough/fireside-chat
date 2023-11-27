use std::sync::Mutex;

mod mistral;
use mistral::types::config::{ArgsToLoadModel, InferenceArgs};
use mistral::utils::load_model;

mod websockets;
use websockets::server::server;

#[tokio::main]
async fn main() {
    // Configure tracing subscriber
    tracing_subscriber::fmt::init();

    // Load Mistral
    // Instantiate args for passing into AppState
    let model_args =
        Mutex::new(load_model(ArgsToLoadModel::new()).expect("*** load_model should work."));
    let inference_args = Mutex::new(InferenceArgs::new());

    // Start server
    server(model_args, inference_args).await;
}
