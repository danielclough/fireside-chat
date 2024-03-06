use std::path::PathBuf;

use tauri::{api::path::app_cache_dir, Config};

pub fn cache_file_path(slug: &str) -> PathBuf {
    let cache_dir = app_cache_dir(&Config::default()).expect("load huggingface/hub cache dir");
    let path = format!("huggingface/hub/{}", slug);
    cache_dir.join(path)
}
