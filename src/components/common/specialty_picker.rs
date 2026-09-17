use leptos::*;
use leptos::html;
use crate::components::Callback;
use crate::i18n::Language;

#[component]
pub fn SpecialtyPicker(
    #[prop(into)] level: Signal<i32>,
    #[prop(into)] modifier: Signal<String>,
    #[prop(into)] label: Signal<String>,
    #[prop(into)] page_ref_str: Signal<String>,
    #[prop(into)] suggested_specialties: Signal<Vec<&'static str>>,
    on_change: Callback<String>,
    #[prop(into, default = None)] on_open_compendium: Option<Callback<()>>,
    #[prop(into, default = None)] modifier_ref: Option<NodeRef<html::Input>>,
) -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let (show_specialty_popover, set_show_specialty_popover) = create_signal(false);

    let on_toggle_click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        set_show_specialty_popover.update(|v| *v = !*v);
    };

    let on_open_comp_cb = on_open_compendium;
    let on_change_cb = on_change;

    view! {
        <div class="specialty-picker-inline">
            <button
                type="button"
                class="specialty-toggle-btn"
                class:has-specialty-active=move || (level.get() >= 4)
                class:popover-open=move || show_specialty_popover.get()
                on:click=on_toggle_click
                title=move || {
                    if level.get() >= 4 {
                        match lang() {
                            Language::PtBr => "Especialidade Ativa (4+ pontos). Clique para escolher especialidades.",
                            Language::EnUs => "Specialty Active (4+ dots). Click to choose specialties.",
                        }
                    } else {
                        match lang() {
                            Language::PtBr => "Especialidades sugeridas (M20, p. 273). Clique para escolher.",
                            Language::EnUs => "Suggested specialties (M20, p. 273). Click to choose.",
                        }
                    }
                }
            >
                <span class="specialty-chevron">"▾"</span>
            </button>

            <div class="specialty-popover-wrapper" class:hidden=move || !show_specialty_popover.get()>
                <div 
                    class="specialty-popover-backdrop"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        set_show_specialty_popover.set(false);
                    }
                />
                <div 
                    class="specialty-selector-popover"
                    on:click=move |ev| ev.stop_propagation()
                >
                    <div class="specialty-popover-header">
                        <div class="panel-header-title">
                            <span class="panel-sparkle">"✦"</span>
                            <span class="panel-trait-name" title=move || label.get()>{move || label.get()}</span>
                        </div>
                        <div class="panel-header-actions">
                            <span class="panel-level-pill">
                                {move || {
                                    let cur = level.get();
                                    if cur >= 4 {
                                        format!("★ Nível {}", cur)
                                    } else {
                                        format!("Nível {}", cur)
                                    }
                                }}
                            </span>
                            <button
                                type="button"
                                class="panel-close-btn"
                                on:click=move |_| set_show_specialty_popover.set(false)
                                title="Fechar"
                            >
                                "✕"
                            </button>
                        </div>
                    </div>

                    <div class="specialty-popover-rule">
                        {move || if level.get() >= 4 {
                            view! {
                                <div class="specialty-unlocked-badge">
                                    <span class="badge-icon">"★"</span>
                                    {match lang() {
                                        Language::PtBr => "4+ Pontos: Especialidade Ativa! (10 = 2 sucessos)",
                                        Language::EnUs => "4+ Dots: Specialty Active! (10 = 2 successes)",
                                    }}
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="specialty-locked-hint">
                                    {match lang() {
                                        Language::PtBr => format!("• Requer 4+ pontos para ativar em rolagens ({})", page_ref_str.get()),
                                        Language::EnUs => format!("• Requires 4+ dots to activate in rolls ({})", page_ref_str.get()),
                                    }}
                                </div>
                            }.into_view()
                        }}
                    </div>

                    <div class="specialty-pills-list">
                        {
                            let on_change_cb = on_change_cb.clone();
                            move || {
                                let specs = suggested_specialties.get();
                                let current_mod = modifier.get();
                                let current_parts: Vec<&str> = current_mod
                                    .split(',')
                                    .map(|s| s.trim())
                                    .filter(|s| !s.is_empty())
                                    .collect();

                                let on_change_iter = on_change_cb.clone();
                                specs.into_iter().map(|spec| {
                                    let is_selected = current_parts.iter().any(|part| part.eq_ignore_ascii_case(spec));
                                    let on_change_spec = on_change_iter.clone();
                                    
                                    view! {
                                        <button
                                            type="button"
                                            class="specialty-pill-btn"
                                            class:active=is_selected
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                let cur = modifier.get();
                                                let mut parts: Vec<String> = cur
                                                    .split(',')
                                                    .map(|s| s.trim().to_string())
                                                    .filter(|s| !s.is_empty())
                                                    .collect();

                                                if let Some(pos) = parts.iter().position(|p| p.eq_ignore_ascii_case(spec)) {
                                                    parts.remove(pos);
                                                } else {
                                                    parts.push(spec.to_string());
                                                }

                                                let new_val = parts.join(", ");
                                                if let Some(m_ref) = modifier_ref {
                                                    if let Some(elem) = m_ref.get() {
                                                        elem.set_value(&new_val);
                                                    }
                                                }
                                                on_change_spec.call(new_val);
                                            }
                                            title=if is_selected {
                                                "Clique para desmarcar"
                                            } else {
                                                "Clique para selecionar"
                                            }
                                        >
                                            {if is_selected { "✓ " } else { "+ " }}
                                            {spec}
                                        </button>
                                    }
                                }).collect_view()
                            }
                        }
                    </div>

                    <div class="specialty-popover-footer">
                        {
                            let on_change_clear = on_change_cb;
                            view! {
                                <button
                                    type="button"
                                    class="specialty-clear-btn"
                                    class:hidden=move || modifier.get().trim().is_empty()
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        if let Some(m_ref) = modifier_ref {
                                            if let Some(elem) = m_ref.get() {
                                                elem.set_value("");
                                            }
                                        }
                                        on_change_clear.call(String::new());
                                    }
                                    title="Limpar especialidades"
                                >
                                    "✕ " {move || match lang() {
                                        Language::PtBr => "Limpar",
                                        Language::EnUs => "Clear",
                                    }}
                                </button>
                            }
                        }

                        {
                            if let Some(on_open_comp) = on_open_comp_cb {
                                view! {
                                    <button
                                        type="button"
                                        class="specialty-compendium-btn"
                                        on:click=move |ev| {
                                            ev.stop_propagation();
                                            set_show_specialty_popover.set(false);
                                            on_open_comp.call(());
                                        }
                                        title=move || format!("Consultar {} no Compêndio M20 ({})", label.get(), page_ref_str.get())
                                    >
                                        <span class="compendium-btn-icon">"📖"</span>
                                        <span class="compendium-btn-text">
                                            {move || match lang() {
                                                Language::PtBr => "Compêndio M20",
                                                Language::EnUs => "M20 Compendium",
                                            }}
                                        </span>
                                        <span class="compendium-btn-page">{move || page_ref_str.get()}</span>
                                    </button>
                                }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
