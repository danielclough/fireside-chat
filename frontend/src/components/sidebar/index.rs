use crate::components::sidebar::inference_config::index::Inference;
use crate::components::sidebar::model_config::index::ModelConfig;
use crate::components::sidebar::network_config::index::NetworkConfig;
use crate::components::sidebar::role_config::index::RoleListContainer;
use crate::components::sidebar::user_config::index::UserConfig;
use crate::functions::rest::role::get_role_list;
use common::database::user::UserForJson;
use common::llm::inference::InferenceArgsForInput;
use common::llm::model_list::{ModelArgs, ModelDLList};
use leptonic::{prelude::Box, tab::Tab, tabs::Tabs, Mount};
use leptos::*;

#[component]
pub fn SideBar(
    ipv4: Signal<String>,
    set_ipv4: WriteSignal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
    inference_args: Signal<InferenceArgsForInput>,
    set_inference_args: WriteSignal<InferenceArgsForInput>,
    fetch_show: Resource<InferenceArgsForInput, ()>,
    model_list: ReadSignal<ModelDLList>,
    model_args: ReadSignal<ModelArgs>,
    active_user: ReadSignal<UserForJson>,
    user: Signal<UserForJson>,
    set_user: WriteSignal<UserForJson>,
) -> impl IntoView {

    let role_list = create_resource(
        || (),
        move |_| async move {
            logging::log!("loading role_list from API");
            get_role_list(ipv4.get()).await
        },
    );

    view! {
        <Tabs mount=Mount::Once>
            <Tab name="user_tab" label="User".into_view()>
                <Box style="width:100%">
                    <UserConfig active_user=active_user.get() user=user set_user=set_user/>
                </Box>
            </Tab>
            <Tab name="models_tab" label="Models".into_view()>
                <Box style="width:100%">
                    <ModelConfig
                        ipv4=ipv4
                        model_list=model_list
                        model_args=model_args
                        gpu_type=gpu_type
                        set_gpu_type=set_gpu_type
                    />
                </Box>
            </Tab>
            <Tab name="inference_tab" label="Inference".into_view()>
                <Box style="width:100%">
                    <Inference
                        ipv4=ipv4
                        inference_args=inference_args
                        set_inference_args=set_inference_args
                        fetch_show=fetch_show
                    />
                </Box>
            </Tab>
            <Tab name="roles_tab" label="Roles".into_view()>
                <Box style="width:100%">
                    {move || {
                        role_list
                            .get()
                            .map(|role_list| {
                                view! {
                                    <RoleListContainer
                                        ipv4=ipv4
                                        role_list=role_list
                                        inference_args=inference_args
                                        set_inference_args=set_inference_args
                                    />
                                }
                            })
                    }}

                </Box>
            </Tab>
            <Tab name="network_tab" label="Network".into_view()>
                <Box style="width:100%">
                    <NetworkConfig ipv4=ipv4 set_ipv4=set_ipv4/>
                </Box>
            </Tab>
        </Tabs>
    }
}
