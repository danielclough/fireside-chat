use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{mistral::types::inference_args::InferenceArgs, server::types::AppState};

// fn to handle websocket connections.
pub async fn get_inference(
    State(state): State<Arc<AppState>>,
) -> Result<Json<InferenceArgs>, StatusCode> {
    let inference_args = state.inference_args.lock().unwrap();

    Ok(Json(*inference_args))
}
