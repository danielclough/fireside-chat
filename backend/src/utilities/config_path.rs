use std::path::PathBuf;

use tauri::{api::path::app_config_dir, Config};

pub fn config_file_path(slug: &str) -> PathBuf {
    let config_dir = app_config_dir(&Config::default()).expect("load tauri config");
    let fireside = "fireside-chat".to_string();
    let config_dir_path = config_dir.join(fireside);
    _ = std::fs::create_dir_all(&config_dir_path);
    config_dir_path.join(slug)
}

pub fn config_file_dir() -> PathBuf {
    let config_dir = app_config_dir(&Config::default()).expect("load tauri config");
    let fireside = "fireside-chat".to_string();
    let path = config_dir.join(fireside);
    _ = std::fs::create_dir_all(&path);
    path
}

pub fn context_file_dir() -> PathBuf {
    let config_dir = app_config_dir(&Config::default()).expect("load tauri config");
    let path = "fireside-chat/context".to_string();
    let path = config_dir.join(path);
    _ = std::fs::create_dir_all(&path);
    path
}
