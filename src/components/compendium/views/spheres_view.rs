//! Visualização e Navegação da Seção de Esferas no Compêndio M20.
//!
//! Capítulo 10: O Livro das Esferas (pp. 504-534).

use leptos::*;
use crate::compendium::spheres::{
    find_unabridged_text, ALL_SPHERES, SphereDefinition, SphereRank, SPHERE_THEORY_RULES,
};
use crate::components::compendium::rich_text::RichTextView;
use crate::components::Callback;
use crate::i18n::Language;

fn get_sphere_icon(id: &str) -> &'static str {
    match id {
        "correspondence" => "🌐",
        "entropy" => "🎲",
        "forces" => "⚡",
        "life" => "🧬",
        "matter" => "🧱",
        "mind" => "🧠",
        "prime" => "☀️",
        "spirit" => "👻",
        "time" => "⏳",
        "data" => "💻",
        "dimensional_science" => "🛸",
        "primal_utility" => "💎",
        "wild_talent" => "💥",
        _ => "🔮",
    }
}

fn get_dots_visual(rank: i32) -> &'static str {
    match rank {
        1 => "● ○ ○ ○ ○",
        2 => "● ● ○ ○ ○",
        3 => "● ● ● ○ ○",
        4 => "● ● ● ● ○",
        5 => "● ● ● ● ●",
        _ => "●",
    }
}

