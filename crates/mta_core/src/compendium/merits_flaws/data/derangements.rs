//! Perturbações Mentais Canônicas de M20 (Derangements).
//!
//! Apêndice II: Perturbações e Insanidade (pp. 649-650).

use super::super::models::DerangementDefinition;

pub const AMNESIA: DerangementDefinition = DerangementDefinition {
    id: "amnesia",
    name: "Amnesia",
    name_pt: "Amnésia",
    page_ref: "M20, p. 649",
    description: "Locking out parts of your memory, you suppress recollections of trauma, horrors, or aspects of identity that feel unsafe or intolerable.",
    description_pt: "Bloqueando partes da própria memória, o personagem reprime lembranças traumáticas, eventos aterrorizantes ou facetas da própria identidade.",
    game_effects: "The character blocks out memories of specific people, past events, or skills. When confronted with undeniable proof of forgotten events, a Willpower roll (diff 7) is required to avoid panic, paralysis, or fugue.",
    game_effects_pt: "O personagem apaga de sua mente memórias de pessoas, períodos de tempo ou fatos perturbadores. Ao ser confrontado com provas irrefutáveis do esquecido, deve passar num teste de Força de Vontade (dif 7) para não entrar em pânico ou choque.",
};

pub const ASPD: DerangementDefinition = DerangementDefinition {
    id: "aspd",
    name: "Antisocial Personality Disorder (ASPD)",
    name_pt: "Transtorno de Personalidade Antissocial (TPAS)",
    page_ref: "M20, p. 649",
    description: "Commonly termed sociopathy or psychopathy, this derangement reflects complete disregard for the emotional lives, safety, and rights of other beings.",
    description_pt: "Frequentemente chamado de sociopatia ou psicopatia, este transtorno manifesta desrespeito crônico e indiferença total pelas emoções, segurança e direitos alheios.",
    game_effects: "The character feels no remorse or empathy. Social rolls based on genuine compassion or building emotional intimacy suffer +2 difficulty. Can remain deceptively charismatic on the surface.",
    game_effects_pt: "O personagem não sente remorso, compaixão ou culpa genuína. Testes sociais baseados em empatia autêntica ou afeto sincero sofrem +2 de dificuldade. Pode se mostrar incrivelmente charmoso e manipulador.",
};

pub const BAD: DerangementDefinition = DerangementDefinition {
    id: "bad",
    name: "Bipolar Affective Disorder (BAD)",
    name_pt: "Transtorno Afetivo Bipolar (TAB)",
    page_ref: "M20, pp. 649-650",
    description: "Also known as manic depression, this disorder swings the character unpredictably between hyperactive manic euphoria and crushing, despondent depression.",
    description_pt: "Também conhecido como depressão maníaca, este distúrbio oscila o personagem entre surtos eufóricos maníacos e fases de depressão profunda e paralisante.",
    game_effects: "In manic phases, character acts impulsively and requires fewer hours of sleep, but adds +1 to +2 difficulty on rolls requiring patience. In depressive phases, all dice pools are reduced by 1, and Willpower recovery is halved.",
    game_effects_pt: "Na fase maníaca, age por impulso sem medir riscos e precisa de pouco sono, mas sofre +1 a +2 de dificuldade em tarefas que exigem cautela. Na fase depressiva, todas as paradas de dados sofrem -1 e a recuperação de Força de Vontade cai pela metade.",
};

pub const DEMENTIA: DerangementDefinition = DerangementDefinition {
    id: "dementia",
    name: "Dementia",
    name_pt: "Demência",
    page_ref: "M20, p. 650",
    description: "A wide-ranging cognitive affliction causing sensory distortion, disinhibition, erratic memory lapses, and severe perceptual disintegration.",
    description_pt: "Aflição cognitiva abrangente que causa distorções sensoriais, perda de inibição moral, lapsos de memória imprevisíveis e desintegração de conexões lógicas.",
    game_effects: "Under stress, the character suffers hallucinations or blurted outbursts that reveal confidential secrets. Perceptions can become confused, shifting sensory priorities at unexpected times.",
    game_effects_pt: "Sob tensão, sofre alucinações auditivas ou visuais e perda de filtros verbais, desabafando segredos ou adotando comportamentos bizarros sem medir consequências.",
};

pub const FUGUE: DerangementDefinition = DerangementDefinition {
    id: "fugue",
    name: "Fugue",
    name_pt: "Estado de Fuga",
    page_ref: "M20, p. 650",
    description: "Dissociative amnesia that forces the character to abandon their life, identity, and companions when psychological pressure becomes unbearable.",
    description_pt: "Amnésia dissociativa grave que compele o indivíduo a abandonar subitamente seu cotidiano, identidade e companheiros quando o estresse psicológico atinge o limite.",
    game_effects: "Triggered by severe trauma or botches on Mind/Willpower rolls. The character wanders away, adopting a whole new temporary persona in a distant neighborhood or city until lucidity returns.",
    game_effects_pt: "Ativado após falha crítica em testes de Força de Vontade ou choque severo. O personagem foge sem destino, adotando uma nova identidade provisória em outro local até recobrar a lucidez semanas depois.",
};

