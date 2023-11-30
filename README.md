# Axum Mistral Candle

Chat Bot implemented in pure Rust using [Mistral-7B](https://mistral.ai/news/announcing-mistral-7b/) with  [HuggingFace/Candle](https://github.com/huggingface/candle/) over [Axum](https://github.com/tokio-rs/axum) Websockets and a [Leptos](https://www.leptos.dev/) (Wasm) frontend!

> This project is a WIP.
> The `main` branch should mostly work, but until I setup up automated testing I expect things to break without being caught.
> Sorry about that. 🤗
>
> I am pretty busy through the holiday season, but I hope to have more time to play with this project in the new year.

## Makefile

If you are using Debian/Ubuntu you should be able to get up and running with a single `make init`.

`make prod` runs both Frontend (Leptos) and Backend (Axum) in with the `--release` flag.

View `make help` to see all commands.

## Backend (Axum)

### Server Config

Backend defaults to `127.0.0.1:3000`.

You can alter this by copying `/backend/.env-example` to `/backend/.env` and setting your desired config there.

### Model Config

Default model options can be configured with `/backend/config_model.yaml`.

#### Cuda

Running with `Cuda`` is the default configuration.

 - `/backend/config_model.yaml` must include `cpu: false`.

And, enable the cuda feature flags must be enabled:

```sh
cargo add candle-core -F "cuda"
cargo add candle-transformers -F "cuda"
```

### Inference Config

Default inference options can be configured with `/backend/config_inference.yaml`


## Frontend (Leptos)

### Trunk

This project serves static files with [Trunk](https://trunkrs.dev/).

Here is a link to the [Trunk config on Github](https://github.com/thedodd/trunk/blob/master/Trunk.toml).

### `.env` Config

Frontend server defaults to `127.0.0.1:8080`

You can alter this by copying `/frontend/.env-example` to `/frontend/.env` and setting your desired config there.

Since the frontend and backend are designed to run separately you must keep the backend Port and IPV4 in sync!