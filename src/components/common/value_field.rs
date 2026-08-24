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
) -> impl IntoView {
    let on_level_change = Rc::new(on_level_change);
    let on_modifier_change = Rc::new(on_modifier_change);
    let (open_popover_idx, set_open_popover_idx) = create_signal(Option::<usize>::None);

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

    let (editing_label, set_editing_label) = if is_editable {
        create_signal(label.get_untracked().is_empty())
    } else {
        create_signal(false)
    };
    let (local_label, set_local_label) = create_signal(label.get_untracked());

    // Sincroniza o valor local quando entra em modo de edição
    if is_editable {
        create_effect(move |_| {
            if editing_label.get() {
                set_local_label.set(label.get());
            }
        });
    }

    let on_level_change_click = on_level_change.clone();
    let on_modifier_input = on_modifier_change.clone();

    view! {
        <div class="attribute-row">
            <div class="tooltip-container">
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
                    move || if editing_label.get() {
                        let on_label_blur = on_label_change.clone();
                        let on_label_enter = on_label_change.clone();
                        view! {
                            <input 
                                type="text" 
                                class="label-edit-input"
                                placeholder="..."
                                maxlength="32"
                                prop:value=local_label
                                on:input=move |ev| set_local_label.set(event_target_value(&ev))
                                on:blur=move |_| {
                                    let val = local_label.get();
                                    if !val.trim().is_empty() {
                                        if let Some(cb) = on_label_blur.as_ref() {
                                            cb.call(val);
                                        }
                                        set_editing_label.set(false);
                                    }
                                }
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" {
                                        let val = local_label.get();
                                        if !val.trim().is_empty() {
                                            if let Some(cb) = on_label_enter.as_ref() {
                                                cb.call(val);
                                            }
                                            set_editing_label.set(false);
                                        }
                                    }
                                }
                                on:mount=move |el: web_sys::HtmlInputElement| { 
                                    let _ = el.focus(); 
                                }
                            />
                        }.into_view()
                    } else {
                        view! {
                            <span 
                                class="attribute-label"
                                class:affinity-active=move || is_starred.map(|s| s.get()).unwrap_or(false)
                                on:dblclick=move |_| if is_editable { set_editing_label.set(true) }
                            >
                                {display_label}
                            </span>
                        }.into_view()
                    }
                }
                <span class="tooltip-text">{tooltip_label}</span>
            </div>

            <div class="modifier-container tooltip-container">
                <input 
                    type="text" 
                    class="field-modifier" 
                    placeholder="..."
                    maxlength="30"
                    prop:value=modifier
                    on:input=move |ev| on_modifier_input(event_target_value(&ev))
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

        runtime.dispose();
    }
}
