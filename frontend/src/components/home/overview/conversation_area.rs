use common::database::{
    conversation::ConversationWithEngagements,
    engagement::{EngagementForJson, EngagementForJsonVec},
    user::UserForJson,
};
use leptonic::{
    components::{
        prelude::Box,
        stack::Stack,
        typography::{H3, H4},
    },
    Size,
};
use leptos::*;

use crate::functions::rest::conversation::get_conversations_by_user_id;

#[component]
pub fn ConversationArea(user: Signal<UserForJson>, database_url: Signal<String>) -> impl IntoView {
    let init_conversations = create_resource(
        || (),
        move |_| async move {
            logging::log!("loading conversations_by_user_id from API");
            get_conversations_by_user_id(user.get().id, database_url.get()).await
        },
    );
    view! {
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
    }
}
