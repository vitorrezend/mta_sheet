use leptos::*;
use super::callback::SafeCallback;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Full,
}

impl ModalSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            ModalSize::Sm => "mta-modal-sm",
            ModalSize::Md => "mta-modal-md",
            ModalSize::Lg => "mta-modal-lg",
            ModalSize::Xl => "mta-modal-xl",
            ModalSize::Full => "mta-modal-full",
        }
    }
}

/// Componente canônico e reutilizável de Modal do Design System MTA Sheet
#[component]
pub fn Modal(
    #[prop(into)] is_open: Signal<bool>,
    on_close: SafeCallback<()>,
    #[prop(into, default = "".into())] title: MaybeSignal<String>,
    #[prop(into, default = None)] subtitle: Option<MaybeSignal<String>>,
    #[prop(default = None)] icon: Option<&'static str>,
    #[prop(default = ModalSize::Md)] size: ModalSize,
    #[prop(default = false)] hide_header: bool,
    #[prop(into, default = "".into())] extra_class: String,
    children: ChildrenFn,
) -> impl IntoView {
    // Fecha o modal ao pressionar ESC
    #[cfg(target_arch = "wasm32")]
    {
        let on_close_esc = on_close.clone();
        let handle = window_event_listener(ev::keydown, move |ev: web_sys::KeyboardEvent| {
            if is_open.try_get_untracked().unwrap_or(false) && ev.key() == "Escape" {
                on_close_esc.call(());
            }
        });
        on_cleanup(move || handle.remove());
    }

    let on_close_backdrop = on_close.clone();
    let size_class = size.as_class();

    view! {
        {
            let on_close_backdrop = on_close_backdrop.clone();
            let on_close = on_close.clone();
            let extra_class = extra_class.clone();
            let children = children.clone();
            let title = title.clone();
            let subtitle = subtitle.clone();
            move || if is_open.get() {
                let on_close_btn = on_close.clone();
                let on_close_bg = on_close_backdrop.clone();
                let card_classes = format!("mta-modal-card {} {}", size_class, extra_class);

                view! {
                    <div 
                        class="mta-modal-overlay" 
                        on:click=move |ev: web_sys::MouseEvent| {
                            if let Some(target) = ev.target() {
                                use wasm_bindgen::JsCast;
                                if let Ok(elem) = target.dyn_into::<web_sys::HtmlElement>() {
                                    if elem.class_list().contains("mta-modal-overlay") {
                                        on_close_bg.call(());
                                    }
                                }
                            }
                        }
                    >
                        <div class=card_classes on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()>
                            {if !hide_header {
                                view! {
                                    <div class="mta-modal-header">
                                        <div class="mta-modal-title-wrap">
                                            {if let Some(ic) = icon {
                                                view! { <span class="mta-modal-icon">{ic}</span> }.into_view()
                                            } else {
                                                view! { <span></span> }.into_view()
                                            }}
                                            <div>
                                                <h3 class="mta-modal-title">{title.get()}</h3>
                                                {if let Some(sub) = &subtitle {
                                                    view! { <p class="mta-modal-subtitle">{sub.get()}</p> }.into_view()
                                                } else {
                                                    view! { <span></span> }.into_view()
                                                }}
                                            </div>
                                        </div>
                                        <button
                                            type="button"
                                            class="mta-modal-close-btn"
                                            on:click=move |_| on_close_btn.call(())
                                            title="Fechar (Esc)"
                                            aria-label="Fechar modal"
                                        >
                                            "×"
                                        </button>
                                    </div>
                                }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }}

                            <div class="mta-modal-body">
                                {children()}
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }
        }
    }
}
