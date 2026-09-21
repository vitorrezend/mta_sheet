use leptos::*;
use leptos_router::*;
use crate::state::{clone_sheet, get_sheet, update_sheet, CharacterData, DotOrigin, keys};
use crate::components::{Callback, Sheet};
use crate::components::mta_sheet::page1::{Abilities, AdvantagesMta, Attributes, InfoHeader, Spheres};
use crate::components::mta_sheet::page2::PageMagicCombat;
use crate::components::mta_sheet::page3::PageExpandedBackgroundsPossessions;
use crate::components::mta_sheet::page4::PageHistoryDescriptionVisuals;
use crate::components::mta_sheet::page5::{
    ArchetypeTarget, CompendiumSection, PageGrimoire, PracticeCompendiumContext, PracticeCompendiumModal,
};
use crate::components::mta_sheet::page6::PageNotes;
use crate::components::mta_sheet::sheet::{
    ActiveDotOriginContext, CostBreakdownModal, QuizModal, SaveStatus, SheetPageTab, SheetTabs, SheetTopBar,
    SheetShareModal,
};

#[derive(Clone, Debug, PartialEq)]
pub enum CompendiumTarget {
    Practice(Option<usize>),
    Instrument(Option<usize>),
    Archetype(Option<ArchetypeTarget>),
    Attribute,
    Background(Option<usize>),
    Sphere(Option<usize>),
    Weapon(Option<usize>),
    Ability(Option<usize>),
    MeritFlaw(Option<usize>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompendiumModalState {
    pub section: CompendiumSection,
    pub query: String,
    pub target: CompendiumTarget,
}

impl Default for CompendiumModalState {
    fn default() -> Self {
        Self {
            section: CompendiumSection::Practices,
            query: String::new(),
            target: CompendiumTarget::Practice(None),
        }
    }
}

fn get_current_time_str() -> &'static str {
    "agora"
}

#[component]
pub fn CharacterSheet() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let get_id_untracked = move || params.with_untracked(|p| p.get("id").cloned().unwrap_or_default());
    let token = move || query.with(|q| q.get("token").cloned());
    let get_token_untracked = move || query.with_untracked(|q| q.get("token").cloned());

    let sheet_resource = create_local_resource(
        move || (id(), token()),
        |(id, token)| async move {
            if id.is_empty() {
                return Err(ServerFnError::new("ID da ficha não fornecido"));
            }
            get_sheet(id, token).await
        }
    );

    let (data, set_data) = create_signal(CharacterData::default());
    let (save_status, set_save_status) = create_signal(SaveStatus::Idle);
    let (is_dirty, set_is_dirty) = create_signal(false);
    let (is_loaded, set_is_loaded) = create_signal(false);
    let (active_origin, set_active_origin) = create_signal(DotOrigin::Base);
    let (active_tab, set_active_tab) = create_signal(SheetPageTab::Main);
    let (show_breakdown, set_show_breakdown) = create_signal(false);
    let (show_share_modal, set_show_share_modal) = create_signal(false);
    let (show_clone_login_modal, set_show_clone_login_modal) = create_signal(false);
    let navigate = use_navigate();

    // Provide the sheet data and active dot origin as context for all child components
    provide_context(set_data);
    provide_context(data);
    provide_context(ActiveDotOriginContext {
        origin: active_origin,
        set_origin: set_active_origin,
    });

    let is_mounted = std::rc::Rc::new(std::cell::Cell::new(true));
    let is_mounted_cleanup = is_mounted.clone();
    let change_seq = std::rc::Rc::new(std::cell::Cell::new(0u64));

    // Salva automaticamente no unmount (ao navegar para qualquer outro lugar da aplicação)
    on_cleanup(move || {
        is_mounted_cleanup.set(false);
        if is_dirty.try_get_untracked().unwrap_or(false) {
            let current_id = get_id_untracked();
            if let Some(current_data) = data.try_get_untracked() {
                if !current_data.can_edit {
                    return;
                }
                if !current_id.is_empty() {
                    spawn_local(async move {
                        let _ = update_sheet(current_id.clone(), current_data).await;
                        crate::logging::log_client(
                            "user_actions",
                            "INFO",
                            "Ficha salva automaticamente ao sair da página (on_cleanup)",
                            Some(&format!("id={}", current_id)),
                        );
                    });
                }
            }
        }
    });

