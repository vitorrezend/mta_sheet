use leptos::*;
use crate::components::sheet::ActiveDotOriginContext;
use crate::state::{CharacterData, DotOrigin};

#[component]
pub fn Arete() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    let name = "Arete";
    let level = Signal::derive(move || data.with(|d| d.attributes.get(name).map(|a| a.level).unwrap_or(1).max(1)));
    let origins = Signal::derive(move || data.with(|d| d.attributes.get(name).map(|a| a.get_origins(10)).unwrap_or_else(|| vec![DotOrigin::Base; 10])));

    let (open_popover_idx, set_open_popover_idx) = create_signal::<Option<usize>>(None);

    let update_level = move |new_val: i32| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
        let val = new_val.max(1);
        set_data.update(|s| {
            s.set_attribute_with_origin(name, Some(val), None, current_origin);
        });
    };

    let update_dot_origin = move |dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            s.set_attribute_dot_origin(name, dot_idx, origin);
        });
    };

    view! {
        <div class="arete-container" on:mouseleave=move |_| set_open_popover_idx.set(None)>
            <h3 class="column-title">"Arete"</h3>
            <div class="dots-container" style="justify-content: center; margin-bottom: 0.5rem; gap: 6px;">
                {(1..=10).map(|i| {
                    let dot_idx = (i - 1) as usize;
                    let is_filled = move || level.get() >= i;
                    let dot_color = move || {
                        if level.get() >= i {
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
                        if level.get() >= i {
                            set_open_popover_idx.update(|cur| {
                                *cur = if *cur == Some(dot_idx) { None } else { Some(dot_idx) };
                            });
                        }
                    };

                    let set_origin_to = move |origin: DotOrigin| {
                        update_dot_origin(dot_idx, origin);
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
                                        (i - 1).max(1)
                                    } else {
                                        i
                                    };
                                    update_level(new_val);
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
                                                title="Pontos de Bônus (Freebies: 4 pts)"
                                            >
                                                <span class="popover-dot dot-bonus"></span> "Bônus"
                                            </button>
                                            <button 
                                                class="popover-btn btn-xp" 
                                                on:click=move |_| set_origin_to(DotOrigin::Experience)
                                                title="Experiência (XP: Atual × 8)"
                                            >
                                                <span class="popover-dot dot-xp"></span> "XP"
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
        </div>
    }
}
