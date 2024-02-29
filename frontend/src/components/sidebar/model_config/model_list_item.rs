use common::llm::model_list::{ModelArgs, ModelDLListEntry};
use leptonic::{
    grid::Col,
    icon::Icon,
    prelude::Box,
    progress_bar::ProgressBar,
    select::Select,
};
use leptos::{html::Input, *};

use crate::functions::rest::llm::{model_download, model_update};

#[component]
pub fn ModelListItem(
    ipv4: Signal<String>,
    item: ModelDLListEntry,
    q_lvl: ReadSignal<String>,
    repo_id: String,
    quantized: bool,
    quantized_current: bool,
    template_current: String,
    tags_enabled: ReadSignal<Vec<String>>,
    gpu_type: Signal<String>,
) -> impl IntoView {
    let (repo_id_in_use, _set_repo_id_in_use) = create_signal(repo_id);
    let (current_repo_id, set_current_repo_id) = create_signal(item.clone().repo_id);
    let (quantized, set_quantized) = create_signal(quantized);

    // do the models have gguf/safetensors?
    let (gguf, set_gguf) = create_signal(false);
    let (safetensors, set_safetensors) = create_signal(false);

    // show current if repo and gguf/safetensors match
    let is_current =
        (current_repo_id.get() == repo_id_in_use.get()) && (quantized_current == quantized.get());

    let (model_config, set_model_config) = create_signal(String::new());
    let (revision, set_revision) = create_signal(String::new());
    let (tokenizer_file, set_tokenizer_file) = create_signal(String::new());
    let (weight_file, set_weight_file) = create_signal(String::new());
    let (use_flash_attn, set_use_flash_attn) = create_signal(false);
    let check_cuda_or_mac = gpu_type.get() == "Mac" || gpu_type.get() == "CUDA";
    let (cpu, set_cpu) = create_signal({
        if gpu_type.get() == "None" {
            true
        } else { !(check_cuda_or_mac && quantized.get()) }
    });

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
    let template_has_n = template_for_select.clone().len();
    let (template, set_template) = create_signal(template_for_select[0].clone());
    let (template_for_select_signal, _set_template_for_select_signal) =
        create_signal(template_for_select.clone());

    let repo_id_input_element: NodeRef<Input> = create_node_ref();
    let gguf_input_element: NodeRef<Input> = create_node_ref();
    let safetensors_input_element: NodeRef<Input> = create_node_ref();

    let (_get_model_args, set_model_args) = create_signal(ModelArgs {
        use_flash_attn: false,
        repo_id: String::new(),
        template: None,
        q_lvl: q_lvl.get(),
        revision: "main".to_string(),
        tokenizer_file: None,
        weight_file: None,
        quantized: quantized.get(),
        cpu: cpu.get(),
    });

    // Set set_model_args
    let set_args_for_form = move |args: ModelArgs| {
        // Set ModelArgs strut
        set_model_args.set(args);

        // Args as individual vars
        set_current_repo_id.set(current_repo_id.get());
        set_model_config.set(model_config.get());
        set_revision.set(revision.get());
        set_tokenizer_file.set(tokenizer_file.get());
        set_weight_file.set(weight_file.get());
        set_quantized.set(quantized.get());
        set_cpu.set(cpu.get());
        set_use_flash_attn.set(use_flash_attn.get());
    };

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
            set_args_for_form(
                if (quantized.get() && gguf.get()) || (!quantized.get() && safetensors.get()) {
                    model_update(set_args_for_json, ipv4.get()).await
                } else {
                    model_download(set_args_for_json, ipv4.get()).await
                },
            );
            _ = leptos_dom::window().location().reload();
        }
    });

    let (loading, set_loading) = create_signal(false);
    let (template_signal, _set_template_signal) = create_signal(template_current);

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
                        let repo_id_element_value = repo_id_input_element
                            .get()
                            .expect("<input> exists")
                            .value();
                        let gguf_element_value = gguf_input_element
                            .get()
                            .expect("<input> exists")
                            .value();
                        let safetensors_element_value = safetensors_input_element
                            .get()
                            .expect("<input> exists")
                            .value();
                        set_current_repo_id.set(repo_id_element_value);
                        set_gguf.set(gguf_element_value == "value");
                        set_safetensors.set(safetensors_element_value == "value");
                        submit.dispatch(());
                    }
                >

                    // invisible inputs for node_ref
                    <input
                        class="hidden"
                        type="text"
                        value=current_repo_id.get()
                        node_ref=repo_id_input_element
                    />
                    <input
                        class="hidden"
                        type="text"
                        value=item.gguf.clone()
                        node_ref=gguf_input_element
                    />
                    <input
                        class="hidden"
                        type="text"
                        value=item.safetensors.clone()
                        node_ref=safetensors_input_element
                    />
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

                        <Show
                            when=move || (template_has_n > 1)
                            fallback=move || view! { <p>{template.get().name}</p> }
                        >
                            <Select
                                options=template_for_select_signal.get()
                                search_text_provider=move |o: Template| format!("{:?}", o.name)
                                render_option=move |o: Template| {
                                    format!(
                                        "{:?}",
                                        if !o.name.is_empty() { o.name } else { "None".to_string() },
                                    )
                                }

                                selected=template
                                set_selected=move |v| set_template.set(v)
                            />
                        </Show>

                    </Box>
                    <Show
                        when=move || !loading.get()
                        fallback=move || view! { <ProgressBar progress=create_signal(None).0/> }
                    >
                        <button
                            disabled=is_current && (&template_signal.get() != "NoModel")
                            class=if is_current && (&template_signal.get() != "NoModel") {
                                "is-current"
                            } else {
                                "not-current"
                            }

                            type="submit"
                        >
                            {move || {
                                if is_current && (&template_signal.get() != "NoModel") {
                                    if quantized.get() {
                                        "Using Quantized! ✅"
                                    } else {
                                        "Using Safetensors! ✅"
                                    }
                                } else if quantized.get() {
                                    if !item.gguf {
                                        "Download Quantized! 📥"
                                    } else {
                                        "Use Quantized! 🗃️"
                                    }
                                } else if !item.safetensors {
                                    "Download Safetensors! 📥"
                                } else {
                                    "Use Safetensors! 🗃️"
                                }
                            }}

                        </button>
                    </Show>
                </form>
            </Col>
        </Show>
    }
}
