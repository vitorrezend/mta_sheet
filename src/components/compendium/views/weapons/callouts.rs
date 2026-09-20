use leptos::*;
use crate::compendium::weapons::{
    THUNDER_PUNCH_TRICK, EIGHT_LIMBS_ARTICLE, DO_RULES_ARTICLE,
};
use crate::i18n::Language;

/// Renderiza a Box Oficial do Livro M20 (p. 449): "Mage Trick: The Thunder Punch"
pub fn render_thunder_punch_callout_box(lang: Signal<Language>) -> impl IntoView {
    let trick = &THUNDER_PUNCH_TRICK;
    view! {
        <div class="thunder-punch-callout-box">
            // Cabeçalho da Box Oficial
            <div class="thunder-punch-header">
                <div class="thunder-punch-hero-icon" style="font-size: 2rem; margin-bottom: 0.25rem;">"⚡"</div>
                <h2 class="thunder-punch-hero-title">
                    {move || trick.title(lang.get())}
                </h2>
                <span class="thunder-punch-hero-subtitle">
                    {move || match lang.get() {
                        Language::PtBr => "M20 • Livro de Regras Básico • Capítulo 9: Combate & Narrativa (p. 449)",
                        Language::EnUs => "M20 Core Rulebook • Chapter 9: Combat & Storytelling (p. 449)",
                    }}
                </span>
            </div>

            // Esferas Envolvidas
            <div class="thunder-punch-spheres-wrap">
                <span class="thunder-punch-spheres-label">
                    "🔮 " {move || match lang.get() {
                        Language::PtBr => "Esferas Aplicáveis & Efeitos de Iluminação:",
                        Language::EnUs => "Applicable Spheres & Enlightened Effects:",
                    }}
                </span>
                <div class="thunder-punch-spheres-list">
                    {move || trick.sphere_tags(lang.get()).iter().map(|&tag| {
                        view! {
                            <span class="thunder-sphere-pill">
                                "✨ " {tag}
                            </span>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Grid dos 4 Pilares Táticos de Regras
            <div class="thunder-tactical-grid">
                <div class="thunder-tactical-card">
                    <span class="thunder-tactical-card-title">
                        "🎯 " {move || match lang.get() {
                            Language::PtBr => "Modificador de Dificuldade",
                            Language::EnUs => "Difficulty Modifier",
                        }}
                    </span>
                    <p class="thunder-tactical-card-desc">
                        {move || trick.difficulty_rule(lang.get())}
                    </p>
                </div>

                <div class="thunder-tactical-card">
                    <span class="thunder-tactical-card-title">
                        "💥 " {move || match lang.get() {
                            Language::PtBr => "Dano & Metamágica",
                            Language::EnUs => "Damage & Metamagick",
                        }}
                    </span>
                    <p class="thunder-tactical-card-desc">
                        {move || trick.damage_rule(lang.get())}
                    </p>
                </div>

                <div class="thunder-tactical-card">
                    <span class="thunder-tactical-card-title">
                        "⚡ " {move || match lang.get() {
                            Language::PtBr => "Coincidente vs Vulgar",
                            Language::EnUs => "Coincidental vs Vulgar",
                        }}
                    </span>
                    <p class="thunder-tactical-card-desc">
                        {move || match lang.get() {
                            Language::PtBr => "Coincidente se parecer golpe perfeito. Vulgar se desproporcional à compleição física.",
                            Language::EnUs => "Coincidental if looking like a perfect strike. Vulgar if visibly defying physical build.",
                        }}
                    </p>
                </div>

                <div class="thunder-tactical-card">
                    <span class="thunder-tactical-card-title">
                        "⚠️ " {move || match lang.get() {
                            Language::PtBr => "Efeito Reverso / Absorção",
                            Language::EnUs => "Backlash / Full Soak",
                        }}
                    </span>
                    <p class="thunder-tactical-card-desc">
                        {move || trick.backlash_rule(lang.get())}
                    </p>
                </div>
            </div>

            // Texto Original / Traduzido do Livro M20 (3 Parágrafos)
            <div class="thunder-punch-body-paragraphs">
                {move || trick.paragraphs(lang.get()).iter().map(|&p| {
                    view! {
                        <p class="thunder-punch-paragraph">
                            {p}
                        </p>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Renderiza o Card Dedicado dos Oito Membros da Maestria (Dô / Akashayana)
pub fn render_eight_limbs_card(lang: Signal<Language>) -> impl IntoView {
    let article = &EIGHT_LIMBS_ARTICLE;
    view! {
        <div class="eight-limbs-callout-box">
            // Cabeçalho da Box Oficial dos Oito Membros
            <div class="eight-limbs-header">
                <div class="eight-limbs-hero-icon" style="font-size: 2.2rem; margin-bottom: 0.25rem;">"🪷"</div>
                <h2 class="eight-limbs-hero-title">
                    {move || article.title(lang.get())}
                </h2>
                <span class="eight-limbs-hero-subtitle">
                    {move || article.subtitle(lang.get())}
                </span>
                <p class="eight-limbs-intro">
                    {move || article.intro(lang.get())}
                </p>
            </div>

            // Grid dos Oito Membros da Maestria
            <div class="eight-limbs-grid">
                {article.limbs.iter().map(|limb| {
                    view! {
                        <div class="limb-card">
                            <div class="limb-card-header">
                                <span class="limb-bullet">"🪷"</span>
                                <h3 class="limb-title">
                                    {move || limb.title(lang.get())}
                                </h3>
                            </div>
                            <p class="limb-desc">
                                {move || limb.description(lang.get())}
                            </p>
                            <div class="limb-abilities-wrap">
                                <span class="limb-abilities-label">
                                    {move || match lang.get() {
                                        Language::PtBr => "Habilidades Associadas:",
                                        Language::EnUs => "Associated Abilities:",
                                    }}
                                </span>
                                <div class="limb-abilities-tags">
                                    {limb.abilities(Language::EnUs).iter().zip(limb.abilities(Language::PtBr).iter()).map(|(&en, &pt)| {
                                        view! {
                                            <span class="limb-ability-badge">
                                                {move || match lang.get() {
                                                    Language::PtBr => pt,
                                                    Language::EnUs => en,
                                                }}
                                            </span>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Card da Regra de Progressão dos Membros
            <div class="eight-limbs-progression-card">
                <div class="progression-header">
                    <span class="progression-icon">"⚖️"</span>
                    <strong class="progression-title">
                        {move || match lang.get() {
                            Language::PtBr => "REGRA DE PROGRESSÃO & ESTUDO DOS MEMBROS",
                            Language::EnUs => "PROGRESSION RULE & STUDY OF THE LIMBS",
                        }}
                    </strong>
                </div>
                <p class="progression-desc">
                    {move || article.progression_rule(lang.get())}
                </p>
            </div>

            // Box Oficial da Regra Opcional: O Caminho Pacífico (The Peaceful Way)
            <div class="peaceful-way-callout-box">
                <div class="peaceful-way-header">
                    <span class="peaceful-way-icon">"🕊️"</span>
                    <h3 class="peaceful-way-title">
                        {move || article.peaceful_way_title(lang.get())}
                    </h3>
                </div>
                <div class="peaceful-way-desc-paragraphs">
                    {move || {
                        let text = article.peaceful_way_rule(lang.get());
                        text.split("\n\n").map(|p| {
                            view! {
                                <p class="peaceful-way-p">{p}</p>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}

/// Renderiza o Card de Regras Canônicas & Treinamento de Dô (M20)
pub fn render_do_rules_card(lang: Signal<Language>) -> impl IntoView {
    let article = &DO_RULES_ARTICLE;
    view! {
        <div class="do-rules-callout-box">
            // Cabeçalho Oficial
            <div class="do-rules-header">
                <div class="do-rules-hero-icon" style="font-size: 2.2rem; margin-bottom: 0.25rem;">"🥋"</div>
                <h2 class="do-rules-hero-title">
                    {move || article.title(lang.get())}
                </h2>
                <span class="do-rules-hero-subtitle">
                    {move || article.subtitle(lang.get())}
                </span>
                <p class="do-rules-overview">
                    {move || article.overview(lang.get())}
                </p>
            </div>

            // Card de Treino Diário e Compromisso
            <div class="do-commitment-card">
                <div class="commitment-header">
                    <span class="commitment-icon">"⏳"</span>
                    <strong class="commitment-title">
                        {move || match lang.get() {
                            Language::PtBr => "COMPROMISSO, TREINAMENTO DIÁRIO & LIMITES",
                            Language::EnUs => "COMMITMENT, DAILY TRAINING & LIMITS",
                        }}
                    </strong>
                </div>
                <p class="commitment-desc">
                    {move || article.commitment(lang.get())}
                </p>
            </div>

            // Grid das Vantagens e Regras do Sistema
            <div class="do-advantages-grid">
                {article.advantages.iter().map(|adv| {
                    view! {
                        <div class="do-advantage-card">
                            <div class="advantage-card-header">
                                <span class="advantage-icon">{adv.icon}</span>
                                <h4 class="advantage-title">{move || adv.title(lang.get())}</h4>
                            </div>
                            <p class="advantage-rule">{move || adv.rule(lang.get())}</p>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
