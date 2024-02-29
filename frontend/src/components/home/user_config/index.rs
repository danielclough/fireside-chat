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
    set_refresh_token: WriteSignal<i32>,
) -> impl IntoView {
    let (init_user, set_init_user) = create_signal(active_user.clone());
    {
        // Set user to value returned from DB
        set_user.set(active_user.clone());
        set_init_user.set(active_user);
    }

    let input_element: NodeRef<Input> = create_node_ref();
    let input_str = String::new();

    create_effect(move |_| {
        if user.get().name != init_user.get().name {
            set_refresh_token.update(|x| *x += 1);
        }
    });

    let update_user = move |e: KeyboardEvent| {
        if e.code() == "Enter" {
            let input_element = input_element.get().expect("<input> to exist");
            let input_string = input_element.value();
            input_element.set_value("");

            switch_users(user, set_user, input_string);
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
