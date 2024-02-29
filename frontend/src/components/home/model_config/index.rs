use crate::components::home::model_config::model_list_container::ModelListContainer;
use common::llm::model_list::{ModelArgs, ModelDLList};
use leptonic::prelude::Box;
use leptos::*;

#[component]
pub fn ModelConfig(
    model_list: ReadSignal<ModelDLList>,
    model_args: ReadSignal<ModelArgs>,
    ipv4: Signal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
    set_refresh_token: WriteSignal<i32>,
) -> impl IntoView {
    view! {
        <Box style="width:100%">
            <ModelListContainer
                ipv4=ipv4
                model_args=model_args
                model_list=model_list
                gpu_type=gpu_type
                set_gpu_type=set_gpu_type
                set_refresh_token=set_refresh_token
            />
        </Box>
    }
}
