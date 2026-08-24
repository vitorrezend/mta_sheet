use leptos::*;
use crate::components::page1::advantages::Vitality;
use crate::components::{ValueField, StableTextArea, Callback};
use crate::state::{CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn GodsAndMonstersAdvantages() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    // Backgrounds helpers (6 standard slots)
    let update_bg = move |name: String, level: Option<i32>, modifier: Option<String>| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
        set_data.update(|s| {
            s.set_attribute_with_origin(&name, level, modifier, current_origin);
        });
    };

    let update_bg_dot = move |name: String, dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            s.set_attribute_dot_origin(&name, dot_idx, origin);
        });
    };

    let render_bg_field = move |index: usize| {
        let key_bg = format!("gods_bg_{}", index);
        let key_label = format!("gods_bg_label_{}", index);

        let k1 = key_bg.clone();
        let k2 = key_bg.clone();
        let k3 = key_bg.clone();
        let k4 = key_bg.clone();
        let k5 = key_bg.clone();
        let k6 = key_bg.clone();
        let kl1 = key_label.clone();
        let kl2 = key_label.clone();

        let label = Signal::derive(move || data.with(|d| d.get_label(&kl1)));
        let level = Signal::derive(move || data.with(|d| d.get_attribute_level(&k1, 0)));
        let modifier = Signal::derive(move || data.with(|d| d.get_attribute_modifier(&k2)));
        let origins = Signal::derive(move || data.with(|d| d.attributes.get(&k3).map(|a| a.get_origins(5)).unwrap_or_else(|| vec![DotOrigin::Base; 5])));

        let on_dot_origin_change = Callback::new(move |(idx, orig)| update_bg_dot(k6.clone(), idx, orig));

        view! {
            <ValueField 
                label=label
                level=level
                modifier=modifier
                origins=origins
                on_level_change=move |v| update_bg(k4.clone(), Some(v), None)
                on_modifier_change=move |m| update_bg(k5.clone(), None, Some(m))
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

    // Willpower Permanent & Temporary
    let update_wp_total = move |val: i32| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
        set_data.update(|s| {
            s.set_attribute_with_origin(crate::state::models::keys::KEY_WILLPOWER_TOTAL, Some(val), None, current_origin);
            s.set_willpower_total(val);
        });
    };

    let wp_total = move || data.with(|d| d.get_attribute_level(crate::state::models::keys::KEY_WILLPOWER_TOTAL, 5));
    let wp_current = move || data.with(|d| d.get_attribute_level(crate::state::models::keys::KEY_WILLPOWER_CURRENT, 5));

    // Gnosis (10 dots + 10 boxes)
    let gnosis_data = move || data.with(|d| d.get_gnosis());
    let update_gnosis_dots = move |val: i32| {
        set_data.update(|s| s.set_gnosis_dots(val));
    };

    // Paradox Pool (20 boxes)
    let paradox_data = move || data.with(|d| d.get_paradox_pool());

    // Essence Pool (50 boxes)
    let essence_data = move || data.with(|d| d.get_essence_pool());

    // Charms dynamic list handlers
    let add_charm = move || {
        set_data.update(|s| {
            let list = s.custom_lists.entry("Charms".to_string()).or_default();
            let next_idx = list.len();
            list.push(format!("custom_charm_{}", next_idx));
        });
    };

    let remove_charm = move |key: String| {
        set_data.update(|s| {
            if let Some(list) = s.custom_lists.get_mut("Charms") {
                list.retain(|k| k != &key);
            }
            s.labels.remove(&key);
        });
    };

    // Gifts dynamic list handlers
    let add_gift = move || {
        set_data.update(|s| {
            let list = s.custom_lists.entry("Gifts".to_string()).or_default();
            let next_idx = list.len();
            list.push(format!("custom_gift_{}", next_idx));
        });
    };

    let remove_gift = move |key: String| {
        set_data.update(|s| {
            if let Some(list) = s.custom_lists.get_mut("Gifts") {
                list.retain(|k| k != &key);
            }
            s.labels.remove(&key);
        });
    };

    // Backgrounds custom list helpers
    let add_bg = move || {
        set_data.update(|s| {
            let list = s.custom_lists.entry("Gods_Backgrounds".to_string()).or_default();
            let next_idx = list.len();
            list.push(format!("custom_gods_bg_{}", next_idx));
        });
    };

    let remove_bg = move |key_id: String| {
        set_data.update(|s| {
            if let Some(list) = s.custom_lists.get_mut("Gods_Backgrounds") {
                list.retain(|k| k != &key_id);
            }
            s.attributes.remove(&key_id);
            s.labels.remove(&format!("label_{}", key_id));
        });
    };

    let render_custom_bg_field = move |key_id: String| {
        let key_bg = key_id.clone();
        let key_label = format!("label_{}", key_id);
        let k_rem = key_id.clone();

        let k1 = key_bg.clone();
        let k2 = key_bg.clone();
        let k3 = key_bg.clone();
        let k4 = key_bg.clone();
        let k5 = key_bg.clone();
        let k6 = key_bg.clone();
        let kl1 = key_label.clone();
        let kl2 = key_label.clone();

        let label = Signal::derive(move || data.with(|d| d.get_label(&kl1)));
        let level = Signal::derive(move || data.with(|d| d.get_attribute_level(&k1, 0)));
        let modifier = Signal::derive(move || data.with(|d| d.get_attribute_modifier(&k2)));
        let origins = Signal::derive(move || data.with(|d| d.attributes.get(&k3).map(|a| a.get_origins(5)).unwrap_or_else(|| vec![DotOrigin::Base; 5])));

        let on_dot_origin_change = Callback::new(move |(idx, orig)| update_bg_dot(k6.clone(), idx, orig));

        view! {
            <ValueField 
                label=label
                level=level
                modifier=modifier
                origins=origins
                on_level_change=move |v| update_bg(k4.clone(), Some(v), None)
                on_modifier_change=move |m| update_bg(k5.clone(), None, Some(m))
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
                on_remove=Callback::new(move |_| remove_bg(k_rem.clone()))
            />
        }
    };

    // Special Advantages dynamic list handlers
    let add_special_adv = move || {
        set_data.update(|s| {
            let list = s.custom_lists.entry("Special_Advantages".to_string()).or_default();
            let next_idx = list.len();
            list.push(format!("custom_special_adv_{}", next_idx));
        });
    };

    let remove_special_adv = move |key: String| {
        set_data.update(|s| {
            if let Some(list) = s.custom_lists.get_mut("Special_Advantages") {
                list.retain(|k| k != &key);
            }
            s.labels.remove(&key);
        });
    };

    view! {
        <div class="group-box gods-box">
            <span class="group-title">"Advantages"</span>
            <div class="advantages-block gods-advantages-grid">
                
                // ── Coluna 1: Charms, Gifts, Gnosis ──
                <div class="advantage-column">
                    <div class="gods-section-box">
                        <h3 class="column-title">"Charms"</h3>
                        <div class="gods-lines-list">
                            {(0..6).map(|i| view! { <GodsLineInput key=format!("charm_{}", i) placeholder="Charm..." /> }).collect_view()}
                            {move || data.with(|d| d.custom_lists.get("Charms").cloned().unwrap_or_default()).into_iter().map(|k| {
                                let k_remove = k.clone();
                                view! {
                                    <GodsLineInput 
                                        key=k.clone() 
                                        placeholder="Charm..." 
                                        is_custom=true 
                                        on_remove=Callback::new(move |_| remove_charm(k_remove.clone())) 
                                    />
                                }
                            }).collect_view()}
                            <button type="button" class="add-field-btn" on:click=move |_| add_charm() title="Adicionar Encanto">+</button>
                        </div>
                    </div>

                    <div class="gods-section-box">
                        <h3 class="column-title">"Gifts"</h3>
                        <div class="gods-lines-list">
                            {(0..6).map(|i| view! { <GodsLineInput key=format!("gift_{}", i) placeholder="Gift..." /> }).collect_view()}
                            {move || data.with(|d| d.custom_lists.get("Gifts").cloned().unwrap_or_default()).into_iter().map(|k| {
                                let k_remove = k.clone();
                                view! {
                                    <GodsLineInput 
                                        key=k.clone() 
                                        placeholder="Gift..." 
                                        is_custom=true 
                                        on_remove=Callback::new(move |_| remove_gift(k_remove.clone())) 
                                    />
                                }
                            }).collect_view()}
                            <button type="button" class="add-field-btn" on:click=move |_| add_gift() title="Adicionar Dom">+</button>
                        </div>
                    </div>

                    <div class="gods-section-box">
                        <h3 class="column-title">"Gnosis"</h3>
                        <div class="gods-gnosis-grid">
                            {(0..10usize).map(|i| {
                                let dot_idx = (i + 1) as i32;
                                let filled_dot = move || dot_idx <= gnosis_data().0;
                                let is_box_active = move || {
                                    let chars: Vec<char> = gnosis_data().1.chars().collect();
                                    i < chars.len() && chars[i] == '1'
                                };
                                view! {
                                    <div class="gnosis-column">
                                        <span
                                            class=move || if filled_dot() { "stat-dot filled-gnosis" } else { "stat-dot empty-dot" }
                                            on:click=move |_| update_gnosis_dots(dot_idx)
                                        ></span>
                                        <div
                                            class="gods-square"
                                            class:box-active=is_box_active
                                            on:click=move |_| set_data.update(|s| s.cycle_gnosis_box(i))
                                        >
                                            {move || if is_box_active() { "✕" } else { "" }}
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>

                // ── Coluna 2: Special Advantages, Willpower, Paradox, Essence ──
                <div class="advantage-column">
                    <div class="gods-section-box">
                        <h3 class="column-title">"Special Advantages"</h3>
                        <div class="gods-lines-list">
                            {(0..6).map(|i| view! { <GodsLineInput key=format!("special_adv_{}", i) placeholder="Special Advantage..." /> }).collect_view()}
                            {move || data.with(|d| d.custom_lists.get("Special_Advantages").cloned().unwrap_or_default()).into_iter().map(|k| {
                                let k_remove = k.clone();
                                view! {
                                    <GodsLineInput 
                                        key=k.clone() 
                                        placeholder="Special Advantage..." 
                                        is_custom=true 
                                        on_remove=Callback::new(move |_| remove_special_adv(k_remove.clone())) 
                                    />
                                }
                            }).collect_view()}
                            <button type="button" class="add-field-btn" on:click=move |_| add_special_adv() title="Adicionar Vantagem Especial">+</button>
                        </div>
                    </div>

                    <div class="gods-section-box">
                        <h3 class="column-title">"Willpower"</h3>
                        <div class="gods-gnosis-grid">
                            {(0..10usize).map(|i| {
                                let dot_idx = (i + 1) as i32;
                                let filled = move || dot_idx <= wp_total();
                                let active = move || dot_idx <= wp_current();
                                view! {
                                    <div class="gnosis-column">
                                        <span
                                            class=move || if filled() { "stat-dot filled-wp" } else { "stat-dot empty-dot" }
                                            on:click=move |_| update_wp_total(dot_idx)
                                        ></span>
                                        <div
                                            class="gods-square"
                                            class:box-active=active
                                            on:click=move |_| {
                                                let cur = wp_current();
                                                let next = if dot_idx == cur { cur - 1 } else { dot_idx };
                                                set_data.update(|s| {
                                                    s.set_attribute(crate::state::models::keys::KEY_WILLPOWER_CURRENT, Some(next), None);
                                                });
                                            }
                                        >
                                            {move || if active() { "✕" } else { "" }}
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    <div class="gods-section-box">
                        <h3 class="column-title">"Paradox"</h3>
                        <div class="gods-paradox-grid">
                            {(0..20).map(|box_i| {
                                let is_active = move || {
                                    let chars: Vec<char> = paradox_data().1.chars().collect();
                                    box_i < chars.len() && chars[box_i] == '1'
                                };
                                view! {
                                    <div
                                        class="gods-square"
                                        class:paradox-active=is_active
                                        on:click=move |_| set_data.update(|s| s.cycle_paradox_box(box_i))
                                    >
                                        {move || if is_active() { "✕" } else { "" }}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    <div class="gods-section-box">
                        <div class="essence-header-row">
                            <div class="essence-title-group">
                                <h3 class="column-title" style="margin-bottom: 0;">"Essence"</h3>
                                <button 
                                    type="button" 
                                    class="clear-essence-btn" 
                                    on:click=move |_| set_data.update(|s| s.clear_essence())
                                    title="Limpar Essência"
                                >
                                    "Limpar"
                                </button>
                            </div>
                            <span class="essence-spent-counter">{move || format!("Spent: {}/50", essence_data().0)}</span>
                        </div>
                        <div class="gods-essence-50-grid">
                            {(0..50).map(|box_i| {
                                let is_active = move || {
                                    let chars: Vec<char> = essence_data().1.chars().collect();
                                    box_i < chars.len() && chars[box_i] == '1'
                                };
                                view! {
                                    <div
                                        class="gods-square"
                                        class:essence-active=is_active
                                        on:click=move |_| set_data.update(|s| s.click_essence_box(box_i))
                                    >
                                        {move || if is_active() { "✕" } else { "" }}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>

                // ── Coluna 3: Backgrounds, Health, Experience ──
                <div class="advantage-column">
                    <div class="gods-section-box">
                        <h3 class="column-title">"Backgrounds"</h3>
                        <div class="gods-bg-list">
                            {(0..6).map(|i| render_bg_field(i)).collect_view()}
                            {move || data.with(|d| d.custom_lists.get("Gods_Backgrounds").cloned().unwrap_or_default()).into_iter().map(|k| {
                                render_custom_bg_field(k)
                            }).collect_view()}
                            <button type="button" class="add-field-btn" on:click=move |_| add_bg() title="Adicionar Antecedente">+</button>
                        </div>
                    </div>

                    <div class="gods-section-box">
                        <Vitality />
                    </div>

                    <div class="gods-section-box">
                        <h3 class="column-title">"Experience"</h3>
                        <StableTextArea
                            class="gods-xp-textarea"
                            placeholder="XP Log / Creation Points..."
                            value=Signal::derive(move || data.with(|d| d.get_label("experience")))
                            on_change=Callback::new(move |v| set_data.update(|s| s.set_label("experience", v)))
                        />
                    </div>
                </div>

            </div>
        </div>
    }
}

#[component]
pub fn GodsLineInput(
    key: String,
    placeholder: &'static str,
    #[prop(optional)] is_custom: bool,
    #[prop(optional)] on_remove: Option<Callback<()>>,
) -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let k_signal = key.clone();
    let val = Signal::derive(move || data.with(|d| d.get_label(&k_signal)));
    let k_input = key;

    view! {
        <div class="gods-line-row">
            <input
                type="text"
                class="gods-line-input"
                placeholder=placeholder
                prop:value=val
                on:input=move |ev| {
                    let v = event_target_value(&ev);
                    let k = k_input.clone();
                    set_data.update(|s| s.set_label(&k, v));
                }
            />
            {if let (true, Some(cb)) = (is_custom, on_remove) {
                view! {
                    <button 
                        type="button" 
                        class="remove-btn" 
                        on:click=move |_| cb.call(()) 
                        title="Remover linha"
                    >
                        "×"
                    </button>
                }.into_view()
            } else {
                ().into_view()
            }}
        </div>
    }
}

