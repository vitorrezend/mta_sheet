use leptos::*;
use crate::state::CharacterData;

#[component]
pub fn QuickCombat() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    // Atributos e estatísticas derivadas
    let dex = Signal::derive(move || data.with(|d| d.get_attribute_level("Destreza", 1)));
    let wits = Signal::derive(move || data.with(|d| d.get_attribute_level("Raciocínio", 1)));
    let stam = Signal::derive(move || data.with(|d| d.get_attribute_level("Vigor", 1)));
    let athletics = Signal::derive(move || data.with(|d| d.get_attribute_level("Atletismo", 0)));
    let armor_rating = Signal::derive(move || data.with(|d| d.armor.rating.trim().parse::<i32>().unwrap_or(0)));

    // Cálculo da penalidade atual de ferimento baseada no dano acumulado na Vitalidade
    let wound_status = Signal::derive(move || {
        let (agg, lethal, bashing) = data.with(|d| d.get_health_counts());
        let total_dmg = agg + lethal + bashing;
        let extra = data.with(|d| d.get_extra_bruised());

        if total_dmg == 0 {
            (0, "0", "saudavel")
        } else if total_dmg <= extra + 1 {
            (0, "0", "escoriado")
        } else if total_dmg <= extra + 3 {
            (-1, "-1", "ferido")
        } else if total_dmg <= extra + 5 {
            (-2, "-2", "grave")
        } else if total_dmg <= extra + 6 {
            (-5, "-5", "aleijado")
        } else {
            (-99, "Incap", "incap")
        }
    });

    let initiative = Signal::derive(move || {
        let base = dex.get() + wits.get();
        let pen = wound_status.get().0;
        if pen == -99 {
            2
        } else {
            (base + pen).max(2)
        }
    });

    let soak = Signal::derive(move || {
        let s = stam.get();
        let arm = armor_rating.get();
        (s, arm, s + arm)
    });

    let dodge = Signal::derive(move || {
        let base = dex.get() + athletics.get();
        let pen = wound_status.get().0;
        if pen == -99 {
            0
        } else {
            (base + pen).max(0)
        }
    });

    let run_speed = Signal::derive(move || {
        dex.get() * 3 + 20
    });

    view! {
        <div class="quick-combat-container">
            <h3 class="column-title">
                {move || match lang() {
                    crate::i18n::Language::PtBr => "Combate & Estatísticas",
                    crate::i18n::Language::EnUs => "Combat & Quick Stats",
                }}
            </h3>

            <div class="qc-grid">
                // 1. Iniciativa
                <div class="qc-card">
                    <div class="qc-header-row">
                        <span class="qc-label">
                            {move || match lang() {
                                crate::i18n::Language::PtBr => "Iniciativa",
                                crate::i18n::Language::EnUs => "Initiative",
                            }}
                        </span>
                        <div class="qc-info-wrapper">
                            <button type="button" class="qc-info-btn" tabindex="0">"i"</button>
                            <div class="qc-tooltip-popover">
                                <strong>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "Iniciativa Base: Destreza + Raciocínio",
                                        crate::i18n::Language::EnUs => "Base Initiative: Dexterity + Wits",
                                    }}
                                </strong>
                                <p>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "No início de cada rodada de combate, role 1d10 e adicione a este valor para definir a ordem de ação. Penalidades de ferimento reduzem a iniciativa (mínimo de 2).",
                                        crate::i18n::Language::EnUs => "At the start of each combat turn, roll 1d10 and add to this score to determine action order. Wound penalties reduce initiative (floor of 2).",
                                    }}
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="qc-value-row">
                        <span class="qc-value-main">{move || initiative.get()}</span>
                        <span class="qc-value-sub">"+ 1d10"</span>
                    </div>
                </div>

                // 2. Absorção (Soak) - Magos como mortais absorvem apenas dano Contusivo naturalmente
                <div class="qc-card">
                    <div class="qc-header-row">
                        <span class="qc-label">
                            {move || match lang() {
                                crate::i18n::Language::PtBr => "Absorção (Cont.)",
                                crate::i18n::Language::EnUs => "Soak (Bash.)",
                            }}
                        </span>
                        <div class="qc-info-wrapper">
                            <button type="button" class="qc-info-btn" tabindex="0">"i"</button>
                            <div class="qc-tooltip-popover">
                                <strong>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "Absorção: Vigor (+ Armadura)",
                                        crate::i18n::Language::EnUs => "Soak: Stamina (+ Armor)",
                                    }}
                                </strong>
                                <p>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "Como humanos, magos absorvem apenas dano Contusivo naturalmente com Vigor (Dif. 6). Dano Letal e Agravado NÃO são absorvidos sem Armadura física ou mágika (Vida/Primórdio).",
                                        crate::i18n::Language::EnUs => "As mortals, mages only soak Bashing damage naturally with Stamina (Diff. 6). Lethal and Aggravated damage CANNOT be soaked without physical Armor or magick (Life/Prime).",
                                    }}
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="qc-value-row">
                        <span class="qc-value-main">{move || soak.get().0}"D"</span>
                        {move || {
                            let arm = soak.get().1;
                            if arm > 0 {
                                view! { <span class="qc-value-sub">{format!("(+{} arm)", arm)}</span> }.into_view()
                            } else {
                                ().into_view()
                            }
                        }}
                    </div>
                </div>

                // 3. Defesa / Esquiva
                <div class="qc-card">
                    <div class="qc-header-row">
                        <span class="qc-label">
                            {move || match lang() {
                                crate::i18n::Language::PtBr => "Esquiva",
                                crate::i18n::Language::EnUs => "Dodge",
                            }}
                        </span>
                        <div class="qc-info-wrapper">
                            <button type="button" class="qc-info-btn" tabindex="0">"i"</button>
                            <div class="qc-tooltip-popover">
                                <strong>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "Ação Defensiva: Destreza + Atletismo",
                                        crate::i18n::Language::EnUs => "Defensive Action: Dexterity + Athletics",
                                    }}
                                </strong>
                                <p>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "Parada de dados usada para esquivar de ataques corporais e projéteis. Cada sucesso cancela 1 sucesso de acerto do atacante.",
                                        crate::i18n::Language::EnUs => "Dice pool used to evade melee and ranged attacks. Each success cancels 1 attack success from the opponent.",
                                    }}
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="qc-value-row">
                        <span class="qc-value-main">{move || dodge.get()}</span>
                        <span class="qc-value-sub">
                            {move || match lang() {
                                crate::i18n::Language::PtBr => "dados",
                                crate::i18n::Language::EnUs => "dice",
                            }}
                        </span>
                    </div>
                </div>

                // 4. Movimento (Caminhada / Corrida)
                <div class="qc-card">
                    <div class="qc-header-row">
                        <span class="qc-label">
                            {move || match lang() {
                                crate::i18n::Language::PtBr => "Movimento",
                                crate::i18n::Language::EnUs => "Movement",
                            }}
                        </span>
                        <div class="qc-info-wrapper">
                            <button type="button" class="qc-info-btn" tabindex="0">"i"</button>
                            <div class="qc-tooltip-popover">
                                <strong>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "Movimento por Turno",
                                        crate::i18n::Language::EnUs => "Movement per Turn",
                                    }}
                                </strong>
                                <p>
                                    {move || match lang() {
                                        crate::i18n::Language::PtBr => "Caminhada: 7 metros/turno. Corrida rápida: (Destreza × 3 + 20) metros em uma ação completa de corrida.",
                                        crate::i18n::Language::EnUs => "Walk: 7 meters/turn. Sprint: (Dexterity × 3 + 20) meters on a full running action.",
                                    }}
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="qc-value-row">
                        <span class="qc-value-main">"7m"</span>
                        <span class="qc-value-sub">{move || format!("/ {}m", run_speed.get())}</span>
                    </div>
                </div>
            </div>

            // 5. Faixa de Penalidade de Ferimento Atual
            <div class="qc-wound-bar">
                <span class="qc-wound-title">
                    {move || match lang() {
                        crate::i18n::Language::PtBr => "Penalidade em Testes:",
                        crate::i18n::Language::EnUs => "Dice Penalty:",
                    }}
                </span>
                <span
                    class="qc-wound-badge"
                    class:wound-healthy=move || wound_status.get().2 == "saudavel"
                    class:wound-mild=move || wound_status.get().2 == "escoriado" || wound_status.get().2 == "ferido"
                    class:wound-severe=move || wound_status.get().2 == "grave" || wound_status.get().2 == "aleijado"
                    class:wound-incap=move || wound_status.get().2 == "incap"
                >
                    {move || {
                        let status = wound_status.get();
                        let current_lang = lang();
                        if status.2 == "saudavel" {
                            match current_lang {
                                crate::i18n::Language::PtBr => "Nenhuma (0)".to_string(),
                                crate::i18n::Language::EnUs => "None (0)".to_string(),
                            }
                        } else if status.2 == "incap" {
                            match current_lang {
                                crate::i18n::Language::PtBr => "Incapacitado".to_string(),
                                crate::i18n::Language::EnUs => "Incapacitated".to_string(),
                            }
                        } else {
                            format!("{} dados", status.1)
                        }
                    }}
                </span>
            </div>
        </div>
    }
}