use leptos::*;
use crate::rooms::{RoomSheetSummary, RoomInitiativeData, InitiativeEntry, update_room_initiative};
use crate::components::common::play_dice_roll_sound;

#[component]
pub fn InitiativeDrawer(
    is_open: ReadSignal<bool>,
    set_is_open: WriteSignal<bool>,
    room_id: Signal<String>,
    initiative: Signal<RoomInitiativeData>,
    sheets: Signal<Vec<RoomSheetSummary>>,
    is_gm: Signal<bool>,
) -> impl IntoView {
    let entries = create_rw_signal(Vec::<InitiativeEntry>::new());
    let round = create_rw_signal(1u32);
    let (new_npc_name, set_new_npc_name) = create_signal(String::new());
    let (new_npc_base, set_new_npc_base) = create_signal(6i32);
    let (is_rolling, set_is_rolling) = create_signal(false);

    // Sincroniza dados da sala vindos do servidor (incluindo inimigos criados pelo GM e rolagens)
    create_effect(move |_| {
        let server_init = initiative.get();
        let current_sheets = sheets.get();

        entries.update(|list| {
            if !server_init.entries.is_empty() {
                // Sincroniza estado compartilhado do servidor
                round.set(server_init.round);
                *list = server_init.entries.clone();
            }

            // Garante que novas fichas que entraram na mesa apareçam na lista
            for s in &current_sheets {
                if !list.iter().any(|e| e.id == s.id) {
                    list.push(InitiativeEntry {
                        id: s.id.clone(),
                        name: s.name.clone(),
                        is_npc: false,
                        is_active: true,
                        base_dex: s.dexterity,
                        base_wits: s.wits,
                        base_total: s.initiative_base,
                        health_penalty: s.health_penalty_val,
                        rolled_die: None,
                        final_total: None,
                    });
                }
            }

            // Atualiza bases de atributos caso a ficha tenha mudado
            for s in &current_sheets {
                if let Some(existing) = list.iter_mut().find(|e| e.id == s.id) {
                    existing.base_dex = s.dexterity;
                    existing.base_wits = s.wits;
                    existing.base_total = s.initiative_base;
                    existing.health_penalty = s.health_penalty_val;
                }
            }
        });
    });

    // Função auxiliar para persistir o estado de iniciativa no servidor quando o GM age
    let sync_to_server = move || {
        if !is_gm.get() {
            return;
        }
        let r_id = room_id.get();
        if r_id.is_empty() {
            return;
        }
        let payload = RoomInitiativeData {
            round: round.get(),
            is_open: is_open.get(),
            entries: entries.get(),
        };

        spawn_local(async move {
            let _ = update_room_initiative(r_id, payload).await;
        });
    };

    // Função de rolagem de iniciativa (WoD: Base + 1d10)
    let roll_initiative = move |_| {
        if !is_gm.get() {
            return;
        }

        play_dice_roll_sound();
        set_is_rolling.set(true);

        #[cfg(target_arch = "wasm32")]
        {
            let mut rng_buf = [0u8; 1];
            entries.update(|list| {
                for entry in list.iter_mut() {
                    if entry.is_active {
                        getrandom::getrandom(&mut rng_buf).ok();
                        let die = (rng_buf[0] % 10 + 1) as i32;
                        entry.rolled_die = Some(die);
                        entry.final_total = Some(entry.base_total + die);
                    } else {
                        entry.rolled_die = None;
                        entry.final_total = None;
                    }
                }

                // Ordenação decrescente: Maior total primeiro
                list.sort_by(|a, b| {
                    match (a.final_total, b.final_total) {
                        (Some(fa), Some(fb)) => {
                            fb.cmp(&fa)
                                .then_with(|| b.base_total.cmp(&a.base_total))
                                .then_with(|| b.base_dex.cmp(&a.base_dex))
                        }
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => b.base_total.cmp(&a.base_total),
                    }
                });
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            entries.update(|list| {
                for (i, entry) in list.iter_mut().enumerate() {
                    if entry.is_active {
                        let die = ((i as i32 * 3 + 7) % 10) + 1;
                        entry.rolled_die = Some(die);
                        entry.final_total = Some(entry.base_total + die);
                    }
                }
                list.sort_by(|a, b| b.final_total.unwrap_or(0).cmp(&a.final_total.unwrap_or(0)));
            });
        }

        sync_to_server();
        set_is_rolling.set(false);
    };

    let next_round = move |_| {
        if !is_gm.get() {
            return;
        }
        round.update(|r| *r += 1);
        entries.update(|list| {
            for e in list.iter_mut() {
                e.rolled_die = None;
                e.final_total = None;
            }
        });
        sync_to_server();
    };

    let add_npc = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if !is_gm.get() {
            return;
        }
        let raw_name = new_npc_name.get();
        let clean_name = raw_name.trim();
        if clean_name.is_empty() {
            return;
        }
        let base = new_npc_base.get().max(1);
        let new_id = format!("npc_{}_{}", clean_name.replace(' ', "_"), entries.with(|l| l.len()));

        entries.update(|list| {
            list.push(InitiativeEntry {
                id: new_id,
                name: clean_name.to_string(),
                is_npc: true,
                is_active: true,
                base_dex: 0,
                base_wits: 0,
                base_total: base,
                health_penalty: 0,
                rolled_die: None,
                final_total: None,
            });
        });

        set_new_npc_name.set(String::new());
        set_new_npc_base.set(6);
        sync_to_server();
    };

    let remove_npc = move |id: String| {
        if !is_gm.get() {
            return;
        }
        entries.update(|list| {
            list.retain(|e| e.id != id);
        });
        sync_to_server();
    };

    let clear_all_npcs = move |_| {
        if !is_gm.get() {
            return;
        }
        entries.update(|list| {
            list.retain(|e| !e.is_npc);
            for e in list.iter_mut() {
                e.rolled_die = None;
                e.final_total = None;
            }
        });
        round.set(1);
        sync_to_server();
    };

    view! {
        // Backdrop transparente para fechar ao clicar fora
        <div
            class=move || if is_open.get() { "initiative-backdrop active" } else { "initiative-backdrop" }
            on:click=move |_| set_is_open.set(false)
        />

        // Gaveta lateral retrátil
        <aside
            class=move || if is_open.get() { "initiative-drawer open" } else { "initiative-drawer" }
            aria-label="Rastreador de Iniciativa"
        >
            <div class="initiative-header">
                <div class="initiative-title-group">
                    <h3 class="initiative-title">"⚔️ Rastreador de Iniciativa"</h3>
                    <span class="initiative-round-badge">
                        {move || format!("Rodada {}", round.get())}
                    </span>
                </div>
                <button
                    class="initiative-close-btn"
                    on:click=move |_| set_is_open.set(false)
                    title="Fechar gaveta"
                >
                    "✕"
                </button>
            </div>

            <div class="initiative-body">
                // Mensagem explicativa rápida
                <div class="initiative-hint">
                    "Iniciativa WoD = (Destreza + Raciocínio) + 1d10. Ações em ordem decrescente."
                </div>

                // Tabela de Participantes e Iniciativa
                <div class="initiative-table-container">
                    <table class="initiative-table">
                        <thead>
                            <tr>
                                <th class="col-act" title="Participa do turno">"Ativo"</th>
                                <th class="col-name">"Participante"</th>
                                <th class="col-base" title="Destreza + Raciocínio">"Base"</th>
                                <th class="col-die" title="Resultado do 1d10">"d10"</th>
                                <th class="col-total" title="Iniciativa Final">"Total"</th>
                                {move || if is_gm.get() {
                                    view! { <th class="col-del">""</th> }.into_view()
                                } else {
                                    view! {}.into_view()
                                }}
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                let list = entries.get();
                                let is_gm_val = is_gm.get();
                                if list.is_empty() {
                                    return view! {
                                        <tr>
                                            <td colspan="6" class="initiative-empty">
                                                "Nenhum participante na iniciativa ainda."
                                            </td>
                                        </tr>
                                    }.into_view();
                                }

                                list.into_iter().enumerate().map(|(idx, entry)| {
                                    let is_leader = idx == 0 && entry.final_total.is_some();
                                    let row_class = if is_leader {
                                        "initiative-row leader"
                                    } else if !entry.is_active {
                                        "initiative-row inactive"
                                    } else {
                                        "initiative-row"
                                    };

                                    let entry_id = entry.id.clone();
                                    let toggle_id = entry.id.clone();
                                    let is_npc = entry.is_npc;

                                    view! {
                                        <tr class=row_class>
                                            <td class="col-act">
                                                {if is_gm_val {
                                                    view! {
                                                        <input type="checkbox" class="initiative-checkbox"
                                                            checked=entry.is_active
                                                            on:change=move |ev| {
                                                                let checked = event_target_checked(&ev);
                                                                let target_id = toggle_id.clone();
                                                                entries.update(|l| {
                                                                    if let Some(e) = l.iter_mut().find(|e| e.id == target_id) {
                                                                        e.is_active = checked;
                                                                    }
                                                                });
                                                                sync_to_server();
                                                            }
                                                        />
                                                    }.into_view()
                                                } else {
                                                    view! {
                                                        <span class="initiative-status-dot">
                                                            {if entry.is_active { "●" } else { "○" }}
                                                        </span>
                                                    }.into_view()
                                                }}
                                            </td>
                                            <td class="col-name">
                                                <div class="participant-info">
                                                    <span class="participant-icon">
                                                        {if entry.is_npc { "💀" } else { "🧙" }}
                                                    </span>
                                                    <span class="participant-name" title=entry.name.clone()>
                                                        {entry.name.clone()}
                                                    </span>
                                                </div>
                                            </td>
                                            <td class="col-base">
                                                {if entry.is_npc {
                                                    format!("{}", entry.base_total)
                                                } else if entry.health_penalty > 0 {
                                                    format!("{} ({}+{} -{})", entry.base_total, entry.base_dex, entry.base_wits, entry.health_penalty)
                                                } else {
                                                    format!("{} ({}+{})", entry.base_total, entry.base_dex, entry.base_wits)
                                                }}
                                            </td>
                                            <td class="col-die">
                                                {match entry.rolled_die {
                                                    Some(10) => view! { <span class="die-badge die-10">"10"</span> }.into_view(),
                                                    Some(1) => view! { <span class="die-badge die-1">"1"</span> }.into_view(),
                                                    Some(d) => view! { <span class="die-badge die-norm">{d}</span> }.into_view(),
                                                    None => view! { <span class="die-none">"-"</span> }.into_view(),
                                                }}
                                            </td>
                                            <td class="col-total">
                                                {match entry.final_total {
                                                    Some(tot) => view! { <span class="initiative-total-val">{tot}</span> }.into_view(),
                                                    None => view! { <span class="initiative-total-none">"-"</span> }.into_view(),
                                                }}
                                            </td>
                                            {if is_gm_val {
                                                if is_npc {
                                                    let del_id = entry_id.clone();
                                                    view! {
                                                        <td class="col-del">
                                                            <button
                                                                class="initiative-del-npc-btn"
                                                                on:click=move |_| remove_npc(del_id.clone())
                                                                title="Remover Inimigo"
                                                            >
                                                                "✕"
                                                            </button>
                                                        </td>
                                                    }.into_view()
                                                } else {
                                                    view! { <td class="col-del"></td> }.into_view()
                                                }
                                            } else {
                                                view! {}.into_view()
                                            }}
                                        </tr>
                                    }
                                }).collect::<Vec<_>>().into_view()
                            }}
                        </tbody>
                    </table>
                </div>

                // Painel de Ações do Mestre (GM Controls)
                {move || if is_gm.get() {
                    view! {
                        <div class="initiative-gm-controls">
                            <h4 class="initiative-section-title">"⚙️ Controles do Narrador"</h4>
                            
                            <div class="initiative-action-buttons">
                                <button
                                    class="initiative-btn-roll"
                                    on:click=roll_initiative
                                    disabled=is_rolling
                                >
                                    "🎲 Rolar Iniciativa"
                                </button>
                                <button
                                    class="initiative-btn-round"
                                    on:click=next_round
                                    title="Avançar para a próxima rodada"
                                >
                                    "🔄 Nova Rodada"
                                </button>
                                <button
                                    class="initiative-btn-clear"
                                    on:click=clear_all_npcs
                                    title="Remover todos os inimigos e reiniciar"
                                >
                                    "🧹 Limpar"
                                </button>
                            </div>

                            <form class="initiative-add-npc-form" on:submit=add_npc>
                                <div class="initiative-form-fields">
                                    <input
                                        type="text"
                                        class="initiative-npc-input"
                                        placeholder="Nome do Inimigo / NPC..."
                                        prop:value=new_npc_name
                                        on:input=move |ev| set_new_npc_name.set(event_target_value(&ev))
                                    />
                                    <div class="initiative-base-wrapper">
                                        <label class="initiative-base-label">"Base:"</label>
                                        <input
                                            type="number"
                                            class="initiative-base-input"
                                            min="1"
                                            max="20"
                                            prop:value=move || new_npc_base.get().to_string()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                    set_new_npc_base.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                                <button type="submit" class="initiative-add-btn">
                                    "➕ Adicionar Inimigo"
                                </button>
                            </form>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="initiative-player-notice">
                            "👁️ Modo Espectador: Acompanhe a ordem de iniciativa definida pelo Narrador."
                        </div>
                    }.into_view()
                }}
            </div>
        </aside>
    }
}
