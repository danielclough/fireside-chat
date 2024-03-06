use std::{collections::HashSet, sync::Mutex};
use tokio::sync::broadcast;

use crate::llm::inference_args::InferenceArgs;
use crate::llm::load_model::{LoadModel, ModelTokenizerDevice};

// Chat shared state
pub struct AppState {
    /// We require unique usernames.
    /// This tracks which usernames have been taken.
    pub user_set: Mutex<HashSet<String>>,
    /// Channel used to send messages to all connected clients.
    pub broadcast_sender: broadcast::Sender<String>,
    /// Share the model, tokenizer, and device into the app.
    pub model_tokenizer_device: Mutex<ModelTokenizerDevice>,
    pub inference_args: Mutex<InferenceArgs>,
    pub model_args: Mutex<LoadModel>,
}
