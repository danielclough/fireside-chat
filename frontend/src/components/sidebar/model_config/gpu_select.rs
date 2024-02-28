use leptonic::select::Select;
use leptos::*;

#[component]
pub fn GpuSelect(
    gpu_type: Signal<String>,
    set_gpu_type: WriteSignal<String>,
    set_repo_id_signal: WriteSignal<String>,
) -> impl IntoView {
    let gpu_for_select = vec!["None".to_string(), "CUDA".to_string(), "Mac".to_string(), "Intel".to_string(), "AMD".to_string()];
    let init_type = gpu_type.get();
    view! {
        <span>"Use GPU? "</span>
        <Select
            options=gpu_for_select.clone()
            search_text_provider=move |o: String| format!("{:?}", o)
            render_option=move |o: String| {
                if o == "Mac".to_string() || o == "CUDA".to_string() {
                    if o != init_type {
                        set_repo_id_signal.set("NoModel".to_string())
                    }
                    format!("{} (gpu: ✅)", o)
                } else {
                    if o != init_type {
                        set_repo_id_signal.set("NoModel".to_string())
                    }
                    format!("{} (gpu: ❌)", o)
                }
            }

            selected=gpu_type
            set_selected=move |v| set_gpu_type.set(v)
        />
    }
}
