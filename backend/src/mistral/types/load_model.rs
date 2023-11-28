use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::mistral::{Config, Model as Mistral};
use candle_transformers::models::quantized_mistral::Model as QMistral;

use anyhow::{Error as E, Result};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::tokenizer::Tokenizer;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum Model {
    Mistral(Mistral),
    Quantized(QMistral),
}

#[derive(Clone)]
pub struct ModelTokenizerDevice {
    pub model: Model,
    pub tokenizer: Tokenizer,
    pub device: Device,
}

#[derive(Deserialize, PartialEq, Debug, Serialize, Clone)]
pub struct LoadModel {
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
impl LoadModel {
    /// Default config to prevent failure.
    /// Will load "./config_model.yaml" if available.
    pub fn load_args() -> LoadModel {
        tracing::debug!("Loading './config_model.yaml' or Default Config.");

        serde_yaml::from_str(
            std::fs::read_to_string("./config_model.yaml")
                .unwrap()
                .as_str(),
        )
        .unwrap_or(LoadModel {
            cpu: false,
            use_flash_attn: false,
            model_id: "lmz/candle-mistral".to_string(),
            revision: "main".to_string(),
            tokenizer_file: None,
            weight_files: None,
            quantized: false,
        })
    }
    pub fn load(args: LoadModel) -> Result<ModelTokenizerDevice> {
        let start = std::time::Instant::now();
        let api = Api::new()?;
        let repo = api.repo(Repo::with_revision(
            args.model_id,
            RepoType::Model,
            args.revision,
        ));
        let tokenizer_filename = match args.tokenizer_file {
            Some(file) => std::path::PathBuf::from(file),
            None => repo.get("tokenizer.json")?,
        };
        let filenames = match args.weight_files {
            Some(files) => files
                .split(',')
                .map(std::path::PathBuf::from)
                .collect::<Vec<_>>(),
            None => {
                if args.quantized {
                    vec![repo.get("model-q4k.gguf")?]
                } else {
                    vec![
                        repo.get("pytorch_model-00001-of-00002.safetensors")?,
                        repo.get("pytorch_model-00002-of-00002.safetensors")?,
                    ]
                }
            }
        };

        tracing::debug!("retrieved the files in {:?}", start.elapsed());

        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

        let start = std::time::Instant::now();
        let config = Config::config_7b_v0_1(args.use_flash_attn);
        let (model, device) = if args.quantized {
            let filename = &filenames[0];
            let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf(filename)?;
            let model = QMistral::new(&config, vb)?;
            (Model::Quantized(model), Device::Cpu)
        } else {
            let device = candle_examples::device(args.cpu)?;
            let dtype = if device.is_cuda() {
                DType::BF16
            } else {
                DType::F32
            };
            let vb = unsafe { VarBuilder::from_mmaped_safetensors(&filenames, dtype, &device)? };
            let model = Mistral::new(&config, vb)?;
            (Model::Mistral(model), device)
        };

        tracing::debug!("loaded the model in {:?}", start.elapsed());

        let model_args_out = ModelTokenizerDevice {
            model,
            tokenizer,
            device,
        };

        Ok(model_args_out)
    }
}