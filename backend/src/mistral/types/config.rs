use candle_transformers::models::mistral::Model as Mistral;
use candle_transformers::models::quantized_mistral::Model as QMistral;

use tokenizers::tokenizer::Tokenizer;

use candle_core::Device;
use serde::Deserialize;

#[derive(Clone)]
pub enum Model {
    Mistral(Mistral),
    Quantized(QMistral),
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct InferenceArgs {
    /// Enable tracing (generates a trace-timestamp.json file).
    pub tracing: bool,
    /// The temperature used to generate samples.
    pub temperature: Option<f64>,
    /// Nucleus sampling probability cutoff.
    pub top_p: Option<f64>,
    /// The seed to use when generating random samples.
    pub seed: u64,
    /// The length of the sample to generate (in tokens).
    pub sample_len: usize,
    /// Penalty to be applied for repeating tokens, 1. means no penalty.
    pub repeat_penalty: f32,
    /// The context size to consider for the repeat penalty.
    pub repeat_last_n: usize,
}
impl InferenceArgs {
    /// Default config to prevent failure.
    /// Will load './config_inference.yaml' if available.
    pub fn new() -> InferenceArgs {
        tracing::debug!("Loading './config_inference.yaml' or Default Config.");

        serde_yaml::from_str(
            std::fs::read_to_string("./config_inference.yaml")
                .unwrap()
                .as_str(),
        )
        .unwrap_or(InferenceArgs {
            tracing: true,
            temperature: Some(0.2),
            top_p: Some(1f64),
            seed: 299792458,
            sample_len: 500,
            repeat_penalty: 1.3,
            repeat_last_n: 150,
        })
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ArgsToLoadModel {
    /// HuggingFace model Id
    pub model_id: String,
    /// HuggingFace model revision
    pub revision: String,
    /// Optional tokenizer file
    pub tokenizer_file: Option<String>,
    /// Optional weight files
    pub weight_files: Option<String>,
    /// Use quantized model
    pub quantized: bool,
    /// Use FlashAttention to enhance memory efficiency
    pub use_flash_attn: bool,
    /// Run on CPU rather than on GPU.
    pub cpu: bool,
}
impl ArgsToLoadModel {
    /// Default config to prevent failure.
    /// Will load "./config_model.yaml" if available.
    pub fn new() -> ArgsToLoadModel {
        tracing::debug!("Loading './config_model.yaml' or Default Config.");

        serde_yaml::from_str(
            std::fs::read_to_string("./config_model.yaml")
                .unwrap()
                .as_str(),
        )
        .unwrap_or(ArgsToLoadModel {
            cpu: false,
            use_flash_attn: false,
            model_id: "lmz/candle-mistral".to_string(),
            revision: "main".to_string(),
            tokenizer_file: None,
            weight_files: None,
            quantized: false,
        })
    }
}

pub struct ModelTokenizerDevice {
    pub model: Model,
    pub tokenizer: Tokenizer,
    pub device: Device,
}
