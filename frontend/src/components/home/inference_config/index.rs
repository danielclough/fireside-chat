use common::llm::inference::{InferenceArgsForInput, InferenceArgsForJson, LoadContext};
use leptonic::components::prelude::*;
use leptos::prelude::*;
use leptos::*;

use crate::functions::rest::llm::{get_inference_args, patch_inference_args};

#[component]
pub fn Inference(
    backend_url: Signal<String>,
    inference_args: Signal<InferenceArgsForInput>,
    set_inference_args: WriteSignal<InferenceArgsForInput>,
) -> impl IntoView {
    let load_context_enum = Signal::derive(move || {
        if inference_args.get().load_context {
            LoadContext::True
        } else {
            LoadContext::False
        }
    });
    let (load_context, set_load_context) = create_signal(load_context_enum.get());

    let (temperature, set_temperature) = create_signal(inference_args.get().temperature);
    let (top_p, set_top_p) = create_signal(inference_args.get().top_p);
    let (seed, set_seed) = create_signal(inference_args.get().seed);
    let (sample_len, set_sample_len) = create_signal(inference_args.get().sample_len);
    Signal::derive(move || format!("{}", inference_args.get().sample_len));
    let (repeat_penalty, set_repeat_penalty) = create_signal(inference_args.get().repeat_penalty);
    Signal::derive(move || format!("{:.1}", inference_args.get().repeat_penalty));
    let (repeat_last_n, set_repeat_last_n) = create_signal(inference_args.get().repeat_last_n);
    Signal::derive(move || format!("{}", inference_args.get().repeat_last_n));

    let inference_args_resource = create_resource(
        move || (inference_args.get(), backend_url.get()),
        move |_| async move {
            logging::log!("loading get_inference_args from API");
            get_inference_args(backend_url.get()).await
        },
    );

    // Set set_inference_args
    let set_args_for_form = move |args: InferenceArgsForInput| {
        // Args as individual vars
        set_top_p.set(args.top_p);
        set_seed.set(args.seed);
        set_sample_len.set(args.sample_len);
        set_repeat_penalty.set(args.repeat_penalty);
        set_repeat_last_n.set(args.repeat_last_n);
        // Bool to Enum
        let load_context_value = if args.load_context {
            LoadContext::True
        } else {
            LoadContext::False
        };
        set_load_context.set(load_context_value);
        // Set InferenceArgs strut
        set_inference_args.set(args);
    };

    let submit_args = create_action(move |_| async move {
        set_args_for_form(
            patch_inference_args(
                InferenceArgsForJson {
                    temperature: temperature.get(),
                    top_p: top_p.get(),
                    seed: seed.get() as u64,
                    sample_len: sample_len.get() as usize,
                    repeat_penalty: repeat_penalty.get() as f32,
                    repeat_last_n: repeat_last_n.get() as usize,
                    load_context: load_context.get() == LoadContext::True,
                    role: inference_args.get().role,
                },
                backend_url.get(),
            )
            .await,
        );
    });
    view! {
        <Transition fallback=move || {
            view! { <p>"Initializing..."</p> }
        }>
            {move || {
                inference_args_resource
                    .get()
                    .map(|inference_args_from_resource| {
                        set_args_for_form(inference_args_from_resource.clone());
                        view! {
                            <Box class="api-box">
                                <P class="above-input">
                                    <strong>"Temperature: "</strong>
                                    "The temperature used to generate samples."
                                </P>
                                <NumberInput
                                    min=0.0
                                    max=1.0
                                    step=0.01
                                    get=temperature
                                    set=set_temperature
                                />
                                <P class="under-input">
                                    "Temperature is: "
                                    {move || inference_args_from_resource.temperature}
                                </P>

                                <P class="above-input">
                                    <strong>"Top_p: "</strong>
                                    "Nucleus sampling probability cutoff."
                                </P>
                                <NumberInput min=0.0 max=1.0 step=0.1 get=top_p set=set_top_p/>
                                <P class="under-input">
                                    "Top_p is: " {move || inference_args_from_resource.top_p}
                                </P>

                                <P class="above-input">
                                    <strong>"Seed: "</strong>
                                    "The seed to use when generating random samples."
                                </P>
                                <NumberInput
                                    min=1.0
                                    max=999999999.0
                                    step=1.0
                                    get=seed
                                    set=set_seed
                                />
                                <P class="under-input">
                                    "Seed is: " {move || inference_args_from_resource.seed}
                                </P>

                                <P class="above-input">
                                    <strong>"Sample_len: "</strong>
                                    "The length of the sample to generate (in tokens)."
                                </P>
                                <NumberInput
                                    min=0.0
                                    max=999.0
                                    step=1.0
                                    get=sample_len
                                    set=set_sample_len
                                />
                                <P class="under-input">
                                    "Sample_len is: "
                                    {move || inference_args_from_resource.sample_len}
                                </P>

                                <P class="above-input">
                                    <strong>"Repeat_penalty: "</strong>
                                    "Penalty to be applied for repeating tokens, 1. means no penalty."
                                </P>
                                <NumberInput
                                    min=0.0
                                    max=2.0
                                    step=0.1
                                    get=repeat_penalty
                                    set=set_repeat_penalty
                                />
                                <P class="under-input">
                                    "Repeat_penalty is: "
                                    {move || inference_args_from_resource.repeat_penalty}
                                </P>

                                <P class="above-input">
                                    <strong>"Repeat_last_n: "</strong>
                                    "The context size to consider for the repeat penalty."
                                </P>
                                <NumberInput
                                    min=1.0
                                    max=999.0
                                    step=1.0
                                    get=repeat_last_n
                                    set=set_repeat_last_n
                                />
                                <P class="under-input">
                                    "Repeat_last_n is: "
                                    {move || inference_args_from_resource.repeat_last_n}
                                </P>

                                <P class="above-input">
                                    <strong>"Load_context: "</strong>
                                    "Load context from backend/context/*"
                                </P>
                                // <Select
                                // options=vec![LoadContext::True, LoadContext::False]
                                // search_text_provider=move |o| format!("{:?}", o)
                                // render_option=move |o| format!("{o:?}")
                                // selected=load_context
                                // set_selected=move |v| set_load_context.set(v)
                                // />
                                <P class="under-input">
                                    "Load_context is: "
                                    {move || format!("{:?}", load_context_enum.get())}
                                </P>

                                <form on:submit=move |ev| {
                                    ev.prevent_default();
                                    submit_args.dispatch(());
                                }>
                                    <button type="submit">"Submit"</button>
                                </form>
                            </Box>
                        }
                    })
            }}

        </Transition>
    }
}
