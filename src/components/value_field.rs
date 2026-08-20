use leptos::*;
use crate::state::DotOrigin;

#[component]
pub fn ValueField(
    label: Signal<String>,
    level: Signal<i32>,
    modifier: Signal<String>,
    on_level_change: impl Fn(i32) + 'static,
    on_modifier_change: impl Fn(String) + 'static,
    #[prop(default = 0)] min_level: i32,
    #[prop(optional)] max_chars: Option<usize>,
    #[prop(optional)] on_remove: Option<Callback<()>>,
    #[prop(default = false)] is_editable: bool,
    #[prop(optional)] on_label_change: Option<Callback<String>>,
    #[prop(optional)] origins: Option<Signal<Vec<DotOrigin>>>,
    #[prop(optional)] on_dot_origin_change: Option<Callback<(usize, DotOrigin)>>,
) -> impl IntoView {
    let on_level_change = store_value(on_level_change);
    let on_modifier_change = store_value(on_modifier_change);
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
        if cur_lvl > 0 && origins.is_some() {
            let orig_list = origins.unwrap().get();
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
                format!("{} (Nível {}: {})", l, cur_lvl, parts.join(" • "))
            } else {
                format!("{} (Nível {})", l, cur_lvl)
            }
        } else {
            l
        }
    };

    let (editing_label, set_editing_label) = create_signal(is_editable && label.get_untracked().is_empty());
    let (local_label, set_local_label) = create_signal(label.get_untracked());

    // Sincroniza o valor local quando entra em modo de edição
    create_effect(move |_| {
        if editing_label.get() {
            set_local_label.set(label.get());
        }
    });

    view! {
        <div class="attribute-row">
            <div class="tooltip-container">
                {move || if editing_label.get() {
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
                                    if let Some(cb) = on_label_change {
                                        cb.call(val);
                                    }
                                    set_editing_label.set(false);
                                }
                            }
                            on:keydown=move |ev| {
                                if ev.key() == "Enter" {
                                    let val = local_label.get();
                                    if !val.trim().is_empty() {
                                        if let Some(cb) = on_label_change {
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
                            on:dblclick=move |_| if is_editable { set_editing_label.set(true) }
                        >
                            {display_label}
                        </span>
                    }.into_view()
                }}
                <span class="tooltip-text">{tooltip_label}</span>
            </div>

            <div class="modifier-container tooltip-container">
                <input 
                    type="text" 
                    class="field-modifier" 
                    placeholder="..."
                    maxlength="30"
                    prop:value=modifier
                    on:input=move |ev| on_modifier_change.with_value(|cb| cb(event_target_value(&ev)))
                />
                <span class="tooltip-text" 
                    class:hidden=move || modifier.get().is_empty()
                >
                    {modifier}
                </span>
            </div>

            <div class="dots-container">
                {move || (1..=5).map(|i| {
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

                    let on_right_click = move |ev: ev::MouseEvent| {
                        ev.prevent_default();
                        if level.get() >= i && on_dot_origin_change.is_some() {
                            set_open_popover_idx.update(|cur| {
                                *cur = if *cur == Some(dot_idx) { None } else { Some(dot_idx) };
                            });
                        }
                    };

                    let set_origin_to = move |origin: DotOrigin| {
                        if let Some(cb) = on_dot_origin_change {
                            cb.call((dot_idx, origin));
                        }
                        set_open_popover_idx.set(None);
                    };

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
                                    on_level_change.with_value(|cb| cb(new_val))
                                }
                                on:contextmenu=on_right_click
                                title="Botão esquerdo: alterar nível | Botão direito: mudar origem (Base/Bônus/XP/Buff)"
                            ></span>

                            {move || if is_popover_open() {
                                view! {
                                    <div class="dot-origin-popover" on:click=move |ev| ev.stop_propagation()>
                                        <div class="popover-title">"Origem do Ponto:"</div>
                                        <div class="popover-options">
                                            <button 
                                                class="popover-btn btn-base" 
                                                on:click=move |_| set_origin_to(DotOrigin::Base)
                                                title="Criação Base"
                                            >
                                                <span class="popover-dot dot-base"></span> "Base"
                                            </button>
                                            <button 
                                                class="popover-btn btn-bonus" 
                                                on:click=move |_| set_origin_to(DotOrigin::Bonus)
                                                title="Pontos de Bônus (Freebies)"
                                            >
                                                <span class="popover-dot dot-bonus"></span> "Bônus"
                                            </button>
                                            <button 
                                                class="popover-btn btn-xp" 
                                                on:click=move |_| set_origin_to(DotOrigin::Experience)
                                                title="Experiência (XP)"
                                            >
                                                <span class="popover-dot dot-xp"></span> "XP"
                                            </button>
                                            <button 
                                                class="popover-btn btn-temp" 
                                                on:click=move |_| set_origin_to(DotOrigin::Temporary)
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
    fn test_value_field_instantiation() {
        let runtime = create_runtime();
        let (level, _) = create_signal(3);
        let (modifier, _) = create_signal("Test".to_string());
        
        let _view = view! {
            <ValueField 
                label=Signal::derive(|| "Força".to_string())
                level=level.into()
                modifier=modifier.into()
                on_level_change=|_| {}
                on_modifier_change=|_| {}
            />
        };
        runtime.dispose();
    }
}
