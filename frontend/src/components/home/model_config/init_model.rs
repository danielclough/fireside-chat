use common::llm::model_list::ModelArgs;
use leptonic::components::{
    button::{Button, ButtonColor, ButtonWrapper},
    modal::{Modal, ModalBody, ModalFooter, ModalHeader, ModalTitle},
};
use leptos::{component, view, IntoView, Show, Signal, SignalGet, WriteSignal};

use crate::components::home::model_config::index::ModelConfig;

#[component]
pub fn InitModelModal<A>(
    #[prop(into)] show_when: Signal<bool>,
    model_args: Signal<ModelArgs>,
    on_accept: A,
    backend_url: Signal<String>,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
    set_model_args: WriteSignal<ModelArgs>,
) -> impl IntoView
where
    A: Fn() + Copy + 'static,
{
    view! {
        <Modal show_when=show_when on_escape=move || (on_accept)()>
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
            <Show when=move || show_when.get()>
                <ModalFooter>
                    <ButtonWrapper>
                        <Button on_press=move |_| (on_accept)() color=ButtonColor::Danger>
                            "Confirm"
                        </Button>
                    </ButtonWrapper>
                </ModalFooter>
            </Show>
        </Modal>
    }
}
