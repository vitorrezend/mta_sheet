use leptos::*;
use crate::state::{CharacterData, DamageType};

// Níveis de saúde padrão (após os níveis extras de Escoriado): (label, penalty)
const BASE_HEALTH_LEVELS: [(&str, Option<&str>); 7] = [
    ("Escoriado",         None),
    ("Machucado",         Some("-1")),
    ("Ferido",            Some("-1")),
    ("Ferido Gravemente", Some("-2")),
    ("Espancado",         Some("-2")),
    ("Aleijado",          Some("-5")),
    ("Incapacitado",      None),
];

#[component]
fn HealthDamageIcon(dmg: DamageType) -> impl IntoView {
    match dmg {
        DamageType::None => view! {
            <span class="dmg-empty"></span>
        }.into_view(),
        DamageType::Bashing => view! {
            <svg class="dmg-svg dmg-bashing-svg" viewBox="0 0 16 16" width="14" height="14">
                <line x1="3" y1="13" x2="13" y2="3" stroke="currentColor" stroke-width="2.4" stroke-linecap="round"/>
            </svg>
        }.into_view(),
        DamageType::Lethal => view! {
            <svg class="dmg-svg dmg-lethal-svg" viewBox="0 0 16 16" width="14" height="14">
                <line x1="3" y1="13" x2="13" y2="3" stroke="currentColor" stroke-width="2.4" stroke-linecap="round"/>
                <line x1="3" y1="3" x2="13" y2="13" stroke="currentColor" stroke-width="2.4" stroke-linecap="round"/>
            </svg>
        }.into_view(),
        DamageType::Aggravated => view! {
            <svg class="dmg-svg dmg-agg-svg" viewBox="0 0 16 16" width="14" height="14">
                <line x1="3" y1="13" x2="13" y2="3" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"/>
                <line x1="3" y1="3" x2="13" y2="13" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"/>
                <line x1="8" y1="2" x2="8" y2="14" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"/>
                <line x1="2" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"/>
            </svg>
        }.into_view(),
    }
}

#[component]
pub fn Vitality() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let counts = move || data.with(|d| d.get_health_counts());
    let is_gm = move || data.with(|d| d.is_gods_and_monsters());
    let extra_bruised = move || data.with(|d| d.get_extra_bruised());
    let total_boxes = move || data.with(|d| d.get_total_health_boxes());
    let total_damaged = move || {
        let (agg, lethal, bashing) = counts();
        agg + lethal + bashing
    };

    view! {
        <div class="vitality-container">
            <div class="vitality-header-row">
                <h3 class="column-title">{move || crate::i18n::tr("vitality", lang())}</h3>
                <div class="vitality-actions">
                    {move || if is_gm() {
                        let extra = extra_bruised();
                        view! {
                            <div class="bruised-controls">
                                <button
                                    type="button"
                                    class="add-field-btn bruised-btn"
                                    on:click=move |_| set_data.update(|s| s.add_extra_bruised())
                                    title=move || crate::i18n::tr("add_extra_bruised", lang())
                                >
                                    "+"
                                </button>
                                {if extra > 0 {
                                    view! {
                                        <button
                                            type="button"
                                            class="add-field-btn bruised-btn remove-bruised"
                                            on:click=move |_| set_data.update(|s| s.remove_extra_bruised())
                                            title=move || crate::i18n::tr("remove_extra_bruised", lang())
                                        >
                                            "-"
                                        </button>
                                    }.into_view()
                                } else {
                                    ().into_view()
                                }}
                            </div>
                        }.into_view()
                    } else {
                        ().into_view()
                    }}
                    <button
                        type="button"
                        class="vitality-reset-btn"
                        on:click=move |_| set_data.update(|s| s.clear_health())
                        title="Limpar todos os danos (Curar totalmente)"
                    >
                        {move || match lang() {
                            crate::i18n::Language::PtBr => "🧹 Limpar",
                            crate::i18n::Language::EnUs => "🧹 Clear",
                        }}
                    </button>
                </div>
            </div>

            <div class="health-levels">
                {move || {
                    let total = total_boxes();
                    let extra = extra_bruised();
                    let current_lang = lang();
                    (0..total).map(|i| {
                        let (label, penalty) = if i < extra {
                            ("Escoriado", None)
                        } else {
                            let base_idx = i - extra;
                            BASE_HEALTH_LEVELS[base_idx.min(6)]
                        };
                        let current = move || data.with(|d| d.get_health(i));
                        let translated_label = crate::i18n::tr_health(label, current_lang);
                        let is_active_wound = move || {
                            let td = total_damaged();
                            td > 0 && i == td - 1
                        };

                        view! {
                            <div
                                class="health-row"
                                class:is-active-wound=is_active_wound
                            >
                                <div class="health-label-group">
                                    <span class="health-label">{translated_label}</span>
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
                                        DamageType::None       => "Vazio (Clique para marcar dano Contundente /)",
                                        DamageType::Bashing    => "Contundente (/) – Clique: Letal ✕ (Botão direito para curar)",
                                        DamageType::Lethal     => "Letal (✕) – Clique: Agravado ✳ (Botão direito para curar)",
                                        DamageType::Aggravated => "Agravado (✳) – Dano Sobrenatural/Fogo (Botão direito para curar)",
                                    }
                                >
                                    {move || view! { <HealthDamageIcon dmg=current() /> }}
                                </div>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>

            <div class="vitality-footer-info">
                {move || {
                    let (agg, lethal, bashing) = counts();
                    let total = agg + lethal + bashing;
                    let current_lang = lang();
                    if total > 0 {
                        view! {
                            <div class="vitality-count-pills">
                                {if agg > 0 {
                                    let txt = match current_lang {
                                        crate::i18n::Language::PtBr => format!("✳ Agravado: {}", agg),
                                        crate::i18n::Language::EnUs => format!("✳ Aggravated: {}", agg),
                                    };
                                    view! { <span class="count-pill pill-agg">{txt}</span> }.into_view()
                                } else { ().into_view() }}
                                {if lethal > 0 {
                                    let txt = match current_lang {
                                        crate::i18n::Language::PtBr => format!("✕ Letal: {}", lethal),
                                        crate::i18n::Language::EnUs => format!("✕ Lethal: {}", lethal),
                                    };
                                    view! { <span class="count-pill pill-lethal">{txt}</span> }.into_view()
                                } else { ().into_view() }}
                                {if bashing > 0 {
                                    let txt = match current_lang {
                                        crate::i18n::Language::PtBr => format!("/ Contundente: {}", bashing),
                                        crate::i18n::Language::EnUs => format!("/ Bashing: {}", bashing),
                                    };
                                    view! { <span class="count-pill pill-bashing">{txt}</span> }.into_view()
                                } else { ().into_view() }}
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <span class="vitality-healthy-tag">
                                {match current_lang {
                                    crate::i18n::Language::PtBr => "✨ Ileso (Sem Danos)",
                                    crate::i18n::Language::EnUs => "✨ Unhurt (No Damage)",
                                }}
                            </span>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
