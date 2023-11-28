use std::sync::Mutex;

mod mistral;
use mistral::types::{inference_args::InferenceArgs, load_model::LoadModel};

mod server;

#[tokio::main]
async fn main() {
    // Configure tracing subscriber
    tracing_subscriber::fmt::init();

    // Load Mistral
    // Instantiate args for passing into AppState
    let model_tokenizer_device =
        Mutex::new(LoadModel::load(LoadModel::load_args()).expect("*** load_model should work."));
    let inference_args = Mutex::new(InferenceArgs::new());

    // Start server
    server::start(model_tokenizer_device, inference_args).await;
}
