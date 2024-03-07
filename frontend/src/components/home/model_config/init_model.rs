use common::llm::model_list::ModelArgs;
use leptonic::modal::{Modal, ModalBody, ModalHeader, ModalTitle};
use leptos::{component, view, IntoView, Signal, WriteSignal};

use crate::components::home::model_config::index::ModelConfig;

#[component]
pub fn InitModelModal(
    #[prop(into)] show_when: Signal<bool>,
    model_args: Signal<ModelArgs>,
    backend_url: Signal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
    set_model_args: WriteSignal<ModelArgs>,
) -> impl IntoView {
    view! {
        <Modal show_when=show_when on_escape=move || {}>
            <ModalHeader>
                <ModalTitle>"Choose a Model!"</ModalTitle>
            </ModalHeader>
            <ModalBody>
                <ModelConfig
                    backend_url=backend_url
                    model_args=model_args
                    set_model_args=set_model_args
                    gpu_type=gpu_type
                    set_gpu_type=set_gpu_type
                />
            </ModalBody>
        </Modal>
    }
}
