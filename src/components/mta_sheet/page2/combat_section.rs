use leptos::*;
use crate::components::{Callback, StableTextArea, StableTextInput};
use crate::state::{CharacterData, WeaponItem};
use crate::compendium::weapons::{WeaponDefinition, ALL_RULE_NOTES};

fn translate_damage_str(val: &str, lang: crate::i18n::Language) -> String {
    let mut s = val.to_string();
    match lang {
        crate::i18n::Language::EnUs => {
            if s.contains("Força") { s = s.replace("Força", "Str"); }
            if s.contains("força") { s = s.replace("força", "str"); }
            if s.contains("Vontade") { s = s.replace("Vontade", "Willpower"); }
            if s.contains("Especial") { s = s.replace("Especial", "Special"); }
            if s.contains("Nenhum") { s = s.replace("Nenhum", "None"); }
            if s.contains("/C") { s = s.replace("/C", "/B"); }
        }
        crate::i18n::Language::PtBr => {
            if s.contains("Str") { s = s.replace("Str", "Força"); }
            if s.contains("str") { s = s.replace("str", "força"); }
            if s.contains("Willpower") { s = s.replace("Willpower", "Vontade"); }
            if s.contains("Special") { s = s.replace("Special", "Especial"); }
            if s.contains("None") { s = s.replace("None", "Nenhum"); }
            if s.contains("/B") { s = s.replace("/B", "/C"); }
        }
    }
    s
}

