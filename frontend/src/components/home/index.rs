use crate::components::home::inference_config::index::Inference;
use crate::components::home::model_config::index::ModelConfig;
use crate::components::home::network_config::index::NetworkConfig;
use crate::components::home::overview::index::Overview;
use crate::components::home::role_config::index::RoleListContainer;
use crate::components::home::user_config::index::UserConfig;
use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::ModelArgs;
use leptonic::{
    components::{prelude::Box, tab::Tab, tabs::Tabs},
    Mount,
};
use leptos::*;

#[component]
pub fn Home(
    backend_url: Signal<String>,
    set_backend_url: WriteSignal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
    inference_args: Signal<InferenceArgsForInput>,
    set_inference_args: WriteSignal<InferenceArgsForInput>,
    model_args: Signal<ModelArgs>,
    set_model_args: WriteSignal<ModelArgs>,
    user: Signal<UserForJson>,
    set_user: WriteSignal<UserForJson>,
    database_url: Signal<String>,
    set_database_url: WriteSignal<String>,
    set_database_error: WriteSignal<bool>,
    set_backend_error: WriteSignal<bool>,
    set_show_network_init_modal: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <Tabs mount=Mount::Once>
            <Tab name="home_tab" label="Home".into_view()>
                <Box style="width:100%">
                    <Overview
                        user=user
                        inference_args=inference_args
                        database_url=database_url
                        model_args=model_args
                    />
                </Box>
            </Tab>
            <Tab name="user_tab" label="User".into_view()>
                <Box style="width:100%">
                    <UserConfig user=user set_user=set_user database_url=database_url/>
                </Box>
            </Tab>
            <Tab name="models_tab" label="Models".into_view()>
                <Box style="width:100%">
                    <ModelConfig
                        backend_url=backend_url
                        model_args=model_args
                        set_model_args=set_model_args
                        gpu_type=gpu_type
                        set_gpu_type=set_gpu_type
                    />
                </Box>
            </Tab>
            <Tab name="inference_tab" label="Inference".into_view()>
                <Box style="width:100%">
                    <Inference
                        backend_url=backend_url
                        inference_args=inference_args
                        set_inference_args=set_inference_args
                    />
                </Box>
            </Tab>
            <Tab name="roles_tab" label="Roles".into_view()>
                <Box style="width:100%">
                    <RoleListContainer
                        backend_url=backend_url
                        inference_args=inference_args
                        set_inference_args=set_inference_args
                    />
                </Box>
            </Tab>
            <Tab name="network_tab" label="Network".into_view()>
                <Box style="width:100%">
                    <NetworkConfig
                        backend_url=backend_url
                        set_backend_url=set_backend_url
                        database_url=database_url
                        set_database_url=set_database_url
                        set_database_error=set_database_error
                        set_backend_error=set_backend_error
                        set_show_network_init_modal=set_show_network_init_modal
                    />
                </Box>
            </Tab>
        </Tabs>
    }
}
