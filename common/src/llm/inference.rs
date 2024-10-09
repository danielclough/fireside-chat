use std::{fmt::Display, str::FromStr};

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

impl FromStr for InferenceArgsForInput {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
impl Display for InferenceArgsForInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
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
