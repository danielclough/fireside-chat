use crate::components::sidebar::model_config::gpu_select::GpuSelect;
use crate::components::sidebar::model_config::model_list_grid::ModelListGrid;
use common::llm::model_list::ModelArgs;
use common::llm::model_list::ModelDLList;
use leptonic::{prelude::Box, select::Select, typography::H1};
use leptos::*;

#[component]
pub fn ModelListContainer(
    model_list: ReadSignal<ModelDLList>,
    model_args: ReadSignal<ModelArgs>,
    ipv4: Signal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
) -> impl IntoView {
    let (repo_id_signal, set_repo_id_signal) = create_signal(model_args.get().repo_id.clone());

    let q_levels = ["q2k", "q3k", "q4_0", "q4_1", "q4k", "q5_0", "q5_1", "q5k", "q6k", "q8_0", "q8_1", "f16",];
    let (q_lvl, set_q_lvl) = create_signal(model_args.get().q_lvl);

    let quantized_safetensors_for_select = vec!["Quantized", "Safetensors"];

    let (quantized_str, set_quantized_str) = create_signal(if model_args.get().quantized {
        quantized_safetensors_for_select[0]
    } else {
        quantized_safetensors_for_select[1]
    });
    
    let (quantized_current, _set_quantized_current) = create_signal(model_args.get().quantized);
    let (template_current, _set_template_current) =
        create_signal(model_args.get().template.unwrap_or(String::new()).clone());
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
                    options=quantized_safetensors_for_select.clone()
                    search_text_provider=move |o: &str| format!("{:?}", o)
                    render_option=move |o: &str| format!("{:?}", o)
                    selected=quantized_str
                    set_selected=move |v| set_quantized_str.set(v)
                />

                <Show when= move || repo_id_signal.get() != "NoModel".to_string()
                    fallback= move || view! {"NoModel"}
                >
                    <a
                        href=format!("https://hf.co/{}", repo_id_signal.get())
                        target="_blank"
                        rel="noreferrer"
                        style="padding:0.25rem;font-size:xx-large;background:#ccc;box-shadow:2px 2px 8px #000;margin:0 1rem;"
                    >
                        {repo_id_signal.get()}
                        " 🔗"
                    </a>
                </Show>
            // <P>"Revision: "{model_args.get().revision}</P>
            </Box>
        </Box>

        // Check if quantized  & check if "Mac" or "CUDA"
        // reload if any of those values change
        <Show when=move || quantized_str.get() == "Quantized"
            fallback= move || view! {
                <Show
                    when=move || gpu_type.get() == "Mac" || gpu_type.get() == "CUDA"
                    fallback=move || {
                        view! {
                            <GpuSelect
                                gpu_type=gpu_type
                                set_gpu_type=set_gpu_type
                                set_repo_id_signal=set_repo_id_signal
                            />
                            <ModelListGrid
                                model_list=model_list
                                tags_all=tags_all
                                ipv4=ipv4
                                template_current=template_current
                                repo_id=repo_id_signal
                                quantized_current=quantized_current
                                quantized=quantized_str.get() == "Quantized"
                                gpu_type=gpu_type
                                q_lvl=q_lvl
                            />
                        }
                    }
                >

                    <GpuSelect
                        gpu_type=gpu_type
                        set_gpu_type=set_gpu_type
                        set_repo_id_signal=set_repo_id_signal
                    />
                    <ModelListGrid
                        model_list=model_list
                        tags_all=tags_all
                        ipv4=ipv4
                        template_current=template_current
                        repo_id=repo_id_signal
                        quantized_current=quantized_current
                        quantized=quantized_str.get() == "Quantized"
                        gpu_type=gpu_type
                        q_lvl=q_lvl
                    />
                </Show>
            }
        >
            <Show
                when=move || gpu_type.get() == "Mac" || gpu_type.get() == "CUDA"
                fallback=move || {
                    view! {
                        <GpuSelect
                            gpu_type=gpu_type
                            set_gpu_type=set_gpu_type
                            set_repo_id_signal=set_repo_id_signal
                        />
                        <ModelListGrid
                            model_list=model_list
                            tags_all=tags_all
                            ipv4=ipv4
                            template_current=template_current
                            repo_id=repo_id_signal
                            quantized_current=quantized_current
                            quantized=quantized_str.get() == "Quantized"
                            gpu_type=gpu_type
                            q_lvl=q_lvl
                        />
                    }
                }
            >
                <GpuSelect
                    gpu_type=gpu_type
                    set_gpu_type=set_gpu_type
                    set_repo_id_signal=set_repo_id_signal
                />
                <ModelListGrid
                    model_list=model_list
                    tags_all=tags_all
                    ipv4=ipv4
                    template_current=template_current
                    repo_id=repo_id_signal
                    quantized_current=quantized_current
                    quantized=quantized_str.get() == "Quantized"
                    gpu_type=gpu_type
                    q_lvl=q_lvl
                />
            </Show>
        </Show>
    }
}
