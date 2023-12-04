use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Model() -> impl IntoView {
    let (text, set_text) = create_signal("text".to_owned());
    let (number, set_number) = create_signal(4.2);
    let number_string = Signal::derive(move || format!("{:.1}", number.get()));

    view! {
        <Box class="api-box">
            <NumberInput min=0.0 max=10.0 step=0.1
                get=number
                set=set_number
            />
            <P class="under-input">"Number is: " {move || number_string.get()}</P>

            <TextInput
                placeholder="This is a placeholder"
                get=text set=set_text
                />
            <P class="under-input">"Text is: " {move || text.get()}</P>

            <Button variant=ButtonVariant::Flat size=ButtonSize::Small on_click=move |_| set_text.set(String::new())>"Submit"</Button>
        </Box>
    }
}
