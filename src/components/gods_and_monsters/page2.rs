use leptos::*;
use crate::components::{ValueField, StableTextArea};
use crate::state::{CharacterData, DotOrigin, WeaponItem, MeritItem, FlawItem};
use crate::components::character_sheet::ActiveDotOriginContext;
use super::advantages::GodsLineInput;

#[component]
pub fn GodsAndMonstersPage2() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    // Other Traits (9 slots with 5 dots)
    let update_other_trait = move |name: String, level: Option<i32>, modifier: Option<String>| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
        set_data.update(|s| {
            s.set_attribute_with_origin(&name, level, modifier, current_origin);
        });
    };

    let update_other_trait_dot = move |name: String, dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            s.set_attribute_dot_origin(&name, dot_idx, origin);
        });
    };

    let render_other_trait_field = move |index: usize| {
        let key_trait = format!("gods_trait_{}", index);
        let key_label = format!("gods_trait_label_{}", index);

        let k1 = key_trait.clone();
        let k2 = key_trait.clone();
        let k3 = key_trait.clone();
        let k4 = key_trait.clone();
        let k5 = key_trait.clone();
        let k6 = key_trait.clone();
        let kl1 = key_label.clone();
        let kl2 = key_label.clone();

        let label = Signal::derive(move || data.with(|d| d.get_label(&kl1)));
        let level = Signal::derive(move || data.with(|d| d.get_attribute_level(&k1, 0)));
        let modifier = Signal::derive(move || data.with(|d| d.get_attribute_modifier(&k2)));
        let origins = Signal::derive(move || data.with(|d| d.attributes.get(&k3).map(|a| a.get_origins(5)).unwrap_or_else(|| vec![DotOrigin::Base; 5])));

        let on_dot_origin_change = Callback::new(move |(idx, orig)| update_other_trait_dot(k6.clone(), idx, orig));

        view! {
            <ValueField 
                label=label
                level=level
                modifier=modifier
                origins=origins
                on_level_change=move |v| update_other_trait(k4.clone(), Some(v), None)
                on_modifier_change=move |m| update_other_trait(k5.clone(), None, Some(m))
                on_dot_origin_change=on_dot_origin_change
                min_level=0
                max_chars=18
                is_editable=true
                on_label_change=Callback::new(move |new_n| {
                    let kl = kl2.clone();
                    set_data.update(|s| {
                        s.set_label(&kl, new_n);
                    });
                })
            />
        }
    };

    // Merits & Flaws updater
    let update_merit = move |idx: usize, name: Option<String>, cost: Option<i32>| {
        set_data.update(|s| {
            if idx >= s.merits.len() {
                s.merits.resize(idx + 1, MeritItem::default());
            }
            if let Some(n) = name {
                s.merits[idx].name = n;
            }
            if let Some(c) = cost {
                s.merits[idx].cost = c;
            }
        });
    };

    let update_flaw = move |idx: usize, name: Option<String>, bonus: Option<i32>| {
        set_data.update(|s| {
            if idx >= s.flaws.len() {
                s.flaws.resize(idx + 1, FlawItem::default());
            }
            if let Some(n) = name {
                s.flaws[idx].name = n;
            }
            if let Some(b) = bonus {
                s.flaws[idx].bonus = b;
            }
        });
    };

    // Combat weapons updater
    let update_weapon = move |idx: usize, field: &'static str, val: String| {
        set_data.update(|s| {
            if idx >= s.weapons.len() {
                s.weapons.resize(idx + 1, WeaponItem::default());
            }
            match field {
                "name" => s.weapons[idx].name = val,
                "diff" => s.weapons[idx].diff = val,
                "damage" => s.weapons[idx].damage = val,
                "range" => s.weapons[idx].range = val,
                "rate" => s.weapons[idx].rate = val,
                "clip" => s.weapons[idx].clip = val,
                _ => {}
            }
        });
    };

    view! {
        <div class="sheet-page-layout page-2-layout gods-sheet-page gods-page-2">
            <div class="gods-page2-grid">
                
                // ── Coluna Esquerda: Other Traits & Expanded Powers ──
                <div class="gods-page2-left-col">
                    <div class="group-box gods-box">
                        <span class="group-title">"Other Traits"</span>
                        <div class="gods-traits-list">
                            {(0..9).map(|i| render_other_trait_field(i)).collect_view()}
                        </div>
                    </div>

                    <div class="group-box gods-box">
                        <span class="group-title">"Charms"</span>
                        <div class="gods-lines-list">
                            {(0..10).map(|i| view! { <GodsLineInput key=format!("exp_charm_{}", i) placeholder="Charm..." /> }).collect_view()}
                        </div>
                    </div>

                    <div class="group-box gods-box">
                        <span class="group-title">"Special Advantages"</span>
                        <div class="gods-lines-list">
                            {(0..10).map(|i| view! { <GodsLineInput key=format!("exp_special_adv_{}", i) placeholder="Special Advantage..." /> }).collect_view()}
                        </div>
                    </div>

                    <div class="group-box gods-box">
                        <span class="group-title">"Gifts"</span>
                        <div class="gods-lines-list">
                            {(0..10).map(|i| view! { <GodsLineInput key=format!("exp_gift_{}", i) placeholder="Gift..." /> }).collect_view()}
                        </div>
                    </div>
                </div>

                // ── Coluna Direita: Merits & Flaws, History, Description, Rules, Combat ──
                <div class="gods-page2-right-col">
                    
                    // Merits & Flaws
                    <div class="group-box gods-box">
                        <span class="group-title">"Merits & Flaws"</span>
                        <div class="gods-merits-flaws-grid">
                            <div class="merits-col">
                                <div class="table-subhead">
                                    <span>"Merit"</span>
                                    <span>"Cost"</span>
                                </div>
                                {(0..7).map(|i| {
                                    let m_name = Signal::derive(move || data.with(|d| d.merits.get(i).map(|m| m.name.clone()).unwrap_or_default()));
                                    let m_cost = Signal::derive(move || data.with(|d| d.merits.get(i).map(|m| if m.cost > 0 { m.cost.to_string() } else { "".to_string() }).unwrap_or_default()));
                                    view! {
                                        <div class="merit-flaw-row">
                                            <input 
                                                type="text" 
                                                class="merit-name-input" 
                                                placeholder="Merit..."
                                                prop:value=m_name 
                                                on:input=move |ev| update_merit(i, Some(event_target_value(&ev)), None) 
                                            />
                                            <input 
                                                type="number" 
                                                class="merit-val-input" 
                                                min="0" 
                                                max="10" 
                                                placeholder="0"
                                                prop:value=m_cost 
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                                                    update_merit(i, None, Some(val));
                                                } 
                                            />
                                        </div>
                                    }
                                }).collect_view()}
                            </div>

                            <div class="flaws-col">
                                <div class="table-subhead">
                                    <span>"Flaw"</span>
                                    <span>"Bonus"</span>
                                </div>
                                {(0..7).map(|i| {
                                    let f_name = Signal::derive(move || data.with(|d| d.flaws.get(i).map(|f| f.name.clone()).unwrap_or_default()));
                                    let f_bonus = Signal::derive(move || data.with(|d| d.flaws.get(i).map(|f| if f.bonus > 0 { f.bonus.to_string() } else { "".to_string() }).unwrap_or_default()));
                                    view! {
                                        <div class="merit-flaw-row">
                                            <input 
                                                type="text" 
                                                class="merit-name-input" 
                                                placeholder="Flaw..."
                                                prop:value=f_name 
                                                on:input=move |ev| update_flaw(i, Some(event_target_value(&ev)), None) 
                                            />
                                            <input 
                                                type="number" 
                                                class="merit-val-input" 
                                                min="0" 
                                                max="10" 
                                                placeholder="0"
                                                prop:value=f_bonus 
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                                                    update_flaw(i, None, Some(val));
                                                } 
                                            />
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    </div>

                    // History
                    <div class="group-box gods-box">
                        <span class="group-title">"History"</span>
                        <StableTextArea
                            class="gods-large-textarea"
                            placeholder="Origin story, mystical pact with the Mage, or supernatural nature..."
                            value=Signal::derive(move || data.with(|d| d.get_label("gods_history")))
                            on_change=Callback::new(move |v| set_data.update(|s| s.set_label("gods_history", v)))
                        />
                    </div>

                    // Description
                    <div class="group-box gods-box">
                        <span class="group-title">"Description"</span>
                        <StableTextArea
                            class="gods-large-textarea"
                            placeholder="Physical appearance, spirit form, size, sounds and peculiar traits..."
                            value=Signal::derive(move || data.with(|d| d.get_label("gods_description")))
                            on_change=Callback::new(move |v| set_data.update(|s| s.set_label("gods_description", v)))
                        />
                    </div>

                    // Special Rules
                    <div class="group-box gods-box">
                        <span class="group-title">"Special Rules"</span>
                        <StableTextArea
                            class="gods-large-textarea"
                            placeholder="Materialization rules, feeding restrictions, banes, weaknesses or magical commands..."
                            value=Signal::derive(move || data.with(|d| d.get_label("gods_special_rules")))
                            on_change=Callback::new(move |v| set_data.update(|s| s.set_label("gods_special_rules", v)))
                        />
                    </div>

                    // Combat Table
                    <div class="group-box gods-box">
                        <span class="group-title">"Combat"</span>
                        <div class="gods-combat-table">
                            <div class="combat-table-head">
                                <span class="head-weapon">"Weapon/Attack"</span>
                                <span class="head-diff">"Diff."</span>
                                <span class="head-dmg">"Damage"</span>
                                <span class="head-range">"Range"</span>
                                <span class="head-rate">"Rate"</span>
                                <span class="head-clip">"Clip"</span>
                            </div>
                            {(0..6).map(|i| {
                                let w_name = Signal::derive(move || data.with(|d| d.weapons.get(i).map(|w| w.name.clone()).unwrap_or_default()));
                                let w_diff = Signal::derive(move || data.with(|d| d.weapons.get(i).map(|w| w.diff.clone()).unwrap_or_default()));
                                let w_dmg = Signal::derive(move || data.with(|d| d.weapons.get(i).map(|w| w.damage.clone()).unwrap_or_default()));
                                let w_range = Signal::derive(move || data.with(|d| d.weapons.get(i).map(|w| w.range.clone()).unwrap_or_default()));
                                let w_rate = Signal::derive(move || data.with(|d| d.weapons.get(i).map(|w| w.rate.clone()).unwrap_or_default()));
                                let w_clip = Signal::derive(move || data.with(|d| d.weapons.get(i).map(|w| w.clip.clone()).unwrap_or_default()));

                                view! {
                                    <div class="combat-row">
                                        <input 
                                            type="text" 
                                            class="col-weapon-name" 
                                            placeholder="Weapon..." 
                                            prop:value=w_name 
                                            on:input=move |ev| update_weapon(i, "name", event_target_value(&ev)) 
                                        />
                                        <input 
                                            type="text" 
                                            class="col-weapon-diff" 
                                            placeholder="6" 
                                            prop:value=w_diff 
                                            on:input=move |ev| update_weapon(i, "diff", event_target_value(&ev)) 
                                        />
                                        <input 
                                            type="text" 
                                            class="col-weapon-dmg" 
                                            placeholder="Str+1" 
                                            prop:value=w_dmg 
                                            on:input=move |ev| update_weapon(i, "damage", event_target_value(&ev)) 
                                        />
                                        <input 
                                            type="text" 
                                            class="col-weapon-range" 
                                            placeholder="-" 
                                            prop:value=w_range 
                                            on:input=move |ev| update_weapon(i, "range", event_target_value(&ev)) 
                                        />
                                        <input 
                                            type="text" 
                                            class="col-weapon-rate" 
                                            placeholder="-" 
                                            prop:value=w_rate 
                                            on:input=move |ev| update_weapon(i, "rate", event_target_value(&ev)) 
                                        />
                                        <input 
                                            type="text" 
                                            class="col-weapon-clip" 
                                            placeholder="-" 
                                            prop:value=w_clip 
                                            on:input=move |ev| update_weapon(i, "clip", event_target_value(&ev)) 
                                        />
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                </div>

            </div>
        </div>
    }
}
