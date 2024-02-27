use crate::components::{
    chat::index::ChatBox, header::index::Header, home::index::Home, sidebar::index::SideBar,
};
use crate::functions::rest::{
    llm::{get_inference_args, get_model_args, get_model_list},
    user::get_active_user,
};
use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::ModelArgs;
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
    let (_model_args, _set_model_args, _) = use_local_storage::<ModelArgs, JsonCodec>("model");
    let init_everything = create_resource(
        || (),
        move |_| async move {
            logging::log!("loading model_list from API");
            (
                get_model_list(ipv4.get()).await,
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

    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Header
                home_view_toggle=home_view_toggle
                home_view=home_view
                set_drawer_state=set_drawer_state
                drawer_state=drawer_state
            />
            <Box id="main-area">
                <Transition fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>
                    {move || {
                        init_everything
                            .get()
                            .map(|(model_list, model_args, active_user)| {
                                view! {
                                    <Show when=move || !home_view.get()>
                                        <ChatBox user=user ipv4=ipv4 set_home_view=set_home_view/>
                                    </Show>

                                    <Home
                                        ipv4=ipv4
                                        user=user
                                        set_user=set_user
                                        inference_args=inference_args
                                        model_args=model_args.clone()
                                        model_list=model_list.clone()
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
                                            active_user=active_user
                                            user=user
                                            set_user=set_user
                                            fetch_show=fetch_show
                                            model_list=model_list
                                            model_args=model_args
                                        />
                                    </Drawer>
                                }
                            })
                    }}

                </Transition>
            </Box>
        </Root>
    }
}
