use common::{
    database::{
        conversation::ConversationWithEngagements,
        engagement::{EngagementForJson, EngagementForJsonVec},
        user::UserForJson,
    },
    llm::{inference::InferenceArgsForInput, model_list::{ModelArgs, ModelDLList}},
};
use leptonic::{{
    prelude::Box,
    stack::Stack,
    typography::{H2, H3, H4},
},
Size,
};
use leptos::*;

use crate::{components::home::overview::{init_model::InitModelModal, init_user::InitUserModal}, functions::rest::conversation::get_conversations_by_user_id};

#[component]
pub fn Overview(
    inference_args: Signal<InferenceArgsForInput>,
    model_args: Signal<ModelArgs>,
    model_list: ReadSignal<ModelDLList>,
    user: Signal<UserForJson>,
    set_user: WriteSignal<UserForJson>,
    ipv4: Signal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
    set_model_args: WriteSignal<ModelArgs>,
) -> impl IntoView {
    let init_conversations = create_resource(
        || (),
        move |_| async move {
            logging::log!("loading conversations_by_user_id from API");
            get_conversations_by_user_id(user.get().id).await
        },
    );
    let (show_user_init_modal, set_show_user_init_modal) = create_signal(user.get().name == *"None" || user.get().name.len() < 2);
    let (show_model_init_modal, _set_show_model_init_modal) = create_signal(model_args.get().clone().template == Some("NoModel".to_string()));

    // let (init_conversations_signal, _set_init_conversations_signal) = create_signal(init_conversations.get());
    view! {
        <Box class="home-container">
            <InitModelModal
                model_args=model_args
                model_list=model_list
                ipv4=ipv4
                show_when=show_model_init_modal
                gpu_type=gpu_type
                set_gpu_type=set_gpu_type
                set_model_args=set_model_args
            />
            <InitUserModal
                set_user=set_user
                user=user
                show_when=show_user_init_modal
                on_accept=move || set_show_user_init_modal.set(false)
                on_cancel=move || set_show_user_init_modal.set(false)
            />
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
                <Box class="args-area">
                    <H3>"Overview"</H3>
                    <ul>
                        <H4>"User: " <code>{user.get().name}</code></H4>
                        <H4>
                            "Role: "
                            <code>
                                {if inference_args.get().role.is_empty() {
                                    "None".to_string()
                                } else {
                                    inference_args.get().role
                                }}

                            </code>
                        </H4>
                    </ul>
                    <H3>"Model"</H3>
                    <ul>
                        <li>
                            <code>{model_args.get().repo_id}</code>
                        </li>
                        <Show
                            when=move || model_args.get().quantized
                            fallback=move || {
                                view! { <li>"cpu: " <code>{model_args.get().cpu}</code></li> }
                            }
                        >

                            <li>"Quantization: " <code>{model_args.get().q_lvl}</code></li>
                        </Show>
                        <li>"revision: " <code>{model_args.get().revision}</code></li>
                        <Show when=move || model_args.get().tokenizer_file.is_some()>
                            <li>tokenizer_file: <code>{model_args.get().tokenizer_file}</code></li>
                        </Show>
                        <Show when=move || model_args.get().weight_file.is_some()>
                            <li>weight_file: <code>{model_args.get().weight_file}</code></li>
                        </Show>

                        <Show when=move || model_args.get().use_flash_attn>
                            <li>"Using Flash Attention!"</li>
                        </Show>
                        <li>"template: " <code>{model_args.get().template}</code></li>
                    </ul>

                    <H3>"Inference"</H3>
                    <ul>
                        <li>
                            "txt / pdf directory: "
                            <code>{format!("{:?}", inference_args.get().load_context)}</code>
                        </li>
                    </ul>
                </Box>
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
