use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use leptos::*;

// ==========================================
// Domain Constants (Mage: The Ascension)
// ==========================================

#[allow(dead_code)]
pub mod keys {
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

    // Character Profile Keys
    pub const KEY_PROFILE_PHOTO: &str = "profile_photo";
    pub const KEY_HISTORY: &str = "profile_history";
    pub const KEY_NOTES: &str = "profile_notes";
}

pub const STANDARD_ATTRIBUTES: [&str; 9] = [
    "Força", "Destreza", "Vigor",
    "Carisma", "Manipulação", "Aparência",
    "Percepção", "Inteligência", "Raciocínio",
];

pub const STANDARD_SPHERES: [&str; 9] = [
    "Correspondência", "Entropia", "Forças",
    "Vida", "Matéria", "Mente",
    "Primórdio", "Espírito", "Tempo",
];

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DamageType {
    #[default]
    None,
    Bashing,
    Lethal,
    Aggravated,
}

impl DamageType {
    pub fn cycle(self) -> Self {
        match self {
            DamageType::None => DamageType::Bashing,
            DamageType::Bashing => DamageType::Lethal,
            DamageType::Lethal => DamageType::Aggravated,
            DamageType::Aggravated => DamageType::None,
        }
    }

    pub fn to_key(self) -> &'static str {
        match self {
            DamageType::None => "none",
            DamageType::Bashing => "bashing",
            DamageType::Lethal => "lethal",
            DamageType::Aggravated => "aggravated",
        }
    }

    pub fn from_key(s: &str) -> Self {
        match s {
            "bashing" => DamageType::Bashing,
            "lethal" => DamageType::Lethal,
            "aggravated" => DamageType::Aggravated,
            _ => DamageType::None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DotOrigin {
    #[default]
    #[serde(alias = "base", alias = "Base")]
    Base,
    #[serde(alias = "bonus", alias = "Bonus")]
    Bonus,
    #[serde(alias = "experience", alias = "Experience", alias = "xp", alias = "XP")]
    Experience,
    #[serde(alias = "temporary", alias = "Temporary", alias = "temp", alias = "Temp")]
    Temporary,
}

impl DotOrigin {
    pub fn as_str(&self) -> &'static str {
        match self {
            DotOrigin::Base => "base",
            DotOrigin::Bonus => "bonus",
            DotOrigin::Experience => "xp",
            DotOrigin::Temporary => "temp",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            DotOrigin::Base => "Criação Base",
            DotOrigin::Bonus => "Pontos de Bônus",
            DotOrigin::Experience => "Experiência (XP)",
            DotOrigin::Temporary => "Magia / Buff",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            DotOrigin::Base => "dot-base",
            DotOrigin::Bonus => "dot-bonus",
            DotOrigin::Experience => "dot-xp",
            DotOrigin::Temporary => "dot-temp",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "bonus" => DotOrigin::Bonus,
            "xp" => DotOrigin::Experience,
            "temp" => DotOrigin::Temporary,
            _ => DotOrigin::Base,
        }
    }
}

pub fn deserialize_flexible_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IntOrString {
        Num(i32),
        Str(String),
    }

    match IntOrString::deserialize(deserializer)? {
        IntOrString::Num(n) => Ok(n),
        IntOrString::Str(s) => Ok(s.trim().parse::<i32>().unwrap_or(0)),
    }
}

