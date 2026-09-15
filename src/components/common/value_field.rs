use leptos::*;
use std::rc::Rc;
use crate::state::DotOrigin;
use super::callback::Callback;

#[component]
pub fn ValueField(
    label: Signal<String>,
    level: Signal<i32>,
    modifier: Signal<String>,
    on_level_change: impl Fn(i32) + 'static,
    on_modifier_change: impl Fn(String) + 'static,
    #[prop(default = 0)] min_level: i32,
    #[prop(default = 5)] max_level: i32,
    #[prop(optional)] max_chars: Option<usize>,
    #[prop(optional)] on_remove: Option<Callback<()>>,
    #[prop(default = false)] is_editable: bool,
    #[prop(optional)] on_label_change: Option<Callback<String>>,
    #[prop(optional)] origins: Option<Signal<Vec<DotOrigin>>>,
    #[prop(optional)] on_dot_origin_change: Option<Callback<(usize, DotOrigin)>>,
    #[prop(optional)] is_starred: Option<Signal<bool>>,
    #[prop(optional)] on_toggle_star: Option<Callback<()>>,
    #[prop(optional)] star_tooltip: Option<&'static str>,
    #[prop(optional)] is_supernatural: Option<Signal<bool>>,
    #[prop(optional)] on_toggle_supernatural: Option<Callback<()>>,
) -> impl IntoView {
    let on_level_change = Rc::new(on_level_change);
    let on_modifier_change = Rc::new(on_modifier_change);
    let (open_popover_idx, set_open_popover_idx) = create_signal(Option::<usize>::None);
    let (show_context_menu, set_show_context_menu) = create_signal(false);
    let (show_supernatural_modal, set_show_supernatural_modal) = create_signal(false);

    let display_label = move || {
        let l = label.get();
        if let Some(max) = max_chars {
            if l.chars().count() > max {
                format!("{}...", l.chars().take(max).collect::<String>())
            } else {
                l
            }
        } else {
            l
        }
    };

    let tooltip_label = move || {
        let l = label.get();
        let cur_lvl = level.get();
        if cur_lvl > 0 {
            if let Some(origins_sig) = origins {
                let orig_list = origins_sig.get();
                let mut base = 0;
                let mut bonus = 0;
                let mut xp = 0;
                let mut temp = 0;
                for i in 0..(cur_lvl as usize) {
                    let orig = if i < orig_list.len() { orig_list[i] } else { DotOrigin::Base };
                    match orig {
                        DotOrigin::Base => base += 1,
                        DotOrigin::Bonus => bonus += 1,
                        DotOrigin::Experience => xp += 1,
                        DotOrigin::Temporary => temp += 1,
                    }
                }
                let mut parts = Vec::new();
                if base > 0 { parts.push(format!("{} Base", base)); }
                if bonus > 0 { parts.push(format!("{} Bônus", bonus)); }
                if xp > 0 { parts.push(format!("{} XP", xp)); }
                if temp > 0 { parts.push(format!("{} Buff", temp)); }

                if !parts.is_empty() {
                    return format!("{} (Nível {}: {})", l, cur_lvl, parts.join(" • "));
                }
            }
            format!("{} (Nível {})", l, cur_lvl)
        } else {
            l
        }
    };

    let on_level_change_click = on_level_change.clone();
    let on_modifier_input = on_modifier_change.clone();

    // Focus-Lock para o campo de modificador (evita perda de cursor ao digitar)
    let modifier_ref = create_node_ref::<html::Input>();
    let is_modifier_focused = create_rw_signal(false);
    let last_modifier_value = create_rw_signal(String::new());

    create_effect(move |_| {
        let val = modifier.get();
        if !is_modifier_focused.get_untracked() {
            if let Some(elem) = modifier_ref.get() {
                elem.set_value(&val);
            }
            let _ = last_modifier_value.try_set(val);
        }
    });

    // Focus-Lock para o campo de modificador dentro do painel sobrenatural
    let is_sup_modifier_focused = create_rw_signal(false);
    let last_sup_modifier_value = create_rw_signal(String::new());

    create_effect(move |_| {
        let val = modifier.get();
        let is_open = show_supernatural_modal.get();
        if is_open && !is_sup_modifier_focused.get_untracked() {
            let _ = last_sup_modifier_value.try_set(val);
        }
    });

    // Focus-Lock para o campo de nome editável (evita perda de cursor ao digitar)
    let label_ref = create_node_ref::<html::Input>();
    let is_label_focused = create_rw_signal(false);
    let last_label_value = create_rw_signal(String::new());

    if is_editable {
        create_effect(move |_| {
            let val = label.get();
            if !is_label_focused.get_untracked() {
                if let Some(elem) = label_ref.get() {
                    elem.set_value(&val);
                }
                let _ = last_label_value.try_set(val);
            }
        });
    }

    let has_supernatural_toggle = on_toggle_supernatural.is_some();
    let has_supernatural_support = has_supernatural_toggle || is_supernatural.is_some();
    let on_toggle_for_menu = on_toggle_supernatural.clone();

    let on_label_right_click = {
        move |ev: ev::MouseEvent| {
            if has_supernatural_toggle {
                ev.prevent_default();
                ev.stop_propagation();
                set_show_context_menu.update(|v| *v = !*v);
            }
        }
    };

    view! {
        <div class="attribute-row">
            <div 
                class="tooltip-container label-cell-container"
                on:contextmenu=on_label_right_click
                title=move || if has_supernatural_toggle {
                    "Botão direito no nome: Alternar Característica Sobrenatural (6º Ponto)"
                } else {
                    ""
                }
            >
                {on_toggle_star.map(|on_star| {
                    let is_active = move || is_starred.map(|s| s.get()).unwrap_or(false);
                    view! {
                        <button 
                            type="button" 
                            class="affinity-star-btn"
                            class:active=is_active
                            on:click=move |ev| {
                                ev.stop_propagation();
                                on_star.call(());
                            }
                            title=move || if is_active() {
                                star_tooltip.unwrap_or("Esfera de Afinidade ativa (Custo XP: Atual × 7). Clique para desmarcar.")
                            } else {
                                "Clique para definir como Esfera de Afinidade (Custo XP: Atual × 7)"
                            }
                        >
                            <span class="affinity-star-icon" class:active=is_active>
                                {move || if is_active() { "★" } else { "☆" }}
                            </span>
                        </button>
                    }
                })}
                {
                    let on_label_change = on_label_change.clone();
                    if is_editable {
                        let on_label_blur = on_label_change.clone();
                        view! {
                            <input 
                                type="text" 
                                node_ref=label_ref
                                class="label-edit-input"
                                placeholder="Nome..."
                                maxlength="32"
                                on:focus=move |_| { let _ = is_label_focused.try_set(true); }
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    let _ = last_label_value.try_set(val);
                                }
                                on:blur=move |_| {
                                    let _ = is_label_focused.try_set(false);
                                    if let Some(elem) = label_ref.get() {
                                        let val = elem.value();
                                        if let Some(cb) = on_label_blur.as_ref() {
                                            cb.call(val.clone());
                                        }
                                        let _ = last_label_value.try_set(val);
                                    }
                                }
                            />
                        }.into_view()
                    } else {
                        view! {
                            <span 
                                class="attribute-label"
                                class:affinity-active=move || is_starred.map(|s| s.get()).unwrap_or(false)
                            >
                                {display_label}
                            </span>
                        }.into_view()
                    }
                }
                <span class="tooltip-text">{tooltip_label}</span>

                {
                    if let Some(on_toggle_cb) = on_toggle_for_menu.as_ref() {
                        let is_sup = move || is_supernatural.map(|s| s.get()).unwrap_or(false) || level.get() >= 6;
                        view! {
                            <div class="context-menu-wrapper" class:hidden=move || !show_context_menu.get()>
                                <div 
                                    class="context-menu-backdrop"
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        set_show_context_menu.set(false);
                                    }
                                    on:contextmenu=move |ev| {
                                        ev.prevent_default();
                                        ev.stop_propagation();
                                        set_show_context_menu.set(false);
                                    }
                                />
                                <div 
                                    class="label-context-menu" 
                                    on:click=move |ev| ev.stop_propagation()
                                >
                                    <button 
                                        type="button" 
                                        class="label-context-item"
                                        on:click=move |_| {
                                            set_show_context_menu.set(false);
                                            set_show_supernatural_modal.set(true);
                                        }
                                    >
                                        <span class="context-item-icon">"✦"</span>
                                        "Escala Sobrenatural (6 a 10)"
                                    </button>
                                    {
                                        let on_click_level_reset = on_level_change_click.clone();
                                        let on_toggle_reset = on_toggle_cb.clone();
                                        move || if is_sup() {
                                            let on_click_level_reset = on_click_level_reset.clone();
                                            let on_toggle_reset = on_toggle_reset.clone();
                                            view! {
                                                <button 
                                                    type="button" 
                                                    class="label-context-item"
                                                    on:click=move |_| {
                                                        set_show_context_menu.set(false);
                                                        on_click_level_reset(5);
                                                        on_toggle_reset.call(());
                                                    }
                                                >
                                                    <span class="context-item-icon">"↺"</span>
                                                    "Resetar para Nível 5 (Mortal)"
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
                    }
                }
            </div>

            <div class="modifier-container tooltip-container">
                <input 
                    type="text" 
                    node_ref=modifier_ref
                    class="field-modifier" 
                    placeholder="..."
                    maxlength="30"
                    on:focus=move |_| { let _ = is_modifier_focused.try_set(true); }
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        let _ = last_modifier_value.try_set(val);
                    }
                    on:blur={
                        let on_mod = on_modifier_input.clone();
                        move |_| {
                            let _ = is_modifier_focused.try_set(false);
                            if let Some(elem) = modifier_ref.get() {
                                let val = elem.value();
                                on_mod(val.clone());
                                let _ = last_modifier_value.try_set(val);
                            }
                        }
                    }
                />
                <span class="tooltip-text" 
                    class:hidden=move || modifier.get().is_empty()
                >
                    {modifier}
                </span>
            </div>

            <div class="dots-container">
                {(1..=max_level).map(|i| {
                    let dot_idx = (i - 1) as usize;
                    let is_filled = move || level.get() >= i;
                    let dot_color = move || {
                        if level.get() >= i {
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

                    let on_dot_change_for_right_click = on_dot_origin_change.clone();
                    let on_right_click = move |ev: ev::MouseEvent| {
                        ev.prevent_default();
                        if level.get() >= i && on_dot_change_for_right_click.is_some() {
                            set_open_popover_idx.update(|cur| {
                                *cur = if *cur == Some(dot_idx) { None } else { Some(dot_idx) };
                            });
                        }
                    };

                    let on_dot_change_for_popover = on_dot_origin_change.clone();
                    let set_origin_to = move |origin: DotOrigin| {
                        if let Some(cb) = on_dot_change_for_popover.as_ref() {
                            cb.call((dot_idx, origin));
                        }
                        set_open_popover_idx.set(None);
                    };

                    let on_click_level = on_level_change_click.clone();

                    let set_origin_base = set_origin_to.clone();
                    let set_origin_bonus = set_origin_to.clone();
                    let set_origin_xp = set_origin_to.clone();
                    let set_origin_temp = set_origin_to;

                    view! {
                        <div class="dot-wrapper">
                            <span 
                                class="dot"
                                class:filled=is_filled
                                class=("dot-base", move || is_filled() && dot_color() == "dot-base")
                                class=("dot-bonus", move || is_filled() && dot_color() == "dot-bonus")
                                class=("dot-xp", move || is_filled() && dot_color() == "dot-xp")
                                class=("dot-temp", move || is_filled() && dot_color() == "dot-temp")
                                on:click=move |_| {
                                    set_open_popover_idx.set(None);
                                    let current = level.get();
                                    let new_val = if i == current {
                                        (i - 1).max(min_level)
                                    } else {
                                        i
                                    };
                                    on_click_level(new_val);
                                }
                                on:contextmenu=on_right_click
                                title="Botão esquerdo: alterar nível | Botão direito: mudar origem (Base/Bônus/XP/Buff)"
                            ></span>

                            {move || if is_popover_open() {
                                let set_base = set_origin_base.clone();
                                let set_bonus = set_origin_bonus.clone();
                                let set_xp = set_origin_xp.clone();
                                let set_temp = set_origin_temp.clone();
                                view! {
                                    <div class="dot-origin-popover" on:click=move |ev| ev.stop_propagation()>
                                        <div class="popover-title">"Origem do Ponto:"</div>
                                        <div class="popover-options">
                                            <button 
                                                class="popover-btn btn-base" 
                                                on:click=move |_| set_base(DotOrigin::Base)
                                                title="Criação Base"
                                            >
                                                <span class="popover-dot dot-base"></span> "Base"
                                            </button>
                                            <button 
                                                class="popover-btn btn-bonus" 
                                                on:click=move |_| set_bonus(DotOrigin::Bonus)
                                                title="Pontos de Bônus (Freebies)"
                                            >
                                                <span class="popover-dot dot-bonus"></span> "Bônus"
                                            </button>
                                            <button 
                                                class="popover-btn btn-xp" 
                                                on:click=move |_| set_xp(DotOrigin::Experience)
                                                title="Experiência (XP)"
                                            >
                                                <span class="popover-dot dot-xp"></span> "XP"
                                            </button>
                                            <button 
                                                class="popover-btn btn-temp" 
                                                on:click=move |_| set_temp(DotOrigin::Temporary)
                                                title="Buff / Magia / Wonder"
                                            >
                                                <span class="popover-dot dot-temp"></span> "Buff"
                                            </button>
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }}
                        </div>
                    }
                }).collect_view()}
            </div>

            {
                if has_supernatural_support {
                    let supernatural_badge_class = move || {
                        let cur = level.get();
                        if cur <= 5 {
                            return "badge-origin-base";
                        }
                        if let Some(sig) = origins {
                            let list = sig.get();
                            let sup_slice = (5..cur.min(10) as usize).filter_map(|i| list.get(i));
                            let mut has_temp = false;
                            let mut has_xp = false;
                            let mut has_bonus = false;
                            for &orig in sup_slice {
                                match orig {
                                    DotOrigin::Temporary => has_temp = true,
                                    DotOrigin::Experience => has_xp = true,
                                    DotOrigin::Bonus => has_bonus = true,
                                    DotOrigin::Base => {}
                                }
                            }
                            if has_temp {
                                "badge-origin-temp"
                            } else if has_xp {
                                "badge-origin-xp"
                            } else if has_bonus {
                                "badge-origin-bonus"
                            } else {
                                "badge-origin-base"
                            }
                        } else {
                            "badge-origin-base"
                        }
                    };

                    let on_click_level_sup = on_level_change_click.clone();
                    let on_dot_change_for_right_click = on_dot_origin_change.clone();
                    let on_dot_change_for_popover = on_dot_origin_change.clone();

                    view! {
                        {move || {
                            let cur_lvl = level.get();
                            let is_modal_open = show_supernatural_modal.get();

                            // 1. Quando nível < 5 e o modal não está aberto: COMPLETAMENTE OCULTO (0px, sem interferir no layout)
                            if cur_lvl < 5 && !is_modal_open {
                                return view! {}.into_view();
                            }

                            let on_click_level_sup = on_click_level_sup.clone();
                            let on_dot_change_for_right_click = on_dot_change_for_right_click.clone();
                            let on_dot_change_for_popover = on_dot_change_for_popover.clone();

                            view! {
                                <div class="supernatural-container supernatural-slot" class:supernatural-empty=move || level.get() == 5>
                                    // 2. Quando nível >= 6: Exibe "|" e o botão compacto com apenas a escrita +1, +2, +3, +4 ou +5
                                    {if cur_lvl >= 6 {
                                        let extra = cur_lvl - 5;
                                        view! {
                                            <span class="supernatural-separator">"|"</span>
                                            <button 
                                                type="button"
                                                class="supernatural-badge"
                                                class=("badge-origin-base", move || supernatural_badge_class() == "badge-origin-base")
                                                class=("badge-origin-bonus", move || supernatural_badge_class() == "badge-origin-bonus")
                                                class=("badge-origin-xp", move || supernatural_badge_class() == "badge-origin-xp")
                                                class=("badge-origin-temp", move || supernatural_badge_class() == "badge-origin-temp")
                                                on:click=move |ev| {
                                                    ev.stop_propagation();
                                                    set_show_supernatural_modal.update(|v| *v = !*v);
                                                }
                                                title=format!("Nível Total: {} (+{} Sobrenatural) • Clique para alterar o valor", cur_lvl, extra)
                                            >
                                                {format!("+{}", extra)}
                                            </button>
                                        }.into_view()
                                    } else if cur_lvl == 5 {
                                        // 3. Quando nível == 5: Exibe apenas "|" e o losango clicável vazio
                                        let on_click_sup = on_click_level_sup.clone();
                                        view! {
                                            <span class="supernatural-separator">"|"</span>
                                            <span 
                                                class="dot dot-supernatural"
                                                on:click=move |ev| {
                                                    ev.stop_propagation();
                                                    on_click_sup(6);
                                                    set_show_supernatural_modal.set(true);
                                                }
                                                title="Clique para abrir a escala sobrenatural (6 a 10)"
                                            ></span>
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }}

                                    // 4. Painel Temporário Flutuante com os 5 Losangos (Níveis 6 a 10)
                                    {if is_modal_open {
                                        let on_click_level_sup = on_click_level_sup.clone();
                                        let on_dot_change_for_right_click = on_dot_change_for_right_click.clone();
                                        let on_dot_change_for_popover = on_dot_change_for_popover.clone();
                                        let on_modifier_input = on_modifier_input.clone();

                                        let save_pending_modifier = std::rc::Rc::new({
                                            let on_mod = on_modifier_input.clone();
                                            move || {
                                                let _ = is_sup_modifier_focused.try_set(false);
                                                let val = last_sup_modifier_value.get_untracked();
                                                on_mod(val);
                                            }
                                        });

                                        view! {
                                            <div class="supernatural-modal-wrapper" on:click=move |ev| ev.stop_propagation()>
                                                <div 
                                                    class="supernatural-backdrop"
                                                    on:click={
                                                        let save_mod = save_pending_modifier.clone();
                                                        move |ev| {
                                                            ev.stop_propagation();
                                                            save_mod();
                                                            set_show_supernatural_modal.set(false);
                                                            set_open_popover_idx.set(None);
                                                        }
                                                    }
                                                />
                                                <div class="supernatural-floating-panel">
                                                    <div class="supernatural-panel-header">
                                                        <div class="panel-header-title">
                                                            <span class="panel-sparkle">"✦"</span>
                                                            <span class="panel-trait-name" title=move || label.get()>{label.get()}</span>
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
                                                                    move |_| {
                                                                        save_mod();
                                                                        set_show_supernatural_modal.set(false);
                                                                        set_open_popover_idx.set(None);
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
                                                                    on_mod(val);
                                                                }
                                                            }
                                                            on:keydown={
                                                                let on_mod = on_modifier_input.clone();
                                                                move |ev: ev::KeyboardEvent| {
                                                                    if ev.key() == "Enter" {
                                                                        let _ = is_sup_modifier_focused.try_set(false);
                                                                        let val = last_sup_modifier_value.get_untracked();
                                                                        on_mod(val);
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
                                                            let set_origin_to = move |origin: DotOrigin| {
                                                                if let Some(cb) = on_dot_change_for_popover.as_ref() {
                                                                    cb.call((dot_idx, origin));
                                                                }
                                                                set_open_popover_idx.set(None);
                                                            };

                                                            let on_click_level_sup = on_click_level_sup.clone();
                                                            let set_base = set_origin_to.clone();
                                                            let set_bonus = set_origin_to.clone();
                                                            let set_xp = set_origin_to.clone();
                                                            let set_temp = set_origin_to;

                                                            let save_pending_modifier = save_pending_modifier.clone();

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
                                                                            on:click={
                                                                                let on_click_sup = on_click_level_sup.clone();
                                                                                let save_mod = save_pending_modifier.clone();
                                                                                move |_| {
                                                                                    save_mod();
                                                                                    set_open_popover_idx.set(None);
                                                                                    let current = level.get();
                                                                                    let new_val = if current == lvl {
                                                                                        lvl - 1
                                                                                    } else {
                                                                                        lvl
                                                                                    };
                                                                                    on_click_sup(new_val);
                                                                                    set_show_supernatural_modal.set(false);
                                                                                }
                                                                            }
                                                                            on:contextmenu=on_right_click
                                                                            title=format!("Nível {} (+{} Sobrenatural) | Clique esquerdo: definir nível | Clique direito: mudar origem", lvl, extra)
                                                                        ></span>

                                                                        {
                                                                            let set_base = set_base.clone();
                                                                            let set_bonus = set_bonus.clone();
                                                                            let set_xp = set_xp.clone();
                                                                            let set_temp = set_temp.clone();
                                                                            move || if is_popover_open() {
                                                                                let set_base = set_base.clone();
                                                                                let set_bonus = set_bonus.clone();
                                                                                let set_xp = set_xp.clone();
                                                                                let set_temp = set_temp.clone();
                                                                                view! {
                                                                                    <div class="dot-origin-popover" on:click=move |ev| ev.stop_propagation()>
                                                                                        <div class="popover-title">{format!("Origem Ponto {}:", lvl)}</div>
                                                                                        <div class="popover-options">
                                                                                            <button 
                                                                                                class="popover-btn btn-base" 
                                                                                                on:click=move |_| set_base(DotOrigin::Base)
                                                                                                title="Criação Base"
                                                                                            >
                                                                                                <span class="popover-dot dot-base"></span> "Base"
                                                                                            </button>
                                                                                            <button 
                                                                                                class="popover-btn btn-bonus" 
                                                                                                on:click=move |_| set_bonus(DotOrigin::Bonus)
                                                                                                title="Pontos de Bônus (Freebies)"
                                                                                            >
                                                                                                <span class="popover-dot dot-bonus"></span> "Bônus"
                                                                                            </button>
                                                                                            <button 
                                                                                                class="popover-btn btn-xp" 
                                                                                                on:click=move |_| set_xp(DotOrigin::Experience)
                                                                                                title="Experiência (XP)"
                                                                                            >
                                                                                                <span class="popover-dot dot-xp"></span> "XP"
                                                                                            </button>
                                                                                            <button 
                                                                                                class="popover-btn btn-temp" 
                                                                                                on:click=move |_| set_temp(DotOrigin::Temporary)
                                                                                                title="Buff / Magia / Wonder"
                                                                                            >
                                                                                                <span class="popover-dot dot-temp"></span> "Buff"
                                                                                            </button>
                                                                                        </div>
                                                                                    </div>
                                                                                }.into_view()
                                                                            } else {
                                                                                view! {}.into_view()
                                                                            }
                                                                        }
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
                                                                move |_| {
                                                                    save_mod();
                                                                    on_reset(5);
                                                                    set_open_popover_idx.set(None);
                                                                    set_show_supernatural_modal.set(false);
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
                                                                move |_| {
                                                                    save_mod();
                                                                    set_open_popover_idx.set(None);
                                                                    set_show_supernatural_modal.set(false);
                                                                }
                                                            }
                                                        >
                                                            "Concluir"
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }}
                                </div>
                            }.into_view()
                        }}
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }
            
            <div class="action-area">
                {on_remove.map(|cb| view! {
                    <button class="remove-btn" on:click=move |_| cb.call(())>"×"</button>
                })}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_field_instantiation_and_reactive_updates() {
        let runtime = create_runtime();
        let (level, set_level) = create_signal(3);
        let (modifier, set_modifier) = create_signal("Test".to_string());
        let (is_starred, set_starred) = create_signal(false);
        let (_star_clicked, set_star_clicked) = create_signal(false);

        let on_toggle_star = Callback::new(move |_| {
            set_star_clicked.set(true);
            set_starred.update(|s| *s = !*s);
        });

        let _view = view! {
            <ValueField 
                label=Signal::derive(|| "Correspondência".to_string())
                level=level.into()
                modifier=modifier.into()
                on_level_change=move |v| set_level.set(v)
                on_modifier_change=move |m| set_modifier.set(m)
                is_starred=is_starred.into()
                on_toggle_star=on_toggle_star
                star_tooltip="Esfera de Afinidade"
            />
        };

        // Simulate signal mutation after initial render to ensure no panic
        set_starred.set(true);
        set_level.set(4);
        set_modifier.set("Especialização".to_string());

        assert_eq!(level.get(), 4);
        assert_eq!(modifier.get(), "Especialização");
        assert_eq!(is_starred.get(), true);

        // Test supernatural trait support
        let (is_supernatural, set_supernatural) = create_signal(false);
        let (_supernatural_toggled, set_supernatural_toggled) = create_signal(false);
        let on_toggle_supernatural = Callback::new(move |_| {
            set_supernatural_toggled.set(true);
            set_supernatural.update(|s| *s = !*s);
        });

        let _view_supernatural = view! {
            <ValueField 
                label=Signal::derive(|| "Força".to_string())
                level=level.into()
                modifier=modifier.into()
                on_level_change=move |v| set_level.set(v)
                on_modifier_change=move |m| set_modifier.set(m)
                is_supernatural=is_supernatural.into()
                on_toggle_supernatural=on_toggle_supernatural
            />
        };

        // Test that setting level to 5 activates the 6th slot automatically
        set_supernatural.set(false);
        set_level.set(5);
        assert_eq!(level.get(), 5);
        assert_eq!(is_supernatural.get(), false);

        set_supernatural.set(true);
        set_level.set(6);
        assert_eq!(level.get(), 6);
        assert_eq!(is_supernatural.get(), true);

        // Test supernatural levels 8 (+3) and 10 (+5)
        set_level.set(8);
        assert_eq!(level.get(), 8);

        set_level.set(10);
        assert_eq!(level.get(), 10);

        runtime.dispose();
    }
}
