use leptos::*;
use crate::state::{CharacterData, MeritItem, FlawItem};

#[component]
pub fn MeritsFlaws() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let render_merit_row = move |idx: usize| {
        view! {
            <div class="merit-row">
                <input 
                    type="text" 
                    class="merit-input name-input"
                    placeholder="Nome da qualidade..."
                    prop:value=move || data.with(|d| d.merits.get(idx).map(|m| m.name.clone()).unwrap_or_default())
                    on:change=move |ev| {
                        let val = event_target_value(&ev);
                        set_data.update(|s| {
                            while s.merits.len() <= idx { s.merits.push(MeritItem::default()); }
                            s.merits[idx].name = val;
                        });
                    }
                    on:blur=move |ev| {
                        let val = event_target_value(&ev);
                        set_data.update(|s| {
                            while s.merits.len() <= idx { s.merits.push(MeritItem::default()); }
                            s.merits[idx].name = val;
                        });
                    }
                />
                <input 
                    type="text" 
                    class="merit-input type-input text-center"
                    placeholder="Tipo"
                    prop:value=move || data.with(|d| d.merits.get(idx).map(|m| m.merit_type.clone()).unwrap_or_default())
                    on:change=move |ev| {
                        let val = event_target_value(&ev);
                        set_data.update(|s| {
                            while s.merits.len() <= idx { s.merits.push(MeritItem::default()); }
                            s.merits[idx].merit_type = val;
                        });
                    }
                    on:blur=move |ev| {
                        let val = event_target_value(&ev);
                        set_data.update(|s| {
                            while s.merits.len() <= idx { s.merits.push(MeritItem::default()); }
                            s.merits[idx].merit_type = val;
                        });
                    }
                />
                <input 
                    type="number" 
                    min="1"
                    max="10"
                    class="merit-input cost-input text-center"
                    prop:value=move || data.with(|d| d.merits.get(idx).map(|m| if m.cost > 0 { m.cost.to_string() } else { String::new() }).unwrap_or_default())
                    on:change=move |ev| {
                        let val = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                        set_data.update(|s| {
                            while s.merits.len() <= idx { s.merits.push(MeritItem::default()); }
                            s.merits[idx].cost = val;
                        });
                    }
                    on:blur=move |ev| {
                        let val = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                        set_data.update(|s| {
                            while s.merits.len() <= idx { s.merits.push(MeritItem::default()); }
                            s.merits[idx].cost = val;
                        });
                    }
                />
            </div>
        }
    };

    let render_flaw_row = move |idx: usize| {
        view! {
            <div class="flaw-row">
                <input 
                    type="text" 
                    class="merit-input name-input"
                    placeholder="Nome do defeito..."
                    prop:value=move || data.with(|d| d.flaws.get(idx).map(|f| f.name.clone()).unwrap_or_default())
                    on:change=move |ev| {
                        let val = event_target_value(&ev);
                        set_data.update(|s| {
                            while s.flaws.len() <= idx { s.flaws.push(FlawItem::default()); }
                            s.flaws[idx].name = val;
                        });
                    }
                    on:blur=move |ev| {
                        let val = event_target_value(&ev);
                        set_data.update(|s| {
                            while s.flaws.len() <= idx { s.flaws.push(FlawItem::default()); }
                            s.flaws[idx].name = val;
                        });
                    }
                />
                <input 
                    type="text" 
                    class="merit-input type-input text-center"
                    placeholder="Tipo"
                    prop:value=move || data.with(|d| d.flaws.get(idx).map(|f| f.flaw_type.clone()).unwrap_or_default())
                    on:change=move |ev| {
                        let val = event_target_value(&ev);
                        set_data.update(|s| {
                            while s.flaws.len() <= idx { s.flaws.push(FlawItem::default()); }
                            s.flaws[idx].flaw_type = val;
                        });
                    }
                    on:blur=move |ev| {
                        let val = event_target_value(&ev);
                        set_data.update(|s| {
                            while s.flaws.len() <= idx { s.flaws.push(FlawItem::default()); }
                            s.flaws[idx].flaw_type = val;
                        });
                    }
                />
                <input 
                    type="number" 
                    min="1"
                    max="10"
                    class="merit-input cost-input text-center"
                    prop:value=move || data.with(|d| d.flaws.get(idx).map(|f| if f.bonus > 0 { f.bonus.to_string() } else { String::new() }).unwrap_or_default())
                    on:change=move |ev| {
                        let val = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                        set_data.update(|s| {
                            while s.flaws.len() <= idx { s.flaws.push(FlawItem::default()); }
                            s.flaws[idx].bonus = val;
                        });
                    }
                    on:blur=move |ev| {
                        let val = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                        set_data.update(|s| {
                            while s.flaws.len() <= idx { s.flaws.push(FlawItem::default()); }
                            s.flaws[idx].bonus = val;
                        });
                    }
                />
            </div>
        }
    };

    view! {
        <div class="group-box merits-flaws-box">
            <div class="group-box-header">
                <span class="group-box-title">"QUALIDADES & DEFEITOS (MERITS & FLAWS)"</span>
            </div>

            <div class="merits-flaws-grid">
                // Coluna: Qualidades (7 Linhas)
                <div class="merits-column">
                    <div class="sub-table-header">
                        <span class="col-name font-bold">"QUALIDADE"</span>
                        <span class="col-type font-bold text-center">"TIPO"</span>
                        <span class="col-cost font-bold text-center">"CUSTO"</span>
                    </div>

                    <div class="merits-list">
                        {(0..7).map(render_merit_row).collect_view()}
                    </div>
                </div>

                // Coluna: Defeitos (7 Linhas)
                <div class="flaws-column">
                    <div class="sub-table-header">
                        <span class="col-name font-bold">"DEFEITO"</span>
                        <span class="col-type font-bold text-center">"TIPO"</span>
                        <span class="col-cost font-bold text-center">"BÔNUS"</span>
                    </div>

                    <div class="flaws-list">
                        {(0..7).map(render_flaw_row).collect_view()}
                    </div>
                </div>
            </div>
        </div>
    }
}
