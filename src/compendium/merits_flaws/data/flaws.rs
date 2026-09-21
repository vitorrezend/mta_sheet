//! Defeitos Canônicos de M20 (Flaws).
//!
//! Apêndice II: Qualidades e Defeitos (pp. 646-650).

use super::super::models::{MeritFlawDefinition, TraitCategory, TraitType};

pub const ADDICTION: MeritFlawDefinition = MeritFlawDefinition {
    id: "addiction",
    name: "Addiction",
    name_pt: "Vício",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Flaw,
    category: TraitCategory::Physical,
    points_str: "1 or 3 pts.",
    points_str_pt: "1 ou 3 pts.",
    available_costs: &[1, 3],
    page_ref: "M20, p. 646",
    description: "You are addicted to a substance or activity that interferes significantly with your daily life, driving you to irrational extremes.",
    description_pt: "Você é viciado em alguma substância ou atividade compulsiva que interfere significativamente na sua rotina, forçando extremos irracionais.",
    system: "A 1-point addiction involves trivial pleasures (gambling, tobacco, video games, social media). A 3-point addiction involves illegal, dangerous, or ruinous habits (hard narcotics, underground fight clubs, severe moral hazards). Deprivation causes escalating penalties and loss of focus.",
    system_pt: "Um vício de 1 ponto envolve hábitos menores (tabaco, cafeína, jogos, redes sociais). Um vício de 3 pontos envolve dependências ilícitas, perigosas ou ruinosas (narcóticos pesados, clubes de luta clandestinos, perigos morais severos). A privação impõe penalidades crescentes e perda de concentração.",
};

pub const CONSTRUCT: MeritFlawDefinition = MeritFlawDefinition {
    id: "construct",
    name: "Construct",
    name_pt: "Constructo",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Flaw,
    category: TraitCategory::Social,
    points_str: "2 pts.",
    points_str_pt: "2 pts.",
    available_costs: &[2],
    page_ref: "M20, p. 648",
    description: "You came from a laboratory or mystical forge rather than a human womb, creating an uncanny disconnect with natural-born society.",
    description_pt: "Você nasceu em um laboratório, cuba clonadora ou forja ritualística em vez de um ventre humano, gerando uma desconexão perturbadora com a sociedade natural.",
    system: "Suffers +1 to +2 difficulty on social interaction rolls with ordinary humans who subconsciously sense your synthetic, biomechanical, or lab-grown artificial nature.",
    system_pt: "Impõe +1 a +2 de penalidade na dificuldade de interações sociais com humanos comuns, que sentem inconscientemente sua natureza sintética, biológica clonada ou artificial.",
};

pub const CURSED: MeritFlawDefinition = MeritFlawDefinition {
    id: "cursed",
    name: "Cursed",
    name_pt: "Amaldiçoado",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Flaw,
    category: TraitCategory::Supernatural,
    points_str: "1 to 5 pts.",
    points_str_pt: "1 a 5 pts.",
    available_costs: &[1, 2, 3, 4, 5],
    page_ref: "M20, pp. 646-647",
    description: "Supernatural misfortune shadows your steps. The more points invested, the more catastrophic and persistent your bad luck becomes.",
    description_pt: "Um infortúnio de origem manifestamente sobrenatural assombra seus passos. Quanto maior a pontuação, mais devastadora e persistente é sua maré de azar.",
    system: "• 1 pt: Minor quirks (electronic glitches, minor misunderstandings).\n• 2 pts: Annoying breakdowns (flat tires at crucial moments, ruined clothes).\n• 3 pts: Chronic misfortune (tools break on botches, animals distrust you).\n• 4 pts: Major collapses (plans fail unexpectedly, inexplicable financial leaks).\n• 5 pts: Pervasive doom (technology fails chronically, pervasive bad luck).",
    system_pt: "• 1 pt: Quinquilharias de azar (falhas técnicas menores, pequenas gafes).\n• 2 pts: Problemas irritantes (pneus furam em emergências, roupas rasgam).\n• 3 pts: Infortúnio crônico (ferramentas quebram com facilidade, animais fogem).\n• 4 pts: Colapsos severos (planos perfeitos desmoronam sem explicação lógica).\n• 5 pts: Maldição implacável (tecnologia engasga sempre, desgraça contínua).",
};

pub const DERANGED: MeritFlawDefinition = MeritFlawDefinition {
    id: "deranged",
    name: "Deranged",
    name_pt: "Perturbado",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Flaw,
    category: TraitCategory::Mental,
    points_str: "3 or 5 pts.",
    points_str_pt: "3 ou 5 pts.",
    available_costs: &[3, 5],
    page_ref: "M20, p. 649",
    description: "You suffer from a lasting, severe mental illness. Although Willpower rolls allow temporary composure, the madness lingers.",
    description_pt: "Você sofre de uma enfermidade psicológica crônica e profunda. Embora testes de Força de Vontade possam garantir lucidez momentânea, a loucura é constante.",
    system: "Select one Derangement from the Compendium. At the 3-point level, it causes troubling but manageable impairments. At the 5-point level, it manifests as dangerous, erratic psychosis that threatens everyone nearby.",
    system_pt: "Escolha uma Perturbação Mental do Compêndio. No nível de 3 pontos, acarreta desafios graves, porém controláveis sob esforço. No nível de 5 pontos, atinge proporções violentas ou psicóticas, tornando o mago um perigo imprevisível.",
};

