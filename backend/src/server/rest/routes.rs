use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

use crate::{
    mistral::types::{inference_args::InferenceArgs, load_model::LoadModel},
    server::types::AppState,
};

// fn to handle getting inferences from frontend
pub async fn get_inference(
    State(state): State<Arc<AppState>>,
) -> Result<Json<InferenceArgs>, StatusCode> {
    let inference_args = state.inference_args.lock().expect("lock state");

    Ok(Json(*inference_args))
}

// fn to handle patching inferences from frontend
pub async fn update_inference(
    State(state): State<Arc<AppState>>,
    Json(args): Json<InferenceArgs>,
) -> Result<Json<InferenceArgs>, StatusCode> {
    // Create args from Json
    let new_args = InferenceArgs { ..args };

    // Mutate AppState
    let mut mutable_state = state.inference_args.lock().unwrap();
    *mutable_state = new_args;

    tracing::debug!("{:?}", state.inference_args);

    Ok(Json(new_args))
}

// fn to handle getting model_args from frontend
pub async fn get_model_args() -> Result<Json<LoadModel>, StatusCode> {
    let model_args = LoadModel::load_current_args();

    Ok(Json(model_args))
}

// fn to handle patching model args from frontend
pub async fn update_model_args(
    State(state): State<Arc<AppState>>,
    Json(args): Json<LoadModel>,
) -> Result<Json<LoadModel>, StatusCode> {
    // Create args from Json
    let new_args = LoadModel { ..args };

    println!("here {:?}\n", new_args);
    let new_model_tokenizer_device =
        LoadModel::load(new_args.clone()).expect("*** load_model should work.");

    println!("here {:?}\n", new_model_tokenizer_device);
    // Save in yaml
    let returned_args = LoadModel::save_args(new_args);

    println!("here {:?}\n", returned_args);
    // Mutate AppState
    let mut mutable_state = state.model_tokenizer_device.lock().unwrap();
    *mutable_state = new_model_tokenizer_device;

    println!("here");
    // tracing::debug!("{:?}", state.model_tokenizer_device);

    Ok(Json(returned_args))
}
