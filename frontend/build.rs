// use std::io::Write;

pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let root_dir: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let generated_dir = root_dir.join("generated");
    // let js_dir = generated_dir.join("js");

    _ = leptonic_theme::generate(generated_dir.join("leptonic"));
    println!("cargo:warning=theme written");

    // std::fs::create_dir_all(js_dir.clone()).unwrap();
    // println!("cargo:warning=js dir created");
}
