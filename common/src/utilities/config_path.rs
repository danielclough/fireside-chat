use std::{env, path::PathBuf};

pub fn config_file_path(slug: &str) -> PathBuf {
    let config_dir = app_config_file_path();
    let fireside = "fireside-chat".to_string();
    let config_dir_path = config_dir.join(fireside);
    _ = std::fs::create_dir_all(&config_dir_path);
    config_dir_path.join(slug)
}

pub fn config_file_dir() -> PathBuf {
    let config_dir = app_config_file_path();
    let fireside = "fireside-chat".to_string();
    let path = config_dir.join(fireside);
    _ = std::fs::create_dir_all(&path);
    path
}

pub fn context_file_dir() -> PathBuf {
    let config_dir = app_config_file_path();
    let path = "fireside-chat/context".to_string();
    let path = config_dir.join(path);
    _ = std::fs::create_dir_all(&path);
    path
}


pub fn app_config_file_path() -> PathBuf {
    match env::var_os("USER") {
        Some(value) => {

            if cfg!(target_os = "macos") {
                PathBuf::from(format!("/Users/{}/.config", value.to_string_lossy()))
            } else if cfg!(target_os = "linux") {
                PathBuf::from(format!("/home/{}/.config", value.to_string_lossy()))
            } else {
                println!("\n\tUnspported OS!!!\n");
                PathBuf::from("/root/.cache")
            }
        },
        None => PathBuf::from("/root/.cache"),
    }
}
