use common::llm::model_list::{ModelArgs, ModelDLListEntry};
use leptonic::components::{grid::Col, icon::Icon, prelude::Box, progress_bar::ProgressBar};
use leptos::*;

use crate::functions::rest::llm::{model_download, model_update};

#[component]
pub fn ModelListItem(
    backend_url: Signal<String>,
    item: ModelDLListEntry,
    q_lvl: ReadSignal<String>,
    repo_id: String,
    quantized: bool,
    quantized_current: bool,
    template_current: String,
    tags_enabled: ReadSignal<Vec<String>>,
    gpu_type: Signal<String>,
    set_model_args: WriteSignal<ModelArgs>,
    init_gpu: ReadSignal<String>,
) -> impl IntoView {
    let (quantized, _set_quantized) = create_signal(quantized);
    let (has_gguf, set_has_gguf) = create_signal(item.gguf);

    // do the models have gguf/safetensors?
    let (has_safetensors, set_has_safetensors) = create_signal(item.safetensors);

    // show current if repo and gguf/safetensors match
    let (current_repo_id, _set_current_repo_id) = create_signal(item.clone().repo_id);
    let (template_signal, _set_template_signal) = create_signal(template_current);
    let (is_current, set_is_current) = create_signal(
        (current_repo_id.get() == repo_id)
            && (quantized_current == quantized.get())
            && (&template_signal.get() != "NoModel")
            && gpu_type.get() == init_gpu.get(),
    );
    let check_cuda_or_mac = gpu_type.get() == "Mac" || gpu_type.get() == "CUDA";
    let (cpu, _set_cpu) = create_signal(!check_cuda_or_mac);

    let (name_signal, _set_name_signal) = create_signal(item.clone().name);
    let (base_signal, _set_base_signal) = create_signal(item.clone().base.to_ascii_uppercase());

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Template {
        name: String,
    }
    let mut template_for_select = item
        .clone()
        .template
        .unwrap_or(vec![String::new()])
        .iter()
        .map(|x| Template {
            name: x.to_string(),
        })
        .collect::<Vec<Template>>();
    if template_for_select.len() > 1 {
        template_for_select.push(Template {
            name: String::new(),
        });
    };
    let _template_has_n = template_for_select.clone().len();
    let (template, _set_template) = create_signal(template_for_select[0].clone());
    let (_template_for_select_signal, _set_template_for_select_signal) =
        create_signal(template_for_select.clone());

    let (loading, set_loading) = create_signal(false);

    let submit = create_action(move |_| {
        // Args as individual vars
        let set_args_for_json = ModelArgs {
            repo_id: current_repo_id.get().to_string(),
            q_lvl: q_lvl.get(),
            revision: "main".to_string(),
            tokenizer_file: None,
            weight_file: None,
            quantized: quantized.get(),
            use_flash_attn: false,
            cpu: cpu.get(),
            template: Some(template.get().name),
        };

        async move {
            if (quantized.get() && has_gguf.get()) || (!quantized.get() && has_safetensors.get()) {
                leptos_dom::log!("Update Model");
                let args_to_set = model_update(set_args_for_json.clone(), backend_url.get()).await;
                set_model_args.set(args_to_set);
                set_is_current.set(true);
                set_loading.set(false);
            } else if quantized.get() && !has_gguf.get() {
                leptos_dom::log!("Download GGUF");
                model_download(set_args_for_json.clone(), backend_url.get()).await;
                // indicate .gguf is downloaded
                set_has_gguf.set(true);
                set_loading.set(false);
            } else if !quantized.get() && !has_safetensors.get() {
                leptos_dom::log!("Download Safetensors");
                model_download(set_args_for_json.clone(), backend_url.get()).await;
                // indicate .safetensors is downloaded
                set_has_safetensors.set(true);
                set_loading.set(false);
            };
        }
    });

    view! {
        <Show when=move || {
            tags_enabled.get().iter().any(|t| item.tags.join(" ").contains(t) && !t.is_empty())
        }>
            <Col class="model-cols" xl=3 lg=4 md=5 sm=6 xs=8>
                <form
                    style=""
                    on:submit=move |ev| {
                        ev.prevent_default();
                        set_loading.set(true);
                        submit.dispatch(());
                    }
                >

                    <a
                        href=format!("https://hf.co/{}", current_repo_id.get())
                        target="_blank"
                        rel="noreferrer"
                        style="padding:0.25rem;font-size:110%;text-align:center;background:#ccc;"
                    >
                        {name_signal.get()}
                        " "
                        <Icon icon=icondata::BsLink45deg/>
                    </a>
                    <Box style="display:flex;flex-direction:row;justify-content:space-around;">
                        <p style="margin:0.25rem;padding:0.25rem;font-size:70%;">
                            {base_signal.get()}
                        </p>

                    // <Show
                    // when=move || (template_has_n > 1)
                    // fallback=move || view! { <p>{template.get().name}</p> }
                    // >
                    // <Select
                    // options=template_for_select_signal.get()
                    // search_text_provider=move |o: Template| format!("{:?}", o.name)
                    // render_option=move |o: Template| {
                    // format!(
                    // "{:?}",
                    // if !o.name.is_empty() { o.name } else { "None".to_string() },
                    // )
                    // }

                    // selected=template
                    // set_selected=move |v| set_template.set(v)
                    // />
                    // </Show>

                    </Box>

                    <Show
                        when=move || !loading.get()
                        fallback=move || view! { <ProgressBar progress=create_signal(None).0/> }
                    >
                        <button
                            disabled=is_current.get()
                            class=if is_current.get() {
                                "is-current"
                            } else {
                                if quantized.get() {
                                    if !has_gguf.get() {
                                        "not-current"
                                    } else {
                                        "not-current-but-has"
                                    }
                                } else if !has_safetensors.get() {
                                    "not-current"
                                } else {
                                    "not-current-but-has"
                                }
                            }

                            type="submit"
                        >
                            {move || {
                                if is_current.get() {
                                    if quantized.get() {
                                        "Using Quantized! ✅"
                                    } else {
                                        "Using Safetensors! ✅"
                                    }
                                } else if quantized.get() {
                                    if !has_gguf.get() {
                                        "Download Quantized! 📥"
                                    } else {
                                        "Use Quantized!"
                                    }
                                } else if !has_safetensors.get() {
                                    "Download Safetensors! 📥"
                                } else {
                                    "Use Safetensors!"
                                }
                            }}

                        </button>
                    </Show>
                </form>
            </Col>
        </Show>
    }
}
