use crate::components::home::network_config::init_network::InitNetworkModal;
use crate::components::{index::Index, landing::index::Landing};

use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::ModelArgs;

use leptonic::components::{root::Root, theme::LeptonicTheme};
use leptos::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;

#[component]
pub fn App() -> impl IntoView {
    // Landing
    //
    let (landing_view, set_landing_view) = create_signal(true);
    let landing_view_toggle = move |_| {
        set_landing_view.update(|value| *value = !*value);
    };

    // Network
    //
    let (database_url, set_database_url, _) =
        use_local_storage::<String, JsonCodec>("database_url");
    let (database_error, set_database_error) = create_signal(false);
    let (backend_url, set_backend_url, _) = use_local_storage::<String, JsonCodec>("backend_url");
    let (backend_error, set_backend_error) = create_signal(false);

    // GPU
    //
    let (gpu_type, set_gpu_type, _) = use_local_storage::<String, JsonCodec>("gpu");
    // set_gpu_type.set("None".to_string());

    // User
    //
    let (user, set_user, _) = use_local_storage::<UserForJson, JsonCodec>("user");

    // Model
    //
    let (model_args, set_model_args, _) = use_local_storage::<ModelArgs, JsonCodec>("model");

    // Inference Args
    //
    // f64 for input values
    let (inference_args, set_inference_args, _) =
        use_local_storage::<InferenceArgsForInput, JsonCodec>("inference");

    // View
    //
    let (home_view, set_home_view) = create_signal(true);

    // Utils
    //

    // Modals
    //
    let (show_network_init_modal, set_show_network_init_modal) =
        create_signal(database_url.get().as_str() == "" || backend_url.get().as_str() == "");

    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Show
                when=move || !landing_view.get()
                fallback=move || {
                    view! { <Landing landing_view_toggle=landing_view_toggle/> }
                }
            >

                <InitNetworkModal
                    show_when=show_network_init_modal
                    backend_url=backend_url
                    set_backend_url=set_backend_url
                    database_url=database_url
                    set_database_url=set_database_url
                    set_database_error=set_database_error
                    set_backend_error=set_backend_error
                    set_show_network_init_modal=set_show_network_init_modal
                />
                <Show when=move || !show_network_init_modal.get()>
                    <Index
                        backend_url=backend_url
                        set_backend_url=set_backend_url
                        gpu_type=gpu_type
                        set_gpu_type=set_gpu_type
                        inference_args=inference_args
                        set_inference_args=set_inference_args
                        user=user
                        set_user=set_user
                        home_view=home_view
                        set_home_view=set_home_view
                        model_args=model_args
                        set_model_args=set_model_args
                        database_url=database_url
                        set_database_url=set_database_url
                        set_database_error=set_database_error
                        set_backend_error=set_backend_error
                        set_show_network_init_modal=set_show_network_init_modal
                    />
                </Show>
            </Show>
        </Root>
    }
}
