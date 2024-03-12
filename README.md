# Fireside Chat

(prev. "Candle Chat")

An LLM interface implemented in pure Rust using [HuggingFace/Candle](https://github.com/huggingface/candle/) over [Axum](https://github.com/tokio-rs/axum) Websockets, an [SQLite](https://https://sqlite.org/index.html) Database, and a [Leptos](https://www.leptos.dev/) (Wasm) frontend packaged with [Tauri](https://tauri.app)!

Watch the introduction video:
[![Watch the video](https://img.youtube.com/vi/Jw1E3LnNG0o/0.jpg)](https://youtu.be/Jw1E3LnNG0o)


## Goals

This project is designed for single and multi-user chat with many Large Language Models (LLMs).

### Features

- Local or Remote Inference Backend
- Local SQLite Database


## Setup / Operation

You can configure your model and default inference settings by putting files in your `Config Directory`.
This is automatically configured when you choose a model in the frontend, but you can manually add models if you like.

Example:

```yaml
# config_model.yaml
repo_id: DanielClough/Candle_Puffin-Phi-v2
q_lvl: q2k
revision: main
tokenizer_file: null
weight_file: null
quantized: true
cpu: false
use_flash_attn: false
template: ShareGPT
```

```yaml
# config_inference.yaml
temperature: 
top_p: 
seed: 299792458
sample_len: 150
repeat_penalty: 1.3
repeat_last_n: 150
load_context: false
role: 
```

If `load_context: true` then you can add (small) in `<Config Directory>/fireside-chat/context/`.
Large files may cause Out Of Memory errors.

### Linux

`Config Directory` is `$XDG_CONFIG_HOME` or `$HOME/.config`

### macOS

`Config Directory` is `$HOME/Library/Application Support`

### Windows

`Config Directory` is `{FOLDERID_RoamingAppData}`

## Development

You can compile with environment variable the `FIRESIDE_BACKEND_URL`, and `FIRESIDE_DATABASE_URL` to call a server other than `localhost`.

This can be configured in `tauri.conf.json`, or in your system environment.

```sh
# eg. for Linux
export FIRESIDE_BACKEND_URL=192.168.1.6 && trunk serve
```

## Limitations

- I am not testing in Mac or Windows environments, so while everything may work fine I could use some help in order to ensure correctness on those systems.