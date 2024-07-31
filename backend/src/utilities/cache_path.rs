use std::{env, path::PathBuf};

use tauri::{api::path::app_cache_dir, Config};

pub fn cache_file_path(slug: &str) -> PathBuf {
    let cache_dir = match env::var_os("USER") {
        Some(value) => {

            if cfg!(target_os = "macos") {
                PathBuf::from(format!("/Users/{}/.cache", value.to_string_lossy()))
            } else if cfg!(target_os = "linux") {
                PathBuf::from(format!("/home/{}/.cache", value.to_string_lossy()))
            } else {
                println!("\n\tUnspported OS!!!\n");
                PathBuf::from(format!("/home/{}/.cache", value.to_string_lossy()))
            }
        },
        None => app_cache_dir(&Config::default()).expect("load huggingface/hub cache dir"),
    };

    let path = format!("huggingface/hub/{}", slug);

    cache_dir.join(path)
}