pub fn deserialize_flexible_attribute_value<'de, D>(deserializer: D) -> Result<AttributeValue, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AttrValOrStringOrNum {
        Attr(AttributeValue),
        Num(i32),
        Str(String),
    }

    match AttrValOrStringOrNum::deserialize(deserializer)? {
        AttrValOrStringOrNum::Attr(a) => Ok(a),
        AttrValOrStringOrNum::Num(n) => Ok(AttributeValue::new(n, String::new())),
        AttrValOrStringOrNum::Str(s) => {
            let n = s.trim().parse::<i32>().unwrap_or(0);
            Ok(AttributeValue::new(n, String::new()))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AttributeValue {
    #[serde(default, deserialize_with = "deserialize_flexible_i32")]
    pub level: i32,
    #[serde(default)]
    pub modifier: String,
    #[serde(default)]
    pub dot_origins: Vec<DotOrigin>,
}

impl AttributeValue {
    pub fn new(level: i32, modifier: String) -> Self {
        Self {
            level,
            modifier,
            dot_origins: vec![DotOrigin::Base; level.max(0) as usize],
        }
    }

    pub fn get_origins(&self, total_dots: usize) -> Vec<DotOrigin> {
        let mut origins = Vec::with_capacity(total_dots);
        for i in 0..total_dots {
            if i < self.dot_origins.len() {
                origins.push(self.dot_origins[i]);
            } else {
                origins.push(DotOrigin::Base);
            }
        }
        origins
    }

    pub fn set_level_with_origin(&mut self, new_level: i32, default_origin: DotOrigin) {
        let old_level = self.level.max(0) as usize;
        let new_len = new_level.max(0) as usize;

        // 1. Garante que todos os pontos que já existiam preservem sua origem (ou Base se não definidos)
        while self.dot_origins.len() < old_level {
            self.dot_origins.push(DotOrigin::Base);
        }

        // 2. Se aumentou de nível, adiciona APENAS os NOVOS pontos com a nova origem
        if new_len > old_level {
            while self.dot_origins.len() < new_len {
                self.dot_origins.push(default_origin);
            }
        } else {
            // Se diminuiu de nível, remove os últimos pontos mantendo as origens dos anteriores
            self.dot_origins.truncate(new_len);
        }

        self.level = new_level;
    }

    pub fn set_dot_origin(&mut self, dot_index: usize, origin: DotOrigin) {
        if dot_index < self.level.max(0) as usize {
            while self.dot_origins.len() <= dot_index {
                self.dot_origins.push(DotOrigin::Base);
            }
            self.dot_origins[dot_index] = origin;
        }
    }

    pub fn count_origins(&self) -> (usize, usize, usize, usize) {
        let mut base = 0;
        let mut bonus = 0;
        let mut xp = 0;
        let mut temp = 0;
        let len = self.level.max(0) as usize;
        for i in 0..len {
            let orig = if i < self.dot_origins.len() {
                self.dot_origins[i]
            } else {
                DotOrigin::Base
            };
            match orig {
                DotOrigin::Base => base += 1,
                DotOrigin::Bonus => bonus += 1,
                DotOrigin::Experience => xp += 1,
                DotOrigin::Temporary => temp += 1,
            }
        }
        (base, bonus, xp, temp)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CostBreakdownItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub level: i32,
    pub bonus_dots: usize,
    pub bonus_cost: i32,
    pub xp_dots: usize,
    pub xp_cost: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CostSummary {
    pub total_bonus_spent: i32,
    pub bonus_limit: i32, // 15
    pub total_xp_spent: i32,
    pub items: Vec<CostBreakdownItem>,
    pub arete_warning: bool,
    pub arete_total: i32,
    pub affinity_sphere: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeritItem {
    pub name: String,
    pub merit_type: String,
    pub cost: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlawItem {
    pub name: String,
    pub flaw_type: String,
    pub bonus: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WonderItem {
    #[serde(default = "default_wonder_id")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_flexible_attribute_value")]
    pub points: AttributeValue,
    #[serde(default, deserialize_with = "deserialize_flexible_attribute_value")]
    pub arete: AttributeValue,
    #[serde(default = "default_wonder_quint_max", deserialize_with = "deserialize_flexible_i32")]
    pub quintessence_max: i32,
    #[serde(default, alias = "quintessence", deserialize_with = "deserialize_flexible_i32")]
    pub quintessence_current: i32,
    #[serde(default)]
    pub description: String,
}

fn default_wonder_id() -> String {
    format!("wonder_{}", uuid::Uuid::new_v4())
}

fn default_wonder_quint_max() -> i32 {
    5
}

impl Default for WonderItem {
    fn default() -> Self {
        Self {
            id: default_wonder_id(),
            name: String::new(),
            points: AttributeValue::default(),
            arete: AttributeValue::default(),
            quintessence_max: 5,
            quintessence_current: 0,
            description: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WeaponItem {
    pub name: String,
    pub diff: String,
    pub damage: String,
    pub range: String,
    pub rate: String,
    pub clip: String,
    pub conceal: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ArmorItem {
    pub class_name: String,
    pub rating: String,
    pub penalty: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterData {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub attributes: HashMap<String, AttributeValue>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub custom_lists: HashMap<String, Vec<String>>,
    
    // Page 2: Magic & Combat
    #[serde(default)]
    pub merits: Vec<MeritItem>,
    #[serde(default)]
    pub flaws: Vec<FlawItem>,
    #[serde(default)]
    pub wonders: Vec<WonderItem>,
    #[serde(default)]
    pub rotes: String,
    #[serde(default)]
    pub weapons: Vec<WeaponItem>,
    #[serde(default)]
    pub armor: ArmorItem,
}

impl CharacterData {
    pub fn new(id: String, name: String) -> Self {
        let mut sheet = Self {
            id,
            name: if name.trim().is_empty() { "Novo Mago".to_string() } else { name },
            attributes: HashMap::new(),
            labels: HashMap::new(),
            custom_lists: HashMap::new(),
            merits: vec![MeritItem::default(); 7],
            flaws: vec![FlawItem::default(); 7],
            wonders: vec![WonderItem::default(); 3],
            rotes: String::new(),
            weapons: vec![WeaponItem::default(); 4],
            armor: ArmorItem::default(),
        };
        sheet.sanitize();
        sheet
    }

    /// Get attribute/ability/sphere level with a default minimum
    pub fn get_attribute_level(&self, name: &str, default_min: i32) -> i32 {
        self.attributes
            .get(name)
            .map(|a| a.level)
            .unwrap_or(default_min)
            .max(default_min)
    }

    /// Get attribute/ability/sphere modifier
    pub fn get_attribute_modifier(&self, name: &str) -> String {
        self.attributes
            .get(name)
            .map(|a| a.modifier.clone())
            .unwrap_or_default()
    }

    /// Set attribute/ability/sphere level and modifier with dot origin tracking
    pub fn set_attribute(&mut self, name: &str, level: Option<i32>, modifier: Option<String>) {
        let entry = self.attributes.entry(name.to_string()).or_default();
        if let Some(l) = level {
            entry.set_level_with_origin(l, DotOrigin::Base);
        }
        if let Some(m) = modifier {
            entry.modifier = m;
        }
    }

    /// Set attribute with a specific default origin for newly added dots
    pub fn set_attribute_with_origin(&mut self, name: &str, level: Option<i32>, modifier: Option<String>, origin: DotOrigin) {
        let entry = self.attributes.entry(name.to_string()).or_default();
        if let Some(l) = level {
            entry.set_level_with_origin(l, origin);
        }
        if let Some(m) = modifier {
            entry.modifier = m;
        }
    }

    /// Set origin of a specific dot index
    pub fn set_attribute_dot_origin(&mut self, name: &str, dot_index: usize, origin: DotOrigin) {
        let entry = self.attributes.entry(name.to_string()).or_default();
        entry.set_dot_origin(dot_index, origin);
    }

    /// Total count of bonus and xp dots across all attributes/abilities/spheres
    pub fn get_total_bonus_and_xp_dots(&self) -> (usize, usize) {
        let mut total_bonus = 0;
        let mut total_xp = 0;
        for attr in self.attributes.values() {
            let (_, bonus, xp, _) = attr.count_origins();
            total_bonus += bonus;
            total_xp += xp;
        }
        (total_bonus, total_xp)
    }

    /// Get affinity sphere name
    pub fn get_affinity_sphere(&self) -> Option<String> {
        let aff = self.labels.get(keys::KEY_AFFINITY_SPHERE).cloned().unwrap_or_default();
        if aff.trim().is_empty() {
            None
        } else {
            Some(aff)
        }
    }

    /// Set affinity sphere name
    pub fn set_affinity_sphere(&mut self, sphere: Option<String>) {
        if let Some(s) = sphere {
            self.labels.insert(keys::KEY_AFFINITY_SPHERE.to_string(), s);
        } else {
            self.labels.remove(keys::KEY_AFFINITY_SPHERE);
        }
    }

    /// Get profile photo (data URL or empty)
    pub fn get_profile_photo(&self) -> String {
        self.labels.get(keys::KEY_PROFILE_PHOTO).cloned().unwrap_or_default()
    }

    /// Set profile photo
    pub fn set_profile_photo(&mut self, photo: String) {
        if photo.trim().is_empty() {
            self.labels.remove(keys::KEY_PROFILE_PHOTO);
        } else {
            self.labels.insert(keys::KEY_PROFILE_PHOTO.to_string(), photo);
        }
    }

    /// Get character history text
    pub fn get_history(&self) -> String {
        self.labels.get(keys::KEY_HISTORY).cloned().unwrap_or_default()
    }

    /// Set character history text
    pub fn set_history(&mut self, history: String) {
        if history.is_empty() {
            self.labels.remove(keys::KEY_HISTORY);
        } else {
            self.labels.insert(keys::KEY_HISTORY.to_string(), history);
        }
    }

    /// Get character notes text
    pub fn get_notes(&self) -> String {
        self.labels.get(keys::KEY_NOTES).cloned().unwrap_or_default()
    }

    /// Set character notes text
    pub fn set_notes(&mut self, notes: String) {
        if notes.is_empty() {
            self.labels.remove(keys::KEY_NOTES);
        } else {
            self.labels.insert(keys::KEY_NOTES.to_string(), notes);
        }
    }

    /// Calculate full cost summary of Freebie Points (Bonus: 15) and Experience Points (XP)
    pub fn calculate_costs(&self) -> CostSummary {
        let affinity_sphere = self.get_affinity_sphere();
        let mut total_bonus_spent = 0;
        let mut total_xp_spent = 0;
        let mut items = Vec::new();
        let mut visited_keys = std::collections::HashSet::new();
        let mut traits_to_process: Vec<(String, String, String, bool, bool, bool, bool, bool, bool)> = Vec::new();

        // 1. Atributos
        for &attr_name in &STANDARD_ATTRIBUTES {
            visited_keys.insert(attr_name.to_string());
            traits_to_process.push((attr_name.to_string(), attr_name.to_string(), "Atributo".to_string(), false, false, false, false, false, false));
        }

        // 2. Habilidades (Talentos, Perícias, Conhecimentos)
        for (cat_key, cat_label) in [
            (keys::CAT_TALENTOS, "Talento"),
            (keys::CAT_PERICIAS, "Perícia"),
            (keys::CAT_CONHECIMENTOS, "Conhecimento"),
        ] {
            if let Some(list) = self.custom_lists.get(cat_key) {
                for id in list {
                    visited_keys.insert(id.clone());
                    let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                    traits_to_process.push((id.clone(), label, cat_label.to_string(), false, false, false, false, false, false));
                }
            }
        }

        // 3. Esferas
        for &sphere_name in &STANDARD_SPHERES {
            visited_keys.insert(sphere_name.to_string());
            traits_to_process.push((sphere_name.to_string(), sphere_name.to_string(), "Esfera".to_string(), true, false, false, false, false, false));
        }

        // 4. Arete
        let arete_val = self.get_attribute_level(keys::KEY_ARETE, 1);
        visited_keys.insert(keys::KEY_ARETE.to_string());
        traits_to_process.push((keys::KEY_ARETE.to_string(), "Arete".to_string(), "Vantagem".to_string(), false, true, false, false, false, false));

        // 5. Força de Vontade
        visited_keys.insert(keys::KEY_WILLPOWER_TOTAL.to_string());
        traits_to_process.push((keys::KEY_WILLPOWER_TOTAL.to_string(), "Força de Vontade".to_string(), "Vantagem".to_string(), false, false, true, false, false, false));

        // 6. Antecedentes
        if let Some(list) = self.custom_lists.get(keys::CAT_ANTECEDENTES) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Antecedente".to_string(), false, false, false, true, false, false));
            }
        }

        // 7. Ressonância
        if let Some(list) = self.custom_lists.get(keys::CAT_RESONANCE) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Ressonância".to_string(), false, false, false, true, false, false));
            }
        }

        // 8. Qualidades (Merits)
        if let Some(list) = self.custom_lists.get(keys::CAT_MERITS) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Qualidade".to_string(), false, false, false, false, true, false));
            }
        }

        // 9. Defeitos (Flaws)
        if let Some(list) = self.custom_lists.get(keys::CAT_FLAWS) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Defeito".to_string(), false, false, false, false, false, true));
            }
        }

        // 10. Quaisquer outros atributos personalizados
        for (id, attr) in &self.attributes {
            if !visited_keys.contains(id) && attr.level > 0 {
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Outro".to_string(), false, false, false, false, false, false));
            }
        }

        for (id, display_name, category, is_sphere, is_arete, is_willpower, is_background, is_merit, is_flaw) in traits_to_process {
            if let Some(attr) = self.attributes.get(&id) {
                let lvl = attr.level.max(0) as usize;
                if lvl == 0 {
                    continue;
                }
                let origins = attr.get_origins(lvl);
                let mut trait_bonus_cost = 0;
                let mut trait_xp_cost = 0;
                let mut bonus_dots = 0;
                let mut xp_dots = 0;

                let is_affinity = is_sphere && affinity_sphere.as_ref().map(|s| s.eq_ignore_ascii_case(&id)).unwrap_or(false);

                if is_flaw {
                    // Defeitos concedem pontos de bônus (1 pt de bônus por nível de defeito)
                    let flaw_pts = lvl as i32;
                    trait_bonus_cost = -flaw_pts;
                    bonus_dots = lvl;
                } else {
                    for (idx, &origin) in origins.iter().take(lvl).enumerate() {
                        match origin {
                            DotOrigin::Bonus => {
                                bonus_dots += 1;
                                let cost = if is_arete {
                                    4
                                } else if is_sphere {
                                    7
                                } else if is_willpower {
                                    1
                                } else if is_background {
                                    1
                                } else if is_merit {
                                    1 // Qualidades: 1 ponto de bônus por bolinha
                                } else if STANDARD_ATTRIBUTES.contains(&id.as_str()) {
                                    5
                                } else {
                                    // Abilities / Ressonância / Outros
                                    2
                                };
                                trait_bonus_cost += cost;
                            }
                            DotOrigin::Experience => {
                                xp_dots += 1;
                                let cost = if is_arete {
                                    idx as i32 * 8
                                } else if is_sphere {
                                    if idx == 0 {
                                        10 // Nova Esfera
                                    } else if is_affinity {
                                        idx as i32 * 7 // Esfera de Afinidade
                                    } else {
                                        idx as i32 * 8 // Outras Esferas
                                    }
                                } else if is_willpower {
                                    idx as i32 * 1
                                } else if is_background {
                                    if idx == 0 { 3 } else { idx as i32 * 3 }
                                } else if is_merit {
                                    if idx == 0 { 2 } else { idx as i32 * 2 }
                                } else if STANDARD_ATTRIBUTES.contains(&id.as_str()) {
                                    idx as i32 * 4
                                } else {
                                    // Abilities
                                    if idx == 0 {
                                        3 // Nova Habilidade
                                    } else {
                                        idx as i32 * 2 // Nível atual * 2
                                    }
                                };
                                trait_xp_cost += cost;
                            }
                            _ => {}
                        }
                    }
                }

                if trait_bonus_cost != 0 || trait_xp_cost > 0 {
                    total_bonus_spent += trait_bonus_cost;
                    total_xp_spent += trait_xp_cost;
                    items.push(CostBreakdownItem {
                        id,
                        name: display_name,
                        category: if is_affinity { "Esfera de Afinidade".to_string() } else { category },
                        level: attr.level,
                        bonus_dots,
                        bonus_cost: trait_bonus_cost,
                        xp_dots,
                        xp_cost: trait_xp_cost,
                    });
                }
            }
        }

        // 11. Maravilhas (Wonders)
        for (w_idx, wonder) in self.wonders.iter().enumerate() {
            if wonder.points.level > 0 {
                let display_name = if wonder.name.trim().is_empty() {
                    format!("Maravilha {}", w_idx + 1)
                } else {
                    format!("Maravilha ({})", wonder.name.trim())
                };
                let id = if !wonder.id.is_empty() { wonder.id.clone() } else { format!("wonder_{}", w_idx) };
                let lvl = wonder.points.level.max(0) as usize;
                let origins = wonder.points.get_origins(lvl);
                let mut trait_bonus_cost = 0;
                let mut trait_xp_cost = 0;
                let mut bonus_dots = 0;
                let mut xp_dots = 0;

                for (idx, &origin) in origins.iter().take(lvl).enumerate() {
                    match origin {
                        DotOrigin::Bonus => {
                            bonus_dots += 1;
                            trait_bonus_cost += 1; // 1 Ponto de Bônus por bolinha de Maravilha
                        }
                        DotOrigin::Experience => {
                            xp_dots += 1;
                            let cost = if idx == 0 { 3 } else { idx as i32 * 3 }; // 3 XP por nível (como Antecedentes)
                            trait_xp_cost += cost;
                        }
                        _ => {}
                    }
                }

                if trait_bonus_cost > 0 || trait_xp_cost > 0 {
                    total_bonus_spent += trait_bonus_cost;
                    total_xp_spent += trait_xp_cost;
                    items.push(CostBreakdownItem {
                        id,
                        name: display_name,
                        category: "Maravilha (Antecedente)".to_string(),
                        level: wonder.points.level,
                        bonus_dots,
                        bonus_cost: trait_bonus_cost,
                        xp_dots,
                        xp_cost: trait_xp_cost,
                    });
                }
            }
        }

        // Arete warning: na criação de ficha (pontos base + bônus), Arete não deve passar de 3
        let arete_creation_dots = self.attributes.get(keys::KEY_ARETE).map(|a| {
            let (_, bonus, _, _) = a.count_origins();
            let base = a.level.max(0) as usize - a.dot_origins.iter().filter(|&&o| o == DotOrigin::Experience || o == DotOrigin::Temporary).count();
            base.max(1) + bonus
        }).unwrap_or(1);
        let arete_warning = arete_creation_dots > 3;

        CostSummary {
            total_bonus_spent,
            bonus_limit: 15,
            total_xp_spent,
            items,
            arete_warning,
            arete_total: arete_val,
            affinity_sphere,
        }
    }

    /// Helper to get single dot cost and explanation for tooltips
    pub fn get_dot_cost_description(trait_name: &str, dot_idx: usize, origin: DotOrigin, is_affinity: bool) -> (i32, String) {
        match origin {
            DotOrigin::Base => (0, "Criação Base (Grátis)".to_string()),
            DotOrigin::Temporary => (0, "Efeito Temporário / Magia".to_string()),
            DotOrigin::Bonus => {
                let cost = if trait_name == keys::KEY_ARETE {
                    4
                } else if STANDARD_SPHERES.contains(&trait_name) {
                    7
                } else if trait_name == keys::KEY_WILLPOWER_TOTAL {
                    1
                } else if STANDARD_ATTRIBUTES.contains(&trait_name) {
                    5
                } else {
                    // Habilidades / Antecedentes / Outros
                    if trait_name.starts_with("bg_") { 1 } else { 2 }
                };
                (cost, format!("{} pts de Bônus", cost))
            }
            DotOrigin::Experience => {
                if trait_name == keys::KEY_ARETE {
                    let cost = dot_idx as i32 * 8;
                    (cost, format!("{} XP (Nível {} -> {})", cost, dot_idx, dot_idx + 1))
                } else if STANDARD_SPHERES.contains(&trait_name) {
                    if dot_idx == 0 {
                        (10, "10 XP (Nova Esfera)".to_string())
                    } else if is_affinity {
                        let cost = dot_idx as i32 * 7;
                        (cost, format!("{} XP (Afinidade Nível {} -> {})", cost, dot_idx, dot_idx + 1))
                    } else {
                        let cost = dot_idx as i32 * 8;
                        (cost, format!("{} XP (Nível {} -> {})", cost, dot_idx, dot_idx + 1))
                    }
                } else if trait_name == keys::KEY_WILLPOWER_TOTAL {
                    let cost = dot_idx as i32 * 1;
                    (cost, format!("{} XP (Nível {} -> {})", cost, dot_idx, dot_idx + 1))
                } else if STANDARD_ATTRIBUTES.contains(&trait_name) {
                    let cost = dot_idx as i32 * 4;
                    (cost, format!("{} XP (Nível {} -> {})", cost, dot_idx, dot_idx + 1))
                } else if trait_name.starts_with("bg_") {
                    let cost = if dot_idx == 0 { 3 } else { dot_idx as i32 * 3 };
                    (cost, format!("{} XP (Nível {} -> {})", cost, dot_idx, dot_idx + 1))
                } else {
                    // Ability
                    if dot_idx == 0 {
                        (3, "3 XP (Nova Habilidade)".to_string())
                    } else {
                        let cost = dot_idx as i32 * 2;
                        (cost, format!("{} XP (Nível {} -> {})", cost, dot_idx, dot_idx + 1))
                    }
                }
            }
        }
    }

    /// Get label value
    pub fn get_label(&self, key: &str) -> String {
        self.labels.get(key).cloned().unwrap_or_default()
    }

    /// Set label value
    pub fn set_label(&mut self, key: &str, val: String) {
        self.labels.insert(key.to_string(), val);
    }

    /// Get Willpower: (total, current)
    pub fn get_willpower(&self) -> (i32, i32) {
        let total = self
            .attributes
            .get(keys::KEY_WILLPOWER_TOTAL)
            .map(|a| a.level)
            .unwrap_or(5)
            .clamp(1, 10);
        let current = self
            .attributes
            .get(keys::KEY_WILLPOWER_CURRENT)
            .map(|a| a.level)
            .unwrap_or(total)
            .clamp(0, total);
        (total, current)
    }

    /// Set Willpower Total (adjusts current if it exceeds new total)
    pub fn set_willpower_total(&mut self, total: i32) {
        let val = total.clamp(1, 10);
        self.attributes
            .entry(keys::KEY_WILLPOWER_TOTAL.to_string())
            .or_default()
            .level = val;
        let current = self
            .attributes
            .entry(keys::KEY_WILLPOWER_CURRENT.to_string())
            .or_default();
        if current.level > val {
            current.level = val;
        }
    }

    /// Set Willpower Current
    pub fn set_willpower_current(&mut self, current: i32) {
        let (total, _) = self.get_willpower();
        let val = current.clamp(0, total);
        self.attributes
            .entry(keys::KEY_WILLPOWER_CURRENT.to_string())
            .or_default()
            .level = val;
    }

    /// Get Arete
    pub fn get_arete(&self) -> i32 {
        self.attributes
            .get(keys::KEY_ARETE)
            .map(|a| a.level)
            .unwrap_or(1)
            .clamp(1, 10)
    }

    /// Set Arete
    pub fn set_arete(&mut self, val: i32) {
        self.attributes
            .entry(keys::KEY_ARETE.to_string())
            .or_default()
            .level = val.clamp(1, 10);
    }

    /// Get Health level damage type at index 0..6
    pub fn get_health(&self, index: usize) -> DamageType {
        let key = format!("{}{}", keys::HEALTH_KEY_PREFIX, index);
        let val = self.labels.get(&key).map(|s| s.as_str()).unwrap_or("none");
        DamageType::from_key(val)
    }

    /// Set Health level damage type at index 0..6
    pub fn set_health(&mut self, index: usize, dmg: DamageType) {
        let key = format!("{}{}", keys::HEALTH_KEY_PREFIX, index);
        self.labels.insert(key, dmg.to_key().to_string());
    }

    /// Get Quintessence and Paradox totals: (quintessence, paradox, raw_string)
    pub fn get_quintessence_paradox(&self) -> (i32, i32, String) {
        let raw = self
            .labels
            .get(keys::KEY_QUINTESSENCE_PARADOX)
            .cloned()
            .unwrap_or_else(|| "0".repeat(20));
        let normalized = if raw.len() == 20 { raw } else { "0".repeat(20) };
        let quint = normalized.chars().filter(|&c| c == '1').count() as i32;
        let paradox = normalized.chars().filter(|&c| c == '2').count() as i32;
        (quint, paradox, normalized)
    }

    /// Cycle box state in Quintessence/Paradox wheel: '0' -> '1' (Quintessence) -> '2' (Paradox) -> '0'
    pub fn cycle_quintessence_paradox_box(&mut self, index: usize) {
        if index >= 20 {
            return;
        }
        let raw = self
            .labels
            .entry(keys::KEY_QUINTESSENCE_PARADOX.to_string())
            .or_insert_with(|| "0".repeat(20));
        if raw.len() != 20 {
            *raw = "0".repeat(20);
        }
        let mut chars: Vec<char> = raw.chars().collect();
        chars[index] = match chars[index] {
            '0' => '1',
            '1' => '2',
            _ => '0',
        };
        *raw = chars.into_iter().collect();
    }

    /// Sanitize data: clamp attributes, ensure valid bounds, fix name
    pub fn sanitize(&mut self) {
        if self.name.trim().is_empty() {
            self.name = "Sem Nome".to_string();
        }

        // Ensure Arete is at least 1
        let arete = self.attributes.entry(keys::KEY_ARETE.to_string()).or_default();
        if arete.level < 1 {
            arete.level = 1;
        } else if arete.level > 10 {
            arete.level = 10;
        }

        // Ensure Willpower values are within valid ranges
        let (total, current) = self.get_willpower();
        self.attributes.entry(keys::KEY_WILLPOWER_TOTAL.to_string()).or_default().level = total;
        self.attributes.entry(keys::KEY_WILLPOWER_CURRENT.to_string()).or_default().level = current;

        // Ensure Quintessence/Paradox states string has 20 characters
        let qp = self.labels.entry(keys::KEY_QUINTESSENCE_PARADOX.to_string()).or_insert_with(|| "0".repeat(20));
        if qp.len() != 20 || !qp.chars().all(|c| c == '0' || c == '1' || c == '2') {
            *qp = "0".repeat(20);
        }

        // Normalize dot origins for all attributes
        for attr in self.attributes.values_mut() {
            let lvl = attr.level.max(0) as usize;
            while attr.dot_origins.len() < lvl {
                attr.dot_origins.push(DotOrigin::Base);
            }
            if attr.dot_origins.len() > lvl {
                attr.dot_origins.truncate(lvl);
            }
        }

        // Ensure minimum slots for Merits (7), Flaws (7) and Weapons (4)
        while self.merits.len() < 7 {
            self.merits.push(MeritItem::default());
        }
        while self.flaws.len() < 7 {
            self.flaws.push(FlawItem::default());
        }
        if self.wonders.is_empty() {
            self.wonders.push(WonderItem::default());
        }
        for wonder in &mut self.wonders {
            if wonder.id.is_empty() {
                wonder.id = format!("wonder_{}", uuid::Uuid::new_v4());
            }
            if wonder.quintessence_max < 5 {
                wonder.quintessence_max = 5;
            } else if wonder.quintessence_max > 20 {
                wonder.quintessence_max = 20;
            } else {
                // Round to nearest multiple of 5
                wonder.quintessence_max = ((wonder.quintessence_max + 4) / 5) * 5;
            }
            wonder.quintessence_current = wonder.quintessence_current.clamp(0, wonder.quintessence_max);

            let p_lvl = wonder.points.level.max(0) as usize;
            while wonder.points.dot_origins.len() < p_lvl {
                wonder.points.dot_origins.push(DotOrigin::Base);
            }
            if wonder.points.dot_origins.len() > p_lvl {
                wonder.points.dot_origins.truncate(p_lvl);
            }

            let a_lvl = wonder.arete.level.max(0) as usize;
            while wonder.arete.dot_origins.len() < a_lvl {
                wonder.arete.dot_origins.push(DotOrigin::Base);
            }
            if wonder.arete.dot_origins.len() > a_lvl {
                wonder.arete.dot_origins.truncate(a_lvl);
            }
        }
        while self.weapons.len() < 4 {
            self.weapons.push(WeaponItem::default());
        }
    }

    /// Resilient JSON recovery for backwards compatibility and damaged data
    pub fn from_raw_json_resilient(id: &str, raw_json: &str) -> Option<Self> {
        let val: serde_json::Value = serde_json::from_str(raw_json).ok()?;
        let mut char_data = CharacterData::new(id.to_string(), "Personagem Recuperado".to_string());

        if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
            char_data.name = name.to_string();
        }

        if let Some(attrs) = val.get("attributes").and_then(|v| v.as_object()) {
            for (k, v) in attrs {
                if let Ok(attr) = serde_json::from_value::<AttributeValue>(v.clone()) {
                    char_data.attributes.insert(k.clone(), attr);
                } else if let Some(n) = v.as_i64() {
                    char_data.attributes.insert(k.clone(), AttributeValue::new(n as i32, String::new()));
                } else if let Some(s) = v.as_str() {
                    let n = s.trim().parse::<i32>().unwrap_or(0);
                    char_data.attributes.insert(k.clone(), AttributeValue::new(n, String::new()));
                }
            }
        }

        if let Some(labels) = val.get("labels").and_then(|v| v.as_object()) {
            for (k, v) in labels {
                if let Some(s) = v.as_str() {
                    char_data.labels.insert(k.clone(), s.to_string());
                }
            }
        }

        if let Some(lists) = val.get("custom_lists").and_then(|v| v.as_object()) {
            for (k, v) in lists {
                if let Some(arr) = v.as_array() {
                    let list: Vec<String> = arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect();
                    char_data.custom_lists.insert(k.clone(), list);
                }
            }
        }

        if let Some(wonders) = val.get("wonders").and_then(|v| v.as_array()) {
            char_data.wonders.clear();
            for w in wonders {
                if let Ok(wonder) = serde_json::from_value::<WonderItem>(w.clone()) {
                    char_data.wonders.push(wonder);
                }
            }
        }

        if let Some(weapons) = val.get("weapons").and_then(|v| v.as_array()) {
            char_data.weapons.clear();
            for w in weapons {
                if let Ok(weapon) = serde_json::from_value::<WeaponItem>(w.clone()) {
                    char_data.weapons.push(weapon);
                }
            }
        }

        if let Some(armor) = val.get("armor") {
            if let Ok(a) = serde_json::from_value::<ArmorItem>(armor.clone()) {
                char_data.armor = a;
            }
        }

        if let Some(rotes) = val.get("rotes").and_then(|v| v.as_str()) {
            char_data.rotes = rotes.to_string();
        }

        char_data.sanitize();
        Some(char_data)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterSummary {
    pub id: String,
    pub name: String,
    pub updated_at: String,
}

// ==========================================
// Server Functions with Robust Error Handling
// ==========================================

#[server(endpoint = "get_sheets")]
pub async fn get_sheets() -> Result<Vec<CharacterSummary>, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_sheets", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let start = std::time::Instant::now();
    let rows = sqlx::query("SELECT id, name, updated_at FROM character_sheets ORDER BY updated_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch sheets from DB", Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao consultar fichas: {}", e))
        })?;

    let count = rows.len();
    let summaries = rows.into_iter().map(|row| CharacterSummary {
        id: row.get("id"),
        name: row.get("name"),
        updated_at: row.get("updated_at"),
    }).collect();

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT character_sheets: retornou {} fichas em {}ms", count, start.elapsed().as_millis()),
        None,
    );

    Ok(summaries)
}

#[server(endpoint = "get_sheet")]
pub async fn get_sheet(id: String) -> Result<CharacterData, ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não fornecido"));
    }

    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_sheet", Some(&format!("id={}", id)));
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let start = std::time::Instant::now();
    let row = sqlx::query("SELECT data FROM character_sheets WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Error querying sheet {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Erro ao buscar ficha no banco: {}", e))
        })?
        .ok_or_else(|| {
            crate::logging::server::write_log(crate::logging::LogCategory::Requests, "WARN", &format!("Sheet with id {} not found", id), None);
            ServerFnError::new(format!("Ficha com ID '{}' não encontrada", id))
        })?;

    let data_json: String = row.get("data");
    let mut data: CharacterData = match serde_json::from_str(&data_json) {
        Ok(d) => d,
        Err(e) => {
            crate::logging::server::write_log(
                crate::logging::LogCategory::Errors,
                "WARN",
                &format!("JSON parsing falhou para ficha {}. Tentando recuperação resiliente...", id),
                Some(&e.to_string()),
            );
            CharacterData::from_raw_json_resilient(&id, &data_json).ok_or_else(|| {
                crate::logging::server::write_log(
                    crate::logging::LogCategory::Errors,
                    "ERROR",
                    &format!("Corrupted JSON for sheet {}", id),
                    Some(&e.to_string()),
                );
                ServerFnError::new(format!("Dados da ficha corrompidos: {}", e))
            })?
        }
    };

    data.sanitize();
    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT character_sheets id='{}' (nome='{}') carregada com sucesso em {}ms", id, data.name, start.elapsed().as_millis()),
        None,
    );

    Ok(data)
}

