use leptos::*;
use crate::state::CharacterState;

/// Tipo de dano em cada caixa de saúde
#[derive(Clone, Copy, PartialEq, Debug)]
enum DamageType {
    None,
    Bashing,   // Contusivo  → risco diagonal /
    Lethal,    // Letal       → X
    Aggravated, // Agravado   → ✦ (X com cruzes)
}

impl DamageType {
    fn cycle(self) -> Self {
        match self {
            DamageType::None       => DamageType::Bashing,
            DamageType::Bashing    => DamageType::Lethal,
            DamageType::Lethal     => DamageType::Aggravated,
            DamageType::Aggravated => DamageType::None,
        }
    }

    fn to_key(self) -> &'static str {
        match self {
            DamageType::None       => "none",
            DamageType::Bashing    => "bashing",
            DamageType::Lethal     => "lethal",
            DamageType::Aggravated => "aggravated",
        }
    }

    fn from_key(s: &str) -> Self {
        match s {
            "bashing"    => DamageType::Bashing,
            "lethal"     => DamageType::Lethal,
            "aggravated" => DamageType::Aggravated,
            _            => DamageType::None,
        }
    }
}

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

fn save_health(index: usize, damage: DamageType) {
    CharacterState::save_label(
        &format!("health_{}", index),
        damage.to_key(),
    );
}

fn load_health(index: usize) -> DamageType {
    let s = CharacterState::load_label(&format!("health_{}", index));
    DamageType::from_key(&s)
}

#[component]
pub fn Vitality() -> impl IntoView {
    // Um sinal por nível de saúde
    let health: Vec<RwSignal<DamageType>> = (0..7)
        .map(|i| create_rw_signal(load_health(i)))
        .collect();

    view! {
        <div class="vitality-container">
            <h3 class="column-title">"Vitalidade"</h3>
            <div class="health-levels">
                {health.into_iter().enumerate().map(|(i, sig)| {
                    let (label, penalty) = HEALTH_LEVELS[i];
                    view! {
                        <div class="health-row">
                            // Label + penalidade
                            <div class="health-label-group">
                                <span class="health-label">{label}</span>
                                {penalty.map(|p| view! {
                                    <span class="health-penalty">{p}</span>
                                })}
                            </div>

                            // Caixa de dano clicável
                            <div
                                class="health-box"
                                class:damage-bashing=move || sig.get() == DamageType::Bashing
                                class:damage-lethal=move || sig.get() == DamageType::Lethal
                                class:damage-aggravated=move || sig.get() == DamageType::Aggravated
                                on:click=move |_| {
                                    let next = sig.get().cycle();
                                    sig.set(next);
                                    save_health(i, next);
                                }
                                title=move || match sig.get() {
                                    DamageType::None       => "Clique para marcar dano",
                                    DamageType::Bashing    => "Contusivo (/) – clique para Letal",
                                    DamageType::Lethal     => "Letal (X) – clique para Agravado",
                                    DamageType::Aggravated => "Agravado (✦) – clique para limpar",
                                }
                            >
                                {move || match sig.get() {
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
        </div>
    }
}
