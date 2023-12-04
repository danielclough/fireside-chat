use leptonic::prelude::*;
use leptos::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct InferenceArgs {
    /// The temperature used to generate samples.
    pub temperature: Option<f64>,
    /// Nucleus sampling probability cutoff.
    pub top_p: Option<f64>,
    /// The seed to use when generating random samples.
    pub seed: u64,
    /// The length of the sample to generate (in tokens).
    pub sample_len: usize,
    /// Penalty to be applied for repeating tokens, 1. means no penalty.
    pub repeat_penalty: f32,
    /// The context size to consider for the repeat penalty.
    pub repeat_last_n: usize,
}

#[component]
pub fn Inference() -> impl IntoView {
    let (temperature, set_temperature) = create_signal(4.2);
    let temperature_string = Signal::derive(move || format!("{:.1}", temperature.get()));

    let (top_p, set_top_p) = create_signal(4.2);
    let top_p_string = Signal::derive(move || format!("{:.1}", top_p.get()));

    let (seed, set_seed) = create_signal(4.2);
    let seed_string = Signal::derive(move || format!("{:.1}", seed.get()));

    let (sample_len, set_sample_len) = create_signal(4.2);
    let sample_len_string = Signal::derive(move || format!("{:.1}", sample_len.get()));

    let (repeat_penalty, set_repeat_penalty) = create_signal(4.2);
    let repeat_penalty_string = Signal::derive(move || format!("{:.1}", repeat_penalty.get()));

    let (repeat_last_n, set_repeat_last_n) = create_signal(4.2);
    let repeat_last_n_string = Signal::derive(move || format!("{:.1}", repeat_last_n.get()));

    view! {
        <Box class="api-box">
            <NumberInput min=0.0 max=10.0 step=0.1
                get=temperature
                set=set_temperature
            />
            <P class="under-input">"Temperature is: " {move || temperature_string.get()}</P>

            <NumberInput min=0.0 max=10.0 step=0.1
                get=top_p
                set=set_top_p
            />
            <P class="under-input">"Top_p is: " {move || top_p_string.get()}</P>

            <NumberInput min=0.0 max=10.0 step=0.1
                get=seed
                set=set_seed
            />
            <P class="under-input">"Seed is: " {move || seed_string.get()}</P>

            <NumberInput min=0.0 max=10.0 step=0.1
                get=sample_len
                set=set_sample_len
            />
            <P class="under-input">"Sample_len is: " {move || sample_len_string.get()}</P>

            <NumberInput min=0.0 max=10.0 step=0.1
                get=repeat_penalty
                set=set_repeat_penalty
            />
            <P class="under-input">"Repeat_penalty is: " {move || repeat_penalty_string.get()}</P>

            <NumberInput min=0.0 max=10.0 step=0.1
                get=repeat_last_n
                set=set_repeat_last_n
            />
            <P class="under-input">"Repeat_last_n is: " {move || repeat_last_n_string.get()}</P>

            <Button variant=ButtonVariant::Flat size=ButtonSize::Small on_click=move |_| todo!()>"Submit"</Button>
        </Box>
    }
}