    create_effect(move |_| {
        if let Some(Ok(fetched_data)) = sheet_resource.get() {
            let _ = set_data.try_set(fetched_data);
            let _ = set_is_loaded.try_set(true);
            let _ = set_is_dirty.try_set(false);
            let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
            crate::logging::log_client(
                "user_actions",
                "INFO",
                &format!("Ficha carregada no navegador: id='{}'", get_id_untracked()),
                None,
            );
        }
    });

    // Marca o formulário como alterado (dirty) e dispara auto-save inteligente (Debounce de 1.2s)
    let change_seq_dirty = change_seq.clone();
    let is_mounted_dirty = is_mounted.clone();
    create_effect(move |_| {
        data.track();
        if is_loaded.try_get_untracked().unwrap_or(false) {
            if let Some(current_data) = data.try_get_untracked() {
                if !current_data.can_edit {
                    return;
                }
            }
            let _ = set_is_dirty.try_set(true);
            let _ = set_save_status.try_set(SaveStatus::Pending);

            let next_seq = change_seq_dirty.get() + 1;
            change_seq_dirty.set(next_seq);

            let seq_check = change_seq_dirty.clone();
            let is_mounted_task = is_mounted_dirty.clone();

            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(1_200).await;
                if seq_check.get() == next_seq && is_dirty.try_get_untracked().unwrap_or(false) {
                    let current_id = get_id_untracked();
                    if let Some(current_data) = data.try_get_untracked() {
                        if !current_data.can_edit {
                            return;
                        }
                        if !current_id.is_empty() {
                            if is_mounted_task.get() {
                                let _ = set_save_status.try_set(SaveStatus::Saving);
                            }
                            match update_sheet(current_id.clone(), current_data).await {
                                Ok(_) => {
                                    if seq_check.get() == next_seq {
                                        let _ = set_is_dirty.try_set(false);
                                        if is_mounted_task.get() {
                                            let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
                                        }
                                        crate::logging::log_client(
                                            "database",
                                            "INFO",
                                            "Auto-save inteligente (debounce 1.2s) executado com sucesso",
                                            Some(&format!("id={}", current_id)),
                                        );
                                    }
                                }
                                Err(e) => {
                                    if is_mounted_task.get() {
                                        let _ = set_save_status.try_set(SaveStatus::Error(e.to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    // Salvamento Periódico como redundância de segurança a cada 30 segundos
    let is_mounted_loop = is_mounted.clone();
    create_effect(move |_| {
        if is_loaded.get() {
            let is_mounted_task = is_mounted_loop.clone();
            spawn_local(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(30_000).await;
                    if !is_mounted_task.get() {
                        break;
                    }

                    let current_data = match data.try_get_untracked() {
                        Some(d) => d,
                        None => break,
                    };

                    if !current_data.can_edit {
                        break;
                    }

                    if is_dirty.try_get_untracked().unwrap_or(false) {
                        let current_id = get_id_untracked();
                        if !current_id.is_empty() && is_mounted_task.get() {
                            let _ = set_save_status.try_set(SaveStatus::Saving);
                            match update_sheet(current_id.clone(), current_data).await {
                                Ok(_) => {
                                    if !is_mounted_task.get() { break; }
                                    let _ = set_is_dirty.try_set(false);
                                    let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
                                }
                                Err(e) => {
                                    if !is_mounted_task.get() { break; }
                                    let _ = set_save_status.try_set(SaveStatus::Error(e.to_string()));
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    // Salvamento manual
    let do_manual_save = Callback::new(move |_: ev::MouseEvent| {
        let current_id = get_id_untracked();
        if let Some(current_data) = data.try_get_untracked() {
            if !current_data.can_edit {
                return;
            }
            if !current_id.is_empty() {
                let _ = set_save_status.try_set(SaveStatus::Saving);
                spawn_local(async move {
                    match update_sheet(current_id.clone(), current_data).await {
                        Ok(_) => {
                            let _ = set_is_dirty.try_set(false);
                            crate::logging::log_client(
                                "user_actions",
                                "INFO",
                                "Salvamento manual acionado pelo usuário",
                                Some(&format!("id={}", current_id)),
                            );
                            let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
                        }
                        Err(e) => {
                            crate::logging::log_client(
                                "errors",
                                "ERROR",
                                "Falha no salvamento manual",
                                Some(&e.to_string()),
                            );
                            let _ = set_save_status.try_set(SaveStatus::Error(e.to_string()));
                        }
                    }
                });
            }
        }
    });

    // Navegação ao clicar em "← Início" garantindo salvamento antes de sair
    let nav_for_back = navigate.clone();
    let on_back_click = Callback::new(move |ev: ev::MouseEvent| {
        ev.prevent_default();
        if is_dirty.try_get_untracked().unwrap_or(false) {
            let current_id = get_id_untracked();
            if let Some(current_data) = data.try_get_untracked() {
                if !current_data.can_edit {
                    nav_for_back.clone()("/", Default::default());
                    return;
                }
                let nav = nav_for_back.clone();
                let _ = set_save_status.try_set(SaveStatus::Saving);
                spawn_local(async move {
                    if !current_id.is_empty() {
                        let _ = update_sheet(current_id.clone(), current_data).await;
                        crate::logging::log_client(
                            "user_actions",
                            "INFO",
                            "Ficha salva automaticamente ao navegar para a tela inicial",
                            Some(&format!("id={}", current_id)),
                        );
                    }
                    nav("/", Default::default());
                });
            } else {
                nav_for_back.clone()("/", Default::default());
            }
        } else {
            nav_for_back.clone()("/", Default::default());
        }
    });

    let (is_cloning, set_is_cloning) = create_signal(false);
    let nav_for_clone = navigate.clone();
    let on_clone_sheet = Callback::new(move |_: ()| {
        let current_id = get_id_untracked();
        if current_id.is_empty() || is_cloning.get() {
            return;
        }
        set_is_cloning.set(true);
        let nav = nav_for_clone.clone();
        let tok = get_token_untracked();
        spawn_local(async move {
            match clone_sheet(current_id, tok).await {
                Ok(new_id) => {
                    crate::logging::log_client(
                        "user_actions",
                        "INFO",
                        "Ficha clonada com sucesso",
                        Some(&format!("new_id={}", new_id)),
                    );
                    nav(&format!("/sheet/{}", new_id), Default::default());
                }
                Err(e) => {
                    crate::logging::log_client(
                        "errors",
                        "ERROR",
                        "Erro ao clonar ficha",
                        Some(&e.to_string()),
                    );
                    set_is_cloning.set(false);
                    let err_str = e.to_string();
                    if err_str.contains("logado") || err_str.contains("autenticado") {
                        set_show_clone_login_modal.set(true);
                    } else if let Some(w) = web_sys::window() {
                        let _ = w.alert_with_message(&format!("Não foi possível clonar a ficha: {}", err_str));
                    }
                }
            }
        });
    });

    let can_edit = Signal::derive(move || data.with(|d| d.can_edit));
    let author_username = Signal::derive(move || data.with(|d| d.author_username.clone()));

    let costs = create_memo(move |_| data.with(|d| d.calculate_costs()));
    let is_public = Signal::derive(move || data.with(|d| d.is_public));

    let on_export_json = Callback::new(move |_| {
        if let Some(current_data) = data.try_get_untracked() {
            crate::components::common::export_character_json(&current_data);
        }
    });

    let on_import_json = Callback::new(move |mut imported_data: CharacterData| {
        let current_id = get_id_untracked();
        imported_data.id = current_id.clone();
        set_data.set(imported_data.clone());
        let _ = set_is_dirty.try_set(true);
        let _ = set_save_status.try_set(SaveStatus::Saving);
        spawn_local(async move {
            if !current_id.is_empty() {
                match update_sheet(current_id.clone(), imported_data).await {
                    Ok(_) => {
                        let _ = set_is_dirty.try_set(false);
                        let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
                        crate::logging::log_client(
                            "user_actions",
                            "INFO",
                            "Ficha importada de arquivo JSON e salva no banco",
                            Some(&format!("id={}", current_id)),
                        );
                    }
                    Err(e) => {
                        let _ = set_save_status.try_set(SaveStatus::Error(e.to_string()));
                    }
                }
            }
        });
    });

    let on_toggle_privacy = Callback::new(move |_| {
        let current_id = get_id_untracked();
        let current_pub = data.with_untracked(|d| d.is_public);
        let new_pub = !current_pub;
        set_data.update(|d| d.is_public = new_pub);
        if !current_id.is_empty() {
            spawn_local(async move {
                let _ = crate::state::set_sheet_visibility(current_id, new_pub).await;
            });
        }
    });

    let is_gods_and_monsters = create_memo(move |_| data.with(|d| d.is_gods_and_monsters()));
    let (show_quiz_modal, set_show_quiz_modal) = create_signal(false);
    let (show_pdf_modal, set_show_pdf_modal) = create_signal(false);
    let (show_practice_modal, set_show_practice_modal) = create_signal(false);
    let (compendium_state, set_compendium_state) = create_signal(CompendiumModalState::default());

    let open_practice_compendium = Callback::new(move |(slot, query): (Option<usize>, String)| {
        set_compendium_state.set(CompendiumModalState {
            section: CompendiumSection::Practices,
            query,
            target: CompendiumTarget::Practice(slot),
        });
        set_show_practice_modal.set(true);
    });

    let open_instrument_compendium = Callback::new(move |(slot, query): (Option<usize>, String)| {
        set_compendium_state.set(CompendiumModalState {
            section: CompendiumSection::Instruments,
            query,
            target: CompendiumTarget::Instrument(slot),
        });
        set_show_practice_modal.set(true);
    });

    let open_archetype_compendium = Callback::new(move |(target, query): (Option<ArchetypeTarget>, String)| {
        set_compendium_state.set(CompendiumModalState {
            section: CompendiumSection::Archetypes,
            query,
            target: CompendiumTarget::Archetype(target),
        });
        set_show_practice_modal.set(true);
    });

    let open_attribute_compendium = Callback::new(move |(_slot, query): (Option<usize>, String)| {
        set_compendium_state.set(CompendiumModalState {
            section: CompendiumSection::Attributes,
            query,
            target: CompendiumTarget::Attribute,
        });
        set_show_practice_modal.set(true);
    });

    let open_weapon_compendium = Callback::new(move |(slot, query): (Option<usize>, String)| {
        set_compendium_state.set(CompendiumModalState {
            section: CompendiumSection::Weapons,
            query,
            target: CompendiumTarget::Weapon(slot),
        });
        set_show_practice_modal.set(true);
    });

    let open_background_compendium = Callback::new(move |(slot, query): (Option<usize>, String)| {
        set_compendium_state.set(CompendiumModalState {
            section: CompendiumSection::Backgrounds,
            query,
            target: CompendiumTarget::Background(slot),
        });
        set_show_practice_modal.set(true);
    });

    let open_sphere_compendium = Callback::new(move |(slot, query): (Option<usize>, String)| {
        set_compendium_state.set(CompendiumModalState {
            section: CompendiumSection::Spheres,
            query,
            target: CompendiumTarget::Sphere(slot),
        });
        set_show_practice_modal.set(true);
    });

    let open_ability_compendium = Callback::new(move |(slot, query): (Option<usize>, String)| {
        set_compendium_state.set(CompendiumModalState {
            section: CompendiumSection::Abilities,
            query,
            target: CompendiumTarget::Ability(slot),
        });
        set_show_practice_modal.set(true);
    });

    let open_merit_flaw_compendium = Callback::new(move |(slot, query): (Option<usize>, String)| {
        set_compendium_state.set(CompendiumModalState {
            section: CompendiumSection::MeritsFlaws,
            query,
            target: CompendiumTarget::MeritFlaw(slot),
        });
        set_show_practice_modal.set(true);
    });

    provide_context(PracticeCompendiumContext {
        open: open_practice_compendium.clone(),
        open_practice: open_practice_compendium,
        open_instrument: open_instrument_compendium,
        open_archetype: open_archetype_compendium,
        open_attribute: open_attribute_compendium,
        open_weapon: open_weapon_compendium,
        open_background: open_background_compendium,
        open_sphere: open_sphere_compendium,
        open_ability: open_ability_compendium,
        open_merit_flaw: open_merit_flaw_compendium,
    });

    let on_practice_selected_from_modal = Callback::new(move |selected_name: String| {
        let slot = match compendium_state.with(|s| s.target.clone()) {
            CompendiumTarget::Practice(slot) => slot,
            _ => None,
        };
        if let Some(idx) = slot {
            set_data.update(|s| {
                if s.grimoire.practices.is_empty() {
                    s.grimoire.practices = vec![String::new(), String::new(), String::new()];
                }
                while s.grimoire.practices.len() <= idx {
                    s.grimoire.practices.push(String::new());
                }
                s.grimoire.practices[idx] = selected_name;
            });
            let _ = set_is_dirty.try_set(true);
        }
    });

    let on_instrument_selected_from_modal = Callback::new(move |selected_name: String| {
        let slot = match compendium_state.with(|s| s.target.clone()) {
            CompendiumTarget::Instrument(slot) => slot,
            _ => None,
        };
        if let Some(idx) = slot {
            set_data.update(|s| {
                if s.grimoire.instruments.is_empty() {
                    s.grimoire.instruments = vec![String::new(), String::new(), String::new()];
                }
                while s.grimoire.instruments.len() <= idx {
                    s.grimoire.instruments.push(String::new());
                }
                s.grimoire.instruments[idx] = selected_name;
            });
            let _ = set_is_dirty.try_set(true);
        }
    });

    let on_archetype_selected_from_modal = Callback::new(move |(target, selected_name): (ArchetypeTarget, String)| {
        let field_key = match target {
            ArchetypeTarget::Nature => "Natureza",
            ArchetypeTarget::Demeanor => "Comportamento",
        };
        set_data.update(|s| {
            s.set_label(field_key, selected_name);
        });
        let _ = set_is_dirty.try_set(true);
    });

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let current_lang_fn = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let on_weapon_selected_from_modal = Callback::new(move |(slot_opt, w_def): (Option<usize>, &'static crate::compendium::weapons::WeaponDefinition)| {
        let lang = current_lang_fn();
        set_data.update(|s| {
            let target_idx = slot_opt.unwrap_or_else(|| {
                s.weapons.iter().position(|w| w.name.trim().is_empty()).unwrap_or(s.weapons.len())
            });
            while s.weapons.len() <= target_idx {
                s.weapons.push(crate::state::WeaponItem::default());
            }
            s.weapons[target_idx] = crate::state::WeaponItem {
                name: w_def.name(lang).to_string(),
                diff: w_def.difficulty.to_string(),
                damage: w_def.damage(lang).to_string(),
                range: w_def.range.to_string(),
                rate: w_def.rate.to_string(),
                clip: w_def.clip.to_string(),
                conceal: w_def.conceal.to_string(),
                notes: w_def.format_notes_str(lang),
            };
        });
        let _ = set_is_dirty.try_set(true);
    });

    let on_maneuver_selected_from_modal = Callback::new(move |(slot_opt, m_def): (Option<usize>, &'static crate::compendium::weapons::CombatManeuver)| {
        let lang = current_lang_fn();
        set_data.update(|s| {
            let target_idx = slot_opt.unwrap_or_else(|| {
                s.weapons.iter().position(|w| w.name.trim().is_empty()).unwrap_or(s.weapons.len())
            });
            while s.weapons.len() <= target_idx {
                s.weapons.push(crate::state::WeaponItem::default());
            }
            s.weapons[target_idx] = m_def.to_weapon_item(lang);
        });
        let _ = set_is_dirty.try_set(true);
    });

    let on_background_selected_from_modal = Callback::new(move |(slot_opt, selected_name, selected_level): (Option<usize>, String, i32)| {
        let slot = slot_opt.or_else(|| match compendium_state.with(|s| s.target.clone()) {
            CompendiumTarget::Background(slot) => slot,
            _ => None,
        });
        let current_origin = active_origin.get();
        set_data.update(|s| {
            let list = s.custom_lists.entry("Antecedentes".to_string()).or_default();
            let target_id = if let Some(idx) = slot {
                if idx < list.len() {
                    list[idx].clone()
                } else {
                    let id = format!("bg_{}", uuid::Uuid::new_v4());
                    list.push(id.clone());
                    id
                }
            } else {
                let id = format!("bg_{}", uuid::Uuid::new_v4());
                list.push(id.clone());
                id
            };
            s.labels.insert(target_id.clone(), selected_name);
            s.set_attribute_with_origin(&target_id, Some(selected_level), None, current_origin);
        });
        let _ = set_is_dirty.try_set(true);
    });

    let on_merit_flaw_selected_from_modal = Callback::new(move |(slot_opt, selected_name, selected_cost, is_flaw): (Option<usize>, String, i32, bool)| {
        let slot = slot_opt.or_else(|| match compendium_state.with(|s| s.target.clone()) {
            CompendiumTarget::MeritFlaw(slot) => slot,
            _ => None,
        });
        let category = if is_flaw { keys::CAT_FLAWS } else { keys::CAT_MERITS };
        let prefix = if is_flaw { "flaw" } else { "merit" };
        let current_origin = active_origin.get();
        set_data.update(|s| {
            let list = s.custom_lists.entry(category.to_string()).or_default();
            let target_id = if let Some(idx) = slot {
                if idx < list.len() {
                    list[idx].clone()
                } else {
                    let id = format!("{}_{}", prefix, uuid::Uuid::new_v4());
                    list.push(id.clone());
                    id
                }
            } else {
                let id = format!("{}_{}", prefix, uuid::Uuid::new_v4());
                list.push(id.clone());
                id
            };
            s.labels.insert(target_id.clone(), selected_name);
            s.set_attribute_with_origin(&target_id, Some(selected_cost), None, current_origin);
        });
        let _ = set_is_dirty.try_set(true);
    });

    let on_ability_selected_from_modal = Callback::new(move |(slot_opt, selected_name): (Option<usize>, String)| {
        let slot = slot_opt.or_else(|| match compendium_state.with(|s| s.target.clone()) {
            CompendiumTarget::Ability(slot) => slot,
            _ => None,
        });
        let category = {
            let matched = crate::compendium::abilities::ALL_ABILITIES.iter().find(|a| {
                selected_name.starts_with(a.name_pt) || selected_name.starts_with(a.name) || selected_name.starts_with(a.id)
            });
            match matched.map(|a| a.category) {
                Some(crate::compendium::abilities::AbilityCategory::Talents) => "Talentos",
                Some(crate::compendium::abilities::AbilityCategory::Skills) => "Perícias",
                Some(crate::compendium::abilities::AbilityCategory::Knowledges) => "Conhecimentos",
                None => "Talentos",
            }
        };
        let category_prefix = match category {
            "Talentos" => "tal",
            "Perícias" => "per",
            "Conhecimentos" => "con",
            _ => "ab",
        };
        set_data.update(|s| {
            let list = s.custom_lists.entry(category.to_string()).or_default();
            let target_id = if let Some(idx) = slot {
                if idx < list.len() {
                    list[idx].clone()
                } else {
                    let id = format!("ab_{}_{}", category_prefix, uuid::Uuid::new_v4());
                    list.push(id.clone());
                    id
                }
            } else {
                let id = format!("ab_{}_{}", category_prefix, uuid::Uuid::new_v4());
                list.push(id.clone());
                id
            };
            s.labels.insert(target_id.clone(), selected_name);
        });
        let _ = set_is_dirty.try_set(true);
    });

    let active_spheres_signal = Signal::derive(move || {
        data.with(|s| s.get_active_spheres())
    });

    let on_sphere_selected_from_modal = Callback::new(move |chosen_id: String| {
        let mut changed = false;
        set_data.update(|s| {
            changed = s.swap_sphere_variant(&chosen_id);
        });
        if changed {
            let _ = set_is_dirty.try_set(true);
        }
    });

    view! {
        <div class="sheet-page-container" class:sheet-readonly=move || !can_edit.get()>
            <leptos_meta::Title text=move || format!("{} | MTA Sheet", data.with(|d| d.get_display_name())) />
            <leptos_meta::Meta
                name="description"
                content=move || {
                    data.with(|d| {
                        let name = d.get_display_name();
                        let trad_val = d.get_tradition();
                        let tradition = if trad_val.trim().is_empty() { "Mago" } else { trad_val.trim() };
                        let concept = d.get_label("Concept");
                        let concept_trimmed = concept.trim();
                        if concept_trimmed.is_empty() {
                            format!("Ficha de personagem {} ({}) para Mago: A Ascensão (M20) no MTA Sheet.", name, tradition)
                        } else {
                            format!("Ficha de personagem {} ({}) - Conceito: {}. Sistema Mago: A Ascensão (M20).", name, tradition, concept_trimmed)
                        }
                    })
                }
            />
            <leptos_meta::Meta
                name="robots"
                content=move || if is_public.get() { "index, follow" } else { "noindex, nofollow" }
            />
            <leptos_meta::Meta property="og:title" content=move || format!("{} | MTA Sheet", data.with(|d| d.get_display_name())) />
            <leptos_meta::Meta
                property="og:description"
                content=move || {
                    data.with(|d| {
                        let name = d.get_display_name();
                        let trad_val = d.get_tradition();
                        let tradition = if trad_val.trim().is_empty() { "Mago" } else { trad_val.trim() };
                        format!("Ficha de personagem {} ({}) para Mago: A Ascensão (M20) no MTA Sheet.", name, tradition)
                    })
                }
            />
            // Barra Superior e Seletor de Modos
            <SheetTopBar 
                active_origin=active_origin
                set_active_origin=set_active_origin
                costs=costs
                set_show_breakdown=set_show_breakdown
                set_show_quiz=set_show_quiz_modal
                save_status=save_status
                is_public=is_public
                on_toggle_privacy=on_toggle_privacy
                on_back_click=on_back_click
                do_manual_save=do_manual_save
                on_export_json=on_export_json
                on_import_json=on_import_json
                set_show_pdf_modal=set_show_pdf_modal
                set_show_share_modal=set_show_share_modal
                can_edit=can_edit
                author_username=author_username
                on_clone_sheet=on_clone_sheet
            />

            // Modal de Compartilhamento (Estilo Google Drive)
            <SheetShareModal
                show_modal=show_share_modal
                set_show_modal=set_show_share_modal
                sheet_id=Signal::derive(move || get_id_untracked())
                sheet_name=Signal::derive(move || data.with(|d| d.get_display_name()))
            />

            // Modal de Aviso de Login para Clonar
            <crate::components::common::Modal
                is_open=show_clone_login_modal
                on_close=crate::components::common::SafeCallback::new(move |_| set_show_clone_login_modal.set(false))
                title="Salvar Cópia da Ficha"
                icon=Some("📋")
                size=crate::components::common::ModalSize::Sm
            >
                <div class="clone-prompt-content">
                    <div class="clone-prompt-icon">"✨"</div>
                    <h4 class="clone-prompt-title">"Crie sua cópia pessoal!"</h4>
                    <p class="clone-prompt-desc">
                        "Para clonar esta ficha e editá-la livremente em sua própria biblioteca, faça login na sua conta ou crie um cadastro gratuito."
                    </p>
                    <div class="clone-prompt-actions">
                        <a href="/login" class="clone-prompt-login-btn">
                            <span>"🔑"</span>
                            <span>"Entrar ou Cadastrar"</span>
                        </a>
                        <button
                            type="button"
                            class="clone-prompt-close-btn"
                            on:click=move |_| set_show_clone_login_modal.set(false)
                        >
                            "Fechar"
                        </button>
                    </div>
                </div>
            </crate::components::common::Modal>

            // Modal de Extrato de Custos
            <CostBreakdownModal 
                costs=costs
                show_breakdown=show_breakdown
                set_show_breakdown=set_show_breakdown
                set_data=set_data
            />

            // Modal de Dossiê e Questionário de Criação (Anexo)
            <QuizModal 
                show_quiz=show_quiz_modal
                set_show_quiz=set_show_quiz_modal
                data=data
                set_data=set_data
            />

            // Modal de Opções de Exportação para PDF (Econômico / Inteligente)
            <crate::components::mta_sheet::sheet::pdf_export_modal::PdfExportModal
                show_modal=show_pdf_modal
                set_show_modal=set_show_pdf_modal
                data=data
                current_active_tab=active_tab.into()
                is_gods_and_monsters=is_gods_and_monsters
            />

            // Barra de Navegação de Abas (Páginas da Ficha)
            <SheetTabs 
                active_tab=active_tab
                set_active_tab=set_active_tab
                is_gods_and_monsters=is_gods_and_monsters.into()
            />

            <div class="sheet-main-content">
                {move || {
                    if !is_loaded.get() {
                        match sheet_resource.get() {
                            Some(Err(e)) => view! { 
                                <div class="error-container">
                                    <p class="error-title">"Erro ao carregar a ficha"</p>
                                    <p class="error-detail">{e.to_string()}</p>
                                    <A href="/" class="back-home-btn">"Voltar para a lista de fichas"</A>
                                </div>
                            }.into_view(),
                            _ => view! {
                                <div class="loading-state"><p>"Carregando Ficha..."</p></div>
                            }.into_view(),
                        }
                    } else {
                        let is_gm = is_gods_and_monsters.get();
                        if is_gm {
                            view! {
                                <Sheet>
                                    <div 
                                        class="sheet-page-tab-pane page-gods-1"
                                        class:tab-hidden=move || active_tab.get() != SheetPageTab::Main
                                    >
                                        <crate::components::gods_and_monsters::GodsAndMonstersPage1 />
                                    </div>

                                    <div 
                                        class="sheet-page-tab-pane page-gods-2"
                                        class:tab-hidden=move || active_tab.get() != SheetPageTab::MagicCombat
                                    >
                                        <crate::components::gods_and_monsters::GodsAndMonstersPage2 />
                                    </div>
                                </Sheet>
                            }.into_view()
                        } else {
                            view! {
                                <Sheet>
                                    <div 
                                        class="sheet-page-tab-pane page-main"
                                        class:tab-hidden=move || active_tab.get() != SheetPageTab::Main
                                    >
                                        <InfoHeader />
                                        <Attributes />
                                        <Abilities />
                                        <Spheres />
                                        <AdvantagesMta />
                                    </div>

                                    <div 
                                        class="sheet-page-tab-pane page-magic-combat"
                                        class:tab-hidden=move || active_tab.get() != SheetPageTab::MagicCombat
                                    >
                                        <PageMagicCombat />
                                    </div>

                                    <div 
                                        class="sheet-page-tab-pane page-expanded"
                                        class:tab-hidden=move || active_tab.get() != SheetPageTab::Expanded
                                    >
                                        <PageExpandedBackgroundsPossessions />
                                    </div>

                                    <div 
                                        class="sheet-page-tab-pane page-history-visuals"
                                        class:tab-hidden=move || active_tab.get() != SheetPageTab::HistoryVisuals
                                    >
                                        <PageHistoryDescriptionVisuals />
                                    </div>

                                    <div 
                                        class="sheet-page-tab-pane page-grimoire"
                                        class:tab-hidden=move || active_tab.get() != SheetPageTab::Grimoire
                                    >
                                        <PageGrimoire />
                                    </div>

                                    <div 
                                        class="sheet-page-tab-pane page-notes"
                                        class:tab-hidden=move || active_tab.get() != SheetPageTab::Notes
                                    >
                                        <PageNotes />
                                    </div>
                                </Sheet>
                            }.into_view()
                        }
                    }
                }}
            </div>

            // Modal de Compêndio M20 Unificado (Práticas, Instrumentos, Arquétipos, Atributos & Armas)
            // Montado na raiz final para isolamento de z-index e precedência de pintura sobre abas e conteúdo
            <PracticeCompendiumModal
                show_modal=show_practice_modal
                set_show_modal=set_show_practice_modal
                initial_query=Signal::derive(move || compendium_state.with(|s| s.query.clone()))
                initial_section=Some(Signal::derive(move || compendium_state.with(|s| s.section)).into())
                on_select_practice=on_practice_selected_from_modal
                on_select_instrument=on_instrument_selected_from_modal
                on_select_archetype=on_archetype_selected_from_modal
                initial_archetype_target=Some(Signal::derive(move || match compendium_state.with(|s| s.target.clone()) {
                    CompendiumTarget::Archetype(t) => t,
                    _ => None,
                }).into())
                target_slot=Some(Signal::derive(move || match compendium_state.with(|s| s.target.clone()) {
                    CompendiumTarget::Weapon(w) => w,
                    CompendiumTarget::Background(b) => b,
                    CompendiumTarget::Ability(a) => a,
                    CompendiumTarget::MeritFlaw(m) => m,
                    _ => None,
                }).into())
                on_select_weapon=on_weapon_selected_from_modal
                on_select_maneuver=on_maneuver_selected_from_modal
                on_select_background=on_background_selected_from_modal
                on_select_sphere=on_sphere_selected_from_modal
                on_select_ability=on_ability_selected_from_modal
                on_select_merit_flaw=on_merit_flaw_selected_from_modal
                active_spheres=active_spheres_signal
            />
        </div>
    }
}
