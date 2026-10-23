//! Bolinhas de nível (1 a 5) com o menu de origem de cada ponto.

use leptos::*;
use crate::state::DotOrigin;
use super::super::callback::Callback;
use super::super::dot_origin_popover::DotOriginPopover;
use super::summary::dot_color;
use super::{with_origins, TraitField};

#[component]
pub fn ValueDots(field: TraitField, min_level: i32, max_level: i32) -> impl IntoView {
    let TraitField { level, origins, popover, on_level_change, on_origin_change, .. } = field;
    view! {
        <div class="dots-container">
            {(1..=max_level).map(|i| {
                let dot_idx = (i - 1) as usize;
                let is_filled = move || level.get() >= i;
                let color = move || with_origins(origins, |o| dot_color(level.get(), i, o));
                let has_color = move |class: &'static str| is_filled() && color() == class;

                let can_pick_origin = on_origin_change.is_some();
                let on_right_click = move |ev: ev::MouseEvent| {
                    ev.prevent_default();
                    if level.get() >= i && can_pick_origin {
                        popover.update(|cur| *cur = if *cur == Some(dot_idx) { None } else { Some(dot_idx) });
                    }
                };
                let on_origin_change = on_origin_change.clone();
                let on_select_origin = Callback::new(move |origin: DotOrigin| {
                    if let Some(cb) = on_origin_change.as_ref() {
                        cb.call((dot_idx, origin));
                    }
                    popover.set(None);
                });
                let on_level_change = on_level_change.clone();

                view! {
                    <div class="dot-wrapper">
                        <span
                            class="dot"
                            class:filled=is_filled
                            class=("dot-base", move || has_color("dot-base"))
                            class=("dot-bonus", move || has_color("dot-bonus"))
                            class=("dot-xp", move || has_color("dot-xp"))
                            class=("dot-temp", move || has_color("dot-temp"))
                            on:click:undelegated=move |_| {
                                popover.set(None);
                                // Clicar na última bolinha marcada desmarca ela
                                let new_val = if i == level.get() { (i - 1).max(min_level) } else { i };
                                on_level_change.call(new_val);
                            }
                            on:contextmenu=on_right_click
                            title="Botão esquerdo: alterar nível | Botão direito: mudar origem (Base/Bônus/XP/Buff)"
                        ></span>
                        <DotOriginPopover
                            lvl=i
                            is_open=Signal::derive(move || popover.get() == Some(dot_idx))
                            on_select=on_select_origin
                        />
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
