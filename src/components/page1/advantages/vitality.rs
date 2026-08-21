use leptos::*;
use crate::state::{CharacterData, DamageType};

// Níveis de saúde: (label, penalty)
const HEALTH_LEVELS: [(&str, Option<&str>); 7] = [
    ("Escoriado",         None),
    ("Machucado",         Some("-1")),
    ("Ferido",            Some("-1")),
    ("Ferido Gravemente", Some("-2")),
    ("Espancado",         Some("-2")),
    ("Aleijado",          Some("-5")),
    ("Incapacitado",      None),
];

#[component]
pub fn Vitality() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let counts = move || data.with(|d| d.get_health_counts());

    view! {
        <div class="vitality-container">
            <div class="vitality-header-row">
                <h3 class="column-title">"Vitalidade"</h3>
                <div class="vitality-actions">
                    <button
                        type="button"
                        class="vitality-reset-btn"
                        on:click=move |_| set_data.update(|s| s.clear_health())
                        title="Limpar todos os danos (Curar totalmente)"
                    >
                        "🧹 Limpar"
                    </button>
                </div>
            </div>

            <div class="health-levels">
                {(0..7).map(|i| {
                    let (label, penalty) = HEALTH_LEVELS[i];
                    let current = move || data.with(|d| d.get_health(i));

                    view! {
                        <div class="health-row">
                            <div class="health-label-group">
                                <span class="health-label">{label}</span>
                                {penalty.map(|p| view! {
                                    <span class="health-penalty">{p}</span>
                                })}
                            </div>

                            <div
                                class="health-box"
                                class:damage-bashing=move || current() == DamageType::Bashing
                                class:damage-lethal=move || current() == DamageType::Lethal
                                class:damage-aggravated=move || current() == DamageType::Aggravated
                                on:click=move |_| {
                                    set_data.update(|s| s.click_health_box(i));
                                }
                                on:contextmenu=move |ev: ev::MouseEvent| {
                                    ev.prevent_default();
                                    set_data.update(|s| s.heal_health_box(i));
                                }
                                title=move || match current() {
                                    DamageType::None       => "Vazio (Clique para marcar dano /)",
                                    DamageType::Bashing    => "Contusivo (/) – Clique para Letal ✕ (Botão direito para curar)",
                                    DamageType::Lethal     => "Letal (✕) – Clique para Agravado ✦ (Botão direito para curar)",
                                    DamageType::Aggravated => "Agravado (✦) – Clique para curar",
                                }
                            >
                                {move || match current() {
                                    DamageType::None       => view! { <span class="dmg-mark"></span> },
                                    DamageType::Bashing    => view! { <span class="dmg-mark dmg-slash">"/"</span> },
                                    DamageType::Lethal     => view! { <span class="dmg-mark dmg-x">"✕"</span> },
                                    DamageType::Aggravated => view! { <span class="dmg-mark dmg-agg">"✦"</span> },
                                }}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            <div class="vitality-footer-info">
                {move || {
                    let (agg, lethal, bashing) = counts();
                    let total = agg + lethal + bashing;
                    if total > 0 {
                        view! {
                            <div class="vitality-count-pills">
                                {if agg > 0 { view! { <span class="count-pill pill-agg">{format!("✦ Agravado: {}", agg)}</span> }.into_view() } else { ().into_view() }}
                                {if lethal > 0 { view! { <span class="count-pill pill-lethal">{format!("✕ Letal: {}", lethal)}</span> }.into_view() } else { ().into_view() }}
                                {if bashing > 0 { view! { <span class="count-pill pill-bashing">{format!("/ Contusivo: {}", bashing)}</span> }.into_view() } else { ().into_view() }}
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <span class="vitality-healthy-tag">"✨ Ileso (Sem Danos)"</span>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
