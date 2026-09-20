use leptos::*;
use crate::compendium::weapons::{
    ALL_RULE_NOTES, RANGED_NOTE_1, RANGED_NOTE_2, RANGED_NOTE_3, RANGED_NOTE_4,
    RANGED_NOTE_5, RANGED_NOTE_6, RANGED_NOTE_7, RANGED_NOTE_8, RANGED_NOTE_9,
    RANGED_NOTE_10, RANGED_NOTE_11, RANGED_NOTE_12,
};
use crate::i18n::Language;

/// Renderiza a tela completa de Legenda Canônica de Combate (M20 pp. 423-426, 450-453)
pub fn render_combat_legend_view(current_lang: Signal<Language>) -> impl IntoView {
    view! {
        <div class="weapon-legend-view">
            <div class="legend-header">
                <h4 class="legend-title">
                    "📜 " {move || match current_lang.get() {
                        Language::PtBr => "Regras de Combate & Notas Canônicas",
                        Language::EnUs => "Combat Rules & Canonical Notes",
                    }}
                </h4>
                <span class="legend-ref">"M20, pp. 450-453"</span>
            </div>

            // Parâmetros Básicos
            <div class="legend-params-grid">
                <div class="legend-param-card">
                    <strong>{move || match current_lang.get() {
                        Language::PtBr => "Dificuldade (Difficulty)",
                        Language::EnUs => "Difficulty (Dif)",
                    }}</strong>
                    <p>{move || match current_lang.get() {
                        Language::PtBr => "Armas brancas: dificuldade do teste de Destreza + Luta/Armas Brancas. Armas de fogo: Dif 6 no alcance listado, Dif 8 no dobro do alcance, Dif 4 a queima-roupa (2 jardas).",
                        Language::EnUs => "Melee: Dexterity + Brawl/Melee difficulty. Firearms: Diff 6 at listed range, Diff 8 at twice range, Diff 4 at point-blank (within 2 yards).",
                    }}</p>
                </div>
                <div class="legend-param-card">
                    <strong>{move || match current_lang.get() {
                        Language::PtBr => "Dano & Tipos (Damage)",
                        Language::EnUs => "Damage & Types",
                    }}</strong>
                    <p>{move || match current_lang.get() {
                        Language::PtBr => "B = Contundente (Bashing) • L = Letal (Lethal) • A = Agravado (Aggravated). Todo dano de armas de fogo e arcos é letal.",
                        Language::EnUs => "B = Bashing • L = Lethal • A = Aggravated. All damage from firearms and bows is lethal.",
                    }}</p>
                </div>
                <div class="legend-param-card">
                    <strong>{move || match current_lang.get() {
                        Language::PtBr => "Cadência & Pente (Rate / Clip)",
                        Language::EnUs => "Rate & Clip",
                    }}</strong>
                    <p>{move || match current_lang.get() {
                        Language::PtBr => "Cadência: tiros ou rajadas por turno. Pente: capacidade de munição. '+1' indica bala extra na câmara pronta para disparo.",
                        Language::EnUs => "Rate: bullets/bursts fired per turn. Clip: ammo capacity. '+1' indicates a round loaded in chamber.",
                    }}</p>
                </div>
                <div class="legend-param-card">
                    <strong>{move || match current_lang.get() {
                        Language::PtBr => "Ocultabilidade (Conceal)",
                        Language::EnUs => "Concealment (Conceal)",
                    }}</strong>
                    <p>{move || match current_lang.get() {
                        Language::PtBr => "P = Bolso (Pocket) • J = Jaqueta (Jacket) • T = Sobretudo (Trenchcoat) • N = N/A (Não Ocultável).",
                        Language::EnUs => "P = Pocket • J = Jacket • T = Trenchcoat • N = N/A (Cannot be concealed).",
                    }}</p>
                </div>
            </div>

            // Notas de Armas Brancas (#1 a #10)
            <div class="legend-notes-title">
                {move || match current_lang.get() {
                    Language::PtBr => "NOTAS DE ARMAS BRANCAS & COMBATE CORPO A CORPO (#1 a #10)",
                    Language::EnUs => "MELEE WEAPONS SPECIAL NOTES (#1 to #10)",
                }}
            </div>
            <div class="legend-notes-list">
                {ALL_RULE_NOTES.iter().map(|n| {
                    view! {
                        <div class="legend-note-row">
                            <span class="note-code-badge melee-badge">{n.code}</span>
                            <div class="note-content-wrap">
                                <strong class="note-title">{move || n.title(current_lang.get())}</strong>
                                <p class="note-desc">{move || n.description(current_lang.get())}</p>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Notas de Armas de Fogo e À Distância (#1 a #12)
            <div class="legend-notes-title" style="margin-top: 1.5rem;">
                {move || match current_lang.get() {
                    Language::PtBr => "NOTAS DE ARMAS DE FOGO & À DISTÂNCIA (#1 a #12 - M20 pp. 452-453)",
                    Language::EnUs => "RANGED WEAPONS SPECIAL NOTES (#1 to #12 - M20 pp. 452-453)",
                }}
            </div>
            <div class="legend-notes-list">
                {[
                    &RANGED_NOTE_1, &RANGED_NOTE_2, &RANGED_NOTE_3, &RANGED_NOTE_4,
                    &RANGED_NOTE_5, &RANGED_NOTE_6, &RANGED_NOTE_7, &RANGED_NOTE_8,
                    &RANGED_NOTE_9, &RANGED_NOTE_10, &RANGED_NOTE_11, &RANGED_NOTE_12,
                ].iter().map(|n| {
                    view! {
                        <div class="legend-note-row">
                            <span class="note-code-badge ranged-badge">{n.code}</span>
                            <div class="note-content-wrap">
                                <strong class="note-title">{move || n.title(current_lang.get())}</strong>
                                <p class="note-desc">{move || n.description(current_lang.get())}</p>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Regras Canônicas de Artes Marciais (M20 pp. 423-426, 580-581)
            <div class="legend-notes-title" style="margin-top: 1.5rem;">
                {move || match current_lang.get() {
                    Language::PtBr => "ARTES MARCIAIS: ESTILOS, MANOBRAS & FOCO MÁGICO (M20 pp. 423-426, 580-581)",
                    Language::EnUs => "MARTIAL ARTS: STYLES, MANEUVERS & MAGICKAL FOCUS (M20 pp. 423-426, 580-581)",
                }}
            </div>
            <div class="legend-params-grid">
                <div class="legend-param-card">
                    <strong>{move || match current_lang.get() {
                        Language::PtBr => "🥊 Estilos Duros (Hard Styles)",
                        Language::EnUs => "🥊 Hard Styles",
                    }}</strong>
                    <p>{move || match current_lang.get() {
                        Language::PtBr => "Focam em impacto direto, golpes lineares devastadores, socos e chutes contundentes (Karatê, Boxe, Muay Thai, Krav Maga, Tae Kwon Do). Priorizam força, velocidade e neutralização ofensiva imediata.",
                        Language::EnUs => "Focus on direct kinetic impact, linear strikes, and punishing punches and kicks (Karate, Boxing, Muay Thai, Krav Maga, Tae Kwon Do). Emphasize force, speed, and immediate offensive neutralization.",
                    }}</p>
                </div>
                <div class="legend-param-card">
                    <strong>{move || match current_lang.get() {
                        Language::PtBr => "🌊 Estilos Suaves (Soft Styles)",
                        Language::EnUs => "🌊 Soft Styles",
                    }}</strong>
                    <p>{move || match current_lang.get() {
                        Language::PtBr => "Focam em movimentos circulares, alavancagem, esquivas fluidas, chaves de articulação e redirecionamento do ímpeto adversário (Aikidô, Judô, Tai Chi Chuan, Jujutsu, Hapkido). Usam a própria força do atacante contra ele.",
                        Language::EnUs => "Focus on circular motion, leverage, fluid evasion, joint locks, and redirecting the opponent's momentum (Aikido, Judo, Tai Chi, Jujutsu, Hapkido). Turn the attacker's own strength against them.",
                    }}</p>
                </div>
                <div class="legend-param-card">
                    <strong>{move || match current_lang.get() {
                        Language::PtBr => "🥋 Aquisição de Manobras (2 por ponto)",
                        Language::EnUs => "🥋 Maneuver Selection (2 per dot)",
                    }}</strong>
                    <p>{move || match current_lang.get() {
                        Language::PtBr => "Um personagem adquire 2 manobras marciais para cada ponto em Artes Marciais (como especialidade de Briga/Luta, Armas Brancas ou Habilidade Secundária). O lutador deve atender aos pré-requisitos de estilo e pontuação mínima.",
                        Language::EnUs => "A character selects 2 martial maneuvers per dot in Martial Arts (as a Brawl/Melee specialty or Secondary Ability). The combatant must meet style requirements and minimum ability ratings.",
                    }}</p>
                </div>
                <div class="legend-param-card">
                    <strong>{move || match current_lang.get() {
                        Language::PtBr => "🔮 Artes Marciais como Foco Mágico (Dô)",
                        Language::EnUs => "🔮 Martial Arts as Magickal Focus (Do)",
                    }}</strong>
                    <p>{move || match current_lang.get() {
                        Language::PtBr => "M20 pp. 580-581: Magos marciais canalizam o Chi através de seus corpos como instrumento focal. Permite conjurar Mágika Iluminada em combate (Forças cinética, Vida estrutural, Mente disciplinada, Correspondência espacial e Tempo acelerado).",
                        Language::EnUs => "M20 pp. 580-581: Martial mages channel Chi through disciplined physical katas as a magickal focus instrument. Enhances combat with Enlightened Magick (kinetic Forces, biological Life, disciplined Mind, spatial Correspondence, and accelerated Time).",
                    }}</p>
                </div>
            </div>
        </div>
    }
}
