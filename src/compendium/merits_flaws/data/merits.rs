//! Qualidades Canônicas de M20 (Merits).
//!
//! Apêndice II: Qualidades e Defeitos (pp. 642-646).

use super::super::models::{MeritFlawDefinition, TraitCategory, TraitType};

pub const ACUTE_SENSES: MeritFlawDefinition = MeritFlawDefinition {
    id: "acute_senses",
    name: "Acute Senses",
    name_pt: "Sentidos Aguçados",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Merit,
    category: TraitCategory::Physical,
    points_str: "1 or 3 pts.",
    points_str_pt: "1 ou 3 pts.",
    available_costs: &[1, 3],
    page_ref: "M20, p. 642",
    description: "Your physical senses are unusually sharp, capturing nuances that few people ever notice.",
    description_pt: "Seus sentidos físicos são extraordinariamente aguçados, captando nuances sutis que passam despercebidas pela maioria das pessoas.",
    system: "Reduces the difficulty of all Perception-based rolls by -2. At the 1-point level, a single sense (vision, hearing, smell, taste, or touch) is acute. At the 3-point level, all five physical senses are equally sharp.",
    system_pt: "Reduz em -2 a dificuldade de todos os testes baseados em Percepção. Por 1 ponto, afeta um único sentido à sua escolha (visão, audição, olfato, paladar ou tato). Por 3 pontos, abrange todos os cinco sentidos simultaneamente.",
};

pub const LANGUAGE: MeritFlawDefinition = MeritFlawDefinition {
    id: "language",
    name: "Language",
    name_pt: "Idioma",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Merit,
    category: TraitCategory::Mental,
    points_str: "1 pt. each",
    points_str_pt: "1 pt. cada",
    available_costs: &[1],
    page_ref: "M20, pp. 642-643",
    description: "Beyond your native language, you understand, speak, read, and write another form of human or mystical communication.",
    description_pt: "Além da sua língua nativa, você compreende, fala, lê e escreve fluentemente outro idioma humano ou dialeto místico antigo.",
    system: "Each 1-point purchase grants fluency in one specific tongue (French, Latin, Mandarin, High Enochian, Jovitos, etc.). If you possess 5 or more Language Merits, you understand linguistic root theory and can attempt to decipher unknown languages (Intelligence + Enigmas, diff 7+).",
    system_pt: "Cada compra de 1 ponto concede fluência plena em uma língua específica (Francês, Latim, Mandarim, Enoquiano Superior, Jovitos, etc.). Ao acumular 5 ou mais Idiomas, você compreende princípios linguísticos profundos e pode decifrar línguas desconhecidas (Inteligência + Enigmas, dif 7+).",
};

pub const DARK_TRIAD: MeritFlawDefinition = MeritFlawDefinition {
    id: "dark_triad",
    name: "Dark Triad",
    name_pt: "Tríade Sombria",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Merit,
    category: TraitCategory::Social,
    points_str: "3 pts.",
    points_str_pt: "3 pts.",
    available_costs: &[3],
    page_ref: "M20, p. 643",
    description: "A charming yet chilling blend of narcissism, Machiavellianism, and sociopathy makes you a smooth-talking rule-breaker who seduces devotees into following your whim.",
    description_pt: "Uma mescla sedutora e arrepiante de narcisismo, maquiavelismo e sociopatia torna você um mestre carismático que convence seguidores a obedecerem seus caprichos mais egoístas.",
    system: "Adds three bonus dice (+3) to all Seduction, Manipulation, Leadership, Subterfuge, and Charisma rolls. However, your complete lack of empathy makes you fundamentally untrustworthy to anyone who realizes your true nature.",
    system_pt: "Adiciona três dados de bônus (+3) a todos os testes de Sedução, Manipulação, Liderança, Lábia e Carisma. Contudo, sua ausência de empatia faz com que você seja considerado perigoso e indigno de confiança por quem descobre sua frieza.",
};

