use leptonic::components::select::Select;
use leptos::*;

#[component]
pub fn GpuSelect(gpu_type: Signal<String>, set_gpu_type: WriteSignal<String>) -> impl IntoView {
    let gpu_for_select = vec![
        "None".to_string(),
        "CUDA".to_string(),
        "Mac".to_string(),
        "Intel".to_string(),
        "AMD".to_string(),
    ];
    view! {
        <span>"Use GPU? "</span>
        <Select
            options=gpu_for_select.clone()
            search_text_provider=move |o: String| format!("{:?}", o)
            render_option=move |o: String| {
                if o == *"Mac" || o == *"CUDA" {
                    format!("{} (gpu: ✅)", o)
                } else {
                    format!("{} (gpu: ❌)", o)
                }
            }

            selected=gpu_type
            set_selected=move |v| set_gpu_type.set(v)
        />
    }
}
