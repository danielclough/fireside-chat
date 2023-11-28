use std::sync::Mutex;

mod mistral;
use mistral::types::{inference_args::InferenceArgs, load_model::LoadModel};

mod server;
use server::app::start;

#[tokio::main]
async fn main() {
    // Load Mistral
    // Instantiate args for passing into AppState
    let model_tokenizer_device =
        Mutex::new(LoadModel::load(LoadModel::load_args()).expect("*** load_model should work."));
    let inference_args = Mutex::new(InferenceArgs::new());

    println!("Model Loaded!\n Starting Server!\n");

    // Start server
    start(model_tokenizer_device, inference_args).await;
}
