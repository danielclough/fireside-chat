# Candle Chat

A multi-user chat bot implemented in pure Rust using [Mistral-7B](https://mistral.ai/news/announcing-mistral-7b/) with  [HuggingFace/Candle](https://github.com/huggingface/candle/) over [Axum](https://github.com/tokio-rs/axum) Websockets and a [Leptos](https://www.leptos.dev/) (Wasm) frontend!

Watch the introduction video:
![Watch the video](https://i9.ytimg.com/vi_webp/Jw1E3LnNG0o/mqdefault.webp?v=656ae3d8&sqp=CKiWuqsG&rs=AOn4CLCAvtmHYvP2ZsOMl7VzTrcih2MexA)

> This project is a WIP.
> The `main` branch should mostly work, but until I setup up automated testing I expect things to break without being caught.
> Sorry about that. 🤗
>
> I am pretty busy through the holiday season, but I hope to have more time to play with this project in the new year.
>
> I will be adding more model options soon-ish.

## Goals

My primary goal is to showcase the awesome power and simplicity of HuggingFace/Candle.

## Setup / Operation

### Debian/Ubuntu

```sh
# install make and git
sudo apt-get install git make

# clone with ssh
git clone git@github.com:danielclough/candle_chat.git
# or,
# clone with https
git clone https://github.com/danielclough/candle_chat.git

# 0) apt-get install
# 1) install rust if not available
# 2) install wasm target if not available
# 3) install trunk if not available
# 4) install cargo-watch if not available
make init

# run release binaries
make prod

# kill running processes
make kill
```

`make prod` runs both Frontend (Leptos) and Backend (Axum) in with the `--release` flag.

View `make help` to see all commands.

## Limitations

In the future I may add a CLI tool for simplifying the setup experience.
For now there will be two binaries in two project folders, one for the Frontend and one for the Backend.
This layout is a consequence of Trunk not working well with workspaces ([Trunk Issue](https://github.com/thedodd/trunk/issues/575#issuecomment-1693471972)).


## Backend (Axum)

You can use yaml files to configure model and inference parameters, or use the defaults.

### Server Config

Backend defaults to `127.0.0.1:3000`.

You can alter this by copying `/backend/.env-example` to `/backend/.env` and setting your desired config there.

### Inference Config

Default inference options can be configured with `/backend/config_inference.yaml`

### Model Config

Default model options can be configured with `/backend/config_model.yaml`.

### Cuda

Running with `Cuda`` is the default configuration.

 - `/backend/config_model.yaml` must include `cpu: false`.

And, enable the cuda feature flags must be enabled:

```sh
cargo add candle-core -F "cuda"
cargo add candle-transformers -F "cuda"
```

## Frontend (Leptos)

Each user has their own chat history.

You can use the sidebar to adjust inference parameters.

The Frontend is static for simple deployment on platforms such as Github Pages, or on a server with Trunk.

## Trunk

This project serves static files with [Trunk](https://trunkrs.dev/).

Here is a link to the [Trunk config on Github](https://github.com/thedodd/trunk/blob/master/Trunk.toml).

### `.env` Config

Frontend server defaults to `127.0.0.1:8080`

You can alter this by copying `/frontend/.env-example` to `/frontend/.env` and setting your desired config there.

Since the frontend and backend are designed to run separately you must keep the backend Port and IPV4 in sync!


## Development

