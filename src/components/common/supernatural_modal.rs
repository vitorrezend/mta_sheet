use leptos::*;
use crate::state::DotOrigin;
use super::callback::Callback;
use super::dot_origin_popover::DotOriginPopover;
use super::value_field::TraitField;

/// Painel da escala sobrenatural (níveis 6 a 10) de um campo de característica.
#[component]
pub fn SupernaturalModal(
    field: TraitField,
    lang: Signal<crate::i18n::Language>,
    on_close: Callback<()>,
) -> impl IntoView {
    let TraitField {
        label,
        level,
        origins,
        modifier,
        suggested_specialties,
        page_ref_str,
        modifier_ref,
        last_modifier_value,
        popover,
        on_level_change,
        on_origin_change,
        on_modifier_change: on_modifier_input,
        on_open_compendium,
    } = field;
    let (open_popover_idx, set_open_popover_idx) = (popover.read_only(), popover.write_only());
    let is_sup_modifier_focused = create_rw_signal(false);
    let last_sup_modifier_value = create_rw_signal(modifier.get_untracked());

    let on_modifier_input_rc = on_modifier_input.clone();
    let save_pending_modifier = std::rc::Rc::new({
        let on_mod = on_modifier_input_rc.clone();
        move || {
            let _ = is_sup_modifier_focused.try_set(false);
            let val = last_sup_modifier_value.get_untracked();
            on_mod.call(val);
        }
    });

    let on_click_level_sup = on_level_change.clone();
    let on_dot_change_for_right_click = on_origin_change.clone();
    let on_dot_change_for_popover = on_origin_change.clone();

    view! {
        <div class="supernatural-modal-wrapper" on:click=move |ev| ev.stop_propagation()>
            <div 
                class="supernatural-backdrop"
                on:click={
                    let save_mod = save_pending_modifier.clone();
                    let on_close_cb = on_close.clone();
                    move |ev| {
                        ev.stop_propagation();
                        save_mod();
                        set_open_popover_idx.set(None);
                        on_close_cb.call(());
                    }
                }
            />
            <div class="supernatural-floating-panel">
                <div class="supernatural-panel-header">
                    <div class="panel-header-title">
                        <span class="panel-sparkle">"✦"</span>
                        <span class="panel-trait-name" title=move || label.get()>{move || label.get()}</span>
                    </div>
                    <div class="panel-header-actions">
                        <span class="panel-level-pill">
                            {move || {
                                let cur = level.get();
                                let extra = (cur - 5).max(0);
                                format!("Nível {} (+{})", cur, extra)
                            }}
                        </span>
                        <button 
                            type="button" 
                            class="panel-close-btn"
                            on:click={
                                let save_mod = save_pending_modifier.clone();
                                let on_close_cb = on_close.clone();
                                move |_| {
                                    save_mod();
                                    set_open_popover_idx.set(None);
                                    on_close_cb.call(());
                                }
                            }
                            title="Fechar"
                        >
                            "✕"
                        </button>
                    </div>
                </div>

                // Campo de escrita curto (Especialização / Modificador / Nota)
                <div class="supernatural-modifier-section">
                    <span class="modifier-section-label">"Especialização / Modificador:"</span>
                    <input 
                        type="text" 
                        class="supernatural-modifier-input"
                        placeholder="Ex: Musculoso, Titânico..."
                        maxlength="35"
                        prop:value=move || {
                            if is_sup_modifier_focused.get() {
                                last_sup_modifier_value.get()
                            } else {
                                modifier.get()
                            }
                        }
                        on:focus={
                            let val = modifier.get_untracked();
                            move |_| { 
                                let _ = is_sup_modifier_focused.try_set(true); 
                                let _ = last_sup_modifier_value.try_set(val.clone());
                            }
                        }
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            let _ = last_sup_modifier_value.try_set(val);
                        }
                        on:blur={
                            let on_mod = on_modifier_input.clone();
                            move |_| {
                                let _ = is_sup_modifier_focused.try_set(false);
                                let val = last_sup_modifier_value.get_untracked();
                                on_mod.call(val);
                            }
                        }
                        on:keydown={
                            let on_mod = on_modifier_input.clone();
                            move |ev: ev::KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    let _ = is_sup_modifier_focused.try_set(false);
                                    let val = last_sup_modifier_value.get_untracked();
                                    on_mod.call(val);
                                    if let Some(target) = ev.target() {
                                        use wasm_bindgen::JsCast;
                                        if let Ok(elem) = target.dyn_into::<web_sys::HtmlElement>() {
                                            let _ = elem.blur();
                                        }
                                    }
                                }
                            }
                        }
                    />
                    {if let Some(specs_signal) = suggested_specialties {
                        let on_modifier_input_sup_spec = on_modifier_input.clone();
                        let on_open_comp_sup = on_open_compendium.clone();
                        let save_pending_modifier_comp = save_pending_modifier.clone();
                        view! {
                            <div class="supernatural-specialties-block">
                                <div class="specialty-pills-list">
                                    {
                                        let on_modifier_input_sup_spec = on_modifier_input_sup_spec.clone();
                                        move || {
                                            let specs = specs_signal.get();
                                            let current_mod = if is_sup_modifier_focused.get() {
                                                last_sup_modifier_value.get()
                                            } else {
                                                modifier.get()
                                            };
                                            let current_parts: Vec<&str> = current_mod
                                                .split(',')
                                                .map(|s| s.trim())
                                                .filter(|s| !s.is_empty())
                                                .collect();

                                            specs.into_iter().map(|spec| {
                                                let is_selected = current_parts.iter().any(|part| part.eq_ignore_ascii_case(spec));
                                                let on_mod = on_modifier_input_sup_spec.clone();
                                                view! {
                                                    <button
                                                        type="button"
                                                        class="specialty-pill-btn"
                                                        class:active=is_selected
                                                        on:click={
                                                            let on_mod = on_mod.clone();
                                                            move |ev| {
                                                                ev.stop_propagation();
                                                                let cur = if is_sup_modifier_focused.get_untracked() {
                                                                    last_sup_modifier_value.get_untracked()
                                                                } else {
                                                                    modifier.get_untracked()
                                                                };
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
                                                                let _ = last_sup_modifier_value.try_set(new_val.clone());
                                                                let _ = is_sup_modifier_focused.try_set(false);
                                                                if let Some(elem) = modifier_ref.get() {
                                                                    elem.set_value(&new_val);
                                                                }
                                                                let _ = last_modifier_value.try_set(new_val.clone());
                                                                on_mod.call(new_val);
                                                            }
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

                                <div class="supernatural-specialties-actions">
                                    {
                                        let on_mod_clear = on_modifier_input_sup_spec.clone();
                                        view! {
                                            <button
                                                type="button"
                                                class="specialty-clear-btn"
                                                class:hidden=move || modifier.get().trim().is_empty()
                                                on:click={
                                                    let on_mod = on_mod_clear.clone();
                                                    move |ev| {
                                                        ev.stop_propagation();
                                                        let _ = last_sup_modifier_value.try_set(String::new());
                                                        let _ = is_sup_modifier_focused.try_set(false);
                                                        if let Some(elem) = modifier_ref.get() {
                                                            elem.set_value("");
                                                        }
                                                        let _ = last_modifier_value.try_set(String::new());
                                                        on_mod.call(String::new());
                                                    }
                                                }
                                                title="Limpar especialidades"
                                            >
                                                "✕ " {move || match lang.get() {
                                                    crate::i18n::Language::PtBr => "Limpar",
                                                    crate::i18n::Language::EnUs => "Clear",
                                                }}
                                            </button>
                                        }
                                    }

                                    {
                                        if let Some(on_open_comp) = on_open_comp_sup.clone() {
                                            let save_mod = save_pending_modifier_comp.clone();
                                            let on_close_cb = on_close.clone();
                                            view! {
                                                <button
                                                    type="button"
                                                    class="specialty-compendium-btn"
                                                    on:click={
                                                        let save_mod = save_mod.clone();
                                                        let on_open_comp = on_open_comp.clone();
                                                        let on_close_cb = on_close_cb.clone();
                                                        move |ev| {
                                                            ev.stop_propagation();
                                                            save_mod();
                                                            set_open_popover_idx.set(None);
                                                            on_close_cb.call(());
                                                            on_open_comp.call(());
                                                        }
                                                    }
                                                    title=move || format!("Consultar {} no Compêndio M20 ({})", label.get(), page_ref_str.get())
                                                >
                                                    <span class="compendium-btn-icon">"📖"</span>
                                                    <span class="compendium-btn-text">
                                                        {move || match lang.get() {
                                                            crate::i18n::Language::PtBr => "Compêndio M20",
                                                            crate::i18n::Language::EnUs => "M20 Compendium",
                                                        }}
                                                    </span>
                                                    <span class="compendium-btn-page">{page_ref_str}</span>
                                                </button>
                                            }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }
                                    }
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>

                <div class="supernatural-slots-track">
                    {(1..=5).map(|extra| {
                        let lvl = 5 + extra;
                        let dot_idx = (lvl - 1) as usize;
                        let is_filled = move || level.get() >= lvl;
                        let dot_color = move || {
                            if level.get() >= lvl {
                                if let Some(sig) = origins {
                                    let list = sig.get();
                                    if dot_idx < list.len() {
                                        list[dot_idx].color_class()
                                    } else {
                                        "dot-base"
                                    }
                                } else {
                                    "dot-base"
                                }
                            } else {
                                ""
                            }
                        };

                        let is_popover_open = move || open_popover_idx.get() == Some(dot_idx);

                        let on_dot_change_for_right_click = on_dot_change_for_right_click.clone();
                        let on_right_click = move |ev: ev::MouseEvent| {
                            ev.prevent_default();
                            ev.stop_propagation();
                            if level.get() >= lvl && on_dot_change_for_right_click.is_some() {
                                set_open_popover_idx.update(|cur| {
                                    *cur = if *cur == Some(dot_idx) { None } else { Some(dot_idx) };
                                });
                            }
                        };

                        let on_dot_change_for_popover = on_dot_change_for_popover.clone();
                        let set_origin_to = Callback::new(move |origin: DotOrigin| {
                            if let Some(cb) = on_dot_change_for_popover.as_ref() {
                                cb.call((dot_idx, origin));
                            }
                            set_open_popover_idx.set(None);
                        });

                        let on_click_level_sup = on_click_level_sup.clone();
                        let save_pending_modifier = save_pending_modifier.clone();
                        let on_close_cb = on_close.clone();

                        view! {
                            <div class="supernatural-slot-column">
                                <span class="slot-extra-badge">{format!("+{}", extra)}</span>
                                <div class="dot-wrapper">
                                    <span 
                                        class="dot dot-supernatural"
                                        class:filled=is_filled
                                        class=("dot-base", move || is_filled() && dot_color() == "dot-base")
                                        class=("dot-bonus", move || is_filled() && dot_color() == "dot-bonus")
                                        class=("dot-xp", move || is_filled() && dot_color() == "dot-xp")
                                        class=("dot-temp", move || is_filled() && dot_color() == "dot-temp")
                                        on:click:undelegated={
                                            let on_click_sup = on_click_level_sup.clone();
                                            let save_mod = save_pending_modifier.clone();
                                            let on_close_cb = on_close_cb.clone();
                                            move |_| {
                                                save_mod();
                                                set_open_popover_idx.set(None);
                                                let current = level.get();
                                                let new_val = if current == lvl {
                                                    lvl - 1
                                                } else {
                                                    lvl
                                                };
                                                on_click_sup.call(new_val);
                                                on_close_cb.call(());
                                            }
                                        }
                                        on:contextmenu=on_right_click
                                        title=format!("Nível {} (+{} Sobrenatural) | Clique esquerdo: definir nível | Clique direito: mudar origem", lvl, extra)
                                    ></span>

                                    <DotOriginPopover 
                                        lvl=lvl 
                                        is_open=Signal::derive(is_popover_open) 
                                        on_select=set_origin_to 
                                    />
                                </div>
                                <span class="slot-lvl-number">{lvl}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>

                <div class="supernatural-panel-footer">
                    <button 
                        type="button" 
                        class="panel-reset-btn"
                        on:click={
                            let on_reset = on_click_level_sup.clone();
                            let save_mod = save_pending_modifier.clone();
                            let on_close_cb = on_close.clone();
                            move |_| {
                                save_mod();
                                on_reset.call(5);
                                set_open_popover_idx.set(None);
                                on_close_cb.call(());
                            }
                        }
                        title="Reduzir para o limite mortal (5 pontos)"
                    >
                        "↺ Resetar (5)"
                    </button>
                    <button 
                        type="button" 
                        class="panel-done-btn"
                        on:click={
                            let save_mod = save_pending_modifier.clone();
                            let on_close_cb = on_close.clone();
                            move |_| {
                                save_mod();
                                set_open_popover_idx.set(None);
                                on_close_cb.call(());
                            }
                        }
                    >
                        "Concluir"
                    </button>
                </div>
            </div>
        </div>
    }
}
