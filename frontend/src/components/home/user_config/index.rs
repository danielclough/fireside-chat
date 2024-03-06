use common::database::user::UserForJson;
use leptonic::typography::{H2, P};
use leptos::{html::Input, *};
use web_sys::KeyboardEvent;

use crate::functions::rest::user::switch_users;

#[component]
pub fn UserConfig(
    active_user: UserForJson,
    user: Signal<UserForJson>,
    set_user: WriteSignal<UserForJson>,
    database_url: Signal<String>,
) -> impl IntoView {
    {
        // Set user to value returned from DB
        set_user.set(active_user.clone());
    }

    let input_element: NodeRef<Input> = create_node_ref();
    let input_str = String::new();

    let update_user = move |e: KeyboardEvent| {
        if e.code() == "Enter" {
            let input_element = input_element.get().expect("<input> to exist");
            let input_string = input_element.value();
            input_element.set_value("");

            switch_users(user, set_user, input_string, database_url);
        }
    };

    view! {
        <H2>"User Config"</H2>

        <P class="above-input">
            <strong>"User: "</strong>
            "Conversations are organized by user."
        </P>

        <input
            type="text"
            name="input"
            placeholder="Change Username"
            value=input_str
            on:keydown=update_user
            node_ref=input_element
        />

        <P class="under-input">"Current User: " {move || user.get().name}</P>
    }
}
