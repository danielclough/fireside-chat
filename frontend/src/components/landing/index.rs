use leptonic::prelude::Box;
use leptos::{component, view, IntoView};
use web_sys::MouseEvent;

#[component]
pub fn Landing<F>(landing_view_toggle: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <Box class="landing">
            <button on:click=landing_view_toggle>"Chat!"</button>
        </Box>
    }
}
