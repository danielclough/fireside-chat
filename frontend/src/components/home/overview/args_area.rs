use common::{
    database::user::UserForJson,
    llm::{inference::InferenceArgsForInput, model_list::ModelArgs},
};
use leptonic::components::{
    prelude::Box,
    typography::{H3, H4},
};
use leptos::*;

#[component]
pub fn ArgsArea(
    inference_args: Signal<InferenceArgsForInput>,
    model_args: Signal<ModelArgs>,
    user: Signal<UserForJson>,
) -> impl IntoView {
    view! {
        <Box class="args-area">
            <H3>"Overview"</H3>
            <ul>
                <H4>"User: " <code>{user.get().name}</code></H4>
                <H4>
                    "Role: "
                    <code>
                        {if inference_args.get().role.is_empty() {
                            "None".to_string()
                        } else {
                            inference_args.get().role
                        }}

                    </code>
                </H4>
            </ul>
            <H3>"Model"</H3>
            <ul>
                <li>
                    <code>{model_args.get().repo_id}</code>
                </li>
                <Show
                    when=move || model_args.get().quantized
                    fallback=move || {
                        view! { <li>"cpu: " <code>{model_args.get().cpu}</code></li> }
                    }
                >

                    <li>"Quantization: " <code>{model_args.get().q_lvl}</code></li>
                </Show>
                <li>"revision: " <code>{model_args.get().revision}</code></li>
                <Show when=move || model_args.get().tokenizer_file.is_some()>
                    <li>tokenizer_file: <code>{model_args.get().tokenizer_file}</code></li>
                </Show>
                <Show when=move || model_args.get().weight_file.is_some()>
                    <li>weight_file: <code>{model_args.get().weight_file}</code></li>
                </Show>

                <Show when=move || model_args.get().use_flash_attn>
                    <li>"Using Flash Attention!"</li>
                </Show>
                <li>"template: " <code>{model_args.get().template}</code></li>
            </ul>

            <H3>"Inference"</H3>
            <ul>
                <li>
                    "txt / pdf directory: "
                    <code>{format!("{:?}", inference_args.get().load_context)}</code>
                </li>
            </ul>
        </Box>
    }
}
