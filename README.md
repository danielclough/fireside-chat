# Axum Mistral Candle

Chat Bot implemented in pure Rust using [Mistral-7B](https://mistral.ai/news/announcing-mistral-7b/) with  [HuggingFace/Candle](https://github.com/huggingface/candle/) over [Axum](https://github.com/tokio-rs/axum) Websockets and a [Leptos](https://www.leptos.dev/) (Wasm) frontend!

## Makefile

If you are using Debian/Ubuntu you should be able to get up and running with a single `make init`.

`make prod` runs both Frontend (Leptos) and Backend (Axum) in with the `--release` flag.

View `make help` to see all commands.

## Backend (Axum)

### Server Config

Websocket defaults to `127.0.0.1:3000` unless `/backend/.env` file includes:
```sh
IPV4=
# and/or
PORT=
```

### Model Config

Default model options can be configured with `/backend/config_model.yaml`.

#### Cuda

In order to configure running the model with `Cuda` adjust
`/backend/config_model.yaml` to say `cpu: false`.

And, enable the cuda feature flags:
```sh
cargo add candle-core -F "cuda"
cargo add candle-transformers -F "cuda"
```

### Inference Config

Default inference options can be configured with `/backend/config_inference.yaml`


### Leptos

Frontend defaults to localhost:8080