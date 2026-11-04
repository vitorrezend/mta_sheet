use leptos::*;
use crate::components::compendium::CompendiumSection;
use crate::i18n::Language;

#[derive(Clone, Copy, Debug)]
pub struct SectionMeta {
    pub section: CompendiumSection,
    pub icon: &'static str,
    pub name_pt: &'static str,
    pub name_en: &'static str,
}

pub const ALL_SECTIONS: &[SectionMeta] = &[
    SectionMeta {
        section: CompendiumSection::Paradigms,
        icon: "☯️",
        name_pt: "Paradigmas",
        name_en: "Paradigms",
    },
    SectionMeta {
        section: CompendiumSection::Practices,
        icon: "📜",
        name_pt: "Práticas Mágicas",
        name_en: "Magickal Practices",
    },
    SectionMeta {
        section: CompendiumSection::Instruments,
        icon: "🛠️",
        name_pt: "Instrumentos & Focos",
        name_en: "Instruments & Focus",
    },
    SectionMeta {
        section: CompendiumSection::Archetypes,
        icon: "🎭",
        name_pt: "Arquétipos",
        name_en: "Archetypes",
    },
    SectionMeta {
        section: CompendiumSection::Attributes,
        icon: "🧠",
        name_pt: "Atributos & Regras",
        name_en: "Attributes & Rules",
    },
    SectionMeta {
        section: CompendiumSection::Backgrounds,
        icon: "👥",
        name_pt: "Antecedentes",
        name_en: "Backgrounds",
    },
    SectionMeta {
        section: CompendiumSection::Spheres,
        icon: "🔮",
        name_pt: "Esferas da Mágika",
        name_en: "Spheres of Magick",
    },
    SectionMeta {
        section: CompendiumSection::Weapons,
        icon: "⚔️",
        name_pt: "Armas & Manobras",
        name_en: "Weapons & Maneuvers",
    },
    SectionMeta {
        section: CompendiumSection::Abilities,
        icon: "🎯",
        name_pt: "Habilidades",
        name_en: "Abilities",
    },
    SectionMeta {
        section: CompendiumSection::MeritsFlaws,
        icon: "✨",
        name_pt: "Qualidades & Defeitos",
        name_en: "Merits & Flaws",
    },
    SectionMeta {
        section: CompendiumSection::Traditions,
        icon: "🏛️",
        name_pt: "Facções da Ascensão",
        name_en: "Ascension Factions",
    },
    SectionMeta {
        section: CompendiumSection::Bestiary,
        icon: "🐺",
        name_pt: "Aliados & Antagonistas",
        name_en: "Allies & Antagonists",
    },
];

/// Número de verbetes de uma seção, calculado a partir dos dados (nunca escrito à mão).
pub fn section_count(section: CompendiumSection) -> usize {
    use crate::compendium as c;
    match section {
        CompendiumSection::Paradigms => c::paradigms::ALL_PARADIGMS.len(),
        CompendiumSection::Practices => c::practices::ALL_PRACTICES.len(),
        CompendiumSection::Instruments => c::instruments::ALL_INSTRUMENTS.len(),
        CompendiumSection::Archetypes => c::archetypes::ALL_ARCHETYPES.len(),
        CompendiumSection::Attributes => c::attributes::ALL_ATTRIBUTES.len(),
        CompendiumSection::Backgrounds => c::backgrounds::ALL_BACKGROUNDS.len(),
        CompendiumSection::Spheres => c::spheres::ALL_SPHERES.len(),
        CompendiumSection::Weapons => c::weapons::ALL_WEAPONS.len() + c::weapons::ALL_COMBAT_MANEUVERS.len(),
        CompendiumSection::Abilities => c::abilities::ALL_ABILITIES.len(),
        CompendiumSection::MeritsFlaws => c::merits_flaws::ALL_MERITS_FLAWS.len() + c::merits_flaws::ALL_DERANGEMENTS.len(),
        CompendiumSection::Traditions => {
            c::traditions::ALL_TRADITIONS.len() + c::traditions::ALL_CONVENTIONS.len() + c::traditions::ALL_DISPARATES.len()
        }
        CompendiumSection::Bestiary => c::bestiary::ALL_BESTIARY.iter().filter(|e| !e.is_article).count(),
    }
}

pub fn get_section_meta(section: CompendiumSection) -> &'static SectionMeta {
    ALL_SECTIONS
        .iter()
        .find(|m| m.section == section)
        .unwrap_or(&ALL_SECTIONS[0])
}

#[component]
pub fn CategorySelector(
    active_section: RwSignal<CompendiumSection>,
    current_lang: Signal<Language>,
    mobile_show_detail: RwSignal<bool>,
) -> impl IntoView {
    let (show_popover, set_show_popover) = create_signal(false);

    let current_meta = Signal::derive(move || {
        get_section_meta(active_section.get())
    });

    let current_index = Signal::derive(move || {
        let cur = active_section.get();
        ALL_SECTIONS.iter().position(|s| s.section == cur).unwrap_or(0)
    });

    let navigate_prev = move |_| {
        let idx = current_index.get();
        let prev_idx = if idx == 0 { ALL_SECTIONS.len() - 1 } else { idx - 1 };
        active_section.set(ALL_SECTIONS[prev_idx].section);
        mobile_show_detail.set(false);
        set_show_popover.set(false);
    };

    let navigate_next = move |_| {
        let idx = current_index.get();
        let next_idx = (idx + 1) % ALL_SECTIONS.len();
        active_section.set(ALL_SECTIONS[next_idx].section);
        mobile_show_detail.set(false);
        set_show_popover.set(false);
    };

    let select_section = move |sec: CompendiumSection| {
        active_section.set(sec);
        mobile_show_detail.set(false);
        set_show_popover.set(false);
    };

    view! {
        <div class="compendium-selector-bar">
            // Backdrop invisível para fechar ao clicar fora
            {move || if show_popover.get() {
                view! {
                    <div 
                        class="compendium-popover-backdrop"
                        on:click=move |_| set_show_popover.set(false)
                    ></div>
                }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}

            <div class="compendium-selector-controls">
                // Botão de navegação anterior
                <button
                    type="button"
                    class="compendium-nav-arrow-btn"
                    on:click=navigate_prev
                    title=move || match current_lang.get() {
                        Language::PtBr => "Seção anterior",
                        Language::EnUs => "Previous section",
                    }
                >
                    "◀"
                </button>

                // Botão Seletor Principal com menu dropdown
                <div class="compendium-selector-dropdown-wrapper">
                    <button
                        type="button"
                        class=move || if show_popover.get() { "compendium-selector-btn active" } else { "compendium-selector-btn" }
                        on:click=move |_| set_show_popover.update(|cur| *cur = !*cur)
                        title=move || match current_lang.get() {
                            Language::PtBr => "Clique para escolher uma seção do Compêndio",
                            Language::EnUs => "Click to choose a Compendium section",
                        }
                    >
                        <span class="selector-icon">{move || current_meta.get().icon}</span>
                        <span class="selector-title">
                            {move || match current_lang.get() {
                                Language::PtBr => current_meta.get().name_pt,
                                Language::EnUs => current_meta.get().name_en,
                            }}
                        </span>
                        <span class="selector-badge">{move || section_count(current_meta.get().section)}</span>
                        <span class="selector-chevron">{move || if show_popover.get() { "▴" } else { "▾" }}</span>
                    </button>

                    // Popover em Grade Rica (3x3)
                    {move || if show_popover.get() {
                        let cur_sec = active_section.get();
                        let lang = current_lang.get();
                        view! {
                            <div class="compendium-category-popover" on:click=move |ev| ev.stop_propagation()>
                                <div class="compendium-category-popover-header">
                                    <span class="popover-heading">
                                        {match lang {
                                            Language::PtBr => "TODAS AS SEÇÕES DO COMPÊNDIO",
                                            Language::EnUs => "ALL COMPENDIUM SECTIONS",
                                        }}
                                    </span>
                                    <span class="popover-counter">
                                        {format!("{} / {}", current_index.get() + 1, ALL_SECTIONS.len())}
                                    </span>
                                </div>
                                <div class="compendium-category-grid">
                                    {ALL_SECTIONS.iter().map(|meta| {
                                        let sec = meta.section;
                                        let is_active = sec == cur_sec;
                                        let icon = meta.icon;
                                        let name = match lang {
                                            Language::PtBr => meta.name_pt,
                                            Language::EnUs => meta.name_en,
                                        };
                                        let count = section_count(meta.section);
                                        view! {
                                            <button
                                                type="button"
                                                class=if is_active { "compendium-category-card active" } else { "compendium-category-card" }
                                                on:click=move |_| select_section(sec)
                                            >
                                                <span class="category-card-icon">{icon}</span>
                                                <div class="category-card-info">
                                                    <span class="category-card-name">{name}</span>
                                                    <span class="category-card-count">
                                                        {match lang {
                                                            Language::PtBr => format!("{} verbetes", count),
                                                            Language::EnUs => format!("{} entries", count),
                                                        }}
                                                    </span>
                                                </div>
                                                {if is_active {
                                                    view! { <span class="category-card-check">"✓"</span> }.into_view()
                                                } else {
                                                    view! { <span></span> }.into_view()
                                                }}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                </div>

                // Botão de navegação próximo
                <button
                    type="button"
                    class="compendium-nav-arrow-btn"
                    on:click=navigate_next
                    title=move || match current_lang.get() {
                        Language::PtBr => "Próxima seção",
                        Language::EnUs => "Next section",
                    }
                >
                    "▶"
                </button>
            </div>

            // Indicador de posição discreto
            <div class="compendium-selector-indicator">
                <span class="indicator-text">
                    {move || format!("{}/{}", current_index.get() + 1, ALL_SECTIONS.len())}
                </span>
            </div>
        </div>
    }
}
