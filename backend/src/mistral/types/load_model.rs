use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::mistral::{Config, Model as Mistral};
use candle_transformers::models::quantized_mistral::Model as QMistral;

use anyhow::{Error as E, Result};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::tokenizer::Tokenizer;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum Model {
    Mistral(Mistral),
    Quantized(QMistral),
}

#[derive(Clone, Debug)]
pub struct ModelTokenizerDevice {
    pub model: Model,
    pub tokenizer: Tokenizer,
    pub device: Device,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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
    /// Save new './config_model.yaml'
    pub fn save_args(args: LoadModel) -> LoadModel {
        let yaml = serde_yaml::to_string(&args).expect("to string");
        std::fs::write("./config_model.yaml", yaml).expect("save file");
        LoadModel { ..args }
    }

    /// Default config to prevent failure.
    /// Will load "./config_model.yaml" if available.
    pub fn load_current_args() -> LoadModel {
        tracing::debug!("Loading './config_model.yaml' or Default Config.");

        let config_model_string = std::fs::read_to_string("./config_model.yaml");
        if config_model_string.is_ok() {
            let unwrapped = &config_model_string.unwrap();
            serde_yaml::from_str(unwrapped.as_str()).unwrap()
        } else {
            LoadModel {
                cpu: false,
                use_flash_attn: false,
                model_id: "lmz/candle-mistral".to_string(),
                revision: "main".to_string(),
                tokenizer_file: None,
                weight_files: None,
                quantized: false,
            }
        }
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

        println!("retrieved the files in {:?}", start.elapsed());

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
            println!("Device: {:?}", device);

            let dtype = if device.is_cuda() {
                DType::BF16
            } else {
                DType::F32
            };
            println!("Dtype: {:?}", dtype);
            let vb = unsafe { VarBuilder::from_mmaped_safetensors(&filenames, dtype, &device)? };
            let model = Mistral::new(&config, vb)?;
            println!("Model Loaded");
            (Model::Mistral(model), device)
        };

        println!("loaded the model in {:?}", start.elapsed());

        let model_args_out = ModelTokenizerDevice {
            model,
            tokenizer,
            device,
        };

        Ok(model_args_out)
    }
}
