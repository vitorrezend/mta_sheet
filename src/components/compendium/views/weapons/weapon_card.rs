use leptos::*;
use crate::compendium::weapons::{explain_weapon_note, WeaponClass, WeaponDefinition};
use crate::components::{Callback, StatBox};
use crate::i18n::Language;

/// Renderiza o painel de leitura detalhada de uma Arma Selecionada
pub fn render_weapon_card(
    weapon: &'static WeaponDefinition,
    current_lang: Signal<Language>,
    target_slot: Option<Signal<Option<usize>>>,
    on_select_act: Option<Callback<(Option<usize>, &'static WeaponDefinition)>>,
    on_close_act: Option<Callback<()>>,
) -> impl IntoView {
    let w_cls = weapon.category.class();
    let slot_idx = target_slot.and_then(|s| s.get());
    let on_sel_header = on_select_act.clone();
    let on_sel_footer = on_select_act;
    let on_cls_header = on_close_act.clone();
    let on_cls_footer = on_close_act;

    view! {
        <div class="weapon-reading-view">
            // Cabeçalho da Arma
            <div class="weapon-detail-header">
                <div class="weapon-title-box">
                    <div class="weapon-main-name">
                        <span class="weapon-detail-icon">{weapon.category.icon()}</span>
                        {move || weapon.name(current_lang.get())}
                    </div>
                    <span class="weapon-alt-name">
                        {move || match current_lang.get() {
                            Language::PtBr => format!("Original em Inglês: {}", weapon.name),
                            Language::EnUs => format!("Nome em Português: {}", weapon.name_pt),
                        }}
                    </span>
                </div>
                <div class="weapon-header-actions-wrap" style="display: flex; align-items: center; gap: 0.65rem; margin-left: auto; flex-wrap: wrap;">
                    <div class="weapon-header-badges">
                        <span class="badge-class">{move || w_cls.name(current_lang.get())}</span>
                        <span class="badge-category">{move || weapon.category.name(current_lang.get())}</span>
                        <span class="badge-page">{weapon.page_ref}</span>
                    </div>
                    {if let Some(cb) = on_sel_header {
                        let w_to_equip = weapon;
                        let on_close_action = on_cls_header;
                        view! {
                            <div class="compendium-header-actions">
                                <button
                                    type="button"
                                    class="compendium-action-btn compendium-btn-emerald"
                                    on:click=move |_| {
                                        cb.call((slot_idx, w_to_equip));
                                        if let Some(c) = &on_close_action {
                                            c.call(());
                                        }
                                    }
                                >
                                    <span class="compendium-btn-icon">"⚔️"</span>
                                    <span class="compendium-btn-label">
                                        {move || match (slot_idx, current_lang.get()) {
                                            (Some(idx), Language::PtBr) => format!("Equipar (#{})", idx + 1),
                                            (Some(idx), Language::EnUs) => format!("Equip (#{})", idx + 1),
                                            (None, Language::PtBr) => "Equipar na Ficha".to_string(),
                                            (None, Language::EnUs) => "Equip on Sheet".to_string(),
                                        }}
                                    </span>
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                </div>
            </div>

            // Banner de Estatísticas da Arma (adaptativo para corpo a corpo ou à distância)
            {
                let is_melee = w_cls == WeaponClass::Melee || (weapon.range == "—" && weapon.rate == "—" && weapon.clip == "—");
                if is_melee {
                    view! {
                        <div class="weapon-stats-banner is-melee">
                            <StatBox
                                label=Signal::derive(move || crate::i18n::tr("diff_header", current_lang.get()).to_uppercase())
                                value=weapon.difficulty.to_string()
                                sub=Signal::derive(move || match current_lang.get() {
                                    Language::PtBr => "Dificuldade".to_string(),
                                    Language::EnUs => "Difficulty".to_string(),
                                })
                                color_class="stat-diff"
                            />

                            <StatBox
                                label=Signal::derive(move || crate::i18n::tr("dmg_header", current_lang.get()).to_uppercase())
                                value=Signal::derive(move || weapon.damage(current_lang.get()).to_string())
                                sub=Signal::derive(move || match current_lang.get() {
                                    Language::PtBr => "Dano / Efeito".to_string(),
                                    Language::EnUs => "Damage / Effect".to_string(),
                                })
                                color_class="stat-dmg"
                                is_damage=true
                            />

                            <StatBox
                                label=Signal::derive(move || crate::i18n::tr("conceal_header", current_lang.get()).to_uppercase())
                                value=weapon.conceal.to_string()
                                sub=Signal::derive(move || match (weapon.conceal, current_lang.get()) {
                                    ("P", Language::PtBr) => "P (Bolso)".to_string(),
                                    ("P", Language::EnUs) => "P (Pocket)".to_string(),
                                    ("J", Language::PtBr) => "J (Jaqueta)".to_string(),
                                    ("J", Language::EnUs) => "J (Jacket)".to_string(),
                                    ("T", Language::PtBr) => "T (Sobretudo)".to_string(),
                                    ("T", Language::EnUs) => "T (Trenchcoat)".to_string(),
                                    _ => "N (Não Ocultável)".to_string(),
                                })
                                color_class="stat-conceal"
                            />
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="weapon-stats-banner">
                            <StatBox
                                label=Signal::derive(move || crate::i18n::tr("diff_header", current_lang.get()).to_uppercase())
                                value=weapon.difficulty.to_string()
                                sub=Signal::derive(move || match current_lang.get() {
                                    Language::PtBr => "Dificuldade".to_string(),
                                    Language::EnUs => "Difficulty".to_string(),
                                })
                                color_class="stat-diff"
                            />

                            <StatBox
                                label=Signal::derive(move || crate::i18n::tr("dmg_header", current_lang.get()).to_uppercase())
                                value=Signal::derive(move || weapon.damage(current_lang.get()).to_string())
                                sub=Signal::derive(move || match current_lang.get() {
                                    Language::PtBr => "Dano".to_string(),
                                    Language::EnUs => "Damage".to_string(),
                                })
                                color_class="stat-dmg"
                                is_damage=true
                            />

                            <StatBox
                                label=Signal::derive(move || crate::i18n::tr("range_header", current_lang.get()).to_uppercase())
                                value=weapon.range.to_string()
                                sub=Signal::derive(move || match current_lang.get() {
                                    Language::PtBr => "Alcance (jardas)".to_string(),
                                    Language::EnUs => "Range (yards)".to_string(),
                                })
                                color_class="stat-range"
                            />

                            <StatBox
                                label=Signal::derive(move || crate::i18n::tr("rate_header", current_lang.get()).to_uppercase())
                                value=weapon.rate.to_string()
                                sub=Signal::derive(move || match current_lang.get() {
                                    Language::PtBr => "Cadência / Turno".to_string(),
                                    Language::EnUs => "Rate of Fire".to_string(),
                                })
                                color_class="stat-rate"
                            />

                            <StatBox
                                label=Signal::derive(move || crate::i18n::tr("clip_header", current_lang.get()).to_uppercase())
                                value=weapon.clip.to_string()
                                sub=Signal::derive(move || match current_lang.get() {
                                    Language::PtBr => "Pente / Munição".to_string(),
                                    Language::EnUs => "Clip Capacity".to_string(),
                                })
                                color_class="stat-clip"
                            />

                            <StatBox
                                label=Signal::derive(move || crate::i18n::tr("conceal_header", current_lang.get()).to_uppercase())
                                value=weapon.conceal.to_string()
                                sub=Signal::derive(move || match (weapon.conceal, current_lang.get()) {
                                    ("P", Language::PtBr) => "Bolso".to_string(),
                                    ("P", Language::EnUs) => "Pocket".to_string(),
                                    ("J", Language::PtBr) => "Jaqueta".to_string(),
                                    ("J", Language::EnUs) => "Jacket".to_string(),
                                    ("T", Language::PtBr) => "Sobretudo".to_string(),
                                    ("T", Language::EnUs) => "Trenchcoat".to_string(),
                                    _ => "Não Ocultável".to_string(),
                                })
                                color_class="stat-conceal"
                            />
                        </div>
                    }.into_view()
                }
            }

            // Seção de Notas de Regras Especiais da Arma
            {if !weapon.notes.is_empty() {
                view! {
                    <div class="weapon-rules-section">
                        <h4 class="weapon-section-title">
                            {move || crate::i18n::tr("weapon_notes_header", current_lang.get())}
                        </h4>
                        <div class="weapon-notes-cards">
                            {weapon.notes.iter().map(|&note_code| {
                                if note_code == "Used as pair" {
                                    view! {
                                        <div class="weapon-rule-card">
                                            <div class="rule-card-header">
                                                <span class="rule-badge">"⚔️ Pair"</span>
                                                <strong class="rule-card-title">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => "Empunhadura em Pares",
                                                        Language::EnUs => "Used as Pair",
                                                    }}
                                                </strong>
                                            </div>
                                            <p class="rule-card-desc">
                                                {move || match current_lang.get() {
                                                    Language::PtBr => "Geralmente empunhadas simultaneamente em ambas as mãos como um par coordenado de lâminas.",
                                                    Language::EnUs => "Commonly employed simultaneously as a coordinated pair of weapons.",
                                                }}
                                            </p>
                                        </div>
                                    }.into_view()
                                } else if let Some((title, desc)) = explain_weapon_note(note_code, current_lang.get(), w_cls) {
                                    view! {
                                        <div class="weapon-rule-card">
                                            <div class="rule-card-header">
                                                <span class="rule-badge">{note_code}</span>
                                                <strong class="rule-card-title">{title}</strong>
                                            </div>
                                            <p class="rule-card-desc">{desc}</p>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="weapon-rule-card">
                                            <span class="rule-badge">{note_code}</span>
                                        </div>
                                    }.into_view()
                                }
                            }).collect_view()}
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="weapon-no-special-rules">
                        <span>"✓"</span>
                        <p>{move || match current_lang.get() {
                            Language::PtBr => "Arma padrão sem penalidades ou requisitos especiais de disparo ou empunhadura.",
                            Language::EnUs => "Standard weapon with no special penalties or wielding requirements.",
                        }}</p>
                    </div>
                }.into_view()
            }}

            // Descrição Canônica e Emprego Tático
            <div class="weapon-desc-section">
                <h4 class="weapon-section-title">
                    {move || match current_lang.get() {
                        Language::PtBr => "DESCRIÇÃO & EMPREGO TÁTICO",
                        Language::EnUs => "DESCRIPTION & TACTICAL USE",
                    }}
                </h4>
                <p class="weapon-desc-text">
                    {move || weapon.description(current_lang.get())}
                </p>
            </div>

            // Aliases / Outros Nomes Conhecidos
            {if weapon.aliases.len() > 1 {
                view! {
                    <div class="weapon-aliases-row">
                        <span class="aliases-label">
                            {move || match current_lang.get() {
                                Language::PtBr => "Outros nomes comuns: ",
                                Language::EnUs => "Common aliases: ",
                            }}
                        </span>
                        <span class="aliases-text">{weapon.aliases.join(" • ")}</span>
                    </div>
                }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}

            // Botão de Ação: Equipar no Slot de Combate da Ficha
            {if let Some(cb) = on_sel_footer {
                let w_to_equip = weapon;
                let on_close_action = on_cls_footer;
                view! {
                    <div class="compendium-footer-actions">
                        <button
                            type="button"
                            class="compendium-action-btn compendium-btn-emerald compendium-btn-large"
                            on:click=move |_| {
                                cb.call((slot_idx, w_to_equip));
                                if let Some(c) = &on_close_action {
                                    c.call(());
                                }
                            }
                        >
                            <span class="compendium-btn-icon">"⚔️"</span>
                            <span class="compendium-btn-label">
                                {move || match (slot_idx, current_lang.get()) {
                                    (Some(idx), Language::PtBr) => format!("Equipar '{}' na Linha #{}", w_to_equip.name(current_lang.get()), idx + 1),
                                    (Some(idx), Language::EnUs) => format!("Equip '{}' in Row #{}", w_to_equip.name(current_lang.get()), idx + 1),
                                    (None, Language::PtBr) => format!("Equipar '{}' na Tabela de Combate", w_to_equip.name(current_lang.get())),
                                    (None, Language::EnUs) => format!("Equip '{}' to Combat Table", w_to_equip.name(current_lang.get())),
                                }}
                            </span>
                        </button>
                    </div>
                }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}
        </div>
    }
}
