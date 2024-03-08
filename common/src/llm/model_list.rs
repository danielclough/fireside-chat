use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelListEntry {
    pub name: String,
    pub repo_id: String,
    pub base: String,
    pub template: String,
    pub n_safetensors: i8,
    pub tags: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Iterable)]
pub struct ModelList {
    pub list: Vec<ModelListEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDLList {
    pub list: Vec<ModelDLListEntry>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModelDLListEntry {
    pub name: String,
    pub repo_id: String,
    pub base: String,
    pub n_safetensors: i8,
    pub template: Option<Vec<String>>,
    pub gguf: bool,
    pub safetensors: bool,
    pub bin: bool,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ModelArgs {
    pub repo_id: String,
    pub q_lvl: String,
    pub revision: String,
    pub tokenizer_file: Option<String>,
    pub weight_file: Option<String>,
    pub quantized: bool,
    pub use_flash_attn: bool,
    pub cpu: bool,
    pub template: Option<String>,
}

impl Default for ModelArgs {
    fn default() -> Self {
        Self {
            repo_id: "NoModel".to_string(),
            q_lvl: "q5k".to_string(),
            revision: String::new(),
            tokenizer_file: None,
            weight_file: None,
            quantized: false,
            use_flash_attn: false,
            cpu: false,
            template: Some("NoModel".to_string()),
        }
    }
}
impl ModelDLList {
    pub fn default() -> ModelDLList {
        ModelDLList { list: vec![] }
    }
    pub fn error() -> ModelDLList {
        ModelDLList { list: vec![] }
    }
}

impl ModelArgs {
    pub fn error() -> ModelArgs {
        ModelArgs {
            repo_id: "LLM Backend Error".to_string(),
            q_lvl: "LLM Backend Error".to_string(),
            revision: "LLM Backend Error".to_string(),
            tokenizer_file: Some("LLM Backend Error".to_string()),
            weight_file: Some("LLM Backend Error".to_string()),
            quantized: false,
            use_flash_attn: false,
            cpu: false,
            template: Some("LLM Backend Error".to_string()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tags {
    pub gguf: bool,
    pub safetensors: bool,
    pub bin: bool,
}

#[derive(Clone, Debug)]
pub enum GPU {
    CUDA,
    Mac,
    Intel,
    AMD,
}
