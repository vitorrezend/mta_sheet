use leptos::*;
use crate::compendium::archetypes::find_archetype;
use crate::compendium::attributes::find_attribute;
use crate::compendium::instruments::{find_instrument, find_theory_article};
use crate::compendium::practices::find_practice;
use crate::compendium::weapons::{find_weapon, CombatManeuver, WeaponDefinition};
use crate::components::Callback;
use crate::i18n::Language;

pub mod views;
pub use views::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompendiumSection {
    Practices,
    Instruments,
    Archetypes,
    Attributes,
    Weapons,
}

#[component]
pub fn CompendiumModal(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    #[prop(into)] initial_query: Signal<String>,
    #[prop(into, default = None)] initial_section: Option<Signal<CompendiumSection>>,
    #[prop(into, default = None)] on_select_practice: Option<Callback<String>>,
    #[prop(into, default = None)] on_select_instrument: Option<Callback<String>>,
    #[prop(into, default = None)] on_select_archetype: Option<Callback<(ArchetypeTarget, String)>>,
    #[prop(into, default = None)] initial_archetype_target: Option<Signal<Option<ArchetypeTarget>>>,
    #[prop(into, default = None)] target_slot: Option<Signal<Option<usize>>>,
    #[prop(into, default = None)] on_select_weapon: Option<Callback<(Option<usize>, &'static WeaponDefinition)>>,
    #[prop(into, default = None)] on_select_maneuver: Option<Callback<(Option<usize>, &'static CombatManeuver)>>,
) -> impl IntoView {
    // Seção ativa (Práticas, Instrumentos, Arquétipos, Atributos ou Armas)
    let active_section = create_rw_signal(CompendiumSection::Practices);

    // ID da prática atualmente selecionada
    let selected_practice_id = create_rw_signal("alchemy".to_string());

    // ID do instrumento atualmente selecionado
    let selected_instrument_id = create_rw_signal("tools_of_focus".to_string());

    // Histórico para navegação entre Práticas e Instrumentos
    let history_practice_id = create_rw_signal(Option::<String>::None);

    // ID do arquétipo atualmente selecionado
    let selected_archetype_id = create_rw_signal("activist".to_string());

    // Alvo do arquétipo (Natureza ou Comportamento)
    let active_archetype_target = create_rw_signal(Option::<ArchetypeTarget>::None);

    // ID do atributo atualmente selecionado
    let selected_attribute_id = create_rw_signal("strength".to_string());

    // ID da arma atualmente selecionada
    let selected_weapon_id = create_rw_signal("katana".to_string());

    // Controle de tela Master-Detail no Mobile (false = lista de itens, true = detalhes do item)
    let mobile_show_detail = create_rw_signal(false);

    let prev_show = create_rw_signal(false);

    // Trava de rolagem do documento (body e html) enquanto o compêndio estiver aberto
    create_effect(move |_| {
        let is_open = show_modal.get();
        if is_open {
            mobile_show_detail.set(false);
        }
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(body) = doc.body() {
                let style = body.style();
                if is_open {
                    let _ = style.set_property("overflow", "hidden");
                } else {
                    let _ = style.remove_property("overflow");
                }
            }
            if let Some(doc_elem) = doc.document_element() {
                use wasm_bindgen::JsCast;
                if let Ok(html_el) = doc_elem.dyn_into::<web_sys::HtmlElement>() {
                    let style = html_el.style();
                    if is_open {
                        let _ = style.set_property("overflow", "hidden");
                    } else {
                        let _ = style.remove_property("overflow");
                    }
                }
            }
        }
    });

    on_cleanup(move || {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(body) = doc.body() {
                let _ = body.style().remove_property("overflow");
            }
            if let Some(doc_elem) = doc.document_element() {
                use wasm_bindgen::JsCast;
                if let Ok(html_el) = doc_elem.dyn_into::<web_sys::HtmlElement>() {
                    let _ = html_el.style().remove_property("overflow");
                }
            }
        }
    });

    // Sincroniza a seleção quando o modal abre (transição false -> true)
    create_effect(move |_| {
        let is_open = show_modal.get();
        let was_open = prev_show.get_untracked();

        if is_open && !was_open {
            prev_show.set(true);

            let mut sec = initial_section
                .map(|s| s.get_untracked())
                .unwrap_or(CompendiumSection::Practices);

            if let Some(target_sig) = initial_archetype_target {
                active_archetype_target.set(target_sig.get_untracked());
            }

            let q = initial_query.get_untracked();
            let trimmed = q.trim();

            if !trimmed.is_empty() {
                // Tenta casar primeiro com a seção já ativa indicada
                match sec {
                    CompendiumSection::Weapons => {
                        if let Some(matched_w) = find_weapon(trimmed) {
                            selected_weapon_id.set(matched_w.id.to_string());
                        } else if let Some(matched_attr) = find_attribute(trimmed) {
                            selected_attribute_id.set(matched_attr.id.to_string());
                            sec = CompendiumSection::Attributes;
                        } else if let Some(matched_p) = find_practice(trimmed) {
                            selected_practice_id.set(matched_p.id.to_string());
                            sec = CompendiumSection::Practices;
                        }
                    }
                    CompendiumSection::Attributes => {
                        if let Some(matched_attr) = find_attribute(trimmed) {
                            selected_attribute_id.set(matched_attr.id.to_string());
                        } else if trimmed.eq_ignore_ascii_case("rule")
                            || trimmed.eq_ignore_ascii_case("regras")
                            || trimmed.eq_ignore_ascii_case("especialidades")
                            || trimmed.eq_ignore_ascii_case("specialties")
                        {
                            selected_attribute_id.set("rule_specialties".to_string());
                        } else if let Some(matched_w) = find_weapon(trimmed) {
                            selected_weapon_id.set(matched_w.id.to_string());
                            sec = CompendiumSection::Weapons;
                        } else if let Some(matched_p) = find_practice(trimmed) {
                            selected_practice_id.set(matched_p.id.to_string());
                            sec = CompendiumSection::Practices;
                        }
                    }
                    CompendiumSection::Archetypes => {
                        if let Some(matched_a) = find_archetype(trimmed) {
                            selected_archetype_id.set(matched_a.id.to_string());
                        } else if trimmed.eq_ignore_ascii_case("theory")
                            || trimmed.eq_ignore_ascii_case("regras")
                            || trimmed.eq_ignore_ascii_case("natureza")
                            || trimmed.eq_ignore_ascii_case("comportamento")
                        {
                            selected_archetype_id.set("theory_nature_demeanor".to_string());
                        } else if let Some(matched_attr) = find_attribute(trimmed) {
                            selected_attribute_id.set(matched_attr.id.to_string());
                            sec = CompendiumSection::Attributes;
                        } else if let Some(matched_w) = find_weapon(trimmed) {
                            selected_weapon_id.set(matched_w.id.to_string());
                            sec = CompendiumSection::Weapons;
                        }
                    }
                    CompendiumSection::Instruments => {
                        if let Some(matched_inst) = find_instrument(trimmed) {
                            selected_instrument_id.set(matched_inst.id.to_string());
                        } else if let Some(matched_art) = find_theory_article(trimmed) {
                            selected_instrument_id.set(matched_art.id.to_string());
                        } else if let Some(matched_p) = find_practice(trimmed) {
                            selected_practice_id.set(matched_p.id.to_string());
                            sec = CompendiumSection::Practices;
                        }
                    }
                    CompendiumSection::Practices => {
                        if let Some(matched_p) = find_practice(trimmed) {
                            selected_practice_id.set(matched_p.id.to_string());
                        } else if let Some(matched_inst) = find_instrument(trimmed) {
                            selected_instrument_id.set(matched_inst.id.to_string());
                            sec = CompendiumSection::Instruments;
                        } else if let Some(matched_w) = find_weapon(trimmed) {
                            selected_weapon_id.set(matched_w.id.to_string());
                            sec = CompendiumSection::Weapons;
                        } else if let Some(matched_attr) = find_attribute(trimmed) {
                            selected_attribute_id.set(matched_attr.id.to_string());
                            sec = CompendiumSection::Attributes;
                        } else if let Some(matched_a) = find_archetype(trimmed) {
                            selected_archetype_id.set(matched_a.id.to_string());
                            sec = CompendiumSection::Archetypes;
                        }
                    }
                }
            }

            active_section.set(sec);
        } else if !is_open && was_open {
            prev_show.set(false);
        }
    });

    let close_modal = move || {
        set_show_modal.set(false);
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    // Permite alternar o idioma diretamente na visualização do compêndio
    let (modal_lang_override, set_modal_lang_override) = create_signal(Option::<Language>::None);
    let current_lang = Signal::derive(move || modal_lang_override.get().unwrap_or_else(lang));

    // Callback para navegação Práticas -> Instrumentos
    let nav_to_instrument = Callback::new(move |(prev_p_id, inst_id): (String, String)| {
        history_practice_id.set(Some(prev_p_id));
        selected_instrument_id.set(inst_id);
        active_section.set(CompendiumSection::Instruments);
    });

    // Callback para retorno Instrumentos -> Práticas
    let back_to_practice = Callback::new(move |prev_p_id: String| {
        selected_practice_id.set(prev_p_id);
        active_section.set(CompendiumSection::Practices);
    });

    let on_close_cb = Callback::new(move |()| close_modal());

    let on_select_p_cb = on_select_practice;
    let on_select_i_cb = on_select_instrument;
    let on_select_a_cb = on_select_archetype;
    let on_select_w_cb = on_select_weapon;
    let on_select_m_cb = on_select_maneuver;

    view! {
        {
            let on_select_p_cb = on_select_p_cb.clone();
            let on_select_i_cb = on_select_i_cb.clone();
            let on_select_a_cb = on_select_a_cb.clone();
            let on_select_w_cb = on_select_w_cb.clone();
            let on_select_m_cb = on_select_m_cb.clone();
            let on_close_cb = on_close_cb.clone();
            let nav_to_instrument = nav_to_instrument.clone();
            let back_to_practice = back_to_practice.clone();
            move || if show_modal.get() {
                let on_select_p_action = on_select_p_cb.clone();
                let on_select_i_action = on_select_i_cb.clone();
                let on_select_a_action = on_select_a_cb.clone();
                let on_select_w_action = on_select_w_cb.clone();
                let on_select_m_action = on_select_m_cb.clone();
                let on_close_action = on_close_cb.clone();
                let nav_to_inst_action = nav_to_instrument.clone();
                let back_to_prac_action = back_to_practice.clone();

                view! {
                    <div
                        class="modal-overlay practice-compendium-overlay"
                        on:click=move |_| close_modal()
                        on:wheel=move |ev| ev.stop_propagation()
                    >
                        <div
                            class="modal-card practice-compendium-modal"
                            on:click=move |ev| ev.stop_propagation()
                            on:wheel=move |ev| ev.stop_propagation()
                        >
                            // Header do Modal
                            <div class="practice-modal-header">
                                <div class="practice-modal-title-wrap">
                                    <span class="practice-modal-icon">
                                        {move || match active_section.get() {
                                            CompendiumSection::Practices => "📜",
                                            CompendiumSection::Instruments => "🛠️",
                                            CompendiumSection::Archetypes => "🎭",
                                            CompendiumSection::Attributes => "🧠",
                                            CompendiumSection::Weapons => "⚔️",
                                        }}
                                    </span>
                                    <div class="practice-modal-title-content">
                                        <span class="practice-modal-badge-sup">"M20 • COMPÊNDIO"</span>
                                        <h3 class="practice-modal-title">
                                            {move || match (active_section.get(), current_lang.get()) {
                                                (CompendiumSection::Practices, Language::PtBr) => "Práticas Mágicas",
                                                (CompendiumSection::Practices, Language::EnUs) => "Magickal Practices",
                                                (CompendiumSection::Instruments, Language::PtBr) => "Instrumentos & Focos",
                                                (CompendiumSection::Instruments, Language::EnUs) => "Instruments & Focus",
                                                (CompendiumSection::Archetypes, Language::PtBr) => "Arquétipos de Personalidade",
                                                (CompendiumSection::Archetypes, Language::EnUs) => "Personality Archetypes",
                                                (CompendiumSection::Attributes, Language::PtBr) => "Atributos & Especialidades",
                                                (CompendiumSection::Attributes, Language::EnUs) => "Attributes & Specialties",
                                                (CompendiumSection::Weapons, Language::PtBr) => "Armas, Manobras & Combate",
                                                (CompendiumSection::Weapons, Language::EnUs) => "Weapons, Maneuvers & Combat",
                                            }}
                                        </h3>
                                        <span class="practice-modal-subtitle">
                                            {move || match (active_section.get(), current_lang.get()) {
                                                (CompendiumSection::Practices, Language::PtBr) => "M20, pp. 573-586 • Capítulo 10: Foco e Artes (Práticas Canônicas)",
                                                (CompendiumSection::Practices, Language::EnUs) => "M20, pp. 573-586 • Chapter 10: Focus and Arts (Canonical Practices)",
                                                (CompendiumSection::Instruments, Language::PtBr) => "M20, pp. 586-608 • Capítulo 10: Foco e Artes (Instrumentos & Teoria Canônica)",
                                                (CompendiumSection::Instruments, Language::EnUs) => "M20, pp. 586-608 • Chapter 10: Focus and Arts (Instruments & Canonical Theory)",
                                                (CompendiumSection::Archetypes, Language::PtBr) => "M20, pp. 264-269 • Capítulo 6: Criação do Personagem (Natureza & Comportamento)",
                                                (CompendiumSection::Archetypes, Language::EnUs) => "M20, pp. 264-269 • Chapter 6: Creating the Character (Nature & Demeanor)",
                                                (CompendiumSection::Attributes, Language::PtBr) => "M20, pp. 273-275 • Capítulo 6: Atributos & Regra Canônica de Especialidades",
                                                (CompendiumSection::Attributes, Language::EnUs) => "M20, pp. 273-275 • Chapter 6: Attributes & Canonical Specialties Rule",
                                                (CompendiumSection::Weapons, Language::PtBr) => "M20, pp. 450-453 • Capítulo 9: Combate & Armamento (Brancas, Fogo, Arremesso)",
                                                (CompendiumSection::Weapons, Language::EnUs) => "M20, pp. 450-453 • Chapter 9: Combat & Weaponry (Melee, Ranged, Thrown)",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <div class="practice-modal-controls">
                                    // Alternador de Idioma no Modal (PT / EN)
                                    <div class="practice-modal-lang-toggle">
                                        <button
                                            type="button"
                                            class=move || if current_lang.get() == Language::PtBr { "lang-btn active" } else { "lang-btn" }
                                            on:click=move |_| set_modal_lang_override.set(Some(Language::PtBr))
                                            title="Visualizar compêndio em Português"
                                        >
                                            "🇧🇷 PT"
                                        </button>
                                        <button
                                            type="button"
                                            class=move || if current_lang.get() == Language::EnUs { "lang-btn active" } else { "lang-btn" }
                                            on:click=move |_| set_modal_lang_override.set(Some(Language::EnUs))
                                            title="View compendium in English"
                                        >
                                            "🇺🇸 EN"
                                        </button>
                                    </div>

                                    <button
                                        type="button"
                                        class="practice-modal-close-btn"
                                        on:click=move |_| close_modal()
                                        title="Fechar modal"
                                    >
                                        "✕"
                                    </button>
                                </div>
                            </div>

                            // Barra de Navegação entre Seções do Compêndio
                            <div class="compendium-nav-bar">
                                <button
                                    type="button"
                                    class=move || if active_section.get() == CompendiumSection::Practices { "compendium-nav-tab active" } else { "compendium-nav-tab" }
                                    on:click=move |_| {
                                        active_section.set(CompendiumSection::Practices);
                                        mobile_show_detail.set(false);
                                    }
                                >
                                    <span class="nav-tab-icon">"📜"</span>
                                    <span class="nav-tab-label">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "Práticas Mágicas",
                                            Language::EnUs => "Magickal Practices",
                                        }}
                                    </span>
                                    <span class="nav-tab-badge">"20"</span>
                                </button>

                                <button
                                    type="button"
                                    class=move || if active_section.get() == CompendiumSection::Instruments { "compendium-nav-tab active" } else { "compendium-nav-tab" }
                                    on:click=move |_| {
                                        active_section.set(CompendiumSection::Instruments);
                                        mobile_show_detail.set(false);
                                    }
                                >
                                    <span class="nav-tab-icon">"🛠️"</span>
                                    <span class="nav-tab-label">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "Instrumentos & Focos",
                                            Language::EnUs => "Instruments & Focus",
                                        }}
                                    </span>
                                    <span class="nav-tab-badge">"54"</span>
                                </button>

                                <button
                                    type="button"
                                    class=move || if active_section.get() == CompendiumSection::Archetypes { "compendium-nav-tab active" } else { "compendium-nav-tab" }
                                    on:click=move |_| {
                                        active_section.set(CompendiumSection::Archetypes);
                                        mobile_show_detail.set(false);
                                    }
                                >
                                    <span class="nav-tab-icon">"🎭"</span>
                                    <span class="nav-tab-label">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "Arquétipos",
                                            Language::EnUs => "Archetypes",
                                        }}
                                    </span>
                                    <span class="nav-tab-badge">"20"</span>
                                </button>

                                <button
                                    type="button"
                                    class=move || if active_section.get() == CompendiumSection::Attributes { "compendium-nav-tab active" } else { "compendium-nav-tab" }
                                    on:click=move |_| {
                                        active_section.set(CompendiumSection::Attributes);
                                        mobile_show_detail.set(false);
                                    }
                                >
                                    <span class="nav-tab-icon">"🧠"</span>
                                    <span class="nav-tab-label">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "Atributos & Regras",
                                            Language::EnUs => "Attributes & Rules",
                                        }}
                                    </span>
                                    <span class="nav-tab-badge">"9"</span>
                                </button>

                                <button
                                    type="button"
                                    class=move || if active_section.get() == CompendiumSection::Weapons { "compendium-nav-tab active" } else { "compendium-nav-tab" }
                                    on:click=move |_| {
                                        active_section.set(CompendiumSection::Weapons);
                                        mobile_show_detail.set(false);
                                    }
                                >
                                    <span class="nav-tab-icon">"⚔️"</span>
                                    <span class="nav-tab-label">
                                         {move || match current_lang.get() {
                                             Language::PtBr => "Armas & Manobras",
                                             Language::EnUs => "Weapons & Maneuvers",
                                         }}
                                     </span>
                                     <span class="nav-tab-badge">"126"</span>
                                </button>
                            </div>

                            // Corpo Modular do Compêndio
                            <div class="practice-modal-body">
                                {move || match active_section.get() {
                                    CompendiumSection::Practices => {
                                        let on_sel = on_select_p_action.clone();
                                        let on_cls = on_close_action.clone();
                                        let on_nav = nav_to_inst_action.clone();
                                        view! {
                                            <PracticesView
                                                selected_practice_id=selected_practice_id
                                                current_lang=current_lang
                                                mobile_show_detail=Some(mobile_show_detail)
                                                on_select_practice=on_sel
                                                on_navigate_to_instrument=Some(on_nav)
                                                on_close=Some(on_cls)
                                            />
                                        }.into_view()
                                    }
                                    CompendiumSection::Instruments => {
                                        let on_sel = on_select_i_action.clone();
                                        let on_cls = on_close_action.clone();
                                        let on_back = back_to_prac_action.clone();
                                        view! {
                                            <InstrumentsView
                                                selected_instrument_id=selected_instrument_id
                                                history_practice_id=history_practice_id
                                                current_lang=current_lang
                                                mobile_show_detail=Some(mobile_show_detail)
                                                on_select_instrument=on_sel
                                                on_back_to_practice=Some(on_back)
                                                on_close=Some(on_cls)
                                            />
                                        }.into_view()
                                    }
                                    CompendiumSection::Archetypes => {
                                        let on_sel = on_select_a_action.clone();
                                        let on_cls = on_close_action.clone();
                                        view! {
                                            <ArchetypesView
                                                selected_archetype_id=selected_archetype_id
                                                current_lang=current_lang
                                                mobile_show_detail=Some(mobile_show_detail)
                                                active_archetype_target=Some(active_archetype_target)
                                                on_select_archetype=on_sel
                                                on_close=Some(on_cls)
                                            />
                                        }.into_view()
                                    }
                                    CompendiumSection::Attributes => {
                                        view! {
                                            <AttributesView
                                                selected_attribute_id=selected_attribute_id
                                                current_lang=current_lang
                                                mobile_show_detail=Some(mobile_show_detail)
                                            />
                                        }.into_view()
                                    }
                                    CompendiumSection::Weapons => {
                                        let on_sel = on_select_w_action.clone();
                                        let on_sel_m = on_select_m_action.clone();
                                        let on_cls = on_close_action.clone();
                                        let t_slot = target_slot;
                                        view! {
                                            <WeaponsView
                                                selected_weapon_id=selected_weapon_id
                                                current_lang=current_lang
                                                mobile_show_detail=Some(mobile_show_detail)
                                                target_slot=t_slot
                                                on_select_weapon=on_sel
                                                on_select_maneuver=on_sel_m
                                                on_close=Some(on_cls)
                                            />
                                        }.into_view()
                                    }
                                }}
                            </div>

                            // Rodapé Unificado do Modal
                            <div class="practice-modal-footer">
                                <span class="practice-footer-hint">
                                    {move || match (active_section.get(), current_lang.get()) {
                                        (CompendiumSection::Practices, Language::PtBr) => "M20, pp. 573-586 • Catálogo Oficial de 20 Práticas e Boxes Canônicos".to_string(),
                                        (CompendiumSection::Practices, Language::EnUs) => "M20, pp. 573-586 • Official 20 Magickal Practices and Sidebars".to_string(),
                                        (CompendiumSection::Instruments, Language::PtBr) => "M20, pp. 586-608 • Catálogo Oficial de 54 Instrumentos e Artigos Teóricos".to_string(),
                                        (CompendiumSection::Instruments, Language::EnUs) => "M20, pp. 586-608 • Official 54 Instruments and Theory Articles".to_string(),
                                        (CompendiumSection::Archetypes, Language::PtBr) => "M20, pp. 264-269 • Catálogo Oficial de 20 Arquétipos (Natureza & Comportamento)".to_string(),
                                        (CompendiumSection::Archetypes, Language::EnUs) => "M20, pp. 264-269 • Official 20 Personality Archetypes (Nature & Demeanor)".to_string(),
                                        (CompendiumSection::Attributes, Language::PtBr) => "M20, pp. 273-275 • Atributos Físicos, Sociais, Mentais & Regra de Especialidades".to_string(),
                                        (CompendiumSection::Attributes, Language::EnUs) => "M20, pp. 273-275 • Physical, Social, Mental Attributes & Specialties Rule".to_string(),
                                        (CompendiumSection::Weapons, Language::PtBr) => "M20, pp. 423-426, 448-453 • Catálogo Oficial de 82 Armas e 35 Manobras & Artes Marciais".to_string(),
                                        (CompendiumSection::Weapons, Language::EnUs) => "M20, pp. 423-426, 448-453 • Official 82 Weapons & 35 Maneuvers / Martial Arts Catalog".to_string(),
                                    }}
                                </span>

                                <div class="practice-footer-actions">
                                    <button
                                        type="button"
                                        class="practice-close-action-btn"
                                        on:click=move |_| close_modal()
                                    >
                                        {move || match current_lang.get() {
                                            Language::PtBr => "Fechar",
                                            Language::EnUs => "Close",
                                        }}
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! { <div></div> }.into_view()
            }
        }
    }
}
