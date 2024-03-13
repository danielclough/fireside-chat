use crate::utilities::cache_path::cache_file_path;
use axum::{extract::Path, http::StatusCode, Json};
use common::llm::model_list::{ModelArgs, ModelDLList, ModelDLListEntry, ModelList};
use glob::glob;

// use serde::{Deserialize, Serialize};

// fn to handle getting model_lists from frontend
pub async fn get_model_list(Path(q_lvl): Path<String>) -> Result<Json<ModelDLList>, StatusCode> {
    let model_list: ModelList = get_default_list();

    let entry_collection = model_list
        .list
        .iter()
        .map(|list| {
            let repo_id_path = format!("models--{}", list.repo_id.clone().replace('/', "--"));
            let mut gguf = false;
            let mut safetensors = false;
            let mut bin = false;
            let cache_file_path = cache_file_path(&repo_id_path);
            if cache_file_path.is_dir() {
                let q_lvl = if q_lvl.as_str() == "NoModel" {
                    ModelArgs::default().q_lvl
                } else {
                    q_lvl.clone()
                };
                // check current level of quantization and then check if .gguf is downloaded
                let gguf_path =
                    format!("{}/snapshots/**/*{}.gguf", cache_file_path.display(), q_lvl);
                for entry in glob(gguf_path.as_str()).unwrap() {
                    if let Ok(_path) = entry {
                        gguf = true;
                    }
                }
                // check if .safetensors are downloaded
                let safetensors_path =
                    format!("{}/snapshots/**/*.safetensors", cache_file_path.display());
                for entry in glob(safetensors_path.as_str()).unwrap() {
                    if let Ok(_path) = entry {
                        safetensors = true;
                    }
                }
                // check if .bin are downloaded
                let bin_path = format!("{}/snapshots/**/*.bin", cache_file_path.display());
                for entry in glob(bin_path.as_str()).unwrap() {
                    if let Ok(_path) = entry {
                        bin = true;
                    }
                }
            };
            ModelDLListEntry {
                name: list.name.clone(),
                repo_id: list.repo_id.clone(),
                template: Some(
                    list.template
                        .clone()
                        .split(',')
                        .map(|x| x.trim().to_string())
                        .collect(),
                ),
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

    // println!("{}", model_list.display());

    Ok(Json(model_dl_list))
}

// fn to handle patching model_lists from frontend
pub async fn update_model_list(
    Path(_q_lvl): Path<String>,
    Json(args): Json<ModelList>,
) -> Result<Json<ModelList>, StatusCode> {
    // Create args from Json
    let new_args = ModelList { ..args };

    // !!TODO!! - Save config with new model.

    Ok(Json(new_args))
}
// Narsil/amall-7b
// meta-llama/Llama-2-7b-hf
pub fn get_default_list() -> ModelList {
    serde_yaml::from_str(
        "---
list:
  -
    name: Mistral-7B-v0.1
    repo_id: DanielClough/Candle_Mistral-7B-v0.1
    base: mistral
    template:
    n_safetensors: 2
    tags:
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
    name: CollectiveCognition-v1.1-Mistral-7B
    repo_id: DanielClough/Candle_CollectiveCognition-v1.1-Mistral-7B
    template: TekniumOld
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: Hermes-Trismegistus-Mistral-7B
    repo_id: DanielClough/Candle_Hermes-Trismegistus-Mistral-7B
    template: Teknium
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: Mistral-Trismegistus-7B
    repo_id: DanielClough/Candle_Mistral-Trismegistus-7B
    template: TekniumOld
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,  
  -
    name: Snorkel-Mistral-PairRM-DPO
    repo_id: DanielClough/Candle_Snorkel-Mistral-PairRM-DPO
    template: MistralInstruct
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: airoboros-mistral2.2-7b
    repo_id: DanielClough/Candle_airoboros-mistral2.2-7b
    template: TekniumOld
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: lvkaokao-mistral-7b-finetuned-orca-dpo-v2
    repo_id: DanielClough/Candle_lvkaokao-mistral-7b-finetuned-orca-dpo-v2
    template: 
    base: mistral
    n_safetensors: 2
    tags: gguf,safetensors,
  -
    name: llava-v1.6-mistral-7b
    repo_id: DanielClough/Candle_llava-v1.6-mistral-7b
    template: MistralInstruct
    base: mistral
    n_safetensors: 4
    tags: gguf,safetensors,
  -
    name: Puffin-Phi-v2
    repo_id: DanielClough/Candle_Puffin-Phi-v2
    template: ShareGPT
    base: phi
    n_safetensors: 1
    tags: gguf,safetensors,
  -
    name: Phi-Hermes-1.3B
    repo_id: DanielClough/Candle_Phi-Hermes-1.3B
    template: Alpaca
    base: phi
    n_safetensors: 1
    tags: gguf,safetensors,
  -
    name: phi-1
    repo_id: DanielClough/Candle_phi-1
    template: PhiCode
    base: phi
    n_safetensors: 1
    tags: code,gguf,safetensors,
  -
    name: phi-1_5
    repo_id: DanielClough/Candle_phi-1_5
    template: PhiCode, PhiChat, PhiQA
    base: phi
    n_safetensors: 1
    tags: code,gguf,safetensors,
  -
    name: phi-2
    repo_id: DanielClough/Candle_phi-2
    template: PhiCode, PhiChat, PhiQA
    base: phi
    n_safetensors: 2
    tags: code,gguf,safetensors,
  -
    name: phi-2_old
    repo_id: DanielClough/Candle_phi-2_old
    template: PhiCode, PhiChat, PhiQA
    base: phi
    n_safetensors: 2
    tags: code,gguf,safetensors,
    
",
    )
    .expect("Model List from String")
}

// -
// name: Mistralic-7B-1
// repo_id: DanielClough/Candle_Mistralic-7B-1
// template: ChatML
// base: mistral
// n_safetensors: 2
// tags: gguf,safetensors,
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

// -
// name: MistralTrix-v1
// repo_id: DanielClough/Candle_MistralTrix-v1
// template: 
// base: mistral
// n_safetensors: 4
// tags: gguf,safetensors,