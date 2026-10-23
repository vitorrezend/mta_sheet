//! Rolador de dados da ficha: monta a parada (Atributo + Habilidade) com os valores da
//! própria ficha e rola. Se a ficha estiver numa sala, a rolagem vai para o chat da mesa
//! (todos veem, com som); senão, é uma rolagem local, só para quem está com a ficha aberta.

use leptos::*;
use crate::components::common::{install_audio_unlock, load_sound_muted, play_dice_roll_sound};
use crate::components::rooms::room_chat::message::RollCard;
use crate::rooms::{get_sheet_room, roll_room_dice, ChatRollData};
use crate::rules::{RollRequest, MAX_DICE, MAX_DIFFICULTY, MIN_DIFFICULTY};
use crate::state::{keys, CharacterData};

/// Opção de traço: (chave na ficha, rótulo exibido, nível atual)
type TraitOption = (String, String, i32);

fn attribute_options(d: &CharacterData) -> Vec<TraitOption> {
    keys::STANDARD_ATTRIBUTES
        .iter()
        .map(|name| (name.to_string(), name.to_string(), d.get_attribute_level(name, 1)))
        .collect()
}

/// Habilidades padrão e personalizadas com pelo menos 1 ponto.
fn ability_options(d: &CharacterData) -> Vec<TraitOption> {
    let standard = keys::STANDARD_TALENTS.iter().chain(keys::STANDARD_SKILLS.iter()).chain(keys::STANDARD_KNOWLEDGES.iter())
        .map(|name| (name.to_string(), name.to_string()));
    let custom = [keys::CAT_TALENTOS, keys::CAT_PERICIAS, keys::CAT_CONHECIMENTOS]
        .iter()
        .flat_map(|cat| d.custom_lists.get(*cat).cloned().unwrap_or_default())
        .map(|key| {
            let label = d.labels.get(&key).filter(|l| !l.trim().is_empty()).cloned().unwrap_or_else(|| key.clone());
            (key, label)
        });
    let mut list: Vec<TraitOption> = standard
        .chain(custom)
        .map(|(key, label)| {
            let level = d.get_attribute_level(&key, 0);
            (key, label, level)
        })
        .filter(|(_, _, level)| *level > 0)
        .collect();
    list.sort_by(|a, b| a.1.cmp(&b.1));
    list.dedup_by(|a, b| a.0 == b.0);
    list
}

fn level_of(options: &[TraitOption], key: &str) -> i32 {
    options.iter().find(|(k, _, _)| k == key).map(|(_, _, l)| *l).unwrap_or(0)
}

fn label_of(options: &[TraitOption], key: &str) -> String {
    options.iter().find(|(k, _, _)| k == key).map(|(_, l, _)| l.clone()).unwrap_or_default()
}

