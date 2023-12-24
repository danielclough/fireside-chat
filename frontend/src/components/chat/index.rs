use leptonic::prelude::*;
use leptos::{html::Input, *};
use leptos_use::{core::ConnectionReadyState, use_websocket, UseWebsocketReturn};
use web_sys::KeyboardEvent;

use crate::components::utils::get_path;

#[component]
pub fn ChatBox() -> impl IntoView {
    // Load dotenv
    let (history, set_history) = create_signal(vec![]);

    fn update_history(&history: &WriteSignal<Vec<String>>, message: String) {
        let _ = &history.update(|history: &mut Vec<_>| history.push(message));

    }

    fn scroll_down() {
        let el = leptos_dom::document().get_element_by_id("chatbox");
        if let Some(el) = el {
            el.last_element_child().unwrap().scroll_into_view()
        }
    }

    // Instantiate addr websocket_server_address with .env or default values.
    let websocket_server_address = get_path("ws");

    let UseWebsocketReturn {
        ready_state,
        message,
        send,
        open,
        close,
        ..
    } = use_websocket(&websocket_server_address);

    let input_element: NodeRef<Input> = create_node_ref();
    let input_str = "";

    let status = move || ready_state.get().to_string();

    let connected = move || ready_state.get() == ConnectionReadyState::Open;

    let open_connection = move |_| {
        open();
        let input_element = input_element.get().expect("<input> to exist");
        input_element.set_placeholder("Enter Username");
    };
    let close_connection = move |_| {
        close();
        let input_element = input_element.get().expect("<input> to exist");
        input_element.set_placeholder("Enter Username");
    };

    let send_message = move |e: KeyboardEvent| {
        if e.code() == "Enter" {
            let input_element = input_element.get().expect("<input> to exist");
            let input_string = input_element.value();
            send(input_string.as_str());
            input_element.set_placeholder("Enter Prompt");
            input_element.set_value("");
        }
    };

    create_effect(move |_| {
        if let Some(m) = message.get() {
            update_history(&set_history, format! {"{:?}", m});
            scroll_down();
        };
    });

    view! {
        <Box style="padding: 0.5em; display: flex; flex-direction: column; overflow-y: scroll; width: 100%; height: auto;">
            <Box id="btn-row">
                <button
                    style:display=move || (if status() == "Open" {"none"} else {"block"}).to_string()
                    on:click=open_connection disabled=connected
                >
                    "Join"
                </button>
                <button  style:display=move || (if status() == "Closed" {"none"} else {"block"}).to_string() on:click=close_connection disabled=move || !connected()>
                    "Close"
                </button>
                <div>
                    <button
                        on:click=move |_| set_history.set(vec![])
                        disabled=move || history.get().is_empty()
                    >
                        "Clear"
                    </button>
                </div>
            </Box>
            <div id="chatbox" >
                <For
                    each=move || history.get().into_iter().enumerate()
                    key=|(index, _)| *index
                    let:item
                >
                    <div>{item.1.replace("\\n","  ").replace("\\\"","'").replace("\"","")}</div>
                </For>
            </div>

            <input
                type="text"
                name="input"
                placeholder="Enter Username"
                value=input_str
                on:keydown=send_message
                disabled=move || !connected()
                node_ref=input_element
            />
        </Box>
    }
}
