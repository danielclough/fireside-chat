use leptonic::components::{
    button::Button,
    typography::{H2, P},
};
use leptos::{html::Input, *};
use web_sys::KeyboardEvent;

#[component]
pub fn NetworkConfig(
    backend_url: Signal<String>,
    set_backend_url: WriteSignal<String>,
    database_url: Signal<String>,
    set_database_url: WriteSignal<String>,
    set_database_error: WriteSignal<bool>,
    set_backend_error: WriteSignal<bool>,
    set_show_network_init_modal: WriteSignal<bool>,
) -> impl IntoView {
    let localhost = "127.0.0.1";
    let hosted_database = "chat-database.danielc.us";
    let hosted_backend = "chat-backend.danielc.us";
    let backend_url_init = std::option_env!("FIRESIDE_BACKEND_URL").unwrap_or(localhost);
    let database_url_init = std::option_env!("FIRESIDE_DATABASE_URL").unwrap_or(localhost);

    let backend_input_element: NodeRef<Input> = create_node_ref();
    let database_input_element: NodeRef<Input> = create_node_ref();
    let backend_input = String::new();
    let database_input = String::new();

    let update_backend_url = move |e: KeyboardEvent| {
        if e.code() == "Enter" {
            let input_element = backend_input_element.get().expect("<input> to exist");
            set_backend_url.set(input_element.value());
            input_element.set_value("");
            reset_if_ready(
                backend_url,
                database_url,
                set_database_error,
                set_backend_error,
                set_show_network_init_modal,
            );
        }
    };

    let update_database_url = move |e: KeyboardEvent| {
        if e.code() == "Enter" {
            let input_element = database_input_element.get().expect("<input> to exist");
            set_database_url.set(input_element.value());
            input_element.set_value("");
            reset_if_ready(
                backend_url,
                database_url,
                set_database_error,
                set_backend_error,
                set_show_network_init_modal,
            );
        }
    };

    view! {
        <H2>"Network Config"</H2>

        <P class="above-input">
            <strong>"Network Backend: "</strong>
        </P>

        <Button
            class="network-button"
            on_press=move |_| {
                set_backend_url.set(localhost.to_string());
                reset_if_ready(
                    backend_url,
                    database_url,
                    set_database_error,
                    set_backend_error,
                    set_show_network_init_modal,
                );
            }
        >

            "Localhost"
        </Button>

        <Button
            class="network-button"
            on_press=move |_| {
                set_backend_url.set(hosted_backend.to_string());
                reset_if_ready(
                    backend_url,
                    database_url,
                    set_database_error,
                    set_backend_error,
                    set_show_network_init_modal,
                );
            }
        >

            {hosted_backend}
        </Button>

        <Show when=move || {
            backend_url_init != hosted_backend && backend_url_init != localhost
                && backend_url_init != ""
        }>
            <Button
                class="network-button"
                on_press=move |_| {
                    set_backend_url.set(backend_url_init.to_string());
                    reset_if_ready(
                        backend_url,
                        database_url,
                        set_database_error,
                        set_backend_error,
                        set_show_network_init_modal,
                    );
                }
            >

                {backend_url_init}
            </Button>
        </Show>

        <input
            type="text"
            name="input"
            placeholder="Other Backend"
            value=backend_input
            on:keydown=update_backend_url
            node_ref=backend_input_element
        />
        <P class="under-input">"Current Network: " {move || backend_url.get()}</P>

        <P class="above-input">
            <strong>"Network Database: "</strong>
        </P>

        <Button
            class="network-button"
            on_press=move |_| {
                set_database_url.set(localhost.to_string());
                reset_if_ready(
                    backend_url,
                    database_url,
                    set_database_error,
                    set_backend_error,
                    set_show_network_init_modal,
                );
            }
        >

            "Localhost"
        </Button>

        <Button
            class="network-button"
            on_press=move |_| {
                set_database_url.set(hosted_database.to_string());
                reset_if_ready(
                    backend_url,
                    database_url,
                    set_database_error,
                    set_backend_error,
                    set_show_network_init_modal,
                );
            }
        >

            {hosted_database}
        </Button>

        <Show when=move || {
            database_url_init != hosted_database && database_url_init != localhost
                && database_url_init != ""
        }>
            <Button
                class="network-button"
                on_press=move |_| {
                    set_database_url.set(database_url_init.to_string());
                    reset_if_ready(
                        backend_url,
                        database_url,
                        set_database_error,
                        set_backend_error,
                        set_show_network_init_modal,
                    );
                }
            >

                {database_url_init}
            </Button>
        </Show>

        <input
            type="text"
            name="input"
            placeholder="Other Database"
            value=database_input
            on:keydown=update_database_url
            node_ref=database_input_element
        />
        <P class="under-input">"Current Network: " {move || database_url.get()}</P>
    }
}

fn reset_if_ready(
    backend_url: Signal<String>,
    database_url: Signal<String>,
    set_database_error: WriteSignal<bool>,
    set_backend_error: WriteSignal<bool>,
    set_show_network_init_modal: WriteSignal<bool>,
) {
    if backend_url.get().as_str() != "" && database_url.get().as_str() != "" {
        set_database_error.set(false);
        set_backend_error.set(false);
        set_show_network_init_modal.set(false);
    }
}
