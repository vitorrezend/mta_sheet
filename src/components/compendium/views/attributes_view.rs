use leptos::*;
use crate::compendium::attributes::{
    ALL_ATTRIBUTES, AttributeDefinition, SPECIALTIES_RULE,
};
use crate::i18n::Language;

#[component]
pub fn AttributesView(
    selected_attribute_id: RwSignal<String>,
    current_lang: Signal<Language>,
) -> impl IntoView {
    let is_attribute_rule = Signal::derive(move || selected_attribute_id.get() == "rule_specialties");
    let active_attribute = Signal::derive(move || {
        let cur_id = selected_attribute_id.get();
        ALL_ATTRIBUTES
            .iter()
            .find(|a| a.id == cur_id)
            .unwrap_or(&ALL_ATTRIBUTES[0])
    });

    view! {
        <div class="compendium-section-split">
            // ================= ATRIBUTOS: Coluna Esquerda (Abas) =================
            <div class="practice-sidebar-pane">
                // Botão especial: Regra de Especialidades (M20, p. 273)
                <div class="special-box-tab-wrap" style="margin-bottom: 0.6rem;">
                    <button
                        type="button"
                        class=move || {
                            if is_attribute_rule.get() {
                                "practice-tab-btn special-box-tab active"
                            } else {
                                "practice-tab-btn special-box-tab"
                            }
                        }
                        on:click=move |_| selected_attribute_id.set("rule_specialties".to_string())
                    >
                        <span class="tab-indicator">"📜"</span>
                        <div class="tab-text-wrap">
                            <span class="tab-name">
                                {move || match current_lang.get() {
                                    Language::PtBr => "Regra: Especialidades",
                                    Language::EnUs => "Rule: Specialties",
                                }}
                            </span>
                            <span class="tab-sub">"M20, p. 273"</span>
                        </div>
                    </button>
                </div>

                // Físicos (Physical)
                <div class="practice-sidebar-title">
                    {move || match current_lang.get() {
                        Language::PtBr => "ATRIBUTOS FÍSICOS (M20, pp. 273-274)",
                        Language::EnUs => "PHYSICAL ATTRIBUTES (M20, pp. 273-274)",
                    }}
                </div>
                <div class="practice-tab-list">
                    {ALL_ATTRIBUTES[0..3].iter().map(|attr: &'static AttributeDefinition| {
                        let a_id = attr.id;
                        let a_page = attr.page_ref;
                        let icon = match attr.id {
                            "strength" => "💪",
                            "dexterity" => "🤸",
                            "stamina" => "🛡️",
                            _ => "✦",
                        };
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    if !is_attribute_rule.get() && selected_attribute_id.get() == a_id {
                                        "practice-tab-btn active"
                                    } else {
                                        "practice-tab-btn"
                                    }
                                }
                                on:click=move |_| selected_attribute_id.set(a_id.to_string())
                            >
                                <span class="tab-indicator">{icon}</span>
                                <div class="tab-text-wrap">
                                    <span class="tab-name">{move || attr.name(current_lang.get())}</span>
                                    <span class="tab-sub">{format!("{} • {}", attr.secondary_name(current_lang.get()), a_page)}</span>
                                </div>
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Sociais (Social)
                <div class="practice-sidebar-title" style="margin-top: 0.6rem;">
                    {move || match current_lang.get() {
                        Language::PtBr => "ATRIBUTOS SOCIAIS (M20, p. 274)",
                        Language::EnUs => "SOCIAL ATTRIBUTES (M20, p. 274)",
                    }}
                </div>
                <div class="practice-tab-list">
                    {ALL_ATTRIBUTES[3..6].iter().map(|attr: &'static AttributeDefinition| {
                        let a_id = attr.id;
                        let a_page = attr.page_ref;
                        let icon = match attr.id {
                            "charisma" => "✨",
                            "manipulation" => "🎭",
                            "appearance" => "💎",
                            _ => "✦",
                        };
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    if !is_attribute_rule.get() && selected_attribute_id.get() == a_id {
                                        "practice-tab-btn active"
                                    } else {
                                        "practice-tab-btn"
                                    }
                                }
                                on:click=move |_| selected_attribute_id.set(a_id.to_string())
                            >
                                <span class="tab-indicator">{icon}</span>
                                <div class="tab-text-wrap">
                                    <span class="tab-name">{move || attr.name(current_lang.get())}</span>
                                    <span class="tab-sub">{format!("{} • {}", attr.secondary_name(current_lang.get()), a_page)}</span>
                                </div>
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Mentais (Mental)
                <div class="practice-sidebar-title" style="margin-top: 0.6rem;">
                    {move || match current_lang.get() {
                        Language::PtBr => "ATRIBUTOS MENTAIS (M20, pp. 274-275)",
                        Language::EnUs => "MENTAL ATTRIBUTES (M20, pp. 274-275)",
                    }}
                </div>
                <div class="practice-tab-list">
                    {ALL_ATTRIBUTES[6..9].iter().map(|attr: &'static AttributeDefinition| {
                        let a_id = attr.id;
                        let a_page = attr.page_ref;
                        let icon = match attr.id {
                            "perception" => "👁️",
                            "intelligence" => "🧠",
                            "wits" => "⚡",
                            _ => "✦",
                        };
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    if !is_attribute_rule.get() && selected_attribute_id.get() == a_id {
                                        "practice-tab-btn active"
                                    } else {
                                        "practice-tab-btn"
                                    }
                                }
                                on:click=move |_| selected_attribute_id.set(a_id.to_string())
                            >
                                <span class="tab-indicator">{icon}</span>
                                <div class="tab-text-wrap">
                                    <span class="tab-name">{move || attr.name(current_lang.get())}</span>
                                    <span class="tab-sub">{format!("{} • {}", attr.secondary_name(current_lang.get()), a_page)}</span>
                                </div>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>

            // ================= ATRIBUTOS: Coluna Direita (Conteúdo) =================
            <div class="practice-detail-pane">
                {move || {
                    if is_attribute_rule.get() {
                        let rule = &SPECIALTIES_RULE;
                        view! {
                            <div class="box-reading-view">
                                <div class="practice-detail-header special-box-header">
                                    <div class="practice-title-group">
                                        <div class="practice-main-name">
                                            "📜 " {rule.title(current_lang.get())}
                                        </div>
                                        <span class="practice-page-badge">
                                            "📖 " {rule.page_ref}
                                        </span>
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {match current_lang.get() {
                                                Language::PtBr => "Origem: ",
                                                Language::EnUs => "Source: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">
                                            {match current_lang.get() {
                                                Language::PtBr => "M20, p. 273 • Capítulo 6: Criação do Personagem (Atributos & Especialidades)",
                                                Language::EnUs => "M20, p. 273 • Chapter 6: Creating the Character (Attributes & Specialties)",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <div class="archetype-mechanics-summary" style="margin-top: 1rem;">
                                    <div class="archetype-mech-item">
                                        <span class="archetype-mech-title">
                                            "★ " {match current_lang.get() {
                                                Language::PtBr => "Regra Essencial dos 4+ Pontos:",
                                                Language::EnUs => "Core 4+ Dots Rule:",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match current_lang.get() {
                                                Language::PtBr => "Ao atingir 4+ pontos em um Atributo ou Habilidade, o jogador pode escolher uma especialidade. Em jogadas relacionadas à especialidade, cada resultado 10 nos dados conta como DOIS sucessos.",
                                                Language::EnUs => "When a character has 4+ dots in an Attribute or Ability Trait, the player may pick a specialty. On rolls related to that specialty, every 10 rolled counts as TWO successes.",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <div class="practice-description-block" style="margin-top: 1.2rem;">
                                    {
                                        let content = rule.content(current_lang.get());
                                        content.split("\n\n").map(|paragraph| {
                                            view! {
                                                <p class="practice-desc-para">{paragraph.to_string()}</p>
                                            }
                                        }).collect_view()
                                    }
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        let attr = active_attribute.get();
                        let attr_icon = match attr.id {
                            "strength" => "💪",
                            "dexterity" => "🤸",
                            "stamina" => "🛡️",
                            "charisma" => "✨",
                            "manipulation" => "🎭",
                            "appearance" => "💎",
                            "perception" => "👁️",
                            "intelligence" => "🧠",
                            "wits" => "⚡",
                            _ => "✦",
                        };
                        let category_class = format!("badge-{}", attr.category);

                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-main-name">
                                            {attr_icon} " " {attr.name(current_lang.get())}
                                            <span class="practice-title-secondary">
                                                " (" {attr.secondary_name(current_lang.get())} ")"
                                            </span>
                                        </div>
                                        <div class="header-badges-row">
                                            <span class=format!("attribute-cat-badge {}", category_class)>
                                                {attr.category_name(current_lang.get())}
                                            </span>
                                            <span class="practice-page-badge">
                                                "📖 " {attr.page_ref}
                                            </span>
                                        </div>
                                    </div>
                                </div>

                                <div class="practice-description-block" style="margin-top: 1rem;">
                                    <p class="practice-desc-para">{attr.description(current_lang.get())}</p>
                                </div>

                                <div class="attribute-ratings-section" style="margin-top: 1.2rem;">
                                    <h4 class="compendium-section-title">
                                        "📊 " {match current_lang.get() {
                                            Language::PtBr => "Escala de Pontuação (1 a 5 Pontos)",
                                            Language::EnUs => "Rating Scale (1 to 5 Dots)",
                                        }}
                                    </h4>
                                    <div class="attribute-ratings-list">
                                        {attr.ratings.iter().map(|rating| {
                                            let is_unlocked = rating.dots >= 4;
                                            let dots_visual = match rating.dots {
                                                1 => "● ○ ○ ○ ○",
                                                2 => "● ● ○ ○ ○",
                                                3 => "● ● ● ○ ○",
                                                4 => "● ● ● ● ○",
                                                5 => "● ● ● ● ●",
                                                _ => "●",
                                            };
                                            view! {
                                                <div 
                                                    class="attribute-rating-card"
                                                    class:rating-unlocked=is_unlocked
                                                >
                                                    <div class="rating-card-header">
                                                        <span class="rating-dots-indicator">{dots_visual}</span>
                                                        <span class="rating-label-title">{rating.title(current_lang.get())}</span>
                                                        {if is_unlocked {
                                                            view! {
                                                                <span class="rating-unlocked-tag">
                                                                    "★ " {match current_lang.get() {
                                                                        Language::PtBr => "Especialidade Desbloqueada",
                                                                        Language::EnUs => "Specialty Unlocked",
                                                                    }}
                                                                </span>
                                                            }.into_view()
                                                        } else {
                                                            view! { <span></span> }.into_view()
                                                        }}
                                                    </div>
                                                    <div class="rating-desc-text">
                                                        {rating.description(current_lang.get())}
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>

                                <div class="attribute-specialties-section" style="margin-top: 1.5rem;">
                                    <h4 class="compendium-section-title">
                                        "🏷️ " {match current_lang.get() {
                                            Language::PtBr => "Especialidades Sugeridas (M20, p. 273)",
                                            Language::EnUs => "Suggested Specialties (M20, p. 273)",
                                        }}
                                    </h4>
                                    <p class="attribute-specialties-hint">
                                        {match current_lang.get() {
                                            Language::PtBr => "Ao atingir 4 ou mais pontos neste atributo, você pode escolher uma ou mais especialidades abaixo no campo de texto entre o nome e as esferas. Em rolagens onde a especialidade se aplica, cada dado com resultado 10 conta como 2 sucessos.",
                                            Language::EnUs => "Upon reaching 4 or more dots in this Attribute, you may choose one or more specialties below in the label field between the name and the dots. In rolls where the specialty applies, every 10 rolled counts as 2 successes.",
                                        }}
                                    </p>
                                    <div class="compendium-specialty-pills">
                                        {attr.specialties(current_lang.get()).iter().map(|spec| {
                                            view! {
                                                <span class="compendium-specialty-pill">
                                                    "✦ " {*spec}
                                                </span>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
