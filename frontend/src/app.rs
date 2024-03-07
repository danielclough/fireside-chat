use crate::components::{index::Index, landing::index::Landing};

use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::{ModelArgs, ModelDLList};

use leptonic::{root::Root, theme::LeptonicTheme};
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
    let localhost = "127.0.0.1";
    let backend_url_init = std::option_env!("FIRESIDE_BACKEND_URL").unwrap_or(localhost);
    let (backend_url, set_backend_url, _) = use_local_storage::<String, JsonCodec>("backend_url");
    // TODO! - Backup with DB
    set_backend_url.set(backend_url_init.to_string());

    let database_url_init = std::option_env!("FIRESIDE_DATABASE_URL").unwrap_or(localhost);
    let (database_url, set_database_url, _) =
        use_local_storage::<String, JsonCodec>("database_url");
    // TODO! - Backup with DB
    set_database_url.set(database_url_init.to_string());

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
    let (model_list, set_model_list) = create_signal::<ModelDLList>(ModelDLList::error());

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
    let (database_error, set_database_error) = create_signal(false);
    let (backend_error, set_backend_error) = create_signal(false);

    // Modals
    //
    let (show_user_init_modal, set_show_user_init_modal) =
        create_signal(user.get().name == *"None" || user.get().name.len() < 2);
    let (show_model_init_modal, _set_show_model_init_modal) =
        create_signal(model_args.get().clone().template == Some("NoModel".to_string()));

use crate::components::home::overview::{init_model::InitModelModal, init_user::InitUserModal};

    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Show
                when=move || !landing_view.get()
                fallback=move || {
                    view! { <Landing landing_view_toggle=landing_view_toggle/> }
                }
            >

                <InitModelModal
                    model_args=model_args
                    model_list=model_list
                    backend_url=backend_url
                    show_when=show_model_init_modal
                    gpu_type=gpu_type
                    set_gpu_type=set_gpu_type
                    set_model_args=set_model_args
                />
                <InitUserModal
                    set_user=set_user
                    user=user
                    show_when=show_user_init_modal
                    on_accept=move || set_show_user_init_modal.set(false)
                    on_cancel=move || set_show_user_init_modal.set(false)
                    database_url=database_url
                />
                <Index
                    backend_url=backend_url
                    set_backend_url=set_backend_url
                    gpu_type=gpu_type
                    set_gpu_type=set_gpu_type
                    inference_args=inference_args
                    set_inference_args=set_inference_args
                    user=user
                    set_user=set_user
                    database_error=database_error
                    backend_error=backend_error
                    home_view=home_view
                    set_home_view=set_home_view
                    model_list_signal=model_list
                    set_model_list_signal=set_model_list
                    model_args_local_storage=model_args
                    set_model_args_local_storage=set_model_args
                    set_backend_error=set_backend_error
                    set_database_error=set_database_error
                    database_url=database_url
                />
            </Show>
        </Root>
    }
}
