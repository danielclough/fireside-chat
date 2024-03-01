use leptonic::typography::{H2, P};
use leptos::{html::Input, *};
use web_sys::KeyboardEvent;

#[component]
pub fn NetworkConfig(
    ipv4: Signal<String>,
    set_ipv4: WriteSignal<String>,
    set_refresh_token: WriteSignal<i32>,
) -> impl IntoView {
    let input_element: NodeRef<Input> = create_node_ref();
    let input_str = String::new();

    let update_ipv4 = move |e: KeyboardEvent| {
        if e.code() == "Enter" {
            let input_element = input_element.get().expect("<input> to exist");
            set_ipv4.set(input_element.value());
            input_element.set_value("");
            set_refresh_token.update(|x| *x += 1);
        }
    };

    view! {
        <H2>"Network Config"</H2>

        <P class="above-input">
            <strong>"Network: "</strong>
            "Network Backend (ipv4)."
        </P>

        <input
            type="text"
            name="input"
            placeholder="Change Network"
            value=input_str
            on:keydown=update_ipv4
            node_ref=input_element
        />

        <P class="under-input">"Current Network: " {move || ipv4.get()}</P>
    }
}
