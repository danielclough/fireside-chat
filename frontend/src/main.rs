#[macro_use]
extern crate dotenv_codegen;

use leptos::*;

mod app;
mod components;
use app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App /> })
}