#[component]
pub fn CombatSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    // Contexto unificado do Compêndio M20 (isolado no nível raiz da ficha)
    let compendium_ctx = use_context::<crate::components::mta_sheet::page5::PracticeCompendiumContext>();

    // Estado da Legenda Rápida (#1 a #10)
    let (show_quick_legend, set_show_quick_legend) = create_signal(false);

    let add_weapon_row = move |_| {
        set_data.update(|s| {
            s.weapons.push(WeaponItem::default());
        });
    };

    let remove_weapon_row = move |idx: usize| {
        set_data.update(|s| {
            if idx < s.weapons.len() && s.weapons.len() > 4 {
                s.weapons.remove(idx);
            } else if idx < s.weapons.len() {
                s.weapons[idx] = WeaponItem::default();
            }
        });
    };

    let total_rows = Signal::derive(move || data.with(|d| d.weapons.len().max(4)));

    // Cálculo dinâmico das larguras das colunas baseado no comprimento do texto das armas
    let col_widths = Signal::derive(move || {
        data.with(|d| {
            let max_dmg_chars = d.weapons.iter()
                .map(|w| w.damage.trim().chars().count())
                .max()
                .unwrap_or(0);

            let max_range_chars = d.weapons.iter()
                .map(|w| w.range.trim().chars().count())
                .max()
                .unwrap_or(0);

            let max_rate_chars = d.weapons.iter()
                .map(|w| w.rate.trim().chars().count())
                .max()
                .unwrap_or(0);

            // DANO baseline é 13.0%.
            // Header é "DANO" (4 caracteres).
            // Para danos mais longos (como armas brancas "Força +1/L" = 10 chars),
            // escala dinamicamente ~0.8% por caractere excedente, garantindo os 3 a 4 caracteres extras solicitados.
            let dmg_extra = if max_dmg_chars > 4 {
                ((max_dmg_chars - 4) as f32 * 0.8).min(6.5)
            } else {
                0.0
            };
            let damage = 13.0 + dmg_extra;

            let diff = 6.5;
            let act = 3.5;
            let conceal = 9.5;
            let clip = 9.5;

            // Se alcance e cadência forem compactos (ex: armas brancas com "—"), redistribui espaço
            let (range, rate) = if max_range_chars <= 2 && max_rate_chars <= 2 && max_dmg_chars > 6 {
                (10.0, 10.5)
            } else {
                (11.0, 11.5)
            };

            let non_weapon = diff + damage + range + rate + clip + conceal + act;
            let weapon = (100.0 - non_weapon).max(25.0);

            (weapon, diff, damage, range, rate, clip, conceal, act)
        })
    });

    let compendium_ctx_for_rows = compendium_ctx.clone();
    let render_weapon_row = move |idx: usize| {
        let compendium_ctx_for_click = compendium_ctx_for_rows.clone();
        let name_val = Signal::derive(move || {
            let l = lang();
            data.with(|d| {
                if let Some(w) = d.weapons.get(idx) {
                    if let Some(entity) = crate::compendium::weapons::find_combat_entity(&w.name) {
                        return entity.name(l).to_string();
                    }
                    w.name.clone()
                } else {
                    String::new()
                }
            })
        });

        let diff_val = Signal::derive(move || {
            let l = lang();
            data.with(|d| {
                if let Some(w) = d.weapons.get(idx) {
                    if let Some(entity) = crate::compendium::weapons::find_combat_entity(&w.name) {
                        let diff_pt = entity.difficulty(crate::i18n::Language::PtBr);
                        let diff_en = entity.difficulty(crate::i18n::Language::EnUs);
                        let w_diff = w.diff.trim();
                        if w_diff.is_empty() || w_diff == diff_pt || w_diff == diff_en {
                            return entity.difficulty(l);
                        }
                    }
                    w.diff.clone()
                } else {
                    String::new()
                }
            })
        });

        let dmg_val = Signal::derive(move || {
            let l = lang();
            data.with(|d| {
                if let Some(w) = d.weapons.get(idx) {
                    if let Some(entity) = crate::compendium::weapons::find_combat_entity(&w.name) {
                        let dmg_pt = entity.damage(crate::i18n::Language::PtBr);
                        let dmg_en = entity.damage(crate::i18n::Language::EnUs);
                        let w_dmg = w.damage.trim();
                        if w_dmg.is_empty() || w_dmg == dmg_pt || w_dmg == dmg_en {
                            return entity.damage(l).to_string();
                        }
                    }
                    translate_damage_str(&w.damage, l)
                } else {
                    String::new()
                }
            })
        });

        let range_val = Signal::derive(move || {
            let l = lang();
            data.with(|d| {
                if let Some(w) = d.weapons.get(idx) {
                    let r = w.range.trim();
                    if r.eq_ignore_ascii_case("c/c") || r.eq_ignore_ascii_case("close") || r.eq_ignore_ascii_case("corpo a corpo") {
                        return match l {
                            crate::i18n::Language::PtBr => "C/C".to_string(),
                            crate::i18n::Language::EnUs => "Close".to_string(),
                        };
                    }
                    if let Some(entity) = crate::compendium::weapons::find_combat_entity(&w.name) {
                        let canon_pt = entity.range(crate::i18n::Language::PtBr);
                        let canon_en = entity.range(crate::i18n::Language::EnUs);
                        if r.is_empty() || r.eq_ignore_ascii_case(canon_pt) || r.eq_ignore_ascii_case(canon_en) {
                            return entity.range(l).to_string();
                        }
                    }
                    w.range.clone()
                } else {
                    String::new()
                }
            })
        });

        let rate_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.rate.clone()).unwrap_or_default()));
        let clip_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.clip.clone()).unwrap_or_default()));
        let conceal_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.conceal.clone()).unwrap_or_default()));

        let notes_vec = Signal::derive(move || {
            let l = lang();
            data.with(|d| {
                if let Some(w) = d.weapons.get(idx) {
                    if let Some(entity) = crate::compendium::weapons::find_combat_entity(&w.name) {
                        let canon_pt = entity.notes(crate::i18n::Language::PtBr);
                        let canon_en = entity.notes(crate::i18n::Language::EnUs);
                        let w_notes = w.notes.trim();
                        if w_notes.is_empty() 
                            || w_notes.eq_ignore_ascii_case(&canon_pt) 
                            || w_notes.eq_ignore_ascii_case(&canon_en) 
                        {
                            let active_notes = entity.notes(l);
                            if !active_notes.is_empty() {
                                return active_notes.split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                            }
                        }
                    }

                    if !w.notes.trim().is_empty() {
                        return w.notes.split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<_>>();
                    }
                    if !w.name.trim().is_empty() {
                        if let Some(def) = crate::compendium::weapons::find_weapon(&w.name) {
                            return def.notes.iter().map(|&s| s.to_string()).collect();
                        }
                    }
                }
                Vec::new()
            })
        });

        view! {
            <tr>
                <td class="td-weapon-name">
                    <div class="weapon-input-container">
                        <button
                            type="button"
                            class="weapon-picker-btn"
                            title=move || {
                                let cur_name = data.with(|d| d.weapons.get(idx).map(|w| w.name.trim().to_string()).unwrap_or_default());
                                match (cur_name.is_empty(), lang()) {
                                    (true, crate::i18n::Language::PtBr) => "Consultar / Preencher com Arma ou Manobra M20".to_string(),
                                    (true, crate::i18n::Language::EnUs) => "Browse / Pre-fill with M20 Weapon or Maneuver".to_string(),
                                    (false, crate::i18n::Language::PtBr) => format!("Índice Rápido: Ver descrição detalhada de '{}' no Compêndio M20", cur_name),
                                    (false, crate::i18n::Language::EnUs) => format!("Quick Index: View detailed description of '{}' in M20 Compendium", cur_name),
                                }
                            }
                            on:click={
                                let compendium_ctx = compendium_ctx_for_click.clone();
                                move |_| {
                                    let cur_name = data.with(|d| d.weapons.get(idx).map(|w| w.name.clone()).unwrap_or_default());
                                    if let Some(ref ctx) = compendium_ctx {
                                        ctx.open_weapon.call((Some(idx), cur_name));
                                    }
                                }
                            }
                        >
                            "⚔️"
                        </button>
                        <div class="weapon-name-inputs-col">
                            <StableTextInput 
                                class="table-cell-input text-left font-bold"
                                placeholder=Signal::derive(move || format!("{}...", crate::i18n::tr("weapon_header", lang())))
                                value=name_val
                                on_change=Callback::new(move |val| {
                                    set_data.update(|s| {
                                        while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                        s.weapons[idx].name = val;
                                    });
                                })
                            />
                            {move || {
                                let notes = notes_vec.get();
                                if !notes.is_empty() {
                                    view! {
                                        <div class="weapon-notes-row">
                                            {notes.into_iter().map(|n_code| {
                                                let expl = crate::compendium::weapons::explain_note(&n_code, lang());
                                                let tooltip = expl.map(|(title, desc)| format!("{}: {}", title, desc)).unwrap_or_else(|| n_code.clone());
                                                view! {
                                                    <span class="weapon-note-badge" title=tooltip>
                                                        {n_code}
                                                    </span>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }
                            }}
                        </div>
                    </div>
                </td>
                <td class="td-diff">
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || String::new())
                        value=diff_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].diff = val;
                            });
                        })
                    />
                </td>
                <td class="td-damage">
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || String::new())
                        value=dmg_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].damage = val;
                            });
                        })
                    />
                </td>
                <td 
                    class="td-range tooltip-container"
                    title=move || {
                        let r = range_val.get();
                        let r_trim = r.trim().to_lowercase();
                        if r_trim == "c/c" || r_trim == "corpo a corpo" || r_trim == "close" {
                            match lang() {
                                crate::i18n::Language::PtBr => "Corpo a Corpo".to_string(),
                                crate::i18n::Language::EnUs => "Close Combat".to_string(),
                            }
                        } else {
                            String::new()
                        }
                    }
                >
                    <StableTextInput 
                        class="table-cell-input text-center font-bold"
                        placeholder=Signal::derive(move || String::new())
                        value=range_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].range = val;
                            });
                        })
                    />
                    {move || {
                        let r = range_val.get();
                        let r_trim = r.trim().to_lowercase();
                        if r_trim == "c/c" || r_trim == "corpo a corpo" || r_trim == "close" {
                            let label = match lang() {
                                crate::i18n::Language::PtBr => "Corpo a Corpo",
                                crate::i18n::Language::EnUs => "Close Combat",
                            };
                            view! {
                                <span class="tooltip-text">{label}</span>
                            }.into_view()
                        } else {
                            view! { <span></span> }.into_view()
                        }
                    }}
                </td>
                <td class="td-rate">
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || String::new())
                        value=rate_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].rate = val;
                            });
                        })
                    />
                </td>
                <td class="td-clip">
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || String::new())
                        value=clip_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].clip = val;
                            });
                        })
                    />
                </td>
                <td class="td-conceal">
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || String::new())
                        value=conceal_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].conceal = val;
                            });
                        })
                    />
                </td>
                <td class="td-row-action">
                    <button
                        type="button"
                        class="weapon-row-delete-btn"
                        title=move || match lang() {
                            crate::i18n::Language::PtBr => "Limpar ou remover esta linha de arma",
                            crate::i18n::Language::EnUs => "Clear or remove this weapon row",
                        }
                        on:click=move |_| remove_weapon_row(idx)
                    >
                        "✕"
                    </button>
                </td>
            </tr>
        }
    };

    view! {
        <div class="group-box combat-section-box">
            <div class="group-box-header">
                <span class="group-box-title">{move || crate::i18n::tr("combat_title", lang())}</span>
            </div>

            <div class="combat-grid">
                // Tabela de Armas e Ataques
                <div class="weapons-table-column">
                    <div class="weapons-table-header-row">
                        <span class="weapons-table-title">{move || match lang() {
                            crate::i18n::Language::PtBr => "ARMAS & ATAQUES",
                            crate::i18n::Language::EnUs => "WEAPONS & ATTACKS",
                        }}</span>

                        <div class="weapons-table-actions">
                            <button
                                type="button"
                                class="compendium-weapon-header-btn"
                                on:click={
                                    let compendium_ctx = compendium_ctx.clone();
                                    move |_| {
                                        if let Some(ref ctx) = compendium_ctx {
                                            ctx.open_weapon.call((None, String::new()));
                                        }
                                    }
                                }
                                title=move || match lang() {
                                    crate::i18n::Language::PtBr => "Abrir Compêndio M20 de Armamento & Combate (82 Armas)",
                                    crate::i18n::Language::EnUs => "Open M20 Weapons & Combat Compendium (82 Weapons)",
                                }
                            >
                                "⚔️ M20"
                            </button>

                            <button
                                type="button"
                                class=move || if show_quick_legend.get() { "toggle-legend-header-btn active" } else { "toggle-legend-header-btn" }
                                on:click=move |_| set_show_quick_legend.update(|v| *v = !*v)
                                title=move || match lang() {
                                    crate::i18n::Language::PtBr => "Exibir/Ocultar Legenda de Regras e Notas (#1 a #10)",
                                    crate::i18n::Language::EnUs => "Toggle Rule Notes Legend (#1 to #10)",
                                }
                            >
                                "📜 " {move || match lang() {
                                    crate::i18n::Language::PtBr => "Legenda",
                                    crate::i18n::Language::EnUs => "Legend",
                                }}
                            </button>

                            <button
                                type="button"
                                class="add-weapon-header-btn"
                                on:click=add_weapon_row
                                title=move || crate::i18n::tr("add_weapon", lang())
                            >
                                {move || match lang() {
                                    crate::i18n::Language::PtBr => "+ Arma",
                                    crate::i18n::Language::EnUs => "+ Weapon",
                                }}
                            </button>
                        </div>
                    </div>

                    <div class="weapons-table-container">
                        <table class="weapons-table">
                            <colgroup>
                                <col class="col-weapon" style=move || format!("width: {:.1}%;", col_widths.get().0) />
                                <col class="col-diff" style=move || format!("width: {:.1}%;", col_widths.get().1) />
                                <col class="col-damage" style=move || format!("width: {:.1}%;", col_widths.get().2) />
                                <col class="col-range" style=move || format!("width: {:.1}%;", col_widths.get().3) />
                                <col class="col-rate" style=move || format!("width: {:.1}%;", col_widths.get().4) />
                                <col class="col-clip" style=move || format!("width: {:.1}%;", col_widths.get().5) />
                                <col class="col-conceal" style=move || format!("width: {:.1}%;", col_widths.get().6) />
                                <col class="col-act" style=move || format!("width: {:.1}%;", col_widths.get().7) />
                            </colgroup>
                            <thead>
                                <tr>
                                    <th class="th-weapon" title=move || crate::i18n::tr("weapon_header", lang())>{move || crate::i18n::tr("weapon_header", lang()).to_uppercase()}</th>
                                    <th class="th-diff" title=move || match lang() {
                                        crate::i18n::Language::PtBr => "Dificuldade",
                                        crate::i18n::Language::EnUs => "Difficulty",
                                    }>{move || crate::i18n::tr("diff_header", lang()).to_uppercase()}</th>
                                    <th class="th-damage" title=move || match lang() {
                                        crate::i18n::Language::PtBr => "Dano (Tipo: B, L, A)",
                                        crate::i18n::Language::EnUs => "Damage (Type: B, L, A)",
                                    }>{move || crate::i18n::tr("dmg_header", lang()).to_uppercase()}</th>
                                    <th class="th-range" title=move || match lang() {
                                        crate::i18n::Language::PtBr => "Alcance em Metros",
                                        crate::i18n::Language::EnUs => "Range in Yards/Meters",
                                    }>{move || crate::i18n::tr("range_header", lang()).to_uppercase()}</th>
                                    <th class="th-rate" title=move || match lang() {
                                        crate::i18n::Language::PtBr => "Cadência de Tiro por Turno",
                                        crate::i18n::Language::EnUs => "Rate of Fire per Turn",
                                    }>{move || crate::i18n::tr("rate_header", lang()).to_uppercase()}</th>
                                    <th class="th-clip" title=move || match lang() {
                                        crate::i18n::Language::PtBr => "Pente / Capacidade de Munição",
                                        crate::i18n::Language::EnUs => "Clip / Ammunition Capacity",
                                    }>{move || crate::i18n::tr("clip_header", lang()).to_uppercase()}</th>
                                    <th class="th-conceal" title=move || match lang() {
                                        crate::i18n::Language::PtBr => "Ocultabilidade (J=Jaqueta, B=Bolso, S=Sobretudo, N=Nenhum)",
                                        crate::i18n::Language::EnUs => "Concealability (J=Jacket, P=Pocket, T=Trenchcoat, N=None)",
                                    }>{move || crate::i18n::tr("conceal_header", lang()).to_uppercase()}</th>
                                    <th class="th-act"></th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || (0..total_rows.get()).map(|idx| render_weapon_row(idx)).collect_view()}
                            </tbody>
                        </table>
                    </div>

                    // Legenda Rápida (#1 a #10) Embutida
                    {move || if show_quick_legend.get() {
                        view! {
                            <div class="weapons-quick-legend-box">
                                <div class="quick-legend-title-row">
                                    <span class="quick-legend-title">
                                        "📜 " {move || crate::i18n::tr("weapon_legend_title", lang())}
                                    </span>
                                    <span class="quick-legend-ref">"M20, pp. 450-451"</span>
                                </div>
                                <div class="quick-legend-params-row">
                                    <span>
                                        <strong>{move || match lang() { crate::i18n::Language::PtBr => "Tipo:", crate::i18n::Language::EnUs => "Type:" }}</strong>
                                        {move || match lang() {
                                            crate::i18n::Language::PtBr => " B = Contundente • L = Letal • A = Agravado",
                                            crate::i18n::Language::EnUs => " B = Bashing • L = Lethal • A = Aggravated",
                                        }}
                                    </span>
                                    <span>
                                        <strong>{move || match lang() { crate::i18n::Language::PtBr => "Ocult.:", crate::i18n::Language::EnUs => "Conceal:" }}</strong>
                                        {move || match lang() {
                                            crate::i18n::Language::PtBr => " P = Bolso • J = Jaqueta • T = Sobretudo • N = N/A",
                                            crate::i18n::Language::EnUs => " P = Pocket • J = Jacket • T = Trenchcoat • N = N/A",
                                        }}
                                    </span>
                                </div>
                                <div class="quick-legend-grid">
                                    {ALL_RULE_NOTES.iter().map(|n| {
                                        view! {
                                            <div class="quick-legend-item">
                                                <span class="quick-note-badge">{n.code}</span>
                                                <span class="quick-note-text">
                                                    <strong>{move || n.title(lang())}</strong> ": " {move || n.description(lang())}
                                                </span>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                </div>

                // Bloco de Armadura Compactado
                <div class="armor-column">
                    <div class="armor-header">
                        <span class="weapons-table-title">{move || crate::i18n::tr("armor_title", lang()).to_uppercase()}</span>
                    </div>

                    <div class="armor-box-content">
                        <div class="armor-field-row">
                            <label class="armor-label">{move || format!("{}:", crate::i18n::tr("armor_class", lang()))}</label>
                            <StableTextInput 
                                class="armor-input"
                                placeholder=Signal::derive(move || match lang() {
                                    crate::i18n::Language::PtBr => "Ex: Couro".to_string(),
                                    crate::i18n::Language::EnUs => "Ex: Leather".to_string(),
                                })
                                value=Signal::derive(move || data.with(|d| d.armor.class_name.clone()))
                                on_change=Callback::new(move |val| {
                                    set_data.update(|s| s.armor.class_name = val);
                                })
                            />
                        </div>

                        <div class="armor-dual-stats-row">
                            <div class="armor-mini-stat">
                                <label class="armor-mini-label" title=move || crate::i18n::tr("armor_rating", lang())>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "Prot.",
                                        crate::i18n::Language::EnUs => "Rating",
                                    }}
                                </label>
                                <StableTextInput 
                                    class="armor-input text-center"
                                    placeholder="1"
                                    value=Signal::derive(move || data.with(|d| d.armor.rating.clone()))
                                    on_change=Callback::new(move |val| {
                                        set_data.update(|s| s.armor.rating = val);
                                    })
                                />
                            </div>

                            <div class="armor-mini-stat">
                                <label class="armor-mini-label" title=move || crate::i18n::tr("armor_penalty", lang())>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "Pen.",
                                        crate::i18n::Language::EnUs => "Pen.",
                                    }}
                                </label>
                                <StableTextInput 
                                    class="armor-input text-center"
                                    placeholder="0"
                                    value=Signal::derive(move || data.with(|d| d.armor.penalty.clone()))
                                    on_change=Callback::new(move |val| {
                                        set_data.update(|s| s.armor.penalty = val);
                                    })
                                />
                            </div>
                        </div>

                        <div class="armor-desc-row">
                            <label class="armor-label">{move || match lang() {
                                crate::i18n::Language::PtBr => "Descrição:",
                                crate::i18n::Language::EnUs => "Description:",
                            }}</label>
                            <StableTextArea 
                                class="armor-desc-textarea"
                                placeholder=Signal::derive(move || match lang() {
                                    crate::i18n::Language::PtBr => "Detalhes...".to_string(),
                                    crate::i18n::Language::EnUs => "Details...".to_string(),
                                })
                                value=Signal::derive(move || data.with(|d| d.armor.description.clone()))
                                on_change=Callback::new(move |val| {
                                    set_data.update(|s| s.armor.description = val);
                                })
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
