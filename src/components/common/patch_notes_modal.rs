use leptos::*;
use super::patch_notes_data::{PATCH_RELEASES, CURRENT_VERSION};
use super::callback::SafeCallback;

#[component]
pub fn PatchNotesModal(
    #[prop(into)] is_open: Signal<bool>,
    on_close: SafeCallback<()>,
) -> impl IntoView {
    let (selected_idx, set_selected_idx) = create_signal(0usize);

    // Fecha o modal ao pressionar ESC de forma 100% segura contra descarte de escopo
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

    view! {
        {
            let on_close_backdrop = on_close_backdrop.clone();
            let on_close = on_close.clone();
            move || if is_open.get() {
                let selected_release = PATCH_RELEASES.get(selected_idx.get()).unwrap_or(&PATCH_RELEASES[0]);
                let on_close_btn = on_close.clone();
                let on_close_bg = on_close_backdrop.clone();

                view! {
                    <div class="patch-modal-overlay" on:click=move |ev: web_sys::MouseEvent| {
                        if let Some(target) = ev.target() {
                            use wasm_bindgen::JsCast;
                            if let Ok(elem) = target.dyn_into::<web_sys::HtmlElement>() {
                                if elem.class_list().contains("patch-modal-overlay") {
                                    on_close_bg.call(());
                                }
                            }
                        }
                    }>
                    <div class="patch-modal-card" on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()>
                        // Cabeçalho do Modal
                        <div class="patch-modal-header">
                            <div class="patch-modal-title-wrap">
                                <span class="patch-modal-icon">"📜"</span>
                                <div>
                                    <h2>"Notas de Atualização & Versões"</h2>
                                    <p>"Acompanhe as novidades, melhorias e correções do MTA Sheet"</p>
                                </div>
                            </div>
                            <button
                                type="button"
                                class="patch-modal-close"
                                on:click=move |_| on_close_btn.call(())
                                title="Fechar (Esc)"
                            >
                                "×"
                            </button>
                        </div>

                        // Seletor de Versões / Tabs
                        <div class="patch-version-selector">
                            {PATCH_RELEASES.iter().enumerate().map(|(idx, rel)| {
                                let is_active = move || selected_idx.get() == idx;
                                let is_current_pkg = rel.version.trim_start_matches('v') == CURRENT_VERSION;

                                view! {
                                    <button
                                        type="button"
                                        class="patch-version-tab"
                                        class:active=is_active
                                        on:click=move |_| set_selected_idx.set(idx)
                                    >
                                        <span>{rel.version}</span>
                                        {if is_current_pkg {
                                            view! { <span class="patch-badge-current">"Atual"</span> }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}
                                    </button>
                                }
                            }).collect_view()}
                        </div>

                        // Conteúdo da Versão Selecionada
                        <div class="patch-modal-body">
                            <div class="patch-release-hero">
                                <div class="patch-release-meta">
                                    <h3>{selected_release.title}</h3>
                                    <span class="patch-release-date">"Lançado em " {selected_release.date}</span>
                                </div>
                                <p class="patch-release-highlight">{selected_release.highlight}</p>
                            </div>

                            <div class="patch-sections-list">
                                {selected_release.sections.iter().map(|sec| {
                                    view! {
                                        <div class="patch-category-card">
                                            <div class="patch-category-header">
                                                <span>{sec.icon}</span>
                                                <span>{sec.category}</span>
                                            </div>
                                            <ul class="patch-items-list">
                                                {sec.items.iter().map(|item| {
                                                    view! {
                                                        <li class="patch-item-entry">
                                                            <span class="patch-item-bullet">"•"</span>
                                                            <span>{*item}</span>
                                                        </li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    </div>
                </div>
            }.into_view()
        } else {
            view! {}.into_view()
        }}
    }
}
