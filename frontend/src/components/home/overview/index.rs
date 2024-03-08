use common::{
    database::{
        conversation::ConversationWithEngagements,
        engagement::{EngagementForJson, EngagementForJsonVec},
        user::UserForJson,
    },
    llm::{inference::InferenceArgsForInput, model_list::ModelArgs},
};
use leptonic::{
    Size,
    {
        prelude::Box,
        stack::Stack,
        typography::{H2, H3, H4},
    },
};
use leptos::*;

use crate::{components::home::overview::args_area::ArgsArea, functions::rest::conversation::get_conversations_by_user_id};

#[component]
pub fn Overview(
    inference_args: Signal<InferenceArgsForInput>,
    model_args: Signal<ModelArgs>,
    user: Signal<UserForJson>,
    database_url: Signal<String>,
) -> impl IntoView {

    let bundle = (inference_args.get(), model_args.get(), user.get(), database_url.get());
    
    let init_conversations = create_resource(
        || (),
        move |_| async move {
            logging::log!("loading conversations_by_user_id from API");
            get_conversations_by_user_id(user.get().id, database_url.get()).await
        },
    );

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

                <Show when= move || bundle == (inference_args.get(), model_args.get(), user.get(), database_url.get())
                    fallback=  move || view! {
                        <ArgsArea 
                            inference_args=inference_args
                            model_args=model_args
                            user=user
                        />
                    }
                >
                    <ArgsArea 
                        inference_args=inference_args
                        model_args=model_args
                        user=user
                    />
                </Show>

                <Box class="conversation-area">
                    <H3>Conversations</H3>
                    <Stack style="justify-content: start;align-items:start;" spacing=Size::Em(0.6)>
                        <For
                            each=move || {
                                init_conversations
                                    .get()
                                    .unwrap_or(
                                        vec![
                                            ConversationWithEngagements {
                                                id: 0,
                                                name: String::new(),
                                                engagements: EngagementForJsonVec {
                                                    list: vec![
                                                        EngagementForJson {
                                                            id: 0,
                                                            conversation_id: 0,
                                                            query: String::new(),
                                                            response: String::new(),
                                                        },
                                                    ],
                                                },
                                                user_id: 0,
                                                model_params: String::new(),
                                                inference_params: String::new(),
                                            },
                                        ],
                                    )
                                    .clone()
                            }

                            key=|list| list.clone()
                            let:item
                        >
                            <div style="background: var(--secondary-background);padding: .25rem .5rem; width:100%;">
                                <H4>{item.name}</H4>
                                <For
                                    each=move || item.engagements.list.clone()
                                    key=|list| list.clone()
                                    let:engagement
                                >
                                    <div style="background: var(--tertiary-background);padding: .1rem .5rem .5rem .1rem; width:100%;">
                                        <p style="margin-top:0; padding: 0 .5rem;">
                                            <strong>{engagement.clone().query}</strong>
                                            <hr/>

                                            <span>{engagement.clone().response}</span>
                                        </p>
                                    </div>
                                </For>
                            </div>
                        </For>
                    </Stack>
                </Box>
                <footer class="home-footer">
                    <ul>
                        <li>
                            <a
                                class="footer-link"
                                href="https://github.com/danielclough/fireside-chat"
                                target="_black"
                                referer="norefer"
                            >
                                Docs
                            </a>
                        </li>
                        <li>
                            <a
                                class="footer-link"
                                href="https://github.com/danielclough/fireside-chat"
                                target="_black"
                                referer="norefer"
                            >
                                Code
                            </a>
                        </li>
                        <li>
                            <a
                                class="footer-link"
                                href="https://github.com/danielclough/fireside-chat/blob/main/LICENSE"
                                target="_black"
                                referer="norefer"
                            >
                                License
                            </a>
                        </li>
                    </ul>
                </footer>
            </Box>
        </Box>
    }
}
