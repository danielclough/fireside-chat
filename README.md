# Axum Mistral Candle

Chat Bot implemented in pure Rust using [Mistral-7B](https://mistral.ai/news/announcing-mistral-7b/) with  [HuggingFace/Candle](https://github.com/huggingface/candle/) over [Axum](https://github.com/tokio-rs/axum) Websockets and a [Leptos](https://www.leptos.dev/) (Wasm) frontend!

## Makefile

If you are using Debian/Ubuntu you should be able to get up and running with a single `make init`.

View `make help` to see all commands.

## Running Server

### Axum

Websocket is on localhost:3000

### Leptos

Frontend is on localhost:8080