use leptos::{html::Input, *};
use leptos_use::{core::ConnectionReadyState, use_websocket, UseWebsocketReturn};
use web_sys::KeyboardEvent;

#[component]
fn WebSocket() -> impl IntoView {
    let (history, set_history) = create_signal(vec![]);

    fn update_history(&history: &WriteSignal<Vec<String>>, message: String) {
        let _ = &history.update(|history: &mut Vec<_>| history.push(message));
    }

    let UseWebsocketReturn {
        ready_state,
        message,
        send,
        open,
        close,
        ..
    } = use_websocket("ws://localhost:3000/websocket");

    let input_element: NodeRef<Input> = create_node_ref();
    let input_str = "";

    let status = move || ready_state().to_string();

    let connected = move || ready_state.get() == ConnectionReadyState::Open;

    let open_connection = move |_| {
        open();
        let input_element = input_element().expect("<input> to exist");
        input_element.set_placeholder("Enter Username");
    };
    let close_connection = move |_| {
        close();
        let input_element = input_element().expect("<input> to exist");
        input_element.set_placeholder("Enter Username");
    };

    let send_message = move |e: KeyboardEvent| {
        if e.code() == "Enter" {
            let input_element = input_element().expect("<input> to exist");
            let input_string = input_element.value();
            send(input_string.as_str());
            input_element.set_placeholder("Enter Prompt");
            input_element.set_value("");

            let el = leptos_dom::document().get_element_by_id("chatbox");
            if let Some(el) = el {
                el.last_element_child().unwrap().scroll_into_view()
            }
        }
    };

    create_effect(move |_| {
        if let Some(m) = message.get() {
            update_history(&set_history, format! {"{:?}", m});
        };
    });

    view! {
        <div>
            <div id="btn-row">
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
                        on:click=move |_| set_history(vec![])
                        disabled=move || history.get().is_empty()
                    >
                        "Clear"
                    </button>
                </div>
            </div>
            <div id="chatbox">
                <For
                    each=move || history.get().into_iter().enumerate()
                    key=|(index, _)| *index
                    let:item
                >
                    <div>{item.1}</div>
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
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(WebSocket)
}
