use leptos::*;
use crate::state::DotOrigin;
use super::callback::Callback;

#[component]
pub fn DotOriginPopover(
    lvl: i32,
    is_open: Signal<bool>,
    on_select: Callback<DotOrigin>,
) -> impl IntoView {
    view! {
        {move || if is_open.get() {
            let on_base = on_select.clone();
            let on_bonus = on_select.clone();
            let on_xp = on_select.clone();
            let on_temp = on_select.clone();

            view! {
                <div class="dot-origin-popover" on:click=move |ev| ev.stop_propagation()>
                    <div class="popover-title">{format!("Origem Ponto {}:", lvl)}</div>
                    <div class="popover-options">
                        <button 
                            type="button"
                            class="popover-btn btn-base" 
                            on:click=move |_| on_base.call(DotOrigin::Base)
                            title="Criação Base"
                        >
                            <span class="popover-dot dot-base"></span> "Base"
                        </button>
                        <button 
                            type="button"
                            class="popover-btn btn-bonus" 
                            on:click=move |_| on_bonus.call(DotOrigin::Bonus)
                            title="Pontos de Bônus (Freebies)"
                        >
                            <span class="popover-dot dot-bonus"></span> "Bônus"
                        </button>
                        <button 
                            type="button"
                            class="popover-btn btn-xp" 
                            on:click=move |_| on_xp.call(DotOrigin::Experience)
                            title="Experiência (XP)"
                        >
                            <span class="popover-dot dot-xp"></span> "XP"
                        </button>
                        <button 
                            type="button"
                            class="popover-btn btn-temp" 
                            on:click=move |_| on_temp.call(DotOrigin::Temporary)
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
    }
}