pub const STORMWARDEN: MeritFlawDefinition = MeritFlawDefinition {
    id: "stormwarden",
    name: "Stormwarden",
    name_pt: "Andarilho da Tempestade",
    technocracy_name: Some("Quantum Voyager"),
    technocracy_name_pt: Some("Viajante Quântico"),
    trait_type: TraitType::Merit,
    category: TraitCategory::Supernatural,
    points_str: "3 or 5 pts.",
    points_str_pt: "3 ou 5 pts.",
    available_costs: &[3, 5],
    page_ref: "M20, pp. 643-644",
    description: "Despite the cosmic fury of the Avatar Storm tearing through the Gauntlet, your Avatar possesses legendary immunity to the spectral glass shards.",
    description_pt: "Apesar da fúria cósmica da Tempestade de Avatares que dilacera a Película, seu Avatar possui uma imunidade lendária e preciosa aos estilhaços dimensionais.",
    system: "For 3 points, you pass unharmed through the Gauntlet without suffering Avatar Storm damage. For 5 points, your protective aura extends to anyone and anything you touch and desire to shield as you step sideways.",
    system_pt: "Por 3 pontos, você atravessa a Película para a Umbra sem sofrer dano algum da Tempestade de Avatares. Por 5 pontos, sua proteção estende-se a qualquer pessoa ou objeto que você toque e decida proteger durante a travessia.",
};

pub const TIES: MeritFlawDefinition = MeritFlawDefinition {
    id: "ties",
    name: "Ties",
    name_pt: "Laços / Contatos",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Merit,
    category: TraitCategory::Social,
    points_str: "3 pts. each",
    points_str_pt: "3 pts. cada",
    available_costs: &[3],
    page_ref: "M20, p. 644",
    description: "You have deep connections, goodwill, and influence within a specific institution, subculture, or criminal network.",
    description_pt: "Você mantém conexões profundas, favores acumulados e influência dentro de uma instituição, órgão governamental, subcultura ou facção específica.",
    system: "Each group (Police, Media, Underworld, Black Market, City Hall, Club Scene) costs 3 points. Grants -2 difficulty to subtle requests (running license plates, exclusive invites) and -1 difficulty to major favors (suppressing headlines, planting rumors).",
    system_pt: "Cada grupo específico (Polícia Civil/Militar, Imprensa, Submundo, Mercado Negro, Câmara Municipal, Baladas) custa 3 pontos. Concede -2 na dificuldade de pedidos sutis (puxar placas, convites VIP) e -1 na dificuldade para favores de peso (abafar notícias, plantar boatos).",
};

pub const BERSERKER: MeritFlawDefinition = MeritFlawDefinition {
    id: "berserker",
    name: "Berserker",
    name_pt: "Berserker",
    technocracy_name: Some("Stress Atavism"),
    technocracy_name_pt: Some("Atavismo de Estresse"),
    trait_type: TraitType::Merit,
    category: TraitCategory::Mental,
    points_str: "4 pts.",
    points_str_pt: "4 pts.",
    available_costs: &[4],
    page_ref: "M20, pp. 644-645",
    description: "Under extreme combat stress or injury, a savage red haze overtakes your consciousness, turning you into an unstoppable engine of slaughter.",
    description_pt: "Sob estresse severo de combate, dor ou humilhação, uma fúria selvagem assume seu controle, transformando você em uma máquina de combate implacável.",
    system: "Failed Willpower roll (diff 8) triggers frenzy. You gain three temporary Bruised health levels, ignore all wound penalties, add +2 Strength and +1 Stamina until every enemy is dead. However, allies and bystanders look like targets, and magick/strategy become impossible.",
    system_pt: "Falhar num teste de Força de Vontade (dif 8) sob estresse ativa a fúria. Você ganha 3 níveis temporários de Vitalidade Escoriado, ignora todas as penalidades de dano, recebe +2 em Força e +1 em Vigor. Porém, aliados e inocentes viram alvos, e mágika e estratégia tornam-se impossíveis.",
};

