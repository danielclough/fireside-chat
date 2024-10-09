use codee::string::FromToStringCodec;
use common::database::{
    conversation::{ConversationForJson, NewConversation},
    engagement::NewEngagement,
    user::UserForJson,
};
use html::Input;
use leptonic::components::{
    prelude::Box,
    toast::{Toast, ToastTimeout, ToastVariant, Toasts},
};
use leptos::{html::Textarea, *};
use leptos_use::{use_websocket, UseWebSocketReturn};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::{
    components::chat::message::ChatMessage,
    functions::{
        chat::{chat_message_state, scroll_down, update_history, ChatMessageState},
        get_path::get_llm_path,
        rest::{
            conversation::post_new_conversation, engagement::post_new_engagement,
            user::get_user_by_name,
        },
    },
};

#[component]
pub fn ChatBox(
    user: Signal<UserForJson>,
    backend_url: Signal<String>,
    set_home_view: WriteSignal<bool>,
    database_url: Signal<String>,
) -> impl IntoView {
    let (history, set_history) = create_signal::<Vec<String>>(vec![]);
    let (username_unset, set_username_unset) = create_signal(true);
    let (response, set_response) = create_signal::<Option<String>>(None);
    let (query, set_query) = create_signal::<Option<String>>(None);
    let (conversation, set_conversation) = create_signal::<Option<ConversationForJson>>(None);
    let (input_string, set_input_string) = create_signal(String::new());
    let (time_to_send, set_time_to_send) = create_signal(false);
    let (text_area_height, set_text_area_height) = create_signal(1);

    // chat_message_state_signal
    let (chat_message_state_signal, set_chat_message_state_signal) =
        create_signal(ChatMessageState::Waiting);

    // Toast send query success or error
    let toasts = expect_context::<Toasts>();

    // Instantiate addr websocket_server_address with .env or default values.
    let websocket_server_address = get_llm_path("websocket", backend_url.get());

    let UseWebSocketReturn {
        ready_state,
        message,
        send,
        // open,
        // close,
        ..
    } = use_websocket::<String, String, FromToStringCodec>(&websocket_server_address);

    let input_element: NodeRef<Input> = create_node_ref();
    let textarea_element: NodeRef<Textarea> = create_node_ref();

    let status = move || ready_state.get().to_string();

    // let connected = move || ready_state.get() == ConnectionReadyState::Open;

    // let open_connection = move |_| {
    //     open();
    //     let input_element = input_element.get().expect("<input> to exist");
    //     input_element.set_placeholder("Enter Username");
    //     input_element.set_value(&user.get().name)
    // };
    // let close_connection = move |_| {
    //     close();
    //     let input_element = input_element.get().expect("<input> to exist");
    //     input_element.set_placeholder("Enter Username");
    //     input_element.set_value(&user.get().name)
    // };

    let send_message = move |e: KeyboardEvent| {
        leptos_dom::log!("{}","Key Pressed");
        if e.key() == "Enter" && !e.shift_key() {
            leptos_dom::log!("{}","Enter; No Shift");
            if username_unset.get() {
                leptos_dom::log!("{}","Username UnSet; Register Name;");
                let input_element = input_element.get().expect("<input> to exist");
                set_input_string.set(input_element.value());
                set_time_to_send.set(true);
                leptos_dom::log!("time to send? {}",time_to_send.get());
                input_element.set_value("");
            } else {
                leptos_dom::log!("{}","Username Set; Send Message;");
                let textarea_element = textarea_element.get().expect("<input> to exist");
                set_input_string.set(textarea_element.value());
                set_time_to_send.set(true);
                leptos_dom::log!("time to send? {}",time_to_send.get());
                textarea_element.set_value("");
            }
        } else if e.key() == "Enter" && e.shift_key() {
            leptos_dom::log!("{}","Enter; With Shift");
            let textarea_element = textarea_element.get().expect("<input> to exist");
            let n_lines = textarea_element.value().split("\\n").collect::<String>().len();
            set_text_area_height.set(n_lines + 2)
        }
    };

    // // send when time_to_send
    create_effect(move |_| {

        // Send
        leptos_dom::log!("Send/Register Effect");
        leptos_dom::log!("Ready to send? {}", time_to_send.get());
        // register username
        if username_unset.get() {
            leptos_dom::log!("Trying to register username? {}", username_unset.get());
            send(&user.get().name);
            leptos_dom::log!("user: {:?}", user.get());
            set_time_to_send.set(false);
        } else if time_to_send.get() {
            // send regular messages
            leptos_dom::log!("Sending now! {}", time_to_send.get());
            send(&input_string.get());
            set_input_string.set(String::new());
            set_time_to_send.set(false);
        }

    });

    // Return to home view if connection closes
    create_effect(move |_| {
        if status() == "Closed" {
            set_home_view.set(true);
        }
    });

    create_effect(move |_| {

        // Message
        if message.get().is_none() {
            return;
        }
        leptos_dom::log!("Message: {:?}", message.get());
        if let Some(m) = message.get() {
            set_chat_message_state_signal.set(chat_message_state(&m, user.get().name));

            match chat_message_state_signal.get() {
                ChatMessageState::Join => {
                    leptos_dom::log!("Register Username!");
                    set_username_unset.set(false);
                }
                ChatMessageState::Coming => {
                    toasts.push(Toast {
                        id: Uuid::new_v4(),
                        created_at: time::OffsetDateTime::now_utc(),
                        variant: ToastVariant::Success,
                        header: "Received!".to_owned().into_view(),
                        body: "Your response will be sent shortly!".to_owned().into_view(),
                        timeout: ToastTimeout::DefaultDelay,
                    });
                }
                ChatMessageState::User => {
                    // add query to state
                    logging::log!("Set Query");
                    set_query.set(Some(m.to_string()));

                    // TODO -- Add params as csv
                    if conversation.get().is_none() {
                        // New Conversation if None
                        spawn_local(async move {
                            logging::log!("New Conversation");
                            let remove_string = format!("{}: ", user.get().name);
                            let new_conversation = post_new_conversation(
                                user.get().id,
                                NewConversation {
                                    name: query
                                        .get()
                                        .expect("query state is some")
                                        .replace(&remove_string, ""),
                                    user_id: user.get().id,
                                    model_params: String::new(),
                                    inference_params: String::new(),
                                },
                                database_url.get(),
                            )
                            .await;

                            // prepare for message by setting values
                            logging::log!("Set Conversation");
                            set_conversation.set(Some(new_conversation));
                        });
                    }
                }
                ChatMessageState::Error => {
                    leptos_dom::log!("Message: {}", m);
                    toasts.push(Toast {
                        id: Uuid::new_v4(),
                        created_at: time::OffsetDateTime::now_utc(),
                        variant: ToastVariant::Error,
                        header: query.get_untracked().into_view(),
                        body: response.get_untracked().into_view(),
                        timeout: ToastTimeout::DefaultDelay,
                    });
                    leptos_dom::log!("Query is some: {}", query.get().is_some());
                    // push to screen but don't add to db
                    if query.get().is_some() {
                        update_history(&set_history, query.get().unwrap(), user);
                    };
                    leptos_dom::log!("Response is some: {}", response.get().is_some());
                    if response.get().is_some() {
                        update_history(&set_history, response.get().unwrap(), user);
                    };
                    // prepare for message by clearing values
                    set_response.set(None);
                    set_query.set(None);
                    scroll_down();
                }
                ChatMessageState::Bot => {
                    // add response to state
                    set_response.set(Some(m.to_string()));
                    if conversation.get().is_none() {
                        // New Conversation if None
                    } else {
                        // New Engagement if Conversation is Some
                        spawn_local(async move {
                            if response.get().is_some() {
                                logging::log!("Add Engagement");
                                let _new_engagement = post_new_engagement(
                                    NewEngagement {
                                        conversation_id: conversation.get().unwrap().id,
                                        query: query.get().expect("query state is some"),
                                        response: response.get().expect("query state is some"),
                                    },
                                    database_url.get(),
                                )
                                .await;
                                // Spawn in order to do last
                                spawn_local(async move {
                                    if query.get().is_some() && response.get().is_some() {
                                        // push to screen
                                        update_history(&set_history, query.get().unwrap(), user);
                                        update_history(&set_history, response.get().unwrap(), user);

                                        logging::log!("Reset Values in Engagement section");
                                        // prepare for message by clearing values
                                        logging::log!("Set Response");
                                        set_response.set(None);
                                        logging::log!("Set Query");
                                        set_query.set(None);

                                        // Scroll into view
                                        scroll_down();
                                    }
                                })
                            }
                        });
                    }
                }
                _ => {
                    println!("Error")
                }
            }
        };
    });

    let user_resource = create_resource(
        move || (database_url.get(), backend_url.get(), user.get()),
        move |_| async move {
            logging::log!("get_user_by_name");
            get_user_by_name(user.get().name, database_url.get()).await
        },
    );

    view! {
        <Transition>
            {move || {
                // leptos_dom::log!("{:?}",user_resource);
                user_resource
                    .get()
                    .map(|user_from_resource| {
                        let input_str = user_from_resource.name;
                        view! {
                            <Box class="outer-container">
                                // moving here allows it work
                                <div class="hidden">{move || time_to_send}</div>
                                <div id="chat-box" class="outer-container">
                                    <For
                                        each=move || history.get().into_iter().enumerate()
                                        key=|(index, _)| *index
                                        let:item
                                    >
                                        <ChatMessage user=user index=item.0 text=item.1/>
                                    </For>
                                </div>

                                <Show
                                    when=move || !username_unset.get()
                                    fallback= move || view! {
                                        <div>
                                            "Register your Username on Server!"
                                        </div>
                                        <input
                                            rows=move || text_area_height.get()
                                            type="text"
                                            name="input"
                                            autofocus
                                            id="chat-box-input"
                                            placeholder=format!("{}, Press Enter to register your name!", user.get().name)
                                            value=user.get().name
                                            on:keydown=send_message
                                            disabled=move || {
                                                chat_message_state_signal.get() == ChatMessageState::Coming
                                            }
                                            node_ref=input_element
                                        />
                                    }
                                >
                                    <div>
                                        {format!("Chatting as: {}", user.get().name)}
                                    </div>
                                    <textarea
                                        rows=move || text_area_height.get()
                                        type="text"
                                        name="input"
                                        autofocus
                                        id="chat-box-input"
                                        placeholder="What is the airspeed velocity of an unladen swallow?"
                                        value=input_str.clone()
                                        on:keydown=send_message
                                        disabled=move || {
                                            chat_message_state_signal.get() == ChatMessageState::Coming
                                        }
                                        node_ref=textarea_element
                                    ></textarea>
                                </Show>
                                <Box id="btn-row">
                                    // <button
                                    // style:display=move || {
                                    // (if status() == "Open" { "none" } else { "block" }).to_string()
                                    // }

                                    // on:click=open_connection
                                    // disabled=connected
                                    // >
                                    // "Join"
                                    // </button>
                                    // <button
                                    // style:display=move || {
                                    // (if status() == "Closed" { "none" } else { "block" }).to_string()
                                    // }

                                    // on:click=close_connection
                                    // disabled=move || !connected()
                                    // >
                                    // "Close"
                                    // </button>
                                    <div>
                                        <button
                                            on:click:undelegated=move |_| {
                                                set_history.set(vec![]);
                                                set_conversation.set(None)
                                            }

                                            disabled=move || history.get().is_empty()
                                            style=move || {
                                                if history.get().is_empty() {
                                                    "display:none;"
                                                } else {
                                                    "position: absolute;right: 0;top: 0;"
                                                }
                                            }
                                        >

                                            "New Conversation"
                                        </button>
                                    </div>

                                    {move || {
                                        if !username_unset.get() && conversation.get().is_none() {
                                            "Ready to Go!".to_string()
                                        } else if username_unset.get() {
                                            String::new()
                                        } else {
                                            format!("Title: {}", conversation.get().unwrap().name)
                                        }
                                    }}

                                </Box>
                            </Box>
                        }
                    })
            }}

        </Transition>
    }
}
