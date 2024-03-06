use crate::functions::get_path::get_llm_path;
use common::llm::{
    inference::{InferenceArgsForInput, InferenceArgsForJson},
    model_list::{ModelArgs, ModelDLList},
};
use gloo_net::{
    http::{Request, Response},
    Error,
};

pub async fn patch_inference_args(
    set_args_for_json: InferenceArgsForJson,
    backend_url: String,
) -> InferenceArgsForInput {
    let path = get_llm_path("inference", backend_url);

    Request::patch(&path)
        .header("Content-Type", "application/json")
        .json(&set_args_for_json)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn get_inference_args(backend_url: String) -> InferenceArgsForInput {
    let path = get_llm_path("inference", backend_url);

    let response: Result<Response, Error> = Request::get(&path).send().await;

    match response {
        Err(err) => {
            println!("\n{}\n", err);
            InferenceArgsForInput {
                temperature: 0.0,
                top_p: 0.0,
                seed: 0f64,
                sample_len: 0f64,
                repeat_penalty: 0f64,
                repeat_last_n: 0f64,
                load_context: false,
                role: String::new(),
            }
        }
        _ => {
            let result: InferenceArgsForJson = response.unwrap().json().await.unwrap();
            InferenceArgsForInput {
                temperature: result.temperature,
                top_p: result.top_p,
                seed: result.seed as f64,
                sample_len: result.sample_len as f64,
                repeat_penalty: result.repeat_penalty as f64,
                repeat_last_n: result.repeat_last_n as f64,
                load_context: result.load_context,
                role: result.role,
            }
        }
    }
}

pub async fn get_model_args(backend_url: String) -> ModelArgs {
    let path = get_llm_path("model", backend_url);

    let response = Request::get(&path).send().await;

    if response.is_ok() {
        response
            .expect("Load model args from API")
            .json()
            .await
            .unwrap()
    } else {
        ModelArgs::error()
    }
}

pub async fn get_model_list(q_lvl: String, backend_url: String) -> ModelDLList {
    let slug = format!("model-list/{}", q_lvl);
    let path = get_llm_path(&slug, backend_url);

    let response = Request::get(&path).send().await;

    if response.is_ok() {
        response
            .expect("Load model list from API")
            .json()
            .await
            .unwrap()
    } else {
        ModelDLList { list: vec![] }
    }
}

pub async fn model_download(model_args: ModelArgs, backend_url: String) -> ModelArgs {
    let path = get_llm_path("model-download", backend_url);
    println!("download working");

    Request::post(&path)
        .header("Content-Type", "application/json")
        .json(&model_args)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn model_update(model_args: ModelArgs, backend_url: String) -> ModelArgs {
    let path = get_llm_path("model", backend_url);

    Request::patch(&path)
        .header("Content-Type", "application/json")
        .json(&model_args)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
