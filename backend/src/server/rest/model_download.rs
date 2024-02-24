// use axum::{extract::Query, http::StatusCode, Json};
use axum::{http::StatusCode, Json};
// use common::types::model_list::ModelArgs;
// use gloo_net::http::QueryParams;
// use serde::{Deserialize, Serialize};

// use crate::{llm::types::load_model::LoadModel, utils::cache_path::cache_file_path};
use crate::llm::load_model::LoadModel;

// #[derive(Deserialize, Serialize)]
// pub struct StatusLink {
//     pub message: String
// }

// // fn to handle checking status of model downloads
// pub async fn check_download_status(
//     Query(repo_id): Query<String>,
// ) {
//     let path_repo = format!("models--{}", repo_id.replace('/', "--"));
//     let exists = cache_file_path(&path_repo).is_dir();
// }

// fn to handle getting model from huggingface
pub async fn download_model(Json(args): Json<LoadModel>) -> Result<Json<LoadModel>, StatusCode> {
    // Create args from Json
    let new_args = LoadModel { ..args.clone() };

    let _ = LoadModel::download(new_args.clone());

    // let status_link = StatusLink {
    //     message: "Downloading.".to_string()
    // };

    Ok(Json(new_args))
}
