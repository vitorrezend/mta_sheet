use leptos::*;
use crate::state::CharacterData;

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

#[component]
pub fn Vitality() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let update_health = move |index: usize, next: DamageType| {
        set_data.update(|s| {
            s.labels.insert(format!("health_{}", index), next.to_key().to_string());
        });
    };

    view! {
        <div class="vitality-container">
            <h3 class="column-title">"Vitalidade"</h3>
            <div class="health-levels">
                {(0..7).map(|i| {
                    let (label, penalty) = HEALTH_LEVELS[i];
                    let current = move || {
                        let s = data.get().labels.get(&format!("health_{}", i)).cloned().unwrap_or_default();
                        DamageType::from_key(&s)
                    };

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
                                    let next = current().cycle();
                                    update_health(i, next);
                                }
                                title=move || match current() {
                                    DamageType::None       => "Clique para marcar dano",
                                    DamageType::Bashing    => "Contusivo (/) – clique para Letal",
                                    DamageType::Lethal     => "Letal (X) – clique para Agravado",
                                    DamageType::Aggravated => "Agravado (✦) – clique para limpar",
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
        </div>
    }
}