pub const MEGALOMANIA: DerangementDefinition = DerangementDefinition {
    id: "megalomania",
    name: "Megalomania",
    name_pt: "Megalomania",
    page_ref: "M20, p. 650",
    description: "An extreme narcissistic fantasy of supreme authority and divine destiny, leading the mage to view everyone else as inferior pawns in their ascension.",
    description_pt: "Fantasia narcisista extrema de poder supremo e destino messiânico, levando o mago a considerar todos os outros como meros peões descartáveis em seu plano triunfal.",
    game_effects: "Cannot admit defeat, error, or submit to authority. Refusing to yield or obey superiors requires a Willpower roll to avoid explosive rage or insolence.",
    game_effects_pt: "Incapaz de admitir erros, recuar ou acatar ordens de superiores sem questionar. Resistir a desafiar abertamente uma autoridade ou rival exige teste de Força de Vontade.",
};

pub const MPD: DerangementDefinition = DerangementDefinition {
    id: "mpd",
    name: "Multiple Personality Disorder (MPD / DID)",
    name_pt: "Transtorno Dissociativo de Identidade (TDI)",
    page_ref: "M20, p. 650",
    description: "The psyche splits into two or more distinct identities, each possessing unique mannerisms, speech patterns, memories, and priorities.",
    description_pt: "A psique divide-se em duas ou mais identidades distintas (alters), cada uma com maneirismos, memórias, habilidades e visões de mundo próprias.",
    game_effects: "Under stress or specific psychological triggers, a different persona takes control. Other personas might not recall what the active personality did during the shift.",
    game_effects_pt: "Sob estresse ou estímulos psicológicos específicos, outra persona assume o controle do corpo. A persona anterior frequentemente sofre blackout sobre o que ocorreu durante a transição.",
};

pub const OCD: DerangementDefinition = DerangementDefinition {
    id: "ocd",
    name: "Obsessive-Compulsive Disorder (OCD)",
    name_pt: "Transtorno Obsessivo-Compulsivo (TOC)",
    page_ref: "M20, p. 650",
    description: "Trapped in anxious loops of doubt and dread, the character performs ritualistic, repetitive behaviors to ward off catastrophic thoughts.",
    description_pt: "Preso em ciclos ansiosos de dúvida e medo incapacitante, o personagem executa comportamentos rituais e repetitivos na tentativa de aplacar pensamentos catastróficos.",
    game_effects: "Preventing the character from completing their compulsive rituals (arranging tools, washing hands, counting) inflicts a cumulative +1 to +2 difficulty on all actions due to sheer panic.",
    game_effects_pt: "Impedir o personagem de concluir seus rituais compulsivos (limpeza, ordenação simétrica, contagem de passos) impõe penalidade de +1 a +2 de dificuldade em todas as ações por angústia e distração.",
};

pub const PARANOIA: DerangementDefinition = DerangementDefinition {
    id: "paranoia",
    name: "Paranoia",
    name_pt: "Paranoia",
    page_ref: "M20, p. 650",
    description: "Every stranger is an assassin, every friend is an undercover agent, and every coincidence is an orchestrated conspiracy to destroy you.",
    description_pt: "Todo estranho é um assassino em potencial, todo aliado é um agente duplo e qualquer coincidência cotidiana é um complô urdido para aniquilá-lo.",
    game_effects: "Adds +1 to difficulty on all social interaction rolls (or +2 with strangers). The character constantly prepares booby traps, surveillance, and suspects allies of betrayal.",
    game_effects_pt: "Aumenta em +1 a dificuldade de todos os testes de interação social (ou +2 com estranhos). O personagem mantém vigilância obsessiva, desconfia dos aliados da cabala e enxerga conspirações em tudo.",
};

pub const SCHIZOPHRENIA: DerangementDefinition = DerangementDefinition {
    id: "schizophrenia",
    name: "Schizophrenia",
    name_pt: "Esquizofrenia",
    page_ref: "M20, p. 650",
    description: "A terrifying breakdown between internal thoughts and external reality, filling the mind with hallucinations, phantom voices, and fragmented delusions.",
    description_pt: "Ruptura assustadora entre o mundo interior e a realidade factual, povoando a mente com alucinações visuais e auditivas vívidas e delírios fragmentados.",
    game_effects: "Under stress, phantom voices and hallucinations mock or mislead the character. In combat or crisis, a Willpower roll (diff 8) is required to distinguish real threats from hallucinations.",
    game_effects_pt: "Sob estresse ou choque místico, vozes e vultos ilusórios perturbam a concentração do mago. Em combate ou crises, exige teste de Força de Vontade (dif 8) para discernir ameaças reais de visões ilusórias.",
};

pub const ALL_DERANGEMENTS: &[DerangementDefinition] = &[
    AMNESIA,
    ASPD,
    BAD,
    DEMENTIA,
    FUGUE,
    MEGALOMANIA,
    MPD,
    OCD,
    PARANOIA,
    SCHIZOPHRENIA,
];
