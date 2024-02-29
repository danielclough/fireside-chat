use crate::components::index::Index;

use crate::functions::rest::{
    llm::{get_inference_args, get_model_args, get_model_list},
    user::get_active_user,
};
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

    // Model
    //
    let (model_args, _set_model_args, _) = use_local_storage::<ModelArgs, JsonCodec>("model");
    let init_everything = create_resource(
        || (),
        move |_| async move {
            logging::log!("loading model_list from API");
            (
                get_model_list(model_args.get().q_lvl, ipv4.get()).await,
                get_model_args(ipv4.get()).await,
                get_active_user().await,
            )
        },
    );

    // Inference Args
    //
    // f64 for input values
    let (inference_args, set_inference_args, _) =
        use_local_storage::<InferenceArgsForInput, JsonCodec>("inference");

    let fetch_args = move |_| async move {
        set_inference_args.set(get_inference_args(ipv4.get()).await);
    };
    let fetch_show = create_resource(move || inference_args.get(), fetch_args);

    // View
    //
    let (home_view, set_home_view) = create_signal(true);

    // Drawer
    //
    let (drawer_state, set_drawer_state) = create_signal(false);

    // Header
    //

    let (database_error, set_database_error) = create_signal(false);
    let (backend_error, set_backend_error) = create_signal(false);

    let (model_list_signal, set_model_list_signal) =
        create_signal::<ModelDLList>(ModelDLList::error());
    let (model_args_signal, set_model_args_signal) = create_signal::<ModelArgs>(ModelArgs::error());
    let (active_user_signal, set_active_user_signal) =
        create_signal::<UserForJson>(UserForJson::error());

    let (_refresh_token, _set_refresh_token) = create_signal(0);
    let (_refreshed_token, _set_refreshed_token) = create_signal(0);

    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Index
                ipv4=ipv4
                set_ipv4=set_ipv4
                gpu_type=gpu_type
                set_gpu_type=set_gpu_type
                inference_args=inference_args
                set_inference_args=set_inference_args
                fetch_show=fetch_show
                user=user
                set_user=set_user
                set_drawer_state=set_drawer_state
                drawer_state=drawer_state
                database_error=database_error
                backend_error=backend_error
                home_view=home_view
                set_home_view=set_home_view
                model_list_signal=model_list_signal
                set_model_list_signal=set_model_list_signal
                model_args_signal=model_args_signal
                set_model_args_signal=set_model_args_signal
                set_active_user_signal=set_active_user_signal
                active_user_signal=active_user_signal
                set_backend_error=set_backend_error
                set_database_error=set_database_error
                init_everything=init_everything
            />
        </Root>
    }
}
