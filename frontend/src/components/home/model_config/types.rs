use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ModelArgsForInput {
    pub repo_id: String,
    pub model_config: String,
    pub revision: Option<String>,
    pub tokenizer_file: Option<String>,
    pub weight_files: Option<String>,
    pub use_flash_attn: bool,
    pub quantized: bool,
    pub q_lvl: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ModelArgsForJson {
    pub repo_id: String,
    pub model_config: String,
    pub revision: Option<String>,
    pub tokenizer_file: Option<String>,
    pub weight_files: Option<String>,
    pub use_flash_attn: bool,
    pub quantized: bool,
    pub q_lvl: bool,
}