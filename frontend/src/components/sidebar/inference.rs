use gloo_net::http::{Request, Response};
use leptonic::prelude::*;
use leptos::prelude::*;
use leptos::*;

use serde::{Deserialize, Serialize};

use crate::components::utils::get_path;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct InferenceArgsForInput {
    pub temperature: f64,
    pub top_p: f64,
    pub seed: f64,
    pub sample_len: f64,
    pub repeat_penalty: f64,
    pub repeat_last_n: f64,
    pub load_context: bool,
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InferenceArgsForJson {
    pub temperature: f64,
    pub top_p: f64,
    pub seed: u64,
    pub sample_len: usize,
    pub repeat_penalty: f32,
    pub repeat_last_n: usize,
    pub load_context: bool,
}

// Load context from backend/context/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum LoadContext {
    True,
    False
}

#[component]
pub fn Inference() -> impl IntoView {
    // f64 for input values
    let (inference_args, set_inference_args) = create_signal(InferenceArgsForInput {
        temperature: 0.0,
        top_p: 0.0,
        seed: 0.0,
        sample_len: 0.0,
        repeat_penalty: 0.0,
        repeat_last_n: 0.0,
        load_context: false,
    });

    let (temperature, set_temperature) = create_signal(0.0);
    let temperature_string =
        Signal::derive(move || format!("{:.1}", inference_args.get().temperature));

    let (top_p, set_top_p) = create_signal(0.0);
    let top_p_string = Signal::derive(move || format!("{:.1}", inference_args.get().top_p));

    let (seed, set_seed) = create_signal(0.0);
    let seed_string = Signal::derive(move || format!("{}", inference_args.get().seed));

    let (sample_len, set_sample_len) = create_signal(0.0);
    let sample_len_string = Signal::derive(move || format!("{}", inference_args.get().sample_len));

    let (repeat_penalty, set_repeat_penalty) = create_signal(0.0);
    let repeat_penalty_string =
        Signal::derive(move || format!("{:.1}", inference_args.get().repeat_penalty));

    let (repeat_last_n, set_repeat_last_n) = create_signal(0.0);
    let repeat_last_n_string =
        Signal::derive(move || format!("{}", inference_args.get().repeat_last_n));

    let (load_context, set_load_context) = create_signal(LoadContext::False);
    let load_context_enum =
        Signal::derive(move || if inference_args.get().load_context == true {LoadContext::True} else {LoadContext::False} );

    // Set set_inference_args
    let set_args_for_form = move |args: InferenceArgsForInput| {
        // Set InferenceArgs strut
        set_inference_args.set(args);

        // Args as individual vars
        set_temperature.set(inference_args.get().temperature);
        set_top_p.set(inference_args.get().top_p);
        set_seed.set(inference_args.get().seed);
        set_sample_len.set(inference_args.get().sample_len);
        set_repeat_penalty.set(inference_args.get().repeat_penalty);
        set_repeat_last_n.set(inference_args.get().repeat_last_n);
        // Bool to Enum
        let load_context_value = if inference_args.get().load_context == true {LoadContext::True} else {LoadContext::False};
        set_load_context.set(load_context_value);
    };

    async fn patch_async(set_args_for_json: InferenceArgsForJson) -> Response {
        let path = get_path("http");

        Request::patch(&path)
            .header("Content-Type", "application/json")
            .json(&set_args_for_json)
            .unwrap()
            .send()
            .await
            .unwrap()
    }

    let submit_args = create_action(move |_| {
        let load_context = if load_context.get() == LoadContext::True {
            true
        } else {
            false
        };
        // Args as individual vars
        let set_args_for_json = InferenceArgsForJson {
            temperature: temperature.get(),
            top_p: top_p.get(),
            seed: seed.get() as u64,
            sample_len: sample_len.get() as usize,
            repeat_penalty: repeat_penalty.get() as f32,
            repeat_last_n: repeat_last_n.get() as usize,
            load_context,
        };
        let resp = patch_async(set_args_for_json);

        async move {
            set_args_for_form(resp.await.json().await.unwrap());
        }
    });

    // !!FIX!! Running Twice
    let fetch_args = move |_| {
        let path = get_path("http");

        async move {
            let args = Request::get(&path)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            set_args_for_form(args);
        }
    };
    let fetch_show = create_resource(move || inference_args.get(), fetch_args);
    view! {
        {move || match fetch_show.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(_data) => view! {
            <Box class="api-box">
                <P class="above-input"><strong>"Temperature: "</strong> "The temperature used to generate samples."</P>
                <NumberInput min=0.0 max=1.0 step=0.01
                    get=temperature
                    set=set_temperature
                />
                <P class="under-input">"Temperature is: " {move || temperature_string.get()}</P>

                <P class="above-input"><strong>"Top_p: "</strong> "Nucleus sampling probability cutoff."</P>
                <NumberInput min=0.0 max=1.0 step=0.1
                    get=top_p
                    set=set_top_p
                />
                <P class="under-input">"Top_p is: " {move || top_p_string.get()}</P>

                <P class="above-input"><strong>"Seed: "</strong> "The seed to use when generating random samples."</P>
                <NumberInput min=1.0 max=999999999.0 step=1.0
                    get=seed
                    set=set_seed
                />
                <P class="under-input">"Seed is: " {move || seed_string.get()}</P>

                <P class="above-input"><strong>"Sample_len: "</strong> "The length of the sample to generate (in tokens)."</P>
                <NumberInput min=0.0 max=999.0 step=1.0
                    get=sample_len
                    set=set_sample_len
                />
                <P class="under-input">"Sample_len is: " {move || sample_len_string.get()}</P>

                <P class="above-input"><strong>"Repeat_penalty: "</strong> "Penalty to be applied for repeating tokens, 1. means no penalty."</P>
                <NumberInput min=0.0 max=2.0 step=0.1
                    get=repeat_penalty
                    set=set_repeat_penalty
                />
                <P class="under-input">"Repeat_penalty is: " {move || repeat_penalty_string.get()}</P>

                <P class="above-input"><strong>"Repeat_last_n: "</strong> "The context size to consider for the repeat penalty."</P>
                <NumberInput min=1.0 max=999.0 step=1.0
                    get=repeat_last_n
                    set=set_repeat_last_n
                />
                <P class="under-input">"Repeat_last_n is: " {move || repeat_last_n_string.get()}</P>

                <P class="above-input"><strong>"Load_context: "</strong> "Load context from backend/context/*"</P>
                <Select
                    options=vec![LoadContext::True, LoadContext::False]
                    search_text_provider=move |o| format!("{:?}", o)
                    render_option=move |o| format!("{o:?}")
                    selected=load_context
                    set_selected=move |v| set_load_context.set(v)
                />
                <P class="under-input">"Load_context is: " {move || format!("{:?}", load_context_enum.get())}</P>
                

                <form on:submit= move |ev| {
                    ev.prevent_default();
                    submit_args.dispatch(());
                }>
                    <button type="submit">"Submit"</button>
                </form>
            </Box>
        }.into_view()
    }}}
}
