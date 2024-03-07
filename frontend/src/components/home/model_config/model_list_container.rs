use crate::components::home::model_config::gpu_select::GpuSelect;
use crate::components::home::model_config::model_list_grid::ModelListGrid;

use common::llm::model_list::{ModelArgs, ModelDLList};
use leptonic::{prelude::Box, select::Select, typography::H1};
use leptos::*;

#[component]
pub fn ModelListContainer(
    model_args: Signal<ModelArgs>,
    backend_url: Signal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
    set_model_args: WriteSignal<ModelArgs>,
    quantized: bool,
    quantized_str: ReadSignal<String>,
    set_quantized_str: WriteSignal<String>,
    q_lvl: ReadSignal<String>,
    model_list: ReadSignal<ModelDLList>,
    quantized_safetensors_for_select: ReadSignal<Vec<String>>,
) -> impl IntoView {
    let (model_list, _set_model_list) = create_signal(model_list.get());

    let (quantized_current, _set_quantized_current) = create_signal(model_args.get().quantized);
    let (template_current, _set_template_current) =
        create_signal(model_args.get().template.unwrap_or_default().clone());
    let (tags_all, _set_tags_all) = create_signal::<Vec<String>>({
        let tag_list_list = model_list
            .get()
            .list
            .iter()
            .map(|x| x.tags.clone())
            .collect::<Vec<Vec<String>>>();
        let mut tag_list = vec![String::new()];
        for l in tag_list_list {
            for m in l {
                if tag_list.iter().any(|c| c == m.as_str()) {
                } else {
                    tag_list.push(m);
                };
            }
        }
        tag_list
    });

    view! {
        <Box style="margin-top:1rem;padding:1rem 0.25rem;display:flex;flex-direction:column;">
            <H1 style="width:100%;padding:0.25rem;text-align:center;box-shadow:2px 2px 8px #000;">
                "Current Model"
            </H1>
            <Box style="display:flex; justify-content:center;display:flex;flex-direction:row;">

                <Select
                    options=quantized_safetensors_for_select.get()
                    search_text_provider=move |o: String| format!("{:?}", o)
                    render_option=move |o: String| format!("{:?}", o)
                    selected=quantized_str
                    set_selected=move |v| set_quantized_str.set(v)
                />

                <Show
                    when=move || model_args.get().repo_id.clone() != *"NoModel"
                    fallback=move || view! { "NoModel" }
                >
                    <a
                        href=format!("https://hf.co/{}", model_args.get().repo_id.clone())
                        target="_blank"
                        rel="noreferrer"
                        style="padding:0.25rem;font-size:xx-large;background:#ccc;box-shadow:2px 2px 8px #000;margin:0 1rem;"
                    >
                        {model_args.get().repo_id.clone()}
                        " 🔗"
                    </a>
                </Show>
            // <P>"Revision: "{model_args.get().revision}</P>
            </Box>
        </Box>

        // Check if quantized  & check if "Mac" or "CUDA"
        // reload if any of those values change
        <Show
            when=move || quantized
            fallback=move || {
                view! {
                    <Show
                        when=move || gpu_type.get() == "Mac" || gpu_type.get() == "CUDA"
                        fallback=move || {
                            view! {
                                <GpuSelect gpu_type=gpu_type set_gpu_type=set_gpu_type/>
                                <ModelListGrid
                                    model_list=model_list
                                    tags_all=tags_all
                                    backend_url=backend_url
                                    template_current=template_current
                                    quantized_current=quantized_current
                                    gpu_type=gpu_type
                                    quantized=quantized
                                    q_lvl=q_lvl
                                    model_args=model_args
                                    set_model_args=set_model_args
                                />
                            }
                        }
                    >

                        <GpuSelect gpu_type=gpu_type set_gpu_type=set_gpu_type/>
                        <ModelListGrid
                            model_list=model_list
                            tags_all=tags_all
                            backend_url=backend_url
                            template_current=template_current
                            quantized_current=quantized_current
                            quantized=quantized_str.get() == "Quantized"
                            gpu_type=gpu_type
                            q_lvl=q_lvl
                            model_args=model_args
                            set_model_args=set_model_args
                        />
                    </Show>
                }
            }
        >

            <Show
                when=move || gpu_type.get() == "Mac" || gpu_type.get() == "CUDA"
                fallback=move || {
                    view! {
                        <GpuSelect gpu_type=gpu_type set_gpu_type=set_gpu_type/>
                        <ModelListGrid
                            model_list=model_list
                            tags_all=tags_all
                            backend_url=backend_url
                            template_current=template_current
                            quantized_current=quantized_current
                            quantized=quantized_str.get() == "Quantized"
                            gpu_type=gpu_type
                            q_lvl=q_lvl
                            model_args=model_args
                            set_model_args=set_model_args
                        />
                    }
                }
            >

                <GpuSelect gpu_type=gpu_type set_gpu_type=set_gpu_type/>
                <ModelListGrid
                    model_list=model_list
                    tags_all=tags_all
                    backend_url=backend_url
                    template_current=template_current
                    quantized_current=quantized_current
                    quantized=quantized_str.get() == "Quantized"
                    gpu_type=gpu_type
                    q_lvl=q_lvl
                    model_args=model_args
                    set_model_args=set_model_args
                />
            </Show>
        </Show>
    }
}