#[component]
pub fn SheetDiceRoller(
    #[prop(into)] sheet_id: Signal<String>,
    data: ReadSignal<CharacterData>,
    #[prop(into)] can_edit: Signal<bool>,
) -> impl IntoView {
    install_audio_unlock();

    let is_open = create_rw_signal(false);
    let attribute = create_rw_signal("Destreza".to_string());
    let ability = create_rw_signal(String::new());
    let extra = create_rw_signal(0i32);
    let difficulty = create_rw_signal(6i32);
    let specialty = create_rw_signal(false);
    let apply_wounds = create_rw_signal(true);
    let is_rolling = create_rw_signal(false);
    let last_roll = create_rw_signal(Option::<(ChatRollData, bool)>::None);
    let error = create_rw_signal(Option::<String>::None);

    // Só consulta a sala quando o painel é aberto pela primeira vez
    let room = create_local_resource(
        move || (sheet_id.get(), is_open.get()),
        |(id, open)| async move {
            if !open || id.is_empty() {
                return None;
            }
            get_sheet_room(id).await.ok().flatten()
        },
    );
    let room_link = move || room.get().flatten();

    let attributes = create_memo(move |_| data.with(attribute_options));
    let abilities = create_memo(move |_| data.with(ability_options));
    let wound_penalty = create_memo(move |_| {
        data.with(|d| {
            let (agg, lethal, bashing) = d.get_health_counts();
            crate::rules::get_combat_health_penalty(agg, lethal, bashing, d.get_extra_bruised())
        })
    });

    let base_pool = move || {
        let attr = attributes.with(|list| level_of(list, &attribute.get()));
        let abil = abilities.with(|list| level_of(list, &ability.get()));
        attr + abil + extra.get()
    };
    let final_pool = move || {
        let pool = base_pool();
        let pool = if apply_wounds.get() { crate::rules::apply_wound_penalty_to_action_pool(pool, wound_penalty.get()) } else { pool };
        pool.clamp(0, MAX_DICE)
    };
    let roll_label = move || {
        let attr = attributes.with(|list| label_of(list, &attribute.get()));
        let abil = abilities.with(|list| label_of(list, &ability.get()));
        if abil.is_empty() { attr } else { format!("{} + {}", attr, abil) }
    };

    let roll = move |_| {
        let pool = final_pool();
        if pool < 1 || is_rolling.get_untracked() {
            return;
        }
        let request = RollRequest::new(pool, difficulty.get_untracked(), specialty.get_untracked(), &roll_label());
        error.set(None);
        if !load_sound_muted() {
            play_dice_roll_sound();
        }
        match room_link() {
            Some(link) => {
                is_rolling.set(true);
                let sheet = Some(sheet_id.get_untracked());
                spawn_local(async move {
                    match roll_room_dice(link.room_id, request.dice_count, request.difficulty, request.specialty, request.label, sheet).await {
                        Ok(msg) => last_roll.set(msg.roll_data.map(|r| (r, true))),
                        Err(e) => error.set(Some(crate::components::rooms::room_chat::store::clean_error(&e.to_string()))),
                    }
                    is_rolling.set(false);
                });
            }
            None => {
                // Ficha fora de sala: rola aqui mesmo, só para quem está vendo
                let dice_results = crate::rules::roll_d10s(request.dice_count);
                let (successes, is_botch) = crate::rules::specialties::evaluate_roll(&dice_results, request.difficulty, request.specialty);
                last_roll.set(Some((
                    ChatRollData {
                        dice_count: request.dice_count,
                        difficulty: request.difficulty,
                        specialty: request.specialty,
                        specialty_text: None,
                        dice_results,
                        successes,
                        is_botch,
                        label: request.label,
                    },
                    false,
                )));
            }
        }
    };

    view! {
        <Show when=move || can_edit.get()>
            <button
                type="button"
                class="sheet-dice-fab"
                class:active=move || is_open.get()
                aria-label="Rolar dados da ficha"
                title="Rolar dados (Atributo + Habilidade)"
                on:click=move |_| is_open.update(|v| *v = !*v)
            >
                "🎲"
            </button>
        </Show>

        <Show when=move || is_open.get() && can_edit.get()>
            <div class="sheet-dice-panel" role="dialog" aria-label="Rolar dados">
                <div class="sheet-dice-header">
                    <strong>"🎲 Rolar dados"</strong>
                    <button type="button" class="sheet-dice-close" aria-label="Fechar" on:click=move |_| is_open.set(false)>"✕"</button>
                </div>
                <p class="sheet-dice-target">
                    {move || match room_link() {
                        Some(link) => format!("➜ Mesa \"{}\": todos na sala veem a rolagem", link.room_name),
                        None => "Rolagem local: a ficha não está em nenhuma sala".to_string(),
                    }}
                </p>

                <div class="sheet-dice-grid">
                    <label>
                        "Atributo"
                        <select on:change=move |ev| attribute.set(event_target_value(&ev))>
                            <For
                                each=move || attributes.get()
                                key=|(key, _, level)| (key.clone(), *level)
                                children=move |(key, label, level)| {
                                    // A opção marca a si mesma: `prop:value` no <select> rodaria antes das opções existirem
                                    let k = key.clone();
                                    view! { <option value=key selected=move || attribute.get() == k>{format!("{} ({})", label, level)}</option> }
                                }
                            />
                        </select>
                    </label>
                    <label>
                        "Habilidade"
                        <select on:change=move |ev| ability.set(event_target_value(&ev))>
                            <option value="" selected=move || ability.get().is_empty()>"— nenhuma —"</option>
                            <For
                                each=move || abilities.get()
                                key=|(key, _, level)| (key.clone(), *level)
                                children=move |(key, label, level)| {
                                    let k = key.clone();
                                    view! { <option value=key selected=move || ability.get() == k>{format!("{} ({})", label, level)}</option> }
                                }
                            />
                        </select>
                    </label>
                    <label>
                        "Dados extras"
                        <div class="sheet-dice-stepper">
                            <button type="button" aria-label="Menos um dado" on:click=move |_| extra.update(|v| *v = (*v - 1).max(-10))>"-"</button>
                            <span>{move || extra.get()}</span>
                            <button type="button" aria-label="Mais um dado" on:click=move |_| extra.update(|v| *v = (*v + 1).min(10))>"+"</button>
                        </div>
                    </label>
                    <label>
                        "Dificuldade"
                        <div class="sheet-dice-stepper">
                            <button type="button" aria-label="Diminuir dificuldade" on:click=move |_| difficulty.update(|v| *v = (*v - 1).max(MIN_DIFFICULTY))>"-"</button>
                            <span>{move || difficulty.get()}</span>
                            <button type="button" aria-label="Aumentar dificuldade" on:click=move |_| difficulty.update(|v| *v = (*v + 1).min(MAX_DIFFICULTY))>"+"</button>
                        </div>
                    </label>
                </div>

                <label class="sheet-dice-check">
                    <input type="checkbox" prop:checked=move || specialty.get() on:change=move |ev| specialty.set(event_target_checked(&ev)) />
                    "⭐ Especialidade (10 conta dobrado)"
                </label>
                <Show when=move || wound_penalty.get() != 0>
                    <label class="sheet-dice-check">
                        <input type="checkbox" prop:checked=move || apply_wounds.get() on:change=move |ev| apply_wounds.set(event_target_checked(&ev)) />
                        {move || format!("🩸 Aplicar penalidade de ferimentos (-{})", wound_penalty.get())}
                    </label>
                </Show>

                <button type="button" class="sheet-dice-roll-btn" on:click=roll disabled=move || final_pool() < 1 || is_rolling.get()>
                    {move || match (is_rolling.get(), final_pool()) {
                        (true, _) => "Rolando…".to_string(),
                        (false, 1) => "Rolar 1 dado".to_string(),
                        (false, n) => format!("Rolar {} dados", n),
                    }}
                </button>

                <Show when=move || error.get().is_some()>
                    <p class="sheet-dice-error" role="alert">{move || error.get().unwrap_or_default()}</p>
                </Show>
                {move || last_roll.get().map(|(roll, in_room)| view! {
                    <div class="sheet-dice-result">
                        <RollCard roll=roll />
                        <small>{if in_room { "Enviada para o chat da mesa" } else { "Só você viu esta rolagem" }}</small>
                    </div>
                })}
            </div>
        </Show>
    }
}
