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
    if state.model_args.lock().is_ok() {
        let mut mutable_arg_state = state.model_args.lock().unwrap();
        *mutable_arg_state = new_args.clone();
        drop(mutable_arg_state);
    };

    println!("rest/model.rs MTD");
    // let model_tokenizer_device: std::sync::MutexGuard<'_, ModelTokenizerDevice> = model_tokenizer_device.lock().expect("*** replace should work.");

    // Mutate AppState
    if state.model_tokenizer_device.lock().is_ok() {
        // Drop current model
        let mut mutable_mtd_state = state.model_tokenizer_device.lock().expect("lock state");
        let model_tokenizer_device: ModelTokenizerDevice =
            LoadModel::load(new_args.clone(), true).expect("*** load_model should work.");
        *mutable_mtd_state = model_tokenizer_device;
        drop(mutable_mtd_state);

        println!("Dropped old and ready to load new model.");

        // load model
        let mut mutable_mtd_state = state.model_tokenizer_device.lock().expect("lock state");
        let no_model = new_args.clone().template.unwrap_or_default() == *"NoModel";
        let model_tokenizer_device: ModelTokenizerDevice =
            LoadModel::load(new_args.clone(), no_model).expect("*** load_model should work.");
        *mutable_mtd_state = model_tokenizer_device;
        drop(mutable_mtd_state);
    }

    Ok(Json(returned_args))
}
