use leptos::*;
use crate::components::character_sheet::ActiveDotOriginContext;
use crate::state::{keys, CharacterData, DotOrigin};

#[component]
pub fn Willpower() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    let total_name = keys::KEY_WILLPOWER_TOTAL;
    let current_name = keys::KEY_WILLPOWER_CURRENT;

    let total = Signal::derive(move || {
        let val = data.with(|d| d.attributes.get(total_name).map(|a| a.level).unwrap_or(5));
        if val == 0 { 5 } else { val }
    });

    let origins = Signal::derive(move || {
        data.with(|d| d.attributes.get(total_name).map(|a| a.get_origins(10)).unwrap_or_else(|| vec![DotOrigin::Base; 10]))
    });

    let current = Signal::derive(move || {
        data.with(|d| d.attributes.get(current_name).map(|a| a.level).unwrap_or(0))
    });

    let (open_popover_idx, set_open_popover_idx) = create_signal::<Option<usize>>(None);

    let update_total = move |new_val: i32| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
        let val = new_val.max(1);
        set_data.update(|s| {
            // Se estiver aumentando acima de 5 e o modo selecionado for Base, define os novos pontos como Bônus (1 pt)
            // Se estiver no modo XP, define como XP (Nível Atual × 1)
            // Se estiver no modo Bônus, define como Bônus
            let target_origin = if val > 5 && current_origin == DotOrigin::Base {
                DotOrigin::Bonus
            } else {
                current_origin
            };
            s.set_attribute_with_origin(total_name, Some(val), None, target_origin);
            if let Some(c_attr) = s.attributes.get_mut(current_name) {
                if c_attr.level > val {
                    c_attr.level = val;
                }
            }
        });
    };

    let update_dot_origin = move |dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            s.set_attribute_dot_origin(total_name, dot_idx, origin);
        });
    };

    let update_current = move |new_val: i32| {
        let val = new_val.clamp(0, total.get());
        set_data.update(|s| {
            s.attributes.entry(current_name.to_string()).or_default().level = val;
        });
    };

    // Resumo de origens para os pontos preenchidos
    let origin_summary = Signal::derive(move || {
        let tot = total.get() as usize;
        let orig_list = origins.get();
        let mut base = 0;
        let mut bonus = 0;
        let mut xp = 0;
        let mut temp = 0;
        for i in 0..tot {
            let orig = if i < orig_list.len() { orig_list[i] } else { DotOrigin::Base };
            match orig {
                DotOrigin::Base => base += 1,
                DotOrigin::Bonus => bonus += 1,
                DotOrigin::Experience => xp += 1,
                DotOrigin::Temporary => temp += 1,
            }
        }
        (base, bonus, xp, temp)
    });

    view! {
        <div class="willpower-container" style="margin-top: 1.2rem;" on:mouseleave=move |_| set_open_popover_idx.set(None)>
            <h3 class="column-title">"Força de Vontade"</h3>
            
            <div class="dots-container" style="justify-content: center; margin-bottom: 0.35rem; gap: 6px;">
                {(1..=10).map(|i| {
                    let dot_idx = (i - 1) as usize;
                    let is_filled = move || total.get() >= i;
                    let dot_color = move || {
                        if total.get() >= i {
                            let list = origins.get();
                            if dot_idx < list.len() {
                                list[dot_idx].color_class()
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
                        if total.get() >= i {
                            set_open_popover_idx.update(|cur| {
                                *cur = if *cur == Some(dot_idx) { None } else { Some(dot_idx) };
                            });
                        }
                    };

                    let set_origin_to = move |origin: DotOrigin| {
                        update_dot_origin(dot_idx, origin);
                        set_open_popover_idx.set(None);
                    };

                    let dot_tooltip = move || {
                        if total.get() >= i {
                            let list = origins.get();
                            let orig = if dot_idx < list.len() { list[dot_idx] } else { DotOrigin::Base };
                            let (_, desc) = CharacterData::get_dot_cost_description(total_name, dot_idx, orig, false);
                            format!("Ponto {}: {} | Botão esquerdo: alterar nível | Botão direito: alterar origem (Base/Bônus/XP/Buff)", i, desc)
                        } else {
                            format!("Ponto {}: Clique para aumentar (use o modo Bônus/XP no topo ou botão direito)", i)
                        }
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
                                    let current_val = total.get();
                                    let new_val = if i == current_val {
                                        (i - 1).max(1)
                                    } else {
                                        i
                                    };
                                    update_total(new_val);
                                }
                                on:contextmenu=on_right_click
                                title=dot_tooltip
                            ></span>

                            {move || if is_popover_open() {
                                view! {
                                    <div class="dot-origin-popover" on:click=move |ev| ev.stop_propagation()>
                                        <div class="popover-title">"Origem do Ponto:"</div>
                                        <div class="popover-options">
                                            <button 
                                                class="popover-btn btn-base" 
                                                on:click=move |_| set_origin_to(DotOrigin::Base)
                                                title="Criação Base (5 pontos iniciais grátis)"
                                            >
                                                <span class="popover-dot dot-base"></span> "Base"
                                            </button>
                                            <button 
                                                class="popover-btn btn-bonus" 
                                                on:click=move |_| set_origin_to(DotOrigin::Bonus)
                                                title="Pontos de Bônus (Freebies: 1 pt por ponto)"
                                            >
                                                <span class="popover-dot dot-bonus"></span> "Bônus (1 pt)"
                                            </button>
                                            <button 
                                                class="popover-btn btn-xp" 
                                                on:click=move |_| set_origin_to(DotOrigin::Experience)
                                                title="Experiência (XP: Nível Atual × 1)"
                                            >
                                                <span class="popover-dot dot-xp"></span> "XP (Atual × 1)"
                                            </button>
                                            <button 
                                                class="popover-btn btn-temp" 
                                                on:click=move |_| set_origin_to(DotOrigin::Temporary)
                                                title="Buff / Efeito Temporário"
                                            >
                                                <span class="popover-dot dot-temp"></span> "Buff"
                                            </button>
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }}
                        </div>
                    }
                }).collect_view()}
            </div>

            // Resumo de composição dos pontos de Força de Vontade
            <div class="willpower-origin-summary">
                {move || {
                    let (base, bonus, xp, temp) = origin_summary.get();
                    view! {
                        <div class="wp-summary-pills">
                            <span class="wp-pill wp-pill-base" title="5 Pontos Iniciais de Criação">
                                {format!("Base: {}", base)}
                            </span>
                            {if bonus > 0 {
                                view! {
                                    <span class="wp-pill wp-pill-bonus" title={format!("{} pontos comprados com Bônus ({} pts)", bonus, bonus)}>
                                        {format!("+{} Bônus", bonus)}
                                    </span>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }}
                            {if xp > 0 {
                                view! {
                                    <span class="wp-pill wp-pill-xp" title={format!("{} pontos comprados com Experiência", xp)}>
                                        {format!("+{} XP", xp)}
                                    </span>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }}
                            {if temp > 0 {
                                view! {
                                    <span class="wp-pill wp-pill-temp" title={format!("{} pontos de Buff temporário", temp)}>
                                        {format!("+{} Buff", temp)}
                                    </span>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }}
                        </div>
                    }
                }}
            </div>

            <div class="dots-container" style="justify-content: center; gap: 6px; margin-top: 0.45rem;">
                {(1..=10).map(|i| {
                    let is_filled = move || current.get() >= i;
                    view! {
                        <span 
                            class="square"
                            class:filled=is_filled
                            on:click=move |_| {
                                let cur = current.get();
                                let new_val = if i == cur {
                                    i - 1
                                } else {
                                    i
                                };
                                update_current(new_val);
                            }
                            title=move || format!("Força de Vontade Temporária ({}/{})", i, total.get())
                        ></span>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_willpower_reactive_scope_instantiation() {
        let runtime = create_runtime();
        let (data, set_data) = create_signal(CharacterData::new("wp_test".to_string(), "Mago".to_string()));
        provide_context(data);
        provide_context(set_data);

        // Instantiate component inside reactive root
        let _view = Willpower();

        // Update willpower level and ensure reactive signals work without un-tracked access panics
        set_data.update(|s| {
            s.set_willpower_total(7);
        });

        assert_eq!(data.get().get_willpower().0, 7);
        runtime.dispose();
    }
}
