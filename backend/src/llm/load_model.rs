#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;

impl LoadModel {
    /// Default config to prevent failure.
    /// Will load "$HOME/.config/fireside-chat/config_model.yaml" if available.
    pub fn load_current_args() -> LoadModel {
        // Tauri config dir
        let config_path = config_file_path("config_model.yaml");
        let config_model_string = std::fs::read_to_string(config_path);

        if config_model_string.is_ok() {
            println!("Loading '$HOME/.config/fireside-chat/config_model.yaml'");
            let unwrapped = &config_model_string.unwrap();
            serde_yaml::from_str(unwrapped.as_str()).unwrap()
        } else {
            println!("Loading Default Config");

            // Template NoModel on start for fast load.
            LoadModel {
                use_flash_attn: false,
                repo_id: "NoModel".to_string(),
                template: Some("NoModel".to_string()),
                q_lvl: "q5k".to_string(),
                revision: "main".to_string(),
                tokenizer_file: None,
                weight_file: None,
                quantized: true,
                cpu: true,
            }
        }
    }
    pub fn get_model_config_type(repo_id: &str) -> Result<String> {
        // get current model from list
        let model_list = get_default_list();
        let current_model = model_list
            .list
            .iter()
            .find(|x| x.repo_id == repo_id)
            .expect("find model");
        Ok(current_model.template.clone())
    }
    pub fn download(args: LoadModel) -> Result<(PathBuf, Vec<PathBuf>, Option<PathBuf>)> {
        // println!("{:?}", args);
        let start = std::time::Instant::now();
        let api = Api::new()?;
        let repo = api.repo(Repo::with_revision(
            args.repo_id.clone(),
            RepoType::Model,
            args.revision,
        ));

        let full_list = get_default_list().list;
        let has_repo = full_list.iter().any(|x| x.repo_id == args.repo_id);
        let current_model: Option<ModelListEntry> = if has_repo {
            Some(Self::current_model(&args.repo_id))
        } else {
            None
        };
        let tokenizer_filename: PathBuf = if current_model.is_some() {
            match args.tokenizer_file {
                Some(config) => std::path::PathBuf::from(config),
                _ => match current_model.unwrap().base.as_str() {
                    "llama2" => {
                        let api = api.model("hf-internal-testing/llama-tokenizer".to_string());
                        api.get("tokenizer.json")?
                    }
                    _ => repo.get("tokenizer.json")?,
                },
            }
        } else {
            panic!("Model Not Found!")
        };

        let filenames = match args.weight_file {
            Some(files) => files
                .split(',')
                .map(std::path::PathBuf::from)
                .collect::<Vec<_>>(),
            None => {
                if args.quantized {
                    let prefix = args.repo_id.split('/').collect::<Vec<&str>>();
                    let model_name = format!("{}_{}.gguf", prefix[1], args.q_lvl);
                    vec![repo.get(&model_name)?]
                } else {
                    get_safetensors(&repo, &args.repo_id)?
                }
            }
        };

        println!("retrieved the files in {:?}", start.elapsed());
        Ok((tokenizer_filename, filenames, None))
    }
    pub fn download_with_config(
        args: LoadModel,
    ) -> Result<(PathBuf, Vec<PathBuf>, Option<PathBuf>)> {
        let start = std::time::Instant::now();

        let api = Api::new()?;
        let revision = args.revision;
        let repo = api.repo(Repo::with_revision(
            args.repo_id.clone(),
            RepoType::Model,
            revision,
        ));

        let tokenizer_filename = repo.get("tokenizer.json")?;
        let config_filename = repo.get("config.json")?;

        let filenames = if args.quantized {
            let prefix = args.repo_id.split('/').collect::<Vec<&str>>();
            let model_name = format!("{}_{}.gguf", prefix[1], args.q_lvl);
            vec![repo.get(&model_name)?]
        } else {
            get_safetensors(&repo, &args.repo_id)?
        };

        println!("retrieved the files in {:?}", start.elapsed());
        Ok((tokenizer_filename, filenames, Some(config_filename)))
    }
    pub fn current_model(repo_id: &str) -> ModelListEntry {
        let model_list: ModelList = get_default_list();
        model_list
            .list
            .iter()
            .find(|x| x.repo_id.to_lowercase() == repo_id.to_lowercase())
            .expect("find model")
            .to_owned()
    }
    pub fn load(args: LoadModel, no_model: bool) -> Result<ModelTokenizerDevice> {
        let current_model: ModelListEntry = if no_model {
            ModelListEntry {
                name: "NoModel".to_string(),
                repo_id: "NoModel".to_string(),
                base: "NoModel".to_string(),
                template: "NoModel".to_string(),
                n_safetensors: 0,
                tags: "NoModel".to_string(),
            }
        } else {
            Self::current_model(&args.repo_id)
        };

        let (tokenizer_filename, filenames, config_filename) = match current_model.base.as_str() {
            "llama" => Self::download_with_config(args.clone())?,
            "phi" => Self::download_with_config(args.clone())?,
            "NoModel" => {
                let dummy_path = config_file_path("NoModel");
                std::fs::write(dummy_path.clone(), "NoModel").expect("save file");
                (dummy_path.clone(), vec![dummy_path], None)
            }
            _ => Self::download(args.clone())?,
        };

        println!("building the model");
        let device = candle_examples::device(args.cpu)?;

        let dtype = match current_model.base.as_str() {
            "llama" => DType::F16,
            "mistral" => {
                if device.is_cuda() {
                    DType::BF16
                } else {
                    DType::F32
                }
            }
            _ => DType::F32,
        };

        let (model, cache) = if no_model {
            println!("Setting up NoModel");
            let m = NoModel {
                value: Tensor::new(&[1f32, 2.], &device)?,
            };
            (Model::NoModel(m), None)
        } else {
            println!("Setting up config for {}", current_model.base);
            match current_model.base.as_str() {
                "llama" => {
                    let config: LlamaConfig =
                        serde_json::from_slice(&std::fs::read(config_filename.unwrap())?)?;
                    let config = config.into_config(args.use_flash_attn);
                    let vb_args = unsafe {
                        VarBuilder::from_mmaped_safetensors(&filenames, DType::F16, &device)?
                    };
                    let cache = LlamaCache::new(false, dtype, &config, &device)?;
                    (
                        Model::Llama(Llama::load(vb_args, &config)?),
                        Some(Cache::LlamaCache(cache)),
                    )
                }
                "llama2" => {
                    if args.quantized {
                        let vb = QLlama2VB::from_gguf(&filenames[0], &device)?;
                        let (_vocab_size, dim) = vb
                            .get_no_shape("model.embed_tokens.weight")?
                            .shape()
                            .dims2()?;
                        let config = match dim {
                            64 => Llama2Config::tiny_260k(),
                            288 => Llama2Config::tiny_15m(),
                            512 => Llama2Config::tiny_42m(),
                            768 => Llama2Config::tiny_110m(),
                            _ => anyhow::bail!("no config for dim {dim}"),
                        };
                        let freq_cis_real = vb
                            .get(
                                (config.seq_len, config.head_size() / 2),
                                "rot.freq_cis_real",
                            )?
                            .dequantize(&device)?;
                        let freq_cis_imag = vb
                            .get(
                                (config.seq_len, config.head_size() / 2),
                                "rot.freq_cis_imag",
                            )?
                            .dequantize(&device)?;

                        let fake_vb = candle_nn::VarBuilder::from_tensors(
                            [
                                ("freq_cis_real".to_string(), freq_cis_real),
                                ("freq_cis_imag".to_string(), freq_cis_imag),
                            ]
                            .into_iter()
                            .collect(),
                            candle_core::DType::F32,
                            &device,
                        );
                        let cache = Llama2Cache::new(true, &config, fake_vb)?;

                        (
                            Model::QLlama2(QLlama2::load(vb, config.clone())?),
                            Some(Cache::Llama2Cache(cache)),
                        )
                    } else {
                        let config = Llama2Config::tiny_15m();
                        let tensors = candle_core::safetensors::load(&filenames[0], &device)?;
                        let vb = candle_nn::VarBuilder::from_tensors(
                            tensors,
                            candle_core::DType::F32,
                            &device,
                        );
                        let cache = Llama2Cache::new(true, &config, vb.pp("rot"))?;

                        (
                            Model::Llama2(Llama2::load(vb, config.clone())?),
                            Some(Cache::Llama2Cache(cache)),
                        )
                    }
                    // else {
                    //     let mut file = std::fs::File::open(config_path)?;
                    //     let config = Llama2Config::from_reader(&mut file)?;
                    //     println!("{config:?}");
                    //     let weights = TransformerWeights::from_reader(&mut file, &config, &device)?;
                    //     let vb = weights.var_builder(&config, &device)?;
                    //     let cache = Llama2Cache::new(true, &config, vb.pp("rot"))?;

                    //     Model::Llama2(Llama2::load(vb, &cache, config.clone())?)
                    // }
                }
                "mistral" => {
                    let model_config = Self::get_model_config_type(&args.repo_id)?;
                    let model_config_str = Some(model_config.as_str());

                    let config = match model_config_str {
                        Some("ChatML") | Some("Teknium") => MistralConfig::config_chat_ml(args.use_flash_attn),
                        Some("Amazon") => {
                            MistralConfig::config_amazon_mistral_lite(args.use_flash_attn)
                        }
                        _ => MistralConfig::config_7b_v0_1(args.use_flash_attn),
                    };
                    if args.quantized {
                        let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf(
                            &filenames[0],
                            &device,
                        )?;
                        (Model::QMistral(QMistral::new(&config, vb)?), None)
                    } else {
                        // is_safetensors
                        let vb_args = unsafe {
                            VarBuilder::from_mmaped_safetensors(&filenames, dtype, &device)?
                        };
                        (Model::Mistral(Mistral::new(&config, vb_args)?), None)
                    }
                }
                // "phi"
                _ => {
                    let (model, cache) = if args.quantized {
                        println!("match config");
                        let config = match args.repo_id.as_str() {
                            "DanielClough/Candle_phi-1" => MixformerConfig::v1(),
                            "DanielClough/Candle_phi-1_5" => MixformerConfig::v1_5(),
                            "DanielClough/Candle_Puffin-Phi-v2" => MixformerConfig::puffin_phi_v2(),
                            "DanielClough/Candle_Phi-Hermes-1.3B" => {
                                MixformerConfig::phi_hermes_1_3b()
                            }
                            // "DanielClough/Candle_phi-2" | "DanielClough/Candle_phi-2_old"
                            _ => MixformerConfig::v2(),
                        };
                        println!("vb");
                        let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf(
                            &filenames[0],
                            &device,
                        )?;

                        println!("match model");
                        let model = match args.repo_id.as_str() {
                            "DanielClough/Candle_phi-2" | "DanielClough/Candle_phi-2_old" => {
                                QMixFormer::new_v2(&config, vb)?
                            }
                            _ => QMixFormer::new(&config, vb)?,
                        };
                        (Model::QuantizedPhi(model), None)
                    } else {
                        // is_safetensors
                        let vb_args = unsafe {
                            VarBuilder::from_mmaped_safetensors(&filenames, dtype, &device)?
                        };
                        let model = match args.repo_id.as_str() {
                            "DanielClough/Candle_phi-1"
                            | "DanielClough/Candle_phi-1_5"
                            | "DanielClough/Candle_Puffin-Phi-v2" => {
                                let config = std::fs::read_to_string(config_filename.unwrap())?;
                                let config: PhiConfig = serde_json::from_str(&config)?;

                                Model::Phi(Phi::new(&config, vb_args)?)
                            }
                            "DanielClough/Candle_Phi-Hermes-1.3B" => {
                                let config = MixformerConfig::phi_hermes_1_3b();
                                Model::MixFormer(MixFormer::new(&config, vb_args)?)
                            }
                            // "DanielClough/Candle_phi-2"
                            _ => {
                                let config = std::fs::read_to_string(config_filename.unwrap())?;
                                let config = serde_json::from_str(&config)?;
                                let phi = Phi::new(&config, vb_args)?;
                                Model::Phi(phi)
                            }
                        };
                        (model, None)
                    };
                    println!("Model Loaded");
                    (model, cache)
                }
            }
        };

        let tokenizer = if no_model {
            None
        } else {
            Some(Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?)
        };

        let model_config = if no_model {
            None
        } else {
            Some(Self::get_model_config_type(&args.repo_id)?)
        };

        let model_args_out = ModelTokenizerDevice {
            model,
            model_cache: cache,
            model_config,
            template: args.template,
            tokenizer,
            device,
        };

        Ok(model_args_out)
    }
    /// Save new './config_model.yaml'
    pub fn save_args(args: LoadModel) -> LoadModel {
        // Tauri config dir
        let yaml = serde_yaml::to_string(&args).expect("to string");
        let config_path = config_file_path("config_model.yaml");
        std::fs::write(config_path, yaml).expect("save file");
        println!("saving {:#?} to config_model.yaml", args);
        LoadModel { ..args }
    }
}

