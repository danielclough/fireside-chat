use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
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

        let inference_args_string = std::fs::read_to_string("./config_inference.yaml");
        if inference_args_string.is_ok() {
            let unwrapped = &inference_args_string.unwrap();
            serde_yaml::from_str(unwrapped.as_str()).unwrap()
        } else {
            InferenceArgs {
                tracing: true,
                temperature: Some(0.2),
                top_p: Some(1f64),
                seed: 299792458,
                sample_len: 500,
                repeat_penalty: 1.3,
                repeat_last_n: 150,
            }
        }
    }
}
