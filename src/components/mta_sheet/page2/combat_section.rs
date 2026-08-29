use leptos::*;
use crate::components::{Callback, StableTextArea, StableTextInput};
use crate::state::{CharacterData, WeaponItem};

#[component]
pub fn CombatSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let render_weapon_row = move |idx: usize| {
        let name_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.name.clone()).unwrap_or_default()));
        let diff_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.diff.clone()).unwrap_or_default()));
        let dmg_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.damage.clone()).unwrap_or_default()));
        let range_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.range.clone()).unwrap_or_default()));
        let rate_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.rate.clone()).unwrap_or_default()));
        let clip_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.clip.clone()).unwrap_or_default()));
        let conceal_val = Signal::derive(move || data.with(|d| d.weapons.get(idx).map(|w| w.conceal.clone()).unwrap_or_default()));

        view! {
            <tr>
                <td>
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
                </td>
                <td>
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || crate::i18n::tr("diff_header", lang()).to_string())
                        value=diff_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].diff = val;
                            });
                        })
                    />
                </td>
                <td>
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || crate::i18n::tr("dmg_header", lang()).to_string())
                        value=dmg_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].damage = val;
                            });
                        })
                    />
                </td>
                <td>
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || crate::i18n::tr("range_header", lang()).to_string())
                        value=range_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].range = val;
                            });
                        })
                    />
                </td>
                <td>
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || crate::i18n::tr("rate_header", lang()).to_string())
                        value=rate_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].rate = val;
                            });
                        })
                    />
                </td>
                <td>
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || crate::i18n::tr("clip_header", lang()).to_string())
                        value=clip_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].clip = val;
                            });
                        })
                    />
                </td>
                <td>
                    <StableTextInput 
                        class="table-cell-input text-center"
                        placeholder=Signal::derive(move || crate::i18n::tr("conceal_header", lang()).to_string())
                        value=conceal_val
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| {
                                while s.weapons.len() <= idx { s.weapons.push(WeaponItem::default()); }
                                s.weapons[idx].conceal = val;
                            });
                        })
                    />
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
                // Tabela de Armas e Ataques (4 Linhas Estáticas)
                <div class="weapons-table-column">
                    <div class="weapons-table-header-row">
                        <span class="weapons-table-title">{move || match lang() {
                            crate::i18n::Language::PtBr => "ARMAS & ATAQUES",
                            crate::i18n::Language::EnUs => "WEAPONS & ATTACKS",
                        }}</span>
                    </div>

                    <div class="weapons-table-container">
                        <table class="weapons-table">
                            <thead>
                                <tr>
                                    <th class="th-weapon">{move || crate::i18n::tr("weapon_header", lang()).to_uppercase()}</th>
                                    <th class="th-stat">{move || crate::i18n::tr("diff_header", lang()).to_uppercase()}</th>
                                    <th class="th-stat">{move || crate::i18n::tr("dmg_header", lang()).to_uppercase()}</th>
                                    <th class="th-stat">{move || crate::i18n::tr("range_header", lang()).to_uppercase()}</th>
                                    <th class="th-stat">{move || crate::i18n::tr("rate_header", lang()).to_uppercase()}</th>
                                    <th class="th-stat">{move || crate::i18n::tr("clip_header", lang()).to_uppercase()}</th>
                                    <th class="th-stat">{move || crate::i18n::tr("conceal_header", lang()).to_uppercase()}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {(0..4).map(render_weapon_row).collect_view()}
                            </tbody>
                        </table>
                    </div>
                </div>

                // Bloco de Armadura
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
                                    crate::i18n::Language::PtBr => "Ex: Jaqueta de Couro".to_string(),
                                    crate::i18n::Language::EnUs => "Ex: Leather Jacket".to_string(),
                                })
                                value=Signal::derive(move || data.with(|d| d.armor.class_name.clone()))
                                on_change=Callback::new(move |val| {
                                    set_data.update(|s| s.armor.class_name = val);
                                })
                            />
                        </div>

                        <div class="armor-field-row">
                            <label class="armor-label">{move || format!("{}:", crate::i18n::tr("armor_rating", lang()))}</label>
                            <StableTextInput 
                                class="armor-input text-center"
                                placeholder="1"
                                value=Signal::derive(move || data.with(|d| d.armor.rating.clone()))
                                on_change=Callback::new(move |val| {
                                    set_data.update(|s| s.armor.rating = val);
                                })
                            />
                        </div>

                        <div class="armor-field-row">
                            <label class="armor-label">{move || format!("{}:", crate::i18n::tr("armor_penalty", lang()))}</label>
                            <StableTextInput 
                                class="armor-input text-center"
                                placeholder="0"
                                value=Signal::derive(move || data.with(|d| d.armor.penalty.clone()))
                                on_change=Callback::new(move |val| {
                                    set_data.update(|s| s.armor.penalty = val);
                                })
                            />
                        </div>

                        <div class="armor-desc-row">
                            <label class="armor-label">{move || match lang() {
                                crate::i18n::Language::PtBr => "Descrição:",
                                crate::i18n::Language::EnUs => "Description:",
                            }}</label>
                            <StableTextArea 
                                class="armor-desc-textarea"
                                placeholder=Signal::derive(move || match lang() {
                                    crate::i18n::Language::PtBr => "Detalhes da proteção...".to_string(),
                                    crate::i18n::Language::EnUs => "Protection details...".to_string(),
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
