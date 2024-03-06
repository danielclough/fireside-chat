use leptonic::typography::{H2, P};
use leptos::{html::Input, *};
use web_sys::KeyboardEvent;

#[component]
pub fn NetworkConfig(
    backend_url: Signal<String>,
    set_backend_url: WriteSignal<String>,
) -> impl IntoView {
    let input_element: NodeRef<Input> = create_node_ref();
    let input_str = String::new();

    let update_backend_url = move |e: KeyboardEvent| {
        if e.code() == "Enter" {
            let input_element = input_element.get().expect("<input> to exist");
            set_backend_url.set(input_element.value());
            input_element.set_value("");
        }
    };

    view! {
        <H2>"Network Config"</H2>

        <P class="above-input">
            <strong>"Network: "</strong>
            "Network Backend (backend_url)."
        </P>

        <input
            type="text"
            name="input"
            placeholder="Change Network"
            value=input_str
            on:keydown=update_backend_url
            node_ref=input_element
        />

        <P class="under-input">"Current Network: " {move || backend_url.get()}</P>
    }
}
