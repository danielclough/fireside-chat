use common::llm::{
    inference::{InferenceArgsForInput, InferenceArgsForJson},
    role_list::RoleListEntry,
};
use leptonic::components::collapsible::{Collapsible, CollapsibleBody, CollapsibleHeader};
use leptos::{html::Input, *};

use crate::functions::rest::role::patch_role_list;

#[component]
pub fn RoleListItem(
    backend_url: Signal<String>,
    item: RoleListEntry,
    inference_args: Signal<InferenceArgsForInput>,
    set_inference_args: WriteSignal<InferenceArgsForInput>,
) -> impl IntoView {
    let item_role_input_element: NodeRef<Input> = create_node_ref();

    // Set set_inference_args
    let set_args_for_form = move |args: InferenceArgsForInput| {
        // Set InferenceArgs strut
        set_inference_args.set(args);
    };

    let submit_args = create_action(move |_| {
        // let load_context = if inference_args.get().load_context {
        //     LoadContext::True
        // } else {
        //     LoadContext::False
        // };
        // Args as individual vars
        let set_args_for_json = InferenceArgsForJson {
            temperature: inference_args.get().temperature,
            top_p: inference_args.get().top_p,
            seed: inference_args.get().seed as u64,
            sample_len: inference_args.get().sample_len as usize,
            repeat_penalty: inference_args.get().repeat_penalty as f32,
            repeat_last_n: inference_args.get().repeat_last_n as usize,
            load_context: inference_args.get().load_context,
            role: inference_args.get().role,
        };

        async move {
            set_args_for_form(patch_role_list(set_args_for_json, backend_url.get()).await);
        }
    });

    view! {
        <Collapsible>
            <CollapsibleHeader
                class=if item.role == inference_args.get().role {
                    "is-current role-header"
                } else {
                    "role-header"
                }

                slot
            >
                <form
                    class="role-header"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        let item_role_element_value = item_role_input_element
                            .get()
                            .expect("<input> exists")
                            .value();
                        set_inference_args
                            .set(InferenceArgsForInput {
                                temperature: inference_args.get().temperature,
                                top_p: inference_args.get().top_p,
                                seed: inference_args.get().seed,
                                sample_len: inference_args.get().sample_len,
                                repeat_penalty: inference_args.get().repeat_penalty,
                                repeat_last_n: inference_args.get().repeat_last_n,
                                load_context: inference_args.get().load_context,
                                role: item_role_element_value,
                            });
                        submit_args.dispatch(());
                    }
                >

                    <p style="width:100%;padding:0.25rem;text-align:center;font-size:xx-large;">
                        <input
                            disabled
                            style="width:100%;"
                            type="text"
                            node_ref=item_role_input_element
                            value=&item.role.to_owned()
                        />
                    </p>

                    <button
                        class=if item.role == inference_args.get().role { "hidden" } else { "" }
                        type="submit"
                    >
                        "Use this"
                    </button>
                </form>
            </CollapsibleHeader>
            <CollapsibleBody slot>{item.prompt}</CollapsibleBody>
        </Collapsible>
    }
}
