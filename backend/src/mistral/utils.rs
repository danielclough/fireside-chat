use candle_transformers::models::mistral::{ Config, Model as Mistral };
use candle_transformers::models::quantized_mistral::Model as QMistral;
use candle_core::{ DType, Device };
use candle_nn::VarBuilder;

use hf_hub::{ api::sync::Api, Repo, RepoType };
use tokenizers::tokenizer::Tokenizer;
use anyhow::{ Error as E, Result };

use super::types::{ Model, ModelTokenizerDevice, ArgsToLoadModel };

pub fn load_model(args: ArgsToLoadModel) -> Result<ModelTokenizerDevice> {
    let start = std::time::Instant::now();
    let api = Api::new()?;
    let repo = api.repo(Repo::with_revision(args.model_id, RepoType::Model, args.revision));
    let tokenizer_filename = match args.tokenizer_file {
        Some(file) => std::path::PathBuf::from(file),
        None => repo.get("tokenizer.json")?,
    };
    let filenames = match args.weight_files {
        Some(files) => files.split(',').map(std::path::PathBuf::from).collect::<Vec<_>>(),
        None => {
            if args.quantized {
                vec![repo.get("model-q4k.gguf")?]
            } else {
                vec![
                    repo.get("pytorch_model-00001-of-00002.safetensors")?,
                    repo.get("pytorch_model-00002-of-00002.safetensors")?
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
        let dtype = if device.is_cuda() { DType::BF16 } else { DType::F32 };
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&filenames, dtype, &device)? };
        let model = Mistral::new(&config, vb)?;
        (Model::Mistral(model), device)
    };

    tracing::debug!("loaded the model in {:?}", start.elapsed());

    let model_args_out = ModelTokenizerDevice { model, tokenizer, device };

    Ok(model_args_out)
}
