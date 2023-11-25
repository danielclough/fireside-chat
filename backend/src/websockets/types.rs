use std::{ collections::HashSet, sync::Mutex };
use tokio::sync::broadcast;

use crate::mistral::types::ModelTokenizerDevice;

// Our shared state
pub struct AppState {
    /// We require unique usernames.
    /// This tracks which usernames have been taken.
    pub user_set: Mutex<HashSet<String>>,
    /// Channel used to send messages to all connected clients.
    pub tx: broadcast::Sender<String>,
    /// Share the model, tokenizer, and device into the app.
    pub model_args: Mutex<ModelTokenizerDevice>,
}
