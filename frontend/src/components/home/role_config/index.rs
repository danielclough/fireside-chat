use crate::functions::rest::role::get_role_list;

use super::role_list_tab::RoleListTab;
use common::llm::inference::InferenceArgsForInput;
use leptonic::{
    components::{tab::Tab, tabs::Tabs},
    Mount,
};
use leptos::*;

#[component]
pub fn RoleListContainer(
    backend_url: Signal<String>,
    inference_args: Signal<InferenceArgsForInput>,
    set_inference_args: WriteSignal<InferenceArgsForInput>,
) -> impl IntoView {
    let role_list_resource = create_resource(
        move || inference_args.get(),
        move |_| async move {
            logging::log!("loading role_list from API");
            get_role_list(backend_url.get()).await
        },
    );

    view! {
        <Transition>
            {move || {
                role_list_resource
                    .get()
                    .map(|role_list| {
                        view! {
                            <Tabs mount=Mount::Once>
                                <Tab name="human" label="Human".into_view()>
                                    <RoleListTab
                                        backend_url=backend_url
                                        role_list=role_list.human.clone()
                                        inference_args=inference_args
                                        set_inference_args=set_inference_args
                                    />
                                </Tab>
                                <Tab name="computer" label="Computer".into_view()>
                                    <RoleListTab
                                        backend_url=backend_url
                                        role_list=role_list.computer.clone()
                                        inference_args=inference_args
                                        set_inference_args=set_inference_args
                                    />
                                </Tab>
                            </Tabs>
                        }
                    })
            }}

        </Transition>
    }
}