pub const ECHOES: MeritFlawDefinition = MeritFlawDefinition {
    id: "echoes",
    name: "Echoes",
    name_pt: "Ecos",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Flaw,
    category: TraitCategory::Supernatural,
    points_str: "1 to 5 pts.",
    points_str_pt: "1 a 5 pts.",
    available_costs: &[1, 2, 3, 4, 5],
    page_ref: "M20, pp. 647-648",
    description: "Weird phenomena and mystical signatures surround you, making your magical nature apparent even when you try to blend into the mundane world.",
    description_pt: "Fenômenos bizarros e assinaturas místicas tangíveis cercam sua presença, revelando sua natureza mágica mesmo quando tenta se misturar à multidão dormente.",
    system: "• 1 pt: Slight quirks (subtle scents, flickering shadows).\n• 2 pts: Noticeable aura (animals shy away, uncomfort around opposite resonances).\n• 3 pts: Obvious signs (plants wither or bloom, holy/unholy symbols repulse you).\n• 4 pts: Glaring anomalies (no reflections, independent shadows; Sleepers can notice).\n• 5 pts: Reality ripples (weather shifts around you, occultists can counterspell you easily).",
    system_pt: "• 1 pt: Sinais sutis (aromas misteriosos no ar, sombras trêmulas).\n• 2 pts: Aura perceptível (animais reagem com medo, desconforto com símbolos opostos).\n• 3 pts: Sinais patentes (flores desabrocham ou murcham, repulsa a ícones sagrados).\n• 4 pts: Anomalias chocantes (ausência de reflexo, sombras autônomas visíveis a Adormecidos).\n• 5 pts: Distorção da realidade (mudanças climáticas locais, vulnerabilidade a crendices e talismãs folclóricos).",
};

pub const ENEMY: MeritFlawDefinition = MeritFlawDefinition {
    id: "enemy",
    name: "Enemy",
    name_pt: "Inimigo",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Flaw,
    category: TraitCategory::Social,
    points_str: "1 to 5 pts.",
    points_str_pt: "1 a 5 pts.",
    available_costs: &[1, 2, 3, 4, 5],
    page_ref: "M20, p. 648",
    description: "Someone hates your guts and actively plots your downfall, humiliation, or agonizing death.",
    description_pt: "Alguém odeia você profundamente e dedica recursos, tempo e influência para arruinar sua vida, reputação ou matá-lo.",
    system: "• 1 pt: Minor grudge from an individual of equal or lesser power.\n• 2-3 pts: Capable rival, hitman, or influential police detective on your trail.\n• 4-5 pts: Powerful syndicate, Technocratic strike team, Nephandic cult, or Vampire Elder hunting you relentlessly.",
    system_pt: "• 1 pt: Rancor pessoal de um rival mundano ou indivíduo de poder modesto.\n• 2-3 pts: Caçador experiente, detetive obstinado ou mago rival com recursos.\n• 4-5 pts: Sindicato Tecnocrata, cabala Nefandi, clã vampírico ancião ou agência de inteligência caçando você ativamente.",
};

pub const PTSD: MeritFlawDefinition = MeritFlawDefinition {
    id: "ptsd",
    name: "PTSD",
    name_pt: "TEPT",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Flaw,
    category: TraitCategory::Mental,
    points_str: "2 to 5 pts.",
    points_str_pt: "2 a 5 pts.",
    available_costs: &[2, 3, 4, 5],
    page_ref: "M20, pp. 648-649",
    description: "Post-Traumatic Stress Disorder resulting from horrific violence, torture, or reality-breaking supernatural shocks triggers hair-trigger fight-or-flight panic.",
    description_pt: "Transtorno de Estresse Pós-Traumático resultante de extrema violência, tortura militar ou choques místicos violentos, provocando ataques de pânico e fuga incontrolável.",
    system: "• 2 pts: Occasional nightmares and anxiety during rare encounters.\n• 3 pts: Phobias triggered by moderate stressors (authority figures, isolation).\n• 4 pts: Severe reactions (near-panic, emotional paralysis around gunshots or sirens).\n• 5 pts: Incapacitating flashbacks, fight-or-flight catatonia in high-stress combat.",
    system_pt: "• 2 pts: Pesadelos ocasionais e calafrios diante de estímulos raros.\n• 3 pts: Fobias ativadas por situações comuns (confronto com superiores, isolamento).\n• 4 pts: Reações severas (pânico, paralisia temporária diante de tiros ou sirenes).\n• 5 pts: Flashbacks paralisantes, catatonia ou desespero violento em combates e crises.",
};

pub const ALL_FLAWS: &[MeritFlawDefinition] = &[
    ADDICTION,
    CONSTRUCT,
    CURSED,
    DERANGED,
    ECHOES,
    ENEMY,
    PTSD,
];
