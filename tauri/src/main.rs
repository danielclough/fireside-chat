// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::server::chat::app;
use database::server::db;
struct Port(u16);

fn main() {
    let localhost = "127.0.0.1";
    let backend_url = std::option_env!("FIRESIDE_BACKEND_URL")
        .unwrap_or(localhost)
        .to_string();
    let port = 16981;

    if backend_url == localhost {
        println!("\nStarting Server on Localhost\n");
        tauri::async_runtime::spawn(app(backend_url, port));
    } else {
        println!("\nExpecting Server on {}\n", &backend_url);
    };

    println!("\nStarting Database on Localhost\n");
    tauri::async_runtime::spawn(db());

    tauri::Builder::default()
        .manage(Port(port))
        .run(tauri::generate_context!())
        .expect("start tauri");
}