fn get_safetensors(repo: &ApiRepo, repo_id: &str) -> Result<Vec<PathBuf>> {
    // get current model from list
    let model_list = get_default_list();
    let current_model = model_list
        .list
        .iter()
        .find(|x| x.repo_id == repo_id)
        .expect("find model");
    // create vec tgo return
    let mut return_vec = vec![];
    // get safetensors
    for n in 1..(current_model.n_safetensors + 1) {
        let get_string = format!(
            "model-0000{}-of-0000{}.safetensors",
            n, current_model.n_safetensors
        );
        println!("getting safetensors: {}", get_string);
        return_vec.push(repo.get(&get_string)?);
    }
    Ok(return_vec)
}
#[derive(Clone)]
pub enum Config {
    LlamaConfig(LoadLlamaConfig),
    Llama2Config(Llama2Config),
    MistralConfig(MistralConfig),
    PhiConfig(PhiConfig),
    MixformerConfig(MixformerConfig),
}

#[derive(Clone)]
pub enum Model {
    Llama(Llama),
    Llama2(Llama2),
    QLlama2(QLlama2),
    Mistral(Mistral),
    QMistral(QMistral),
    MixFormer(MixFormer),
    Phi(Phi),
    QuantizedPhi(QMixFormer),
    NoModel(NoModel),
}

