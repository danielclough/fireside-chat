use leptonic::components::modal::{Modal, ModalBody, ModalHeader, ModalTitle};
use leptos::{component, view, IntoView, Signal, WriteSignal};

use crate::components::home::network_config::index::NetworkConfig;

#[component]
pub fn InitNetworkModal(
    #[prop(into)] show_when: Signal<bool>,
    backend_url: Signal<String>,
    set_backend_url: WriteSignal<String>,
    database_url: Signal<String>,
    set_database_url: WriteSignal<String>,
    set_database_error: WriteSignal<bool>,
    set_backend_error: WriteSignal<bool>,
    set_show_network_init_modal: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <Modal show_when=show_when on_escape=move || {}>
            <ModalHeader>
                <ModalTitle>"Choose a Network!"</ModalTitle>
            </ModalHeader>
            <ModalBody>
                <NetworkConfig
                    backend_url=backend_url
                    set_backend_url=set_backend_url
                    database_url=database_url
                    set_database_url=set_database_url
                    set_database_error=set_database_error
                    set_backend_error=set_backend_error
                    set_show_network_init_modal=set_show_network_init_modal
                />
            </ModalBody>
        </Modal>
    }
}
