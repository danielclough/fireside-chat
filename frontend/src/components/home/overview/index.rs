use common::{
    database::user::UserForJson,
    llm::{inference::InferenceArgsForInput, model_list::ModelArgs},
};
use leptonic::components::{
    prelude::Box,
    typography::{H2, H3},
};
use leptos::*;

use crate::components::home::overview::{args_area::ArgsArea, conversation_area::ConversationArea};

#[component]
pub fn Overview(
    inference_args: Signal<InferenceArgsForInput>,
    model_args: Signal<ModelArgs>,
    user: Signal<UserForJson>,
    database_url: Signal<String>,
) -> impl IntoView {
    let (conversation_bundle, _) = create_signal((user.get(), database_url.get()));

    let (args_bundle, _) = create_signal((
        inference_args.get(),
        model_args.get(),
        user.get(),
        database_url.get(),
    ));

    view! {
        <Box class="home-container">
            <Box class="wrapper">
                <article class="about-area">
                    <Box id="home-tagline">
                        <img height="250px" width="250px" src="/images/icon.png" alt="logo"/>
                        <div>
                            <blockquote>
                                <H2 style="text-align:center;">"A Blazing LLM Interface"</H2>
                                Implemented in pure Rust using
                                <a class="text-link" href="https://github.com/huggingface/candle/">
                                    HuggingFace/Candle
                                </a>
                                over
                                <a class="text-link" href="https://github.com/tokio-rs/axum">
                                    Axum
                                </a>
                                Websockets, an
                                <a class="text-link" href="https://https://sqlite.org/index.html">
                                    SQLite
                                </a>
                                Database, and a
                                <a class="text-link" href="https://www.leptos.dev/">
                                    Leptos
                                </a>
                                (WASM) frontend packaged with
                                <a class="text-link" href="https://tauri.app">
                                    Tauri
                                </a>
                                for cross platform deployment!
                            </blockquote>
                        </div>
                    </Box>
                    <Box id="home-info">
                        <H3>"Getting Chatting:"</H3>
                        <p>"Start a new chat by clicking the top center \"Start Chat\" button."</p>
                        <H3>"🤗 Have a Great Day! 🤗"</H3>
                        <p>"Do something nice for yourself, or someone else."</p>
                    </Box>
                </article>

                <Show
                    when=move || {
                        args_bundle.get()
                            == (
                                inference_args.get(),
                                model_args.get(),
                                user.get(),
                                database_url.get(),
                            )
                    }

                    fallback=move || {
                        view! {
                            <ArgsArea
                                inference_args=inference_args
                                model_args=model_args
                                user=user
                            />
                        }
                    }
                >

                    <ArgsArea inference_args=inference_args model_args=model_args user=user/>
                </Show>
                <Show
                    when=move || { conversation_bundle.get() == (user.get(), database_url.get()) }

                    fallback=move || {
                        view! { <ConversationArea database_url=database_url user=user/> }
                    }
                >

                    <ConversationArea database_url=database_url user=user/>
                </Show>
            </Box>
        </Box>
    }
}