#[derive(Clone)]
pub enum Cache {
    LlamaCache(LlamaCache),
    Llama2Cache(Llama2Cache),
    // QLlama2Cache(QLlama2Cache),
}

#[derive(Clone)]
pub struct ModelTokenizerDevice {
    pub model: Model,
    pub model_cache: Option<Cache>,
    pub model_config: Option<String>,
    pub template: Option<String>,
    pub tokenizer: Option<Tokenizer>,
    pub device: Device,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LoadModel {
    /// HuggingFace repo Id
    pub repo_id: String,
    /// Quantization level
    pub q_lvl: String,
    /// HuggingFace model revision
    pub revision: String,
    /// Optional tokenizer file
    pub tokenizer_file: Option<String>,
    /// Optional weight files
    pub weight_file: Option<String>,
    /// Use quantized model
    pub quantized: bool,
    /// Use CPU not GPU
    pub cpu: bool,
    /// Use FlashAttention to enhance memory efficiency
    pub use_flash_attn: bool,
    /// Optional template format
    pub template: Option<String>,
}

#[derive(Clone)]
pub struct NoModel {
    pub value: Tensor,
}

use crate::server::rest::model_list::get_default_list;
use crate::utilities::config_path::config_file_path;
use common::llm::model_list::{ModelList, ModelListEntry};

use anyhow::{Error as E, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use hf_hub::api::sync::ApiRepo;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;

use candle_transformers::models::{
    llama::{Cache as LlamaCache, Config as LoadLlamaConfig, Llama, LlamaConfig},
    llama2_c::{Cache as Llama2Cache, Config as Llama2Config, Llama as Llama2},
    mistral::{Config as MistralConfig, Model as Mistral},
    mixformer::{Config as MixformerConfig, MixFormerSequentialForCausalLM as MixFormer},
    phi::{Config as PhiConfig, Model as Phi},
    quantized_llama2_c::{QLlama as QLlama2, VarBuilder as QLlama2VB},
    quantized_mistral::Model as QMistral,
    quantized_mixformer::MixFormerSequentialForCausalLM as QMixFormer,
};
