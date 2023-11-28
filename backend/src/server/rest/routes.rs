use axum::{extract::State, http::StatusCode, Json};
use std::sync::{Arc, Mutex};

use crate::{
    mistral::types::{inference_args::InferenceArgs, load_model::ModelTokenizerDevice},
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
    State(mut state): State<Arc<AppState>>,
    Json(args): Json<InferenceArgs>,
) -> Result<Json<InferenceArgs>, StatusCode> {
    // Create args from Json
    let new_args = InferenceArgs { ..args };

    // Mutate AppState
    let old_state = state.clone();
    state = Arc::new(AppState {
        user_set: Mutex::new(old_state.user_set.lock().unwrap().clone()),
        tx: old_state.tx.clone(),
        model_tokenizer_device: Mutex::new(ModelTokenizerDevice {
            ..old_state.model_tokenizer_device.lock().unwrap().clone()
        }),
        inference_args: Mutex::new(new_args),
    });

    tracing::debug!("{:?}", state.inference_args);

    Ok(Json(new_args))
}
