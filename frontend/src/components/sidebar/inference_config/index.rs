use common::llm::inference::{InferenceArgsForInput, InferenceArgsForJson, LoadContext};
use leptonic::prelude::*;
use leptos::prelude::*;
use leptos::*;

use crate::functions::rest::llm::patch_inference_args;

#[component]
pub fn Inference(
    ipv4: Signal<String>,
    inference_args: Signal<InferenceArgsForInput>,
    set_inference_args: WriteSignal<InferenceArgsForInput>,
    fetch_show: Resource<InferenceArgsForInput, ()>,
) -> impl IntoView {

    let load_context_enum = Signal::derive(move || {
        if inference_args.get().load_context {
            LoadContext::True
        } else {
            LoadContext::False
        }
    });
    let (load_context, set_load_context) = create_signal(load_context_enum.get());

    let (role, set_role) = create_signal(inference_args.get().role);

    let (temperature, set_temperature) = create_signal(inference_args.get().temperature);
    let temperature_string =
        Signal::derive(move || format!("{:.1}", inference_args.get().temperature));

    let (top_p, set_top_p) = create_signal(inference_args.get().top_p);
    let top_p_string = Signal::derive(move || format!("{:.1}", inference_args.get().top_p));

    let (seed, set_seed) = create_signal(inference_args.get().seed);
    let seed_string = Signal::derive(move || format!("{}", inference_args.get().seed));

    let (sample_len, set_sample_len) = create_signal(inference_args.get().sample_len);
    let sample_len_string =
        Signal::derive(move || format!("{}", inference_args.get().sample_len));

    let (repeat_penalty, set_repeat_penalty) = create_signal(inference_args.get().repeat_penalty);
    let repeat_penalty_string =
        Signal::derive(move || format!("{:.1}", inference_args.get().repeat_penalty));

    let (repeat_last_n, set_repeat_last_n) = create_signal(inference_args.get().repeat_last_n);
    let repeat_last_n_string =
        Signal::derive(move || format!("{}", inference_args.get().repeat_last_n));

    // Set set_inference_args
    let set_args_for_form = move |args: InferenceArgsForInput| {
        // Set InferenceArgs strut
        set_inference_args.set(args);

        // Args as individual vars
        set_role.set(inference_args.get().role);
        set_temperature.set(inference_args.get().temperature);
        set_top_p.set(inference_args.get().top_p);
        set_seed.set(inference_args.get().seed);
        set_sample_len.set(inference_args.get().sample_len);
        set_repeat_penalty.set(inference_args.get().repeat_penalty);
        set_repeat_last_n.set(inference_args.get().repeat_last_n);
        // Bool to Enum
        let load_context_value = if inference_args.get().load_context {
            LoadContext::True
        } else {
            LoadContext::False
        };
        set_load_context.set(load_context_value);
    };

    let submit_args = create_action(move |_| {
        let load_context = load_context.get() == LoadContext::True;
        // Args as individual vars
        let set_args_for_json = InferenceArgsForJson {
            temperature: temperature.get(),
            top_p: top_p.get(),
            seed: seed.get() as u64,
            sample_len: sample_len.get() as usize,
            repeat_penalty: repeat_penalty.get() as f32,
            repeat_last_n: repeat_last_n.get() as usize,
            load_context,
            role: role.get(),
        };

        async move {
            set_args_for_form(patch_inference_args(set_args_for_json, ipv4.get()).await);
            _ = leptos_dom::window().location().reload();
        }
    });
    view! {
        {move || match fetch_show.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(_data) => {
                view! {
                    <Box class="api-box">
                        <P class="above-input">
                            <strong>"Temperature: "</strong>
                            "The temperature used to generate samples."
                        </P>
                        <NumberInput min=0.0 max=1.0 step=0.01 get=temperature set=set_temperature/>
                        <P class="under-input">
                            "Temperature is: " {move || temperature_string.get()}
                        </P>

                        <P class="above-input">
                            <strong>"Top_p: "</strong>
                            "Nucleus sampling probability cutoff."
                        </P>
                        <NumberInput min=0.0 max=1.0 step=0.1 get=top_p set=set_top_p/>
                        <P class="under-input">"Top_p is: " {move || top_p_string.get()}</P>

                        <P class="above-input">
                            <strong>"Seed: "</strong>
                            "The seed to use when generating random samples."
                        </P>
                        <NumberInput min=1.0 max=999999999.0 step=1.0 get=seed set=set_seed/>
                        <P class="under-input">"Seed is: " {move || seed_string.get()}</P>

                        <P class="above-input">
                            <strong>"Sample_len: "</strong>
                            "The length of the sample to generate (in tokens)."
                        </P>
                        <NumberInput min=0.0 max=999.0 step=1.0 get=sample_len set=set_sample_len/>
                        <P class="under-input">
                            "Sample_len is: " {move || sample_len_string.get()}
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
                            "Repeat_penalty is: " {move || repeat_penalty_string.get()}
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
                            "Repeat_last_n is: " {move || repeat_last_n_string.get()}
                        </P>

                        <P class="above-input">
                            <strong>"Load_context: "</strong>
                            "Load context from backend/context/*"
                        </P>
                        <Select
                            options=vec![LoadContext::True, LoadContext::False]
                            search_text_provider=move |o| format!("{:?}", o)
                            render_option=move |o| format!("{o:?}")
                            selected=load_context
                            set_selected=move |v| set_load_context.set(v)
                        />
                        <P class="under-input">
                            "Load_context is: " {move || format!("{:?}", load_context_enum.get())}
                        </P>

                        <form on:submit=move |ev| {
                            ev.prevent_default();
                            submit_args.dispatch(());
                        }>
                            <button type="submit">"Submit"</button>
                        </form>
                    </Box>
                }
                    .into_view()
            }
        }}
    }
}
