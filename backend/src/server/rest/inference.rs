use axum::{extract::State, http::StatusCode, Json};
use common::llm::inference::InferenceArgsForJson;
use std::sync::Arc;

use crate::{llm::inference_args::InferenceArgs, server::types::AppState};

// fn to handle getting inferences from frontend
pub async fn get_inference(
    State(state): State<Arc<AppState>>,
) -> Result<Json<InferenceArgsForJson>, StatusCode> {
    let args = state.inference_args.lock().expect("lock state");

    let role = match &args.role {
        Some(r) => r.to_string(),
        None => String::new(),
    };

    let args_for_json = InferenceArgsForJson {
        temperature: args.temperature.unwrap_or(0.0),
        top_p: args.top_p.unwrap_or(0.0),
        seed: args.seed,
        sample_len: args.sample_len,
        repeat_penalty: args.repeat_penalty,
        repeat_last_n: args.repeat_last_n,
        load_context: args.load_context,
        role,
    };
    Ok(Json(args_for_json))
}

// fn to handle patching inferences from frontend
pub async fn update_inference(
    State(state): State<Arc<AppState>>,
    Json(args): Json<InferenceArgsForJson>,
) -> Result<Json<InferenceArgsForJson>, StatusCode> {
    // Create args from Json
    let new_args = InferenceArgs {
        temperature: Some(args.temperature),
        top_p: Some(args.top_p),
        seed: args.seed,
        sample_len: args.sample_len,
        repeat_penalty: args.repeat_penalty,
        repeat_last_n: args.repeat_last_n,
        load_context: args.load_context,
        role: Some(args.role.to_string()),
    };

    // Save in yaml
    let _ = InferenceArgs::save_args(new_args.clone());

    // Mutate AppState
    let mut mutable_state = state.inference_args.lock().unwrap();
    *mutable_state = new_args.clone();
    drop(mutable_state);

    tracing::debug!("{:?}", state.inference_args);

    let args_for_json = InferenceArgsForJson {
        temperature: args.temperature,
        top_p: args.top_p,
        seed: args.seed,
        sample_len: args.sample_len,
        repeat_penalty: args.repeat_penalty,
        repeat_last_n: args.repeat_last_n,
        load_context: args.load_context,
        role: args.role.to_string(),
    };

    Ok(Json(args_for_json))
}