#[server(endpoint = "create_sheet")]
pub async fn create_sheet(name: String) -> Result<String, ServerFnError> {
    let clean_name = name.trim().to_string();
    let final_name = if clean_name.is_empty() { "Novo Mago".to_string() } else { clean_name };

    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in create_sheet", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let start = std::time::Instant::now();
    let id = Uuid::new_v4().to_string();
    let initial_data = CharacterData::new(id.clone(), final_name.clone());

    let data_json = serde_json::to_string(&initial_data).map_err(|e: serde_json::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Serialization error creating sheet", Some(&e.to_string()));
        ServerFnError::new(format!("Falha ao serializar dados iniciais: {}", e))
    })?;

    sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&final_name)
        .bind(data_json)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to insert new sheet {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao salvar nova ficha no banco: {}", e))
        })?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("CREATE SHEET: Nova ficha criada id='{}', nome='{}' em {}ms", id, final_name, start.elapsed().as_millis()),
        None,
    );

    Ok(id)
}

#[server(endpoint = "update_sheet")]
pub async fn update_sheet(id: String, data: CharacterData) -> Result<(), ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não pode ser vazio"));
    }

    let mut data = data;
    data.sanitize();

    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in update_sheet", Some(&format!("id={}", id)));
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let start = std::time::Instant::now();
    let data_json = serde_json::to_string(&data).map_err(|e: serde_json::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Serialization error updating sheet {}", id), Some(&e.to_string()));
        ServerFnError::new(format!("Falha ao serializar dados da ficha: {}", e))
    })?;

    let payload_kb = (data_json.len() as f64) / 1024.0;
    let result = sqlx::query("UPDATE character_sheets SET name = ?, data = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(&data.name)
        .bind(data_json)
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to update sheet {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao atualizar dados da ficha no banco: {}", e))
        })?;

    if result.rows_affected() == 0 {
        crate::logging::server::write_log(crate::logging::LogCategory::Requests, "WARN", &format!("Ficha com ID '{}' não encontrada para atualização", id), None);
        return Err(ServerFnError::new(format!("Ficha com ID '{}' não encontrada para atualização", id)));
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("UPDATE character_sheets id='{}' (nome='{}') salva com sucesso em {}ms ({:.1} KB)", id, data.name, start.elapsed().as_millis(), payload_kb),
        None,
    );

    Ok(())
}

