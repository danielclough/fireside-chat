use common::database::user::UserForJson;
use leptos::*;

#[component]
pub fn ChatMessage(user: Signal<UserForJson>, index: usize, text: String) -> impl IntoView {
    let user_response = index % 2 == 0;
    let bot_response = index % 2 != 0;

    let (username_signal, _set_username_signal) = create_signal(user.get().name);
    let (username_prefix_signal, _set_username_prefix_signal) =
        create_signal(format!("{}: ", user.get().name));
    let (text_signal, _set_text_signal) = create_signal(text);

    let text_vec: Vec<String> = if user_response {
        text_signal
            .get()
            .replace(username_prefix_signal.get().as_str(), "")
            .split("\\n")
            .map(|x| x.replace("\\\"", "'").replace('\"', "").to_string())
            .collect()
    } else {
        text_signal
            .get()
            .replace("Bot: ", "")
            .split("\\n")
            .map(|x| x.replace("\\\"", "'").replace('\"', "").to_string())
            .collect()
    };

    let (text_vec_signal, _set_text_vec_signal) = create_signal(text_vec);

    view! {
        <Show when=move || user_response>
            <div class="chat-user-response-tab">{username_signal.get()}</div>
            <div class="chat-user-response">
                <For each=move || text_vec_signal.get() key=|list| list.clone() let:engagement>
                    <p>{engagement}</p>
                </For>
            </div>
        </Show>

        <Show when=move || bot_response>
            <div class="chat-bot-response-tab">"Bot"</div>
            <div class="chat-bot-response">
                <For each=move || text_vec_signal.get() key=|list| list.clone() let:engagement>
                    <p>{engagement}</p>
                </For>
            </div>
        </Show>
    }
}
