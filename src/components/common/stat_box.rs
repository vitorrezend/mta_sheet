use leptos::*;

/// Componente canônico de Caixa de Estatística / Atributo Tático
#[component]
pub fn StatBox(
    #[prop(into)] label: MaybeSignal<String>,
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(into, default = "".into())] sub: MaybeSignal<String>,
    #[prop(into, default = "")] color_class: &'static str,
    #[prop(default = false)] is_damage: bool,
    #[prop(into, default = "".into())] extra_class: String,
) -> impl IntoView {
    let base_box_class = if is_damage {
        format!("stat-box stat-box-damage {}", extra_class)
    } else {
        format!("stat-box {}", extra_class)
    };

    let value_for_class = value.clone();
    let value_for_text = value.clone();
    let value_for_title = value;

    view! {
        <div 
            class=base_box_class
            title=move || value_for_title.get()
        >
            <span class="stat-label">
                {move || label.get()}
            </span>
            <span 
                class=move || {
                    let v = value_for_class.get();
                    let len = v.chars().count();
                    let density = if len > 12 {
                        "stat-value-long"
                    } else if len > 6 {
                        "stat-value-dense"
                    } else {
                        ""
                    };
                    let wrap_cls = if v.contains(' ') || len > 7 { "stat-value-wrap" } else { "" };
                    format!("stat-value {} {} {} {}", color_class, density, wrap_cls, extra_class).trim().to_string()
                }
            >
                {move || value_for_text.get()}
            </span>
            {move || {
                let s = sub.get();
                if !s.is_empty() {
                    view! {
                        <span class="stat-sub">
                            {s}
                        </span>
                    }.into_view()
                } else {
                    view! { <span></span> }.into_view()
                }
            }}
        </div>
    }
}
