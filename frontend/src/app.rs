use crate::components::{
    chat::index::ChatBox, header::index::Header, home::index::Home, sidebar::index::SideBar,
};
use crate::functions::rest::{
    llm::{get_inference_args, get_model_args, get_model_list},
    user::get_active_user,
};
use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::{ModelArgs, ModelDLList};
use leptonic::modal::{Modal, ModalBody, ModalHeader, ModalTitle};
use leptonic::typography::{H1, H2, H3, H4};
use leptonic::{
    drawer::{Drawer, DrawerSide},
    root::Root,
    {prelude::Box, theme::LeptonicTheme},
};
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
    let home_view_toggle = move |_| {
        set_home_view.update(|value| *value = !*value);
        // FIX refresh conversations
        if home_view.get() {
            _ = leptos_dom::window().location().reload();
        };
    };

    let (database_error, set_database_error) = create_signal(false);
    let (backend_error, set_backend_error) = create_signal(false);

    let (model_list_signal, set_model_list_signal) =
        create_signal::<ModelDLList>(ModelDLList::error());
    let (model_args_signal, set_model_args_signal) = create_signal::<ModelArgs>(ModelArgs::error());
    let (active_user_signal, set_active_user_signal) =
        create_signal::<UserForJson>(UserForJson::error());

    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Header
                home_view_toggle=home_view_toggle
                home_view=home_view
                set_drawer_state=set_drawer_state
                drawer_state=drawer_state
                database_error=database_error
                backend_error=backend_error
            />
            <Box id="main-area">

                <Show
                    when=move || (!database_error.get() && !backend_error.get())
                    fallback=move || {
                        view! {
                            <Box class="outer-container">
                                <H2>"Troubleshoot and Restart (top right)"</H2>
                                <Show when=move || database_error.get()>
                                    <H3>"⚠️ Database Error ⚠️"</H3>
                                </Show>
                                <br/>
                                <Show when=move || backend_error.get()>
                                    <H3>"⚠️ Backend Error ⚠️"</H3>
                                </Show>
                                <H4>
                                    "If you open the app through the command line you should find useful debugging info!"
                                </H4>
                            </Box>
                        }
                    }
                >

                    {move || {
                        init_everything
                            .get()
                            .map(|(model_list, model_args, active_user)| {
                                {
                                    set_model_list_signal.set(model_list);
                                    set_model_args_signal.set(model_args);
                                    set_active_user_signal.set(active_user);
                                }
                                if model_args_signal.get().repo_id
                                    == "LLM Backend Error".to_string()
                                {
                                    set_backend_error.set(true);
                                }
                                if active_user_signal.get().name == "Database Error".to_string() {
                                    set_database_error.set(true);
                                }
                                view! {
                                    <Show when=move || !home_view.get()>
                                        <ChatBox user=user ipv4=ipv4 set_home_view=set_home_view/>
                                    </Show>
                                    <Home
                                        ipv4=ipv4
                                        user=user
                                        set_user=set_user
                                        inference_args=inference_args
                                        model_args=model_args_signal
                                        model_list=model_list_signal
                                        gpu_type=gpu_type
                                        set_gpu_type=set_gpu_type
                                    />

                                    <Drawer
                                        id="sidebar-container"
                                        side=DrawerSide::Right
                                        shown=drawer_state
                                    >
                                        <SideBar
                                            gpu_type=gpu_type
                                            set_gpu_type=set_gpu_type
                                            ipv4=ipv4
                                            set_ipv4=set_ipv4
                                            inference_args=inference_args
                                            set_inference_args=set_inference_args
                                            user=user
                                            set_user=set_user
                                            fetch_show=fetch_show
                                            active_user=active_user_signal
                                            model_list=model_list_signal
                                            model_args=model_args_signal
                                        />
                                    </Drawer>
                                }
                            })
                    }}

                </Show>
            </Box>
        </Root>
    }
}
