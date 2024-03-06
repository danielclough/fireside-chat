use serde::{Deserialize, Serialize};

use crate::utilities::config_path::{config_file_path, context_file_dir};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InferenceArgs {
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
    /// Load context from backend/context/*
    pub load_context: bool,
    /// Load Role if Template supports it.
    pub role: Option<String>,
}

impl InferenceArgs {
    /// Default config to prevent failure.
    /// Will load "$HOME/.config/fireside-chat/config_inference.yaml" if available.
    pub fn load_current_args() -> InferenceArgs {
        // Tauri config dir
        let config_path = config_file_path("config_inference.yaml");
        let config_inference_string = std::fs::read_to_string(config_path);

        let inference_args = if config_inference_string.is_ok() {
            println!("Loading '$HOME/.config/fireside-chat/config_inference.yaml'");
            let unwrapped = &config_inference_string.unwrap();
            serde_yaml::from_str(unwrapped.as_str()).unwrap()
        } else {
            println!("Loading Default InferenceArgs");

            InferenceArgs {
                temperature: None,
                top_p: None,
                seed: 299792458,
                sample_len: 150,
                repeat_penalty: 1.3,
                repeat_last_n: 150,
                load_context: false,
                role: None,
            }
        };
        if inference_args.load_context {
            let dir = context_file_dir();
            println!("Loading (small) files from: \n{:?}\n", dir);
        };
        inference_args
    }
    /// Save new './config_inference.yaml'
    pub fn save_args(args: InferenceArgs) -> InferenceArgs {
        // Tauri config dir
        let yaml = serde_yaml::to_string(&args).expect("to string");
        let config_path = config_file_path("config_inference.yaml");
        std::fs::write(config_path, yaml).expect("save file");
        println!("saving {:#?} to config_inference.yaml", args);
        InferenceArgs { ..args }
    }
    // pub fn new() -> InferenceArgs {
    //     tracing::debug!("Loading '$HOME/.config/fireside-chat/config_inference.yaml' or Default Config.");

    //     let config_path = config_file_path("config_inference.yaml");
    //     let inference_args_string = std::fs::read_to_string(config_path);
    //     if inference_args_string.is_ok() {
    //         let unwrapped = &inference_args_string.unwrap();
    //         serde_yaml::from_str(unwrapped.as_str()).unwrap()
    //     } else {
    //         InferenceArgs {
    //             temperature: None,
    //             top_p: None,
    //             seed: 299792458,
    //             sample_len: 150,
    //             repeat_penalty: 1.3,
    //             repeat_last_n: 150,
    //             load_context: false,
    //             role: None
    //         }
    //     }
    // }
}
