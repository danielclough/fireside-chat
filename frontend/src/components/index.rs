use crate::components::home::index::Home;
use crate::components::{chat::index::ChatBox, header::index::Header};
use crate::functions::rest::{
    llm::{get_model_args, get_model_list},
    user::get_user_by_name,
};

use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::{ModelArgs, ModelDLList};

use leptonic::typography::{H2, H3, H4};
use leptonic::{
    root::Root,
    {prelude::Box, theme::LeptonicTheme},
};
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
    database_error: ReadSignal<bool>,
    backend_error: ReadSignal<bool>,
    set_database_error: WriteSignal<bool>,
    set_backend_error: WriteSignal<bool>,
    home_view: ReadSignal<bool>,
    set_home_view: WriteSignal<bool>,
    model_list_signal: ReadSignal<ModelDLList>,
    set_model_list_signal: WriteSignal<ModelDLList>,
    model_args_local_storage: Signal<ModelArgs>,
    set_model_args_local_storage: WriteSignal<ModelArgs>,
    database_url: Signal<String>,
) -> impl IntoView {
    let init_everything = create_resource(
        move || {
            (
                model_args_local_storage.get(),
                user.get(),
                inference_args.get(),
                backend_url.get(),
            )
        },
        move |_| async move {
            logging::log!("loading model_list from API");
            logging::log!("loading model_args from API");
            logging::log!("loading active_user from API");
            (
                get_model_list(model_args_local_storage.get().q_lvl, backend_url.get()).await,
                get_model_args(backend_url.get()).await,
                get_user_by_name(user.get().name, database_url.get()).await,
            )
        },
    );

    // Header
    //
    let home_view_toggle = move |_| {
        if database_error.get() || backend_error.get() {
            // Refresh error
            set_home_view.update(|value| *value = true);
        } else {
            set_home_view.update(|value| *value = !*value);
        };
    };

    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Header
                home_view_toggle=home_view_toggle
                home_view=home_view
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

                    // the fallback will show initially
                    // on subsequent reloads, the current child will
                    <Transition // continue showing
                    fallback=move || {
                        view! { <p>"Initializing..."</p> }
                    }>
                        {move || {
                            init_everything
                                .get()
                                .map(|(model_list, model_args, active_user)| {
                                    leptos_dom::log!("Init Map");
                                    {
                                        set_model_list_signal.set(model_list);
                                        set_model_args_local_storage.set(model_args);
                                    }
                                    if model_args_local_storage.get().repo_id
                                        == *"LLM Backend Error"
                                    {
                                        set_backend_error.set(true);
                                    }
                                    if user.get().name == *"Database Error" {
                                        set_database_error.set(true);
                                    }
                                    view! {
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
                                                model_list=model_list_signal
                                                model_args=model_args_local_storage
                                                set_model_args=set_model_args_local_storage
                                                database_url=database_url
                                            />
                                        </Show>
                                    }
                                })
                        }}

                    </Transition>
                </Show>
            </Box>
        </Root>
    }
}
