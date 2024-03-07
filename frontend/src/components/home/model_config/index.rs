use crate::{components::home::model_config::model_list_container::ModelListContainer, functions::rest::llm::get_model_list};
use common::llm::model_list::{ModelArgs, ModelDLList};
use leptonic::prelude::Box;
use leptos::*;

#[component]
pub fn ModelConfig(
    backend_url: Signal<String>,
    model_args: Signal<ModelArgs>,
    set_model_args: WriteSignal<ModelArgs>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
) -> impl IntoView {
    let _q_levels = [
        "q2k", "q3k", "q4_0", "q4_1", "q4k", "q5_0", "q5_1", "q5k", "q6k", "q8_0", "q8_1", "f16",
    ];
    let (q_lvl, _set_q_lvl) = create_signal(model_args.get().q_lvl);

    let model_list_resource = create_resource(
        || (),
        move |_| async move {
            logging::log!("loading model_list from API");
            get_model_list(q_lvl.get(), backend_url.get()).await
        },
    );

    let quantized_safetensors_for_select = vec!["Quantized", "Safetensors"];
    let (quantized_safetensors_for_select_signal, _) = create_signal(quantized_safetensors_for_select.clone().iter().map(|x| x.to_string()).collect::<Vec<String>>());
    let (quantized_str, set_quantized_str) = create_signal(if model_args.get().quantized {
        quantized_safetensors_for_select[0].to_string()
    } else {
        quantized_safetensors_for_select[1].to_string()
    });

    view! {
        <Transition fallback=move || {
            view! { <p>"Initializing..."</p> }
        }>
            {move || {
                model_list_resource.get().map(|model_list|{
                    let (model_list_signal, _) = create_signal(model_list);
                    view! {
                        <Box style="width:100%">
                            <ModelListContainer
                                backend_url=backend_url
                                model_args=model_args
                                set_model_args=set_model_args
                                gpu_type=gpu_type
                                set_gpu_type=set_gpu_type
                                quantized_str=quantized_str
                                set_quantized_str=set_quantized_str
                                quantized=quantized_str.get() == "Quantized"
                                q_lvl=q_lvl
                                model_list=model_list_signal
                                quantized_safetensors_for_select=quantized_safetensors_for_select_signal
                            />
                        </Box>
                    }
                })

            }}
        </Transition>
    }
}
