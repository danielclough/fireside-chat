use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

use crate::{
    llm::load_model::{LoadModel, ModelTokenizerDevice},
    server::types::AppState,
};

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

    // Save in yaml
    let returned_args = LoadModel::save_args(new_args.clone());

    // Mutate AppState
    let mut mutable_arg_state = state.model_args.lock().unwrap();
    *mutable_arg_state = new_args.clone();

    println!("rest/model.rs MTD");
    // let model_tokenizer_device: std::sync::MutexGuard<'_, ModelTokenizerDevice> = model_tokenizer_device.lock().expect("*** replace should work.");
    
    // Mutate AppState
    let mut mutable_mtd_state = state.model_tokenizer_device.lock().expect("lock state");
    
    // load model
    let no_model = new_args.clone().template.unwrap_or(String::new()) == *"NoModel";
    let model_tokenizer_device: ModelTokenizerDevice = LoadModel::load(new_args.clone(),no_model).expect("*** load_model should work.");
    *mutable_mtd_state = model_tokenizer_device;


    Ok(Json(returned_args))
}
