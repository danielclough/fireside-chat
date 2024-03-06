use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct InferenceArgsForInput {
    pub temperature: f64,
    pub top_p: f64,
    pub seed: f64,
    pub sample_len: f64,
    pub repeat_penalty: f64,
    pub repeat_last_n: f64,
    pub load_context: bool,
    pub role: String,
}

impl Default for InferenceArgsForInput {
    fn default() -> Self {
        Self {
            temperature: 0.0,
            top_p: 0.0,
            seed: 0.0,
            sample_len: 0.0,
            repeat_penalty: 0.0,
            repeat_last_n: 0.0,
            load_context: false,
            role: String::new(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct InferenceArgsForJson {
    pub temperature: f64,
    pub top_p: f64,
    pub seed: u64,
    pub sample_len: usize,
    pub repeat_penalty: f32,
    pub repeat_last_n: usize,
    pub load_context: bool,
    pub role: String,
}

// Load context from backend/context/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LoadContext {
    True,
    False,
}
