use crate::components::home::index::Home;
use crate::components::{chat::index::ChatBox, header::index::Header};

use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::ModelArgs;

use leptonic::components::prelude::Box;
use leptos::*;

#[component]
pub fn InnerWrapper(
    backend_url: Signal<String>,
    set_backend_url: WriteSignal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
    inference_args: Signal<InferenceArgsForInput>,
    set_inference_args: WriteSignal<InferenceArgsForInput>,
    user: Signal<UserForJson>,
    set_user: WriteSignal<UserForJson>,
    home_view: ReadSignal<bool>,
    set_home_view: WriteSignal<bool>,
    model_args: Signal<ModelArgs>,
    set_model_args: WriteSignal<ModelArgs>,
    database_url: Signal<String>,
    set_database_url: WriteSignal<String>,
    set_database_error: WriteSignal<bool>,
    set_backend_error: WriteSignal<bool>,
    set_show_network_init_modal: WriteSignal<bool>,
) -> impl IntoView {
    // Header
    //
    let home_view_toggle = move |_| {
        set_home_view.update(|value| *value = !*value);
    };

    view! {
        <Header home_view_toggle=home_view_toggle home_view=home_view/>
        <Box id="main-area">

            <Show
                when=move || home_view.get()
                fallback=move || {
                    view! {
                        <ChatBox
                            user=user
                            backend_url=backend_url
                            set_home_view=set_home_view
                            database_url=database_url
                        />
                    }
                }
            >

                <Home
                    gpu_type=gpu_type
                    set_gpu_type=set_gpu_type
                    backend_url=backend_url
                    set_backend_url=set_backend_url
                    inference_args=inference_args
                    set_inference_args=set_inference_args
                    user=user
                    set_user=set_user
                    model_args=model_args
                    set_model_args=set_model_args
                    database_url=database_url
                    set_database_url=set_database_url
                    set_database_error=set_database_error
                    set_backend_error=set_backend_error
                    set_show_network_init_modal=set_show_network_init_modal
                />
            </Show>
        </Box>
    }
}