pub const UMBRAL_AFFINITY: MeritFlawDefinition = MeritFlawDefinition {
    id: "umbral_affinity",
    name: "Umbral Affinity",
    name_pt: "Afinidade Umbral",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Merit,
    category: TraitCategory::Supernatural,
    points_str: "4 pts.",
    points_str_pt: "4 pts.",
    available_costs: &[4],
    page_ref: "M20, p. 645",
    description: "A heritage connected to shapeshifters, ancient spirits, or pagan ley lines grants you natural kinship with the spirit worlds.",
    description_pt: "Uma herança ancestral ligada a transmorfos, espíritos arcanos ou linhagens pagãs concede afinidade inata e acolhimento nos Reinos Espirituais.",
    system: "You suffer no penalties from 1st and 2nd degree Umbral Acclimation, and higher stages are reduced by one level. You do not suffer Disembodiment until six full moon cycles (roughly six months) have passed.",
    system_pt: "Você não sofre penalidades por Aclimatação Umbral de 1º e 2º graus, e estágios superiores são reduzidos em um nível. Você só corre risco de Desincorporação após seis ciclos lunares completos (aproximadamente seis meses).",
};

pub const TOO_TOUGH_TO_DIE: MeritFlawDefinition = MeritFlawDefinition {
    id: "too_tough_to_die",
    name: "Too Tough to Die",
    name_pt: "Duro de Matar",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Merit,
    category: TraitCategory::Physical,
    points_str: "5 pts.",
    points_str_pt: "5 pts.",
    available_costs: &[5],
    page_ref: "M20, p. 645",
    description: "You possess extraordinary physiological resilience, pushing through bullet wounds and trauma that would kill an ordinary human on the spot.",
    description_pt: "Você possui uma resistência fisiológica descomunal, suportando tiros, facadas e traumas que matariam ou incapacitariam um humano comum no ato.",
    system: "Allows your character to soak lethal damage with Stamina without needing Life magick. (Does NOT soak aggravated damage; lethal wounds still heal at standard human recovery rates).",
    system_pt: "Permite absorver dano Letal diretamente com a parada de Vigor, sem necessidade de mágika de Vida. (NÃO absorve dano Agravado; os ferimentos letais absorvidos ainda exigem o tempo normal de recuperação médica).",
};

pub const TRUE_FAITH: MeritFlawDefinition = MeritFlawDefinition {
    id: "true_faith",
    name: "True Faith",
    name_pt: "Fé Verdadeira",
    technocracy_name: None,
    technocracy_name_pt: None,
    trait_type: TraitType::Merit,
    category: TraitCategory::Supernatural,
    points_str: "7 pts.",
    points_str_pt: "7 pts.",
    available_costs: &[7],
    page_ref: "M20, pp. 645-646",
    description: "Pure, unshakeable devotion to a higher divine power, pantheon, or absolute creed that transcends human frailty and manifest miracles.",
    description_pt: "Devoção pura, profunda e inabalável ao Divino, a um panteão ou a um credo cósmico autêntico que transcende as fraquezas humanas e canaliza milagres.",
    system: "Grants 1 dot of True Faith. Adds +1 die per point to Willpower rolls, provides innate countermagick against supernatural powers, and repels vampires and demons with holy symbols.",
    system_pt: "Concede 1 ponto de Fé Verdadeira. Acrescenta +1 dado por ponto em testes de Força de Vontade, fornece contramágika inata contra hostilidades sobrenaturais e repele vampiros e demônios ao empunhar símbolos sagrados.",
};

pub const ALL_MERITS: &[MeritFlawDefinition] = &[
    ACUTE_SENSES,
    LANGUAGE,
    DARK_TRIAD,
    STORMWARDEN,
    TIES,
    BERSERKER,
    UMBRAL_AFFINITY,
    TOO_TOUGH_TO_DIE,
    TRUE_FAITH,
];