#[server(endpoint = "delete_sheet")]
pub async fn delete_sheet(id: String) -> Result<(), ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não pode ser vazio"));
    }

    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in delete_sheet", Some(&format!("id={}", id)));
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let start = std::time::Instant::now();
    let result = sqlx::query("DELETE FROM character_sheets WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to delete sheet {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao excluir ficha do banco: {}", e))
        })?;

    if result.rows_affected() == 0 {
        return Err(ServerFnError::new(format!("Ficha com ID '{}' não encontrada", id)));
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("DELETE SHEET: Ficha '{}' excluída com sucesso em {}ms", id, start.elapsed().as_millis()),
        None,
    );

    Ok(())
}

// ==========================================
// Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_type_cycle() {
        let dmg = DamageType::None;
        assert_eq!(dmg.cycle(), DamageType::Bashing);
        assert_eq!(dmg.cycle().cycle(), DamageType::Lethal);
        assert_eq!(dmg.cycle().cycle().cycle(), DamageType::Aggravated);
        assert_eq!(dmg.cycle().cycle().cycle().cycle(), DamageType::None);
    }

    #[test]
    fn test_willpower_clamp_and_current_adjustment() {
        let mut char_data = CharacterData::new("123".to_string(), "Hermes".to_string());
        char_data.set_willpower_total(8);
        char_data.set_willpower_current(7);

        let (total, current) = char_data.get_willpower();
        assert_eq!(total, 8);
        assert_eq!(current, 7);

        // Lowering total below current must clamp current
        char_data.set_willpower_total(5);
        let (total, current) = char_data.get_willpower();
        assert_eq!(total, 5);
        assert_eq!(current, 5);
    }

    #[test]
    fn test_arete_bounds() {
        let mut char_data = CharacterData::new("123".to_string(), "Hermes".to_string());
        assert_eq!(char_data.get_arete(), 1);

        char_data.set_arete(4);
        assert_eq!(char_data.get_arete(), 4);

        char_data.set_arete(15);
        assert_eq!(char_data.get_arete(), 10);
    }

    #[test]
    fn test_quintessence_paradox_counting() {
        let mut char_data = CharacterData::new("123".to_string(), "Hermes".to_string());
        let (q, p, _) = char_data.get_quintessence_paradox();
        assert_eq!(q, 0);
        assert_eq!(p, 0);

        char_data.cycle_quintessence_paradox_box(0); // -> '1' (Quintessence)
        char_data.cycle_quintessence_paradox_box(1); // -> '1'
        char_data.cycle_quintessence_paradox_box(1); // -> '2' (Paradox)

        let (q, p, _) = char_data.get_quintessence_paradox();
        assert_eq!(q, 1);
        assert_eq!(p, 1);
    }

    #[test]
    fn test_sanitize_fixes_corrupt_data() {
        let mut char_data = CharacterData {
            id: "test".to_string(),
            name: "   ".to_string(),
            attributes: HashMap::new(),
            labels: HashMap::new(),
            custom_lists: HashMap::new(),
            ..Default::default()
        };

        char_data.sanitize();
        assert_eq!(char_data.name, "Sem Nome");
        assert_eq!(char_data.get_arete(), 1);
        assert_eq!(char_data.get_willpower(), (5, 5));
    }

    #[test]
    fn test_dot_origins_tracking_and_counting() {
        let mut attr = AttributeValue::new(3, "Espadas".to_string());
        assert_eq!(attr.dot_origins.len(), 3);
        assert_eq!(attr.count_origins(), (3, 0, 0, 0));

        // Add 1 dot with Bonus origin
        attr.set_level_with_origin(4, DotOrigin::Bonus);
        assert_eq!(attr.level, 4);
        assert_eq!(attr.count_origins(), (3, 1, 0, 0));

        // Add 1 dot with XP origin
        attr.set_level_with_origin(5, DotOrigin::Experience);
        assert_eq!(attr.level, 5);
        assert_eq!(attr.count_origins(), (3, 1, 1, 0));

        // Manually toggle dot 0 to Temporary
        attr.set_dot_origin(0, DotOrigin::Temporary);
        assert_eq!(attr.count_origins(), (2, 1, 1, 1));

        // Lower level to 3 truncates the last dots (leaving Temporary at 0 and Base at 1, 2)
        attr.set_level_with_origin(3, DotOrigin::Base);
        assert_eq!(attr.level, 3);
        assert_eq!(attr.count_origins(), (2, 0, 0, 1));
    }

    #[test]
    fn test_legacy_empty_origins_upgrade_preserves_base_dots() {
        // Simulates an existing attribute from DB with level 3 and empty dot_origins
        let mut attr = AttributeValue {
            level: 3,
            modifier: String::new(),
            dot_origins: Vec::new(),
        };

        // Adding 4th dot with Bonus mode must keep first 3 as Base and only 4th as Bonus
        attr.set_level_with_origin(4, DotOrigin::Bonus);
        assert_eq!(attr.level, 4);
        assert_eq!(attr.dot_origins, vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Bonus]);
        assert_eq!(attr.count_origins(), (3, 1, 0, 0));

        // Adding 5th dot with XP mode must add 5th as XP
        attr.set_level_with_origin(5, DotOrigin::Experience);
        assert_eq!(attr.level, 5);
        assert_eq!(attr.dot_origins, vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Experience]);
        assert_eq!(attr.count_origins(), (3, 1, 1, 0));
    }

    #[test]
    fn test_calculate_costs_freebies_and_xp() {
        let mut char_data = CharacterData::new("c1".to_string(), "Mago Teste".to_string());

        // 1. Força (Attribute): Level 4 -> [Base, Base, Bonus, Experience]
        // Bonus at idx 2: 5 pts
        // XP at idx 3: idx 3 * 4 = 12 XP
        char_data.attributes.insert("Força".to_string(), AttributeValue {
            level: 4,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Experience],
        });

        // 2. Custom Ability (Talento): Level 3 -> [Base, Bonus, Experience]
        // Bonus at idx 1: 2 pts
        // XP at idx 2: idx 2 * 2 = 4 XP
        char_data.custom_lists.insert(keys::CAT_TALENTOS.to_string(), vec!["tal_1".to_string()]);
        char_data.labels.insert("tal_1".to_string(), "Prontidão".to_string());
        char_data.attributes.insert("tal_1".to_string(), AttributeValue {
            level: 3,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Experience],
        });

        // 3. Forças (Sphere): Level 2 -> [Base, Experience] (Affinity Sphere)
        // XP at idx 1 (Affinity): 1 * 7 = 7 XP
        char_data.set_affinity_sphere(Some("Forças".to_string()));
        char_data.attributes.insert("Forças".to_string(), AttributeValue {
            level: 2,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Experience],
        });

        // 4. Correspondência (Sphere): Level 2 -> [Base, Experience] (Other Sphere)
        // XP at idx 1 (Non-affinity): 1 * 8 = 8 XP
        char_data.attributes.insert("Correspondência".to_string(), AttributeValue {
            level: 2,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Experience],
        });

        // 5. Arete: Level 2 -> [Base, Bonus]
        // Bonus at idx 1: 4 pts
        char_data.attributes.insert(keys::KEY_ARETE.to_string(), AttributeValue {
            level: 2,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Bonus],
        });

        // 6. Willpower Total: Level 6 -> [Base x 5, Bonus]
        // Bonus at idx 5: 1 pt
        char_data.attributes.insert(keys::KEY_WILLPOWER_TOTAL.to_string(), AttributeValue {
            level: 6,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Bonus],
        });

        let summary = char_data.calculate_costs();

        // Total Bonus: 5 (Força) + 2 (Talento) + 4 (Arete) + 1 (Willpower) = 12 pts
        assert_eq!(summary.total_bonus_spent, 12);
        assert_eq!(summary.bonus_limit, 15);
        assert_eq!(summary.arete_warning, false); // Arete is 2 (<= 3)

        // Total XP: 12 (Força) + 4 (Talento) + 7 (Forças Afinidade) + 8 (Correspondência) = 31 XP
        assert_eq!(summary.total_xp_spent, 31);
    }

    #[test]
    fn test_calculate_costs_with_merits_and_flaws() {
        let mut char_data = CharacterData::new("c2".to_string(), "Mago Merits Test".to_string());

        // Qualidade: Level 3 com 2 Bônus -> +2 pontos de bônus
        char_data.custom_lists.insert(keys::CAT_MERITS.to_string(), vec!["merit_1".to_string()]);
        char_data.labels.insert("merit_1".to_string(), "Avatar Focado".to_string());
        char_data.attributes.insert("merit_1".to_string(), AttributeValue {
            level: 3,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Bonus],
        });

        // Defeito: Level 3 -> concede 3 pontos de bônus (-3 no total_bonus_spent)
        char_data.custom_lists.insert(keys::CAT_FLAWS.to_string(), vec!["flaw_1".to_string()]);
        char_data.labels.insert("flaw_1".to_string(), "Inimigo Jurado".to_string());
        char_data.attributes.insert("flaw_1".to_string(), AttributeValue {
            level: 3,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Base],
        });

        let summary = char_data.calculate_costs();
        // Total Bonus: +2 (Qualidade) - 3 (Defeito) = -1 pts
        assert_eq!(summary.total_bonus_spent, -1);
    }

    #[test]
    fn test_calculate_costs_with_wonders() {
        let mut char_data = CharacterData::new("c3".to_string(), "Mago Wonder Test".to_string());

        // Maravilha: Nível 3 (1 Base, 1 Bonus, 1 XP)
        // Bonus at idx 1: 1 pt
        // XP at idx 2: 2 * 3 = 6 XP
        char_data.wonders = vec![WonderItem {
            id: "w1".to_string(),
            name: "Anel do Éter".to_string(),
            points: AttributeValue {
                level: 3,
                modifier: String::new(),
                dot_origins: vec![DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Experience],
            },
            arete: AttributeValue {
                level: 2,
                modifier: String::new(),
                dot_origins: vec![DotOrigin::Base, DotOrigin::Base],
            },
            quintessence_max: 10,
            quintessence_current: 7,
            description: "Armazena quintessência e manipula forças".to_string(),
        }];

        let summary = char_data.calculate_costs();
        assert_eq!(summary.total_bonus_spent, 1);
        assert_eq!(summary.total_xp_spent, 6);
    }

    #[test]
    fn test_legacy_string_wonders_deserialization() {
        let legacy_json = r#"{
            "id": "legacy_char_1",
            "name": "Mago do Passado",
            "attributes": {},
            "labels": {},
            "custom_lists": {},
            "wonders": [
                {
                    "name": "Grimório Antigo",
                    "points": "5",
                    "arete": "3",
                    "quintessence": "4",
                    "description": "Livro de couro antigo"
                }
            ]
        }"#;

        let res: Result<CharacterData, _> = serde_json::from_str(legacy_json);
        assert!(res.is_ok(), "Failed to deserialize legacy CharacterData: {:?}", res.err());
        
        let mut char_data = res.unwrap();
        char_data.sanitize();

        assert_eq!(char_data.wonders.len(), 1);
        let wonder = &char_data.wonders[0];
        assert_eq!(wonder.name, "Grimório Antigo");
        assert_eq!(wonder.points.level, 5);
        assert_eq!(wonder.arete.level, 3);
        assert_eq!(wonder.quintessence_max, 5);
        assert_eq!(wonder.quintessence_current, 4);
    }

    #[test]
    fn test_schema_evolution_fuzzing_all_field_type_permutations() {
        // Test combinations: numbers, numeric strings, empty strings, missing fields
        let permutations = vec![
            // 1. All strings format
            r#"{"id":"fuzz_1","name":"Fuzz 1","attributes":{"Força":{"level":"4","modifier":"","dot_origins":[]}},"labels":{},"custom_lists":{},"wonders":[{"name":"W1","points":"3","arete":"2","quintessence":"5","description":""}]}"#,
            // 2. All numbers format
            r#"{"id":"fuzz_2","name":"Fuzz 2","attributes":{"Força":{"level":4,"modifier":"","dot_origins":[]}},"labels":{},"custom_lists":{},"wonders":[{"name":"W2","points":{"level":3,"modifier":"","dot_origins":[]},"arete":{"level":2,"modifier":"","dot_origins":[]},"quintessence_max":10,"quintessence_current":5,"description":""}]}"#,
            // 3. Mixed empty strings and defaults
            r#"{"id":"fuzz_3","name":"","attributes":{},"labels":{},"custom_lists":{},"wonders":[{"name":"W3","points":"","arete":"","quintessence":"","description":""}]}"#,
            // 4. Missing fields entirely
            r#"{"id":"fuzz_4","name":"Fuzz 4","attributes":{}}"#,
        ];

        for (idx, json_str) in permutations.into_iter().enumerate() {
            let res: Result<CharacterData, _> = serde_json::from_str(json_str);
            assert!(res.is_ok(), "Permutation {} failed to deserialize: {:?}", idx + 1, res.err());
            let mut char_data = res.unwrap();
            char_data.sanitize();
            assert!(!char_data.name.is_empty(), "Sanitize should ensure non-empty name");
        }
    }

    #[test]
    fn test_resilient_recovery_from_heavily_corrupted_json() {
        // Severely mangled JSON with unexpected arrays and mixed structures
        let broken_json = r#"{
            "id": "broken_123",
            "name": "Mago Sobrevivente",
            "attributes": {
                "Força": "4",
                "Destreza": 3,
                "Vigor": { "level": "5", "modifier": "Resistente", "dot_origins": ["bonus"] }
            },
            "labels": {
                "Conceito": "Sobrevivente",
                "profile_history": "Histórico longo..."
            },
            "custom_lists": {
                "Talentos": ["tal_1", "tal_2"]
            },
            "wonders": [
                { "name": "Amuleto", "points": "2", "arete": 1, "quintessence": "5" }
            ],
            "rotes": "Fórmulas de sobrevivência"
        }"#;

        let recovered = CharacterData::from_raw_json_resilient("broken_123", broken_json);
        assert!(recovered.is_some(), "Resilient recovery must succeed on salvageable JSON");
        let data = recovered.unwrap();

        assert_eq!(data.name, "Mago Sobrevivente");
        assert_eq!(data.get_attribute_level("Força", 0), 4);
        assert_eq!(data.get_attribute_level("Destreza", 0), 3);
        assert_eq!(data.get_attribute_level("Vigor", 0), 5);
        assert_eq!(data.get_label("Conceito"), "Sobrevivente");
        assert_eq!(data.wonders.len(), 1);
        assert_eq!(data.wonders[0].name, "Amuleto");
        assert_eq!(data.wonders[0].points.level, 2);
    }
}
