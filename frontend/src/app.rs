use crate::components::index::Index;

use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::{ModelArgs, ModelDLList};

use leptonic::{root::Root, theme::LeptonicTheme};
use leptos::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;

#[component]
pub fn App() -> impl IntoView {
    // Network
    //
    let localhost = "127.0.0.1";
    let ipv4_init = std::option_env!("FIRESIDE_BACKEND_IPV4").unwrap_or(localhost);
    let (ipv4, set_ipv4, _) = use_local_storage::<String, JsonCodec>("ipv4");
    // TODO! - Backup with DB
    set_ipv4.set(ipv4_init.to_string());

    // GPU
    //
    let (gpu_type, set_gpu_type, _) = use_local_storage::<String, JsonCodec>("gpu");
    // set_gpu_type.set("None".to_string());

    // User
    //
    let (user, set_user, _) = use_local_storage::<UserForJson, JsonCodec>("user");
    let (active_user_signal, set_active_user_signal) =
        create_signal::<UserForJson>(UserForJson::error());

    // Model
    //
    let (model_args, _set_model_args, _) = use_local_storage::<ModelArgs, JsonCodec>("model");
    let (model_list_signal, set_model_list_signal) =
        create_signal::<ModelDLList>(ModelDLList::error());
    let (model_args_signal, set_model_args_signal) = create_signal::<ModelArgs>(ModelArgs::error());

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
    let (refresh_token, set_refresh_token) = create_signal(0);
    let (database_error, set_database_error) = create_signal(false);
    let (backend_error, set_backend_error) = create_signal(false);

    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Show when= move || refresh_token.get() % 2 == 0
                fallback= move || view! {
                    <Index
                        ipv4=ipv4
                        set_ipv4=set_ipv4
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
                        model_list_signal=model_list_signal
                        set_model_list_signal=set_model_list_signal
                        model_args=model_args
                        model_args_signal=model_args_signal
                        set_model_args_signal=set_model_args_signal
                        set_active_user_signal=set_active_user_signal
                        active_user_signal=active_user_signal
                        set_backend_error=set_backend_error
                        set_database_error=set_database_error
                        set_refresh_token=set_refresh_token
                    />
                }
                >
                <Index
                    ipv4=ipv4
                    set_ipv4=set_ipv4
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
                    model_list_signal=model_list_signal
                    set_model_list_signal=set_model_list_signal
                    model_args=model_args
                    model_args_signal=model_args_signal
                    set_model_args_signal=set_model_args_signal
                    set_active_user_signal=set_active_user_signal
                    active_user_signal=active_user_signal
                    set_backend_error=set_backend_error
                    set_database_error=set_database_error
                    set_refresh_token=set_refresh_token
                />
            </Show>
        </Root>
    }
}
