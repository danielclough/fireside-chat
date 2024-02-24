use crate::{llm::load_model::LoadModel, utilities::cache_path::cache_file_path};
use axum::{http::StatusCode, Json};
use common::llm::model_list::{ModelDLList, ModelDLListEntry, ModelList};
use glob::glob;

// use serde::{Deserialize, Serialize};

// fn to handle getting model_lists from frontend
pub async fn get_model_list() -> Result<Json<ModelDLList>, StatusCode> {
    let model_list: ModelList = get_default_list();

    let entry_collection = model_list
        .list
        .iter()
        .map(|list| {
            let repo_id_path = format!("models--{}", list.repo_id.clone().replace('/', "--"));
            let gguf;
            let safetensors;
            let bin;
            if cache_file_path(&repo_id_path).is_dir() {
              // check current level of quantization and then check if .gguf is downloaded
              let args = LoadModel::load_current_args();
              let gguf_path = format!("{repo_id_path}/**/{}.gguf", args.q_lvl);
              gguf = glob(&gguf_path).is_ok();
              // check if .safetensors are downloaded
              let safetensors_path = format!("{repo_id_path}/**/*.safetensors");
              safetensors = glob(&safetensors_path).is_ok();
              // check if .bin are downloaded
              let bin_path = format!("{repo_id_path}/**/*.bin");
              bin = glob(&bin_path).is_ok();
            } else {
              gguf = false;
              safetensors = false;
              bin = false;
            }

            ModelDLListEntry {
                name: list.name.clone(),
                repo_id: list.repo_id.clone(),
                template: Some(list.template.clone().split(',').map(|x| x.trim().to_string()).collect()),
                base: list.base.clone(),
                n_safetensors: list.n_safetensors,
                gguf,
                safetensors,
                bin,
                tags: list.tags.split(',').map(|x| x.trim().to_string()).collect(),
            }
        })
        .collect();

    let model_dl_list: ModelDLList = ModelDLList {
        list: entry_collection,
    };

    // println!("{:?}", model_list);

    Ok(Json(model_dl_list))
}

// fn to handle patching model_lists from frontend
pub async fn update_model_list(Json(args): Json<ModelList>) -> Result<Json<ModelList>, StatusCode> {
    // Create args from Json
    let new_args = ModelList { ..args };

    // !!TODO!! - Save config with new model.

    Ok(Json(new_args))
}
// Narsil/amall-7b
// meta-llama/Llama-2-7b-hf
pub fn get_default_list() -> ModelList {
    serde_yaml::from_str("---
list:
  -
    name: Mistral-7B-v0.1
    repo_id: DanielClough/Candle_Mistral-7B-v0.1
    base: mistral
    template:
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: MistralLite
    repo_id: DanielClough/Candle_MistralLite
    template: Amazon
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: Mistral-7B-Instruct-v0.1
    repo_id: DanielClough/Candle_Mistral-7B-Instruct-v0.1
    template: MistralInstruct
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: Mistral-7B-Instruct-v0.2
    repo_id: DanielClough/Candle_Mistral-7B-Instruct-v0.2
    template: MistralInstruct
    base: mistral
    n_safetensors: 3
    tags: gguf,safetensors,
  -
    name: Mistralic-7B-1
    repo_id: DanielClough/Candle_Mistralic-7B-1
    template: ChatML
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: dolphin-2.2.1-mistral-7b
    repo_id: DanielClough/Candle_dolphin-2.2.1-mistral-7b
    template: ChatML
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: Mistral-7B-OpenOrca
    repo_id: DanielClough/Candle_Mistral-7B-OpenOrca
    template: ChatML
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: OpenHermes-2.5-Mistral-7B
    repo_id: DanielClough/Candle_OpenHermes-2.5-Mistral-7B
    template: ChatML
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: phi-1
    repo_id: DanielClough/Candle_phi-1
    template: PhiCode
    base: phi
    n_safetensors: 1
    tags: gguf,safetensors,code,
  -
    name: phi-1_5
    repo_id: DanielClough/Candle_phi-1_5
    template: PhiCode, PhiChat, PhiQA
    base: phi
    n_safetensors: 1
    tags: gguf,safetensors,code,
  -
    name: phi-2
    repo_id: DanielClough/Candle_phi-2
    template: PhiCode, PhiChat, PhiQA
    base: phi
    n_safetensors: 2
    tags: gguf,safetensors,code,
  -
    name: phi-2_old
    repo_id: DanielClough/Candle_phi-2_old
    template: PhiCode, PhiChat, PhiQA
    base: phi
    n_safetensors: 2
    tags: gguf,safetensors,code,
  -
    name: Puffin-Phi-v2
    repo_id: DanielClough/Candle_Puffin-Phi-v2
    template: ShareGPT
    base: phi
    n_safetensors: 1
    tags: safetensors,
  -
    name: Phi-Hermes-1.3B
    repo_id: DanielClough/Candle_Phi-Hermes-1.3B
    template: Alpaca
    base: phi
    n_safetensors: 1
    tags: safetensors,
",
    )
    .expect("Model List from String")
}

// -
// name: SOLAR-10.7B-v1.0
// repo_id: DanielClough/Candle_SOLAR-10.7B-v1.0
// base: llama
// template:
// n_safetensors: 5
// tags: gguf, n_safetensors,
// -
// name: SOLAR-10.7B-Instruct-v1.0
// repo_id: DanielClough/Candle_SOLAR-10.7B-Instruct-v1.0
// template: SolarInstruct
// base: llama
// n_safetensors: 5
// tags: gguf, n_safetensors,
// -
// name: OrcaMini-3B
// repo_id: DanielClough/Candle_OrcaMini-3B
// template: ChatML
// base: llama
// n_safetensors: 3
// tags: gguf, n_safetensors,
// -
// name: TinyLlama-1.1B-Chat-v1.0
// repo_id: DanielClough/Candle_TinyLlama-1.1B-Chat-v1.0
// template: Zephyr
// base: llama
// n_safetensors: 1
// tags: gguf, n_safetensors,
// -
// name: karpathy/tinyllamas
// repo_id: karpathy/tinyllamas
// template: None
// base: llama2
// n_safetensors: 0
// tags: gguf, n_safetensors,


// -
// name: TinyMistral-248M
// repo_id: DanielClough/Candle_TinyMistral-248M
// base: mistral
// template:
// n_safetensors: 1
// tags: gguf, n_safetensors,
      // -
      // name: tiny-mistral
      // repo_id: DanielClough/Candle_tiny-mistral
      // base: mistral
      // template:
      // n_safetensors: 1
      // tags: gguf, n_safetensors,