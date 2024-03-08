use common::database::user::UserForJson;
use leptonic::components::{
    button::{Button, ButtonColor, ButtonWrapper},
    input::TextInput,
    modal::{Modal, ModalBody, ModalFooter, ModalHeader, ModalTitle},
};
use leptos::{
    component, create_signal, view, IntoView, Signal, SignalGet, SignalUpdate, WriteSignal,
};

use crate::functions::rest::user::switch_users;

#[component]
pub fn InitUserModal<A, C>(
    #[prop(into)] show_when: Signal<bool>,
    set_user: WriteSignal<UserForJson>,
    user: Signal<UserForJson>,
    on_accept: A,
    on_cancel: C,
    database_url: Signal<String>,
) -> impl IntoView
where
    A: Fn() + Copy + 'static,
    C: Fn() + Copy + 'static,
{
    let (input, set_input) = create_signal(String::new());

    let confirmed = move || input.get().len() > 2;
    let disabled = Signal::derive(move || !confirmed());

    let on_accept = move || {
        switch_users(user, set_user, input.get(), database_url);
        set_input.update(|it| it.clear());
        (on_accept)();
    };
    let on_cancel = move || {
        set_input.update(|it| it.clear());
        (on_cancel)();
    };

    view! {
        <Modal show_when=show_when on_escape=on_cancel>
            <ModalHeader>
                <ModalTitle>"Create Username!"</ModalTitle>
            </ModalHeader>
            <ModalBody>
                "Please enter a username (with more than 2 characters)."
                <TextInput get=input set=set_input/>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button
                        on_press=move |_| (on_accept)()
                        disabled=disabled
                        color=ButtonColor::Danger
                    >
                        "Confirm"
                    </Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