#[component]
pub fn SpheresView(
    selected_sphere_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
    #[prop(into, default = None)] on_close: Option<Callback<()>>,
    #[prop(into, default = None)] on_select_sphere: Option<Callback<String>>,
    #[prop(into, default = None)] active_spheres: Option<Signal<Vec<String>>>,
) -> impl IntoView {
    let _ = on_close;
    let (search_filter, set_search_filter) = create_signal(String::new());
    let (show_unabridged, set_show_unabridged) = create_signal(true);
    let (selected_specialties, set_selected_specialties) = create_signal(std::collections::HashSet::<String>::new());

    create_effect(move |_| {
        let _ = selected_sphere_id.get();
        set_selected_specialties.set(std::collections::HashSet::new());
    });

    let filtered_spheres = Signal::derive(move || {
        let q = search_filter.get().trim().to_lowercase();
        if q.is_empty() {
            return ALL_SPHERES.to_vec();
        }
        ALL_SPHERES
            .iter()
            .copied()
            .filter(|s| {
                s.name.to_lowercase().contains(&q)
                    || s.name_pt.to_lowercase().contains(&q)
                    || s.technocracy_equivalent.map(|t| t.to_lowercase().contains(&q)).unwrap_or(false)
                    || s.technocracy_equivalent_pt.map(|t| t.to_lowercase().contains(&q)).unwrap_or(false)
                    || s.mystic_equivalent.map(|m| m.to_lowercase().contains(&q)).unwrap_or(false)
                    || s.mystic_equivalent_pt.map(|m| m.to_lowercase().contains(&q)).unwrap_or(false)
                    || s.specialties.to_lowercase().contains(&q)
                    || s.specialties_pt.to_lowercase().contains(&q)
            })
            .collect::<Vec<_>>()
    });

    let active_sphere = Signal::derive(move || {
        let cur_id = selected_sphere_id.get();
        ALL_SPHERES
            .iter()
            .find(|s| s.id == cur_id)
            .copied()
            .unwrap_or(ALL_SPHERES[0])
    });

    let is_rules_selected = Signal::derive(move || selected_sphere_id.get() == "theory_sphere_rules");

    view! {
        <div
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= COLUNA ESQUERDA: LISTA DE ESFERAS =================
            <div class="practice-sidebar-pane">
                // Campo de Busca
                <div class="practice-search-box" style="margin-bottom: 0.6rem;">
                    <input
                        type="text"
                        class="practice-search-input"
                        placeholder=move || match current_lang.get() {
                            Language::PtBr => "Buscar Esfera (ex: Correspondência, Forças, Mente...)",
                            Language::EnUs => "Search Sphere (e.g. Correspondence, Forces, Mind...)",
                        }
                        prop:value=search_filter
                        on:input=move |ev| set_search_filter.set(event_target_value(&ev))
                    />
                    {move || if !search_filter.get().is_empty() {
                        view! {
                            <button
                                type="button"
                                class="search-clear-btn"
                                on:click=move |_| set_search_filter.set(String::new())
                                title="Limpar busca"
                            >
                                "×"
                            </button>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                </div>

                // Botão especial: Regras Gerais das Esferas (M20, pp. 511-512)
                <div class="special-box-tab-wrap" style="margin-bottom: 0.6rem;">
                    <button
                        type="button"
                        class=move || {
                            if is_rules_selected.get() {
                                "practice-tab-btn special-box-tab active"
                            } else {
                                "practice-tab-btn special-box-tab"
                            }
                        }
                        on:click=move |_| {
                            selected_sphere_id.set("theory_sphere_rules".to_string());
                            if let Some(msd) = mobile_show_detail {
                                msd.set(true);
                            }
                        }
                    >
                        <span class="tab-indicator">"📜"</span>
                        <div class="tab-text-wrap">
                            <span class="tab-name">
                                {move || match current_lang.get() {
                                    Language::PtBr => "Regras Gerais das Esferas",
                                    Language::EnUs => "General Sphere Rules",
                                }}
                            </span>
                            <span class="tab-sub">"M20, pp. 511-512"</span>
                        </div>
                    </button>
                </div>

                <div class="practice-sidebar-title">
                    {move || match current_lang.get() {
                        Language::PtBr => format!("ESFERAS DA MÁGIKA ({})", filtered_spheres.get().len()),
                        Language::EnUs => format!("SPHERES OF MAGICK ({})", filtered_spheres.get().len()),
                    }}
                </div>

                <div class="practice-tab-list">
                    {move || {
                        let list = filtered_spheres.get();
                        let cur_id = selected_sphere_id.get();
                        let is_rule = is_rules_selected.get();
                        let lang = current_lang.get();

                        list.into_iter().map(|sphere: SphereDefinition| {
                            let s_id = sphere.id;
                            let is_active = !is_rule && cur_id == s_id;
                            let icon = get_sphere_icon(s_id);

                            view! {
                                <button
                                    type="button"
                                    class=if is_active { "practice-tab-btn active" } else { "practice-tab-btn" }
                                    on:click=move |_| {
                                        selected_sphere_id.set(s_id.to_string());
                                        if let Some(msd) = mobile_show_detail {
                                            msd.set(true);
                                        }
                                    }
                                >
                                    <span class="tab-indicator">{icon}</span>
                                    <div class="tab-text-wrap">
                                        <div class="tab-name-row" style="display: flex; align-items: center; justify-content: space-between; gap: 0.3rem;">
                                            <span class="tab-name">{sphere.name(lang)}</span>
                                            {if sphere.technocracy_equivalent.is_some() {
                                                view! {
                                                    <span class="compendium-sub-pill" style="font-size: 0.65rem; padding: 0.1rem 0.35rem; border-radius: 4px; background: rgba(59, 130, 246, 0.12); color: #2563eb;">
                                                        "Tecnocracia"
                                                    </span>
                                                }.into_view()
                                            } else if sphere.mystic_equivalent.is_some() {
                                                view! {
                                                    <span class="compendium-sub-pill" style="font-size: 0.65rem; padding: 0.1rem 0.35rem; border-radius: 4px; background: rgba(147, 51, 234, 0.12); color: #7c3aed;">
                                                        "Tecnomagia"
                                                    </span>
                                                }.into_view()
                                            } else if sphere.is_optional_rule {
                                                view! {
                                                    <span class="compendium-sub-pill" style="font-size: 0.65rem; padding: 0.1rem 0.35rem; border-radius: 4px; background: rgba(239, 68, 68, 0.12); color: #dc2626;">
                                                        "Opcional"
                                                    </span>
                                                }.into_view()
                                            } else {
                                                view! { <span></span> }.into_view()
                                            }}
                                        </div>
                                        <span class="tab-sub">{format!("{} • {}", sphere.secondary_name(lang), sphere.page_ref)}</span>
                                    </div>
                                </button>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>

            // ================= COLUNA DIREITA: DETALHES DA ESFERA =================
            <div class="practice-detail-pane">
                // Botão de Retorno no Mobile
                {move || mobile_show_detail.map(|msd| {
                    view! {
                        <button
                            type="button"
                            class="compendium-mobile-back-btn"
                            on:click=move |_| msd.set(false)
                        >
                            <span class="back-arrow">"←"</span>
                            <span>{match current_lang.get() {
                                Language::PtBr => "Voltar para Lista de Esferas",
                                Language::EnUs => "Back to Spheres List",
                            }}</span>
                        </button>
                    }
                })}

                {move || {
                    let lang = current_lang.get();

                    if is_rules_selected.get() {
                        let rule = &SPHERE_THEORY_RULES;
                        view! {
                            <div class="box-reading-view grimoire-reading-view">
                                <div class="grimoire-hero-banner special-box-header">
                                    <div class="grimoire-hero-top">
                                        <div class="grimoire-hero-left">
                                            <div class="grimoire-hero-icon-box">"📜"</div>
                                            <div class="grimoire-hero-titles">
                                                <div class="grimoire-hero-sup">
                                                    <span class="grimoire-badge-cat">"M20 • REGRA CANÔNICA"</span>
                                                </div>
                                                <h2 class="grimoire-hero-title">{rule.title(lang)}</h2>
                                                <div class="grimoire-hero-sub">
                                                    <span class="grimoire-hero-page">"📖 " {rule.page_ref}</span>
                                                    <span class="grimoire-hero-secondary">
                                                        {match lang {
                                                            Language::PtBr => "Livro das Esferas (Metafísica & Regras Gerais)",
                                                            Language::EnUs => "Book of Spheres (Metaphysics & General Rules)",
                                                        }}
                                                    </span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="archetype-mechanics-summary" style="margin-top: 1rem;">
                                    <div class="archetype-mech-item">
                                        <span class="archetype-mech-title">
                                            "★ " {match lang {
                                                Language::PtBr => "Os 5 Postos de Domínio (Ranks 1 a 5):",
                                                Language::EnUs => "The 5 Ranks of Expertise (1 to 5):",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match lang {
                                                Language::PtBr => "1: Percepção (Sentidos Sutis) • 2: Manipulação (Pequenas Alterações) • 3: Controle (Mudanças Drásticas) • 4: Comando (Criações Complexas) • 5: Mestria (Transmutações Totais e Rompimento dos Limites).",
                                                Language::EnUs => "1: Perception (Subtle Senses) • 2: Manipulation (Minor Alterations) • 3: Control (Drastic Changes) • 4: Command (Complex Creations) • 5: Mastery (Total Transmutations & Pushing the Limits).",
                                            }}
                                        </span>
                                    </div>
                                    <div class="archetype-mech-item" style="margin-top: 0.5rem;">
                                        <span class="archetype-mech-title">
                                            "★ " {match lang {
                                                Language::PtBr => "Esferas de Padrão & Primórdio:",
                                                Language::EnUs => "Pattern Spheres & Prime:",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match lang {
                                                Language::PtBr => "Forças, Vida e Matéria regem as formas da Criação. Criar um Padrão do nada exige Primórdio 2+ para alimentar a matéria-prima mística, ou Primórdio 1 para carregar com Quintessência.",
                                                Language::EnUs => "Forces, Life, and Matter govern the forms of Creation. Creating a Pattern from nothing requires Prime 2+ to fuel mystical substance, or Prime 1 to infuse with Quintessence.",
                                            }}
                                        </span>
                                    </div>
                                    <div class="archetype-mech-item" style="margin-top: 0.5rem;">
                                        <span class="archetype-mech-title">
                                            "★ " {match lang {
                                                Language::PtBr => "Efeitos Conjuntos & Travamento de Padrões:",
                                                Language::EnUs => "Conjunctional Effects & Pattern Locking:",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match lang {
                                                Language::PtBr => "Esferas podem ser combinadas para feitiços compostos. Feitiços sustentados que afetam ou alteram seres vivos/estruturas demandam 'travar' o efeito usando Vida ou Matéria correspondente.",
                                                Language::EnUs => "Spheres can be combined for compound spells. Sustained effects affecting or altering living beings/structures require 'locking' the effect using appropriate Life or Matter.",
                                            }}
                                        </span>
                                    </div>
                                    <div class="archetype-mech-item" style="margin-top: 0.5rem;">
                                        <span class="archetype-mech-title">
                                            "★ " {match lang {
                                                Language::PtBr => "Especialidades de Esfera (Arete / Esfera 4+):",
                                                Language::EnUs => "Sphere Specialties (Arete / Sphere 4+):",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match lang {
                                                Language::PtBr => "Ao atingir Esfera 4, o mago ganha uma Especialidade (ex: Tempestades, Cibernética, Necromancia). '10's contam em dobro no teste de Arete ao usar a especialidade. Especialidades extras custam 4 XP.",
                                                Language::EnUs => "Upon reaching Sphere 4, the mage gains a Specialty (e.g. Storms, Cybernetics, Necromancy). '10's count double on Arete rolls using the specialty. Extra specialties cost 4 XP.",
                                            }}
                                        </span>
                                    </div>
                                    <div class="archetype-mech-item" style="margin-top: 0.5rem;">
                                        <span class="archetype-mech-title">
                                            "★ " {match lang {
                                                Language::PtBr => "Esferas Tecnocráticas & Incompatibilidade:",
                                                Language::EnUs => "Technocratic Spheres & Incompatibility:",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match lang {
                                                Language::PtBr => "Dados substitui Correspondência; Utilidade Primordial substitui Primórdio; Ciência Dimensional substitui Espírito. Não é permitido comprar ambas simultaneamente; migrar exige tempo de desaprendizado e ajuste de paradigma.",
                                                Language::EnUs => "Data replaces Correspondence; Primal Utility replaces Prime; Dimensional Science replaces Spirit. Cannot purchase both simultaneously; transitioning requires unlearning and paradigm retraining.",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <div class="practice-description-block" style="margin-top: 1.2rem;">
                                    <RichTextView text=Signal::derive(move || rule.content(lang).to_string()) />
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        let sphere = active_sphere.get();
                        let s_icon = get_sphere_icon(sphere.id);

                        view! {
                            <div class="practice-reading-view grimoire-reading-view">
                                <div class="grimoire-hero-banner">
                                    <div class="grimoire-hero-top">
                                        <div class="grimoire-hero-left">
                                            <div class="grimoire-hero-icon-box">{s_icon}</div>
                                            <div class="grimoire-hero-titles">
                                                <div class="grimoire-hero-sup">
                                                    <span class="grimoire-badge-cat">"M20 • ESFERA DA MÁGIKA"</span>
                                                    {if let Some((label, val)) = sphere.equivalent_note(lang) {
                                                        view! {
                                                            <span class="grimoire-badge-tec">
                                                                {label} ": " {val}
                                                            </span>
                                                        }.into_view()
                                                    } else {
                                                        view! { <span></span> }.into_view()
                                                    }}
                                                    {if let Some(equiv_id) = sphere.technocracy_equivalent.or(sphere.mystic_equivalent) {
                                                        if let Some(target) = crate::compendium::spheres::find_sphere(equiv_id) {
                                                            let t_id = target.id.to_string();
                                                            view! {
                                                                <button
                                                                    type="button"
                                                                    class="compendium-sub-pill compendium-sub-pill-purple"
                                                                    style="cursor: pointer; font-size: 0.72rem;"
                                                                    on:click=move |_| {
                                                                        selected_sphere_id.set(t_id.clone());
                                                                    }
                                                                    title=match lang {
                                                                        Language::PtBr => format!("Alternar visualização para {}", target.name(lang)),
                                                                        Language::EnUs => format!("Switch view to {}", target.name(lang)),
                                                                    }
                                                                >
                                                                    "⇄ " {match lang {
                                                                        Language::PtBr => format!("Ver {}", target.name(lang)),
                                                                        Language::EnUs => format!("View {}", target.name(lang)),
                                                                    }}
                                                                </button>
                                                            }.into_view()
                                                        } else {
                                                            view! { <span></span> }.into_view()
                                                        }
                                                    } else {
                                                        view! { <span></span> }.into_view()
                                                    }}
                                                </div>
                                                <h2 class="grimoire-hero-title">{sphere.name(lang)}</h2>
                                                <div class="grimoire-hero-sub">
                                                    <span class="grimoire-hero-secondary">" (" {sphere.secondary_name(lang)} ")"</span>
                                                    <span class="grimoire-hero-page">"📖 " {sphere.page_ref}</span>
                                                    <span style="font-style: italic; color: #d5c7ab;">"— " {sphere.subtitle(lang)}</span>
                                                </div>
                                            </div>
                                        </div>

                                        // Ações do Cabeçalho: Ativação de Variante na Ficha
                                        {
                                            let act_spheres = active_spheres.clone();
                                            let on_sel_sph = on_select_sphere.clone();
                                            let s_id = sphere.id.to_string();
                                            let has_variant = sphere.technocracy_equivalent.is_some() || sphere.mystic_equivalent.is_some();

                                            move || {
                                                if let Some(ref list_sig) = act_spheres {
                                                    let cur_active_list = list_sig.get();
                                                    let is_active = cur_active_list.iter().any(|id| id == &s_id);

                                                    view! {
                                                        <div class="compendium-header-actions">
                                                            {if is_active {
                                                                view! {
                                                                    <button type="button" class="compendium-action-btn compendium-btn-active" disabled="disabled">
                                                                        <span class="compendium-btn-icon">"✓"</span>
                                                                        <span class="compendium-btn-label">
                                                                            {match lang {
                                                                                Language::PtBr => if has_variant { "Variante Ativa na Ficha" } else { "Esfera Ativa na Ficha" },
                                                                                Language::EnUs => if has_variant { "Active Variant on Sheet" } else { "Active Sphere on Sheet" },
                                                                            }}
                                                                        </span>
                                                                    </button>
                                                                }.into_view()
                                                            } else if let (false, true, Some(cb)) = (is_active, has_variant, on_sel_sph.clone()) {
                                                                let chosen_id = s_id.clone();
                                                                view! {
                                                                    <button
                                                                        type="button"
                                                                        class="compendium-action-btn compendium-btn-purple"
                                                                        on:click=move |_| {
                                                                            cb.call(chosen_id.clone());
                                                                        }
                                                                        title=match lang {
                                                                            Language::PtBr => format!("Ativar {} como a variante em uso na sua ficha", sphere.name(lang)),
                                                                            Language::EnUs => format!("Activate {} as the variant on your character sheet", sphere.name(lang)),
                                                                        }
                                                                    >
                                                                        <span class="compendium-btn-icon">"✦"</span>
                                                                        <span class="compendium-btn-label">
                                                                            {match lang {
                                                                                Language::PtBr => "Ativar esta Variante na Ficha",
                                                                                Language::EnUs => "Activate this Variant on Sheet",
                                                                            }}
                                                                        </span>
                                                                    </button>
                                                                }.into_view()
                                                            } else {
                                                                view! { <span></span> }.into_view()
                                                            }}
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! { <span></span> }.into_view()
                                                }
                                            }
                                        }
                                    </div>
                                </div>

                                // Especializações Canônicas
                                <div class="practice-aliases" style="margin-top: 0.7rem;">
                                    <span class="practice-aliases-label">
                                        {match lang {
                                            Language::PtBr => "Especializações: ",
                                            Language::EnUs => "Specialties: ",
                                        }}
                                    </span>
                                    <span class="practice-aliases-val">
                                        {sphere.specialties(lang)}
                                    </span>
                                </div>

                            // Alternância entre Modo Texto Integral M20 e Resumo dos Postos
                            <div class="compendium-view-mode-toggle" style="margin-top: 1rem;">
                                <button
                                    type="button"
                                    class=move || if show_unabridged.get() { "view-mode-btn active" } else { "view-mode-btn" }
                                    on:click=move |_| set_show_unabridged.set(true)
                                >
                                    "📖 " {match lang {
                                        Language::PtBr => "Texto Integral M20",
                                        Language::EnUs => "Unabridged M20",
                                    }}
                                </button>
                                <button
                                    type="button"
                                    class=move || if !show_unabridged.get() { "view-mode-btn active" } else { "view-mode-btn" }
                                    on:click=move |_| set_show_unabridged.set(false)
                                >
                                    "⚡ " {match lang {
                                        Language::PtBr => "Resumo dos Postos",
                                        Language::EnUs => "Ranks Summary",
                                    }}
                                </button>
                            </div>

                            {move || {
                                if show_unabridged.get() {
                                    if let Some(unabridged_text) = find_unabridged_text(sphere.id, lang) {
                                        let text_sig = Signal::derive(move || unabridged_text.to_string());
                                        return view! {
                                            <div class="practice-description-block" style="margin-top: 0.8rem;">
                                                <RichTextView text=text_sig />
                                            </div>
                                        }.into_view();
                                    }
                                }

                                // Modo Resumo / Postos
                                view! {
                                    <div class="compendium-quick-summary-view" style="margin-top: 1rem;">
                                        <div class="practice-summary-box" style="padding: 1rem; border-radius: 8px; background: var(--surface-card, #f8fafc); border: 1px solid var(--border-color, #e2e8f0); margin-bottom: 1.2rem;">
                                            <h4 style="margin: 0 0 0.5rem 0; font-size: 0.95rem; color: var(--text-primary, #1e293b);">
                                                {match lang {
                                                    Language::PtBr => "Visão Geral",
                                                    Language::EnUs => "Overview",
                                                }}
                                            </h4>
                                            <p style="margin: 0; font-size: 0.9rem; line-height: 1.5; color: var(--text-secondary, #475569);">
                                                {sphere.description(lang)}
                                            </p>
                                        </div>

                                        <div class="compendium-ratings-header" style="font-weight: 700; font-size: 0.95rem; color: var(--text-primary, #1e293b); margin-bottom: 0.8rem; display: flex; align-items: center; gap: 0.4rem;">
                                            "📊 " {match lang {
                                                Language::PtBr => "Graduações & Postos (1 a 5)",
                                                Language::EnUs => "Sphere Ranks (1 to 5)",
                                            }}
                                        </div>

                                        <div class="compendium-ratings-list" style="display: flex; flex-direction: column; gap: 0.75rem;">
                                            {sphere.ranks.iter().map(|r: &SphereRank| {
                                                let dots_vis = get_dots_visual(r.rank);
                                                view! {
                                                    <div class="compendium-rating-card" style="padding: 0.85rem 1rem; border-radius: 8px; border: 1px solid var(--border-color, #e2e8f0); background: var(--surface-card, #fff);">
                                                        <div class="rating-card-top" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.4rem;">
                                                            <span class="rating-dots" style="font-weight: 700; font-size: 0.9rem; color: #4f46e5; letter-spacing: 0.05em;">
                                                                {dots_vis} " (" {r.rank} ")"
                                                            </span>
                                                            <span class="rating-title" style="font-weight: 600; font-size: 0.88rem; color: var(--text-primary, #1e293b);">
                                                                {r.name(lang)}
                                                            </span>
                                                        </div>
                                                        <div class="rating-card-desc" style="font-size: 0.85rem; line-height: 1.45; color: var(--text-secondary, #475569);">
                                                            {r.description(lang)}
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                        <div class="attribute-specialties-section" style="margin-top: 1.5rem;">
                                            <h4 class="compendium-section-title">
                                                "🏷️ " {match lang {
                                                    Language::PtBr => format!("Especialidades Canônicas da Esfera ({})", sphere.page_ref),
                                                    Language::EnUs => format!("Canonical Sphere Specialties ({})", sphere.page_ref),
                                                }}
                                            </h4>
                                            <p class="attribute-specialties-hint">
                                                {match lang {
                                                    Language::PtBr => "Ao atingir 4 ou mais pontos nesta Esfera (ou Arete 4+), o mago ganha uma Especialidade. Em rolagens de Arete utilizando a Especialidade, cada resultado 10 conta como 2 sucessos. Especialidades adicionais podem ser adquiridas por 4 pontos de XP.",
                                                    Language::EnUs => "Upon reaching 4 or more dots in this Sphere (or Arete 4+), the mage gains a Specialty. On Arete rolls employing the Specialty, every 10 rolled counts as 2 successes. Additional specialties may be purchased for 4 XP.",
                                                }}
                                            </p>
                                            <div class="compendium-specialty-pills">
                                                {sphere.suggested_specialties(lang).into_iter().map(|spec| {
                                                    let is_selected = move || selected_specialties.with(|s| s.contains(spec));
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class="compendium-specialty-pill"
                                                            class:active=is_selected
                                                            on:click=move |_| {
                                                                set_selected_specialties.update(|set| {
                                                                    if set.contains(spec) {
                                                                        set.remove(spec);
                                                                    } else {
                                                                        set.insert(spec.to_string());
                                                                    }
                                                                });
                                                            }
                                                            title=match lang {
                                                                Language::PtBr => "Clique para selecionar/desmarcar especialidade",
                                                                Language::EnUs => "Click to select/deselect specialty",
                                                            }
                                                        >
                                                            {move || if is_selected() { "✓ " } else { "✦ " }}
                                                            {spec}
                                                        </button>
                                                    }
                                                }).collect_view()}
                                            </div>

                                            {move || {
                                                let sel = selected_specialties.get();
                                                if !sel.is_empty() {
                                                    let specs_str = sel.into_iter().collect::<Vec<_>>().join(", ");
                                                    view! {
                                                        <div class="compendium-specialty-selection-box" style="margin-top: 0.8rem; padding: 0.6rem 0.8rem; background: rgba(99, 102, 241, 0.08); border: 1px solid #6366f1; border-radius: 6px; display: flex; align-items: center; justify-content: space-between; gap: 0.5rem;">
                                                            <div style="font-size: 0.85rem; color: #1e293b;">
                                                                <strong>{match lang { Language::PtBr => "Selecionadas: ", Language::EnUs => "Selected: " }}</strong>
                                                                {specs_str.clone()}
                                                            </div>
                                                            <button
                                                                type="button"
                                                                class="panel-reset-btn"
                                                                style="border-color: #6366f1; color: #4338ca; font-weight: 700; white-space: nowrap;"
                                                                on:click=move |_| set_selected_specialties.set(std::collections::HashSet::new())
                                                            >
                                                                {match lang { Language::PtBr => "Limpar", Language::EnUs => "Clear" }}
                                                            </button>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! { <span></span> }.into_view()
                                                }
                                            }}
                                        </div>
                                    </div>
                            }.into_view()
                            }}
                        </div>
                    }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
