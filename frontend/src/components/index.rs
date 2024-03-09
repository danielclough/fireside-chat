use crate::components::home::model_config::init_model::InitModelModal;
use crate::components::home::user_config::init_user::InitUserModal;
use crate::components::inner_wrapper::InnerWrapper;

use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::ModelArgs;

use leptos::*;

#[component]
pub fn Index(
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
    // Modals
    //
    let (show_user_init_modal, set_show_user_init_modal) =
        create_signal(user.get().name == *"None" || user.get().name.len() < 2);
    let (show_model_init_modal, set_show_model_init_modal) =
        create_signal(model_args.get().clone().template == Some("NoModel".to_string()));

    view! {
        <InitModelModal
            show_when=show_model_init_modal
            on_accept=move || set_show_model_init_modal.set(false)
            model_args=model_args
            backend_url=backend_url
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

        <Show when=move || (!show_user_init_modal.get() && !show_model_init_modal.get())>
            <InnerWrapper
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
    }
}
