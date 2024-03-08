use crate::components::home::model_config::model_list_chip::ModelListChip;
use crate::components::home::model_config::model_list_item::ModelListItem;
use common::llm::model_list::{ModelArgs, ModelDLList};
use leptonic::{
    components::{
        grid::{Grid, Row},
        typography::H1,
    },
    Size,
};
use leptos::*;

#[component]
pub fn ModelListGrid(
    model_list: ReadSignal<ModelDLList>,
    tags_all: ReadSignal<Vec<String>>,
    backend_url: Signal<String>,
    template_current: ReadSignal<String>,
    quantized_current: ReadSignal<bool>,
    quantized: bool,
    gpu_type: Signal<String>,
    q_lvl: ReadSignal<String>,
    model_args: Signal<ModelArgs>,
    set_model_args: WriteSignal<ModelArgs>,
    init_gpu: ReadSignal<String>,
) -> impl IntoView {
    let (tags_enabled, set_tags_enabled) = create_signal::<Vec<String>>({
        let mut list = tags_all.get()[1..tags_all.get().len()].to_owned();
        if quantized {
            list.retain(|x| x != "safetensors")
        } else {
            list.retain(|x| x != "gguf")
        };
        list
    });

    let (all_enabled, _set_all_enabled) =
        create_signal::<bool>(tags_all.get().len() == tags_enabled.get().len());

    view! {
        <Grid style="padding:1rem;" spacing=Size::Em(0.6)>
            <H1 style="width:100%;padding:0.25rem;text-align:center;box-shadow:2px 2px 8px #000;">
                "Model List"
            </H1>

            <Row>
                <For
                    each=move || tags_all.get().into_iter().enumerate()
                    key=|(index, _)| *index
                    let:item
                >
                    <ModelListChip
                        name=item.clone().1
                        set_tags_enabled=set_tags_enabled
                        all_enabled=all_enabled
                        tags_enabled=tags_enabled
                        tags_all=tags_all
                    />
                </For>
            </Row>

            <Row>
                <For each=move || model_list.get().list.clone() key=|list| list.clone() let:item>
                    <ModelListItem
                        backend_url=backend_url
                        item=item
                        template_current=template_current.get()
                        repo_id=model_args.get().repo_id
                        quantized_current=quantized_current.get()
                        quantized=quantized
                        tags_enabled=tags_enabled
                        gpu_type=gpu_type
                        q_lvl=q_lvl
                        set_model_args=set_model_args
                        init_gpu=init_gpu
                    />
                </For>
            </Row>
        </Grid>
    }
}
