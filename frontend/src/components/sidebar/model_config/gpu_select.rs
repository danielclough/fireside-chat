use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn GpuSelect(
    quantized: bool,
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
) -> impl IntoView {
    let gpu_for_select = vec!["None".to_string(), "CUDA".to_string(), "Mac".to_string(), "Intel".to_string(), "AMD".to_string()];
    view! {
        <span>"GPU Type: "</span>
        <Select
            options=gpu_for_select.clone()
            search_text_provider=move |o: String| format!("{:?}", o)
            render_option=move |o: String| {
                if o == "".to_string() || o == "None".to_string() {
                    "None (gpu: ❌)".to_string()
                } else if quantized == true && o == "Mac".to_string() {
                    format!("{} (gpu: ✅)", o)
                } else if o == "None".to_string() {
                    format!("{} (gpu: ❌)", o)
                } else if quantized == true && o != "Mac".to_string() {
                    format!("{} (gpu: ❌)", o)
                } else {
                    format!("{} (gpu: ✅)", o)
                }
            }

            selected=gpu_type
            set_selected=move |v| set_gpu_type.set(v)
        />
    }
}
