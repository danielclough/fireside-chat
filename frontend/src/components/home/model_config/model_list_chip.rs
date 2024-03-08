use leptonic::components::{
    button::Button,
    chip::{Chip, ChipColor},
};
use leptos::*;

#[component]
pub fn ModelListChip(
    name: String,
    tags_enabled: ReadSignal<Vec<String>>,
    set_tags_enabled: WriteSignal<Vec<String>>,
    all_enabled: ReadSignal<bool>,
    tags_all: ReadSignal<Vec<String>>,
) -> impl IntoView {
    let (name_signal, _set_name_signal) = create_signal(name.clone());
    let catch_all = name_signal.get() == String::new();

    view! {
        <Show
            when=move || {
                if catch_all && all_enabled.get() {
                    true
                } else {
                    tags_enabled.get().iter().any(|t| *t == name.clone())
                }
            }

            fallback=move || {
                view! {
                    <Button on_press=move |_| {
                        if catch_all {
                            set_tags_enabled.update(|x| *x = vec![String::new()]);
                        } else {
                            set_tags_enabled.update(|x| x.push(name_signal.get()));
                            set_tags_enabled.update(|x| x.retain(|x| !x.is_empty()));
                        };
                    }>
                        {if name_signal.get() != String::new() {
                            name_signal.get()
                        } else {
                            "DISABLE ALL".to_string()
                        }}

                    </Button>
                }
            }
        >

            <Chip
                color=ChipColor::Success
                dismissible=move |_| {
                    if catch_all {
                        set_tags_enabled.update(|x| *x = tags_all.get());
                        set_tags_enabled.update(|x| x.retain(|x| x != name_signal.get().as_str()));
                    } else {
                        set_tags_enabled.update(|x| x.retain(|x| x != name_signal.get().as_str()));
                        if tags_enabled.get().is_empty() {
                            set_tags_enabled.update(|x| x.push("".to_string()));
                        }
                    };
                }
            >

                {if name_signal.get() != String::new() {
                    name_signal.get()
                } else {
                    "EMPTY".to_string()
                }}

            </Chip>
        </Show>
    }
}
