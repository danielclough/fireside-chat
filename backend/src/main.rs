use std::sync::Mutex;

mod mistral;
use mistral::types::{inference_args::InferenceArgs, load_model::LoadModel};

mod server;
use server::app::start;

#[shuttle_runtime::main]
pub async fn axum() -> shuttle_axum::ShuttleAxum {
    // Load Mistral
    // Instantiate args for passing into AppState
    let model_tokenizer_device: Mutex<mistral::types::load_model::ModelTokenizerDevice> =
        Mutex::new(
            LoadModel::load(LoadModel::load_current_args()).expect("*** load_model should work."),
        );
    let inference_args = Mutex::new(InferenceArgs::new());

    println!("Model Loaded!\n Starting Server!\n");

    // Start server
    Ok(start(model_tokenizer_device, inference_args).await.into())
}
