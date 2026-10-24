// ==========================================
// Domain Constants (Mage: The Ascension)
// ==========================================

// Header Fields
pub const HEADER_NOME: &str = "Nome";
pub const HEADER_JOGADOR: &str = "Jogador";
pub const HEADER_CRONICA: &str = "Cronica";
pub const HEADER_NATUREZA: &str = "Natureza";
pub const HEADER_ESSENCIA: &str = "Essencia";
pub const HEADER_COMPORTAMENTO: &str = "Comportamento";
pub const HEADER_TRADICAO: &str = "Tradicao";
pub const HEADER_CONCEITO: &str = "Conceito";
pub const HEADER_CABALA: &str = "Cabala";
pub const FIELD_EXPERIENCE: &str = "Experiência";

// Core Special Advantages
pub const KEY_ARETE: &str = "Arete";
pub const KEY_WILLPOWER_TOTAL: &str = "willpower_total";
pub const KEY_WILLPOWER_CURRENT: &str = "willpower_current";
pub const KEY_QUINTESSENCE_PARADOX: &str = "quintessence_paradox_states";
pub const KEY_AFFINITY_SPHERE: &str = "affinity_sphere";
pub const HEALTH_KEY_PREFIX: &str = "health_";

// Categories
pub const CAT_TALENTOS: &str = "Talentos";
pub const CAT_PERICIAS: &str = "Perícias";
pub const CAT_CONHECIMENTOS: &str = "Conhecimentos";
pub const CAT_ANTECEDENTES: &str = "Antecedentes";
pub const CAT_RESONANCE: &str = "Resonance";
pub const CAT_MERITS: &str = "Qualidades";
pub const CAT_FLAWS: &str = "Defeitos";
pub const CAT_OTHER_TRAITS: &str = "other_traits";

// Character Profile Keys
pub const KEY_PROFILE_PHOTO: &str = "profile_photo";
pub const KEY_HISTORY: &str = "profile_history";
pub const KEY_NOTES: &str = "profile_notes";

pub const STANDARD_ATTRIBUTES: [&str; 9] = [
    "Força", "Destreza", "Vigor",
    "Carisma", "Manipulação", "Aparência",
    "Percepção", "Inteligência", "Raciocínio",
];

pub const STANDARD_TALENTS: [&str; 10] = [
    "Prontidão", "Esportes", "Briga", "Esquiva", "Consciência", 
    "Expressão", "Intimidação", "Liderança", "Manha", "Lábia"
];

pub const STANDARD_SKILLS: [&str; 10] = [
    "Ofícios", "Condução", "Etiqueta", "Armas de Fogo", "Meditação", 
    "Armas Brancas", "Performance", "Furtividade", "Sobrevivência", "Tecnologia"
];

pub const STANDARD_KNOWLEDGES: [&str; 10] = [
    "Acadêmicos", "Computador", "Cosmologia", "Enigmas", "Investigação", 
    "Direito", "Medicina", "Ocultismo", "Esotérica", "Ciência"
];

pub const STANDARD_SPHERES: [&str; 9] = [
    "Correspondência", "Entropia", "Forças",
    "Vida", "Matéria", "Mente",
    "Primórdio", "Espírito", "Tempo",
];
