use common::database::user::UserForJson;
use leptonic::components::typography::{H2, P};
use leptos::{html::Input, *};
use web_sys::KeyboardEvent;

use crate::functions::rest::user::{get_user_by_name, switch_users};

#[component]
pub fn UserConfig(
    user: Signal<UserForJson>,
    set_user: WriteSignal<UserForJson>,
    database_url: Signal<String>,
) -> impl IntoView {
    let input_element: NodeRef<Input> = create_node_ref();
    let (input_signal, set_input_signal) = create_signal(String::new());

    let user_resource = create_resource(
        || (),
        move |_| async move {
            logging::log!("loading user from API");
            get_user_by_name(user.get().name, database_url.get()).await
        },
    );

    let update_user = move |e: KeyboardEvent| {
        if e.code() == "Enter" {
            let input_element = input_element.get().expect("<input> to exist");
            set_input_signal.set(input_element.value());
            input_element.set_value("");

            switch_users(user, set_user, input_signal.get(), database_url);
        }
    };

    view! {
        <Transition fallback=move || {
            view! { <p>"Initializing..."</p> }
        }>
            {move || {
                user_resource
                    .get()
                    .map(|user_from_resource| {
                        set_user.set(user_from_resource);
                        view! {
                            <H2>"User Config"</H2>

                            <P class="above-input">
                                <strong>"User: "</strong>
                                "Conversations are organized by user."
                            </P>

                            <input
                                type="text"
                                name="input"
                                placeholder="Choose Username"
                                value=input_signal
                                on:keydown=update_user
                                node_ref=input_element
                            />

                            <P class="under-input">"Current User: " {move || user.get().name}</P>
                        }
                    })
            }}

        </Transition>
    }
}
