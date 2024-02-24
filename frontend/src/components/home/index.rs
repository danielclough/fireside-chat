use common::{
    database::{
        conversation::ConversationWithEngagements,
        engagement::{EngagementForJson, EngagementForJsonVec},
        user::UserForJson,
    },
    llm::{inference::InferenceArgsForInput, model_list::{ModelArgs, ModelDLList}},
};
use leptonic::prelude::*;
use leptos::*;

use crate::{components::home::{init_model::InitModelModal, init_user::InitUserModal}, functions::rest::conversation::get_conversations_by_user_id};

#[component]
pub fn Home(
    inference_args: Signal<InferenceArgsForInput>,
    model_args: ModelArgs,
    user: Signal<UserForJson>,
    set_user: WriteSignal<UserForJson>,
    model_list: ModelDLList,
    ipv4: Signal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>
) -> impl IntoView {
    let init_conversations = create_resource(
        || (),
        move |_| async move {
            logging::log!("loading model_list from API");
            get_conversations_by_user_id(user.get().id).await
        },
    );
    let (show_user_init_modal, set_show_user_init_modal) = create_signal(user.get().name == "None".to_string() || user.get().name.len() < 2);
    let (show_model_init_modal, _set_show_model_init_modal) = create_signal(model_args.clone().template == Some("NoModel".to_string()));

    // let (init_conversations_signal, _set_init_conversations_signal) = create_signal(init_conversations.get());
    view! {
        <Box class="outer-container">
            <InitModelModal
                model_args=model_args.clone()
                model_list=model_list
                ipv4=ipv4
                show_when=show_model_init_modal
                gpu_type=gpu_type
                set_gpu_type=set_gpu_type
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
                        <H3>"Change Preferences:"</H3>
                        <p>"Set preferences by clicking the top left toggle switch."</p>
                        <H3>"Getting Chatting:"</H3>
                        <p>"Start a new chat by clicking the top center \"Start Chat\" button."</p>
                        <H3>"🤗 Have a Great Day! 🤗"</H3>
                        <p>"Do something nice for yourself, or someone else."</p>
                    </Box>
                </article>
                <aside class="args-area">
                    <H3>"User Parameters"</H3>
                    <ul>
                        <li>"Username: " {user.get().name}</li>
                    </ul>
                    <H3>"Model Parameters"</H3>
                    <ul>
                        <li>
                            "repo_id: "
                            {if model_args.template == Some("NoModel".to_string()) {
                                "None".to_string()
                            } else {
                                model_args.repo_id
                            }}

                        </li>
                        <li>"q_lvl: " {model_args.q_lvl}</li>
                        <li>"revision: " {model_args.revision}</li>
                        <li>tokenizer_file: {model_args.tokenizer_file}</li>
                        <li>weight_file: {model_args.weight_file}</li>
                        <li>"quantized: " {model_args.quantized}</li>
                        <li>"use_flash_attn: " {model_args.use_flash_attn}</li>
                        <li>"template: " {model_args.template}</li>
                    </ul>

                    <H3>"Inference Parameters"</H3>
                    <ul>
                        <li>"role: " {inference_args.get().role}</li>
                        <li>"temperature: " {inference_args.get().temperature}</li>
                        <li>"top_p: " {inference_args.get().top_p}</li>
                        <li>"seed: " {inference_args.get().seed}</li>
                        <li>"sample_len: " {inference_args.get().sample_len}</li>
                        <li>"repeat_penalty: " {inference_args.get().repeat_penalty}</li>
                        <li>"repeat_last_n: " {inference_args.get().repeat_last_n}</li>
                        <li>
                            "load_context: " {format!("{:?}", inference_args.get().load_context)}
                        </li>
                    </ul>
                </aside>
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
