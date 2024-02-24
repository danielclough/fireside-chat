use crate::components::sidebar::model_config::gpu_select::GpuSelect;
use crate::components::sidebar::model_config::model_list_grid::ModelListGrid;
use common::llm::model_list::ModelArgs;
use common::llm::model_list::ModelDLList;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn ModelListContainer(
    model_list: ModelDLList,
    model_args: ModelArgs,
    ipv4: Signal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>
) -> impl IntoView {
    let (repo_id, _set_repo_id) = create_signal(model_args.repo_id.clone());

    let quantized_safetensors_for_select = vec!["Quantized", "Safetensors"];
    let q_bool = if model_args.quantized {
        quantized_safetensors_for_select[0]
    } else {
        quantized_safetensors_for_select[1]
    };
    let (quantized_current, _set_quantized_current) = create_signal(model_args.quantized);
    let (template_current, _set_template_current) =
        create_signal(model_args.template.unwrap_or(String::new()).clone());
    let (quantized, set_quantized) = create_signal(q_bool);
    let (model_list_signal, _set_model_list_signal) = create_signal(model_list);
    let (tags_all, _set_tags_all) = create_signal::<Vec<String>>({
        let tag_list_list = model_list_signal
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

    let (tags_enabled, set_tags_enabled) =
        create_signal::<Vec<String>>(tags_all.get()[1..tags_all.get().len()].to_owned());
    let (all_enabled, _set_all_enabled) =
        create_signal::<bool>(tags_all.get().len() == tags_enabled.get().len());

    view! {
        <Box style="margin-top:1rem;padding:1rem 0.25rem;display:flex;flex-direction:column;">
            <H1 style="width:100%;padding:0.25rem;text-align:center;box-shadow:2px 2px 8px #000;">
                "Current Model"
            </H1>
            <Box style="display:flex; justify-content:center;display:flex;flex-direction:row;">

                <Select
                    options=quantized_safetensors_for_select.clone()
                    search_text_provider=move |o: &str| format!("{:?}", o)
                    render_option=move |o: &str| format!("{:?}", o)
                    selected=quantized
                    set_selected=move |v| set_quantized.set(v)
                />

                <a
                    href=format!("https://hf.co/{}", model_args.repo_id)
                    target="_blank"
                    rel="noreferrer"
                    style="padding:0.25rem;font-size:xx-large;background:#ccc;box-shadow:2px 2px 8px #000;margin:0 1rem;"
                >
                    {model_args.repo_id}
                    " 🔗"
                </a>
            // <P>"Revision: "{model_args.revision}</P>
            </Box>
        </Box>

        // If quantized check if "Mac"
        <Show when=move || quantized.get() == "Quantized">
            <Show
                when=move || gpu_type.get() == "Mac"
                fallback=move || {
                    view! {
                        <GpuSelect
                            quantized=quantized.get() == "Quantized"
                            gpu_type=gpu_type
                            set_gpu_type=set_gpu_type
                        />
                        <ModelListGrid
                            model_list_signal=model_list_signal
                            tags_all=tags_all
                            tags_enabled=tags_enabled
                            set_tags_enabled=set_tags_enabled
                            all_enabled=all_enabled
                            ipv4=ipv4
                            template_current=template_current
                            repo_id=repo_id
                            quantized_current=quantized_current
                            quantized=quantized.get() == "Quantized"
                            gpu_type=gpu_type
                        />
                    }
                }
            >

                <GpuSelect
                    quantized=quantized.get() == "Quantized"
                    gpu_type=gpu_type
                    set_gpu_type=set_gpu_type
                />
                <ModelListGrid
                    model_list_signal=model_list_signal
                    tags_all=tags_all
                    tags_enabled=tags_enabled
                    set_tags_enabled=set_tags_enabled
                    all_enabled=all_enabled
                    ipv4=ipv4
                    template_current=template_current
                    repo_id=repo_id
                    quantized_current=quantized_current
                    quantized=quantized.get() == "Quantized"
                    gpu_type=gpu_type
                />
            </Show>
        </Show>
        // If safetensors check if "None"
        <Show when=move || quantized.get() != "Quantized">
            <Show
                when=move || gpu_type.get() == "None"
                fallback=move || {
                    view! {
                        <GpuSelect
                            quantized=quantized.get() == "Quantized"
                            gpu_type=gpu_type
                            set_gpu_type=set_gpu_type
                        />
                        <ModelListGrid
                            model_list_signal=model_list_signal
                            tags_all=tags_all
                            tags_enabled=tags_enabled
                            set_tags_enabled=set_tags_enabled
                            all_enabled=all_enabled
                            ipv4=ipv4
                            template_current=template_current
                            repo_id=repo_id
                            quantized_current=quantized_current
                            quantized=quantized.get() == "Quantized"
                            gpu_type=gpu_type
                        />
                    }
                }
            >

                <GpuSelect
                    quantized=quantized.get() == "Quantized"
                    gpu_type=gpu_type
                    set_gpu_type=set_gpu_type
                />
                <ModelListGrid
                    model_list_signal=model_list_signal
                    tags_all=tags_all
                    tags_enabled=tags_enabled
                    set_tags_enabled=set_tags_enabled
                    all_enabled=all_enabled
                    ipv4=ipv4
                    template_current=template_current
                    repo_id=repo_id
                    quantized_current=quantized_current
                    quantized=quantized.get() == "Quantized"
                    gpu_type=gpu_type
                />
            </Show>
        </Show>
    }
}
