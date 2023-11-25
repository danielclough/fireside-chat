use std::sync::Mutex;

mod mistral;
use mistral::types::ArgsToLoadModel;
use mistral::utils::load_model;

mod websockets;
use websockets::server::server;

#[tokio::main]
async fn main() {
    // Load Mistral
    let config_string = std::fs
        ::read_to_string("./config_model.yaml")
        .expect("Load config_model.yaml");
    let args_to_load_model: ArgsToLoadModel = serde_yaml
        ::from_str(config_string.as_str())
        .expect("config_model.yaml to struct");
    let model_args = Mutex::new(load_model(args_to_load_model).unwrap());

    // Start server
    server(model_args).await;
}
