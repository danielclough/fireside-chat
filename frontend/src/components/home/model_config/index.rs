use crate::components::home::model_config::model_list_container::ModelListContainer;
use common::llm::model_list::{ModelArgs, ModelDLList};
use leptonic::prelude::Box;
use leptos::*;

#[component]
pub fn ModelConfig(
    model_list: ReadSignal<ModelDLList>,
    ipv4: Signal<String>,
    model_args: Signal<ModelArgs>,
    set_model_args: WriteSignal<ModelArgs>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <Box style="width:100%">
            <ModelListContainer
                model_list=model_list
                ipv4=ipv4
                model_args=model_args
                set_model_args=set_model_args
                gpu_type=gpu_type
                set_gpu_type=set_gpu_type
            />
        </Box>
    }
}
