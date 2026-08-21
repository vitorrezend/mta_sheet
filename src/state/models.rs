use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub const CAT_OTHER_TRAITS: &str = "other_traits";

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

        while self.dot_origins.len() < old_level {
            self.dot_origins.push(DotOrigin::Base);
        }

        if new_len > old_level {
            while self.dot_origins.len() < new_len {
                self.dot_origins.push(default_origin);
            }
        } else {
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
    #[serde(default)]
    pub image_url: String,
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
            image_url: String::new(),
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
pub struct CharacterSummary {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub tradition: String,
    #[serde(default)]
    pub essence: String,
    #[serde(default = "default_arete")]
    pub arete: i32,
    #[serde(default = "default_willpower")]
    pub willpower: i32,
    #[serde(default)]
    pub photo_url: String,
    #[serde(default, alias = "active_spheres")]
    pub spheres: Vec<(String, i32)>,
    #[serde(default)]
    pub is_public: bool,
    #[serde(default)]
    pub is_owner: bool,
    pub updated_at: String,
}

fn default_arete() -> i32 { 1 }
fn default_willpower() -> i32 { 5 }

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExpandedBackgroundsData {
    #[serde(default)]
    pub allies: String,
    #[serde(default)]
    pub contacts: String,
    #[serde(default)]
    pub fame: String,
    #[serde(default)]
    pub influence: String,
    #[serde(default)]
    pub library: String,
    #[serde(default)]
    pub node: String,
    #[serde(default)]
    pub resources: String,
    #[serde(default)]
    pub retainers: String,
    #[serde(default)]
    pub sanctum: String,
    #[serde(default)]
    pub other_title: String,
    #[serde(default)]
    pub other_text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PossessionsData {
    #[serde(default)]
    pub gear_carried: String,
    #[serde(default)]
    pub equipment_owned: String,
    #[serde(default)]
    pub foci: String,
    #[serde(default)]
    pub familiar: String,
    #[serde(default)]
    pub grimoire: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ChantryEntry {
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterHistoryData {
    #[serde(default)]
    pub history: String,
    #[serde(default)]
    pub goals_destiny: String,
    #[serde(default)]
    pub seekings: String,
    #[serde(default)]
    pub quiets: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterDescriptionData {
    #[serde(default)]
    pub age: String,
    #[serde(default)]
    pub apparent_age: String,
    #[serde(default)]
    pub date_of_birth: String,
    #[serde(default)]
    pub age_of_awakening: String,
    #[serde(default)]
    pub hair: String,
    #[serde(default)]
    pub eyes: String,
    #[serde(default)]
    pub race: String,
    #[serde(default)]
    pub nationality: String,
    #[serde(default)]
    pub height: String,
    #[serde(default)]
    pub weight: String,
    #[serde(default)]
    pub sex: String,
    #[serde(default)]
    pub physical_description: String,
    #[serde(default)]
    pub avatar_nature: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterVisualsData {
    #[serde(default)]
    pub cabal_chart_url: String,
    #[serde(default)]
    pub character_sketch_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterData {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub is_public: bool,
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

    // Page 3: Expanded Backgrounds, Possessions & Chantry
    #[serde(default)]
    pub expanded_backgrounds: ExpandedBackgroundsData,
    #[serde(default)]
    pub possessions: PossessionsData,
    #[serde(default)]
    pub chantry: Vec<ChantryEntry>,

    // Page 4: History, Description & Visuals
    #[serde(default)]
    pub history_data: CharacterHistoryData,
    #[serde(default)]
    pub description_data: CharacterDescriptionData,
    #[serde(default)]
    pub visuals: CharacterVisualsData,
}

impl CharacterData {
    pub fn new(id: String, name: String) -> Self {
        let mut sheet = Self {
            id,
            name: if name.trim().is_empty() { "Novo Mago".to_string() } else { name },
            is_public: false,
            attributes: HashMap::new(),
            labels: HashMap::new(),
            custom_lists: HashMap::new(),
            merits: vec![MeritItem::default(); 7],
            flaws: vec![FlawItem::default(); 7],
            wonders: vec![WonderItem::default(); 3],
            rotes: String::new(),
            weapons: vec![WeaponItem::default(); 4],
            armor: ArmorItem::default(),
            expanded_backgrounds: ExpandedBackgroundsData::default(),
            possessions: PossessionsData::default(),
            chantry: vec![ChantryEntry::default(); 3],
            history_data: CharacterHistoryData::default(),
            description_data: CharacterDescriptionData::default(),
            visuals: CharacterVisualsData::default(),
        };
        sheet.sanitize();
        sheet
    }

    pub fn get_attribute_level(&self, name: &str, default_min: i32) -> i32 {
        self.attributes
            .get(name)
            .map(|a| a.level)
            .unwrap_or(default_min)
            .max(default_min)
    }

    pub fn get_attribute_modifier(&self, name: &str) -> String {
        self.attributes
            .get(name)
            .map(|a| a.modifier.clone())
            .unwrap_or_default()
    }

    pub fn set_attribute(&mut self, name: &str, level: Option<i32>, modifier: Option<String>) {
        let entry = self.attributes.entry(name.to_string()).or_default();
        if let Some(l) = level {
            entry.set_level_with_origin(l, DotOrigin::Base);
        }
        if let Some(m) = modifier {
            entry.modifier = m;
        }
    }

    pub fn set_attribute_with_origin(&mut self, name: &str, level: Option<i32>, modifier: Option<String>, origin: DotOrigin) {
        let entry = self.attributes.entry(name.to_string()).or_default();
        if let Some(l) = level {
            entry.set_level_with_origin(l, origin);
        }
        if let Some(m) = modifier {
            entry.modifier = m;
        }
    }

    pub fn set_attribute_dot_origin(&mut self, name: &str, dot_index: usize, origin: DotOrigin) {
        let entry = self.attributes.entry(name.to_string()).or_default();
        entry.set_dot_origin(dot_index, origin);
    }

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

    pub fn get_affinity_sphere(&self) -> Option<String> {
        let aff = self.labels.get(keys::KEY_AFFINITY_SPHERE).cloned().unwrap_or_default();
        if aff.trim().is_empty() {
            None
        } else {
            Some(aff)
        }
    }

    pub fn set_affinity_sphere(&mut self, sphere: Option<String>) {
        if let Some(s) = sphere {
            self.labels.insert(keys::KEY_AFFINITY_SPHERE.to_string(), s);
        } else {
            self.labels.remove(keys::KEY_AFFINITY_SPHERE);
        }
    }

    pub fn get_profile_photo(&self) -> String {
        self.labels.get(keys::KEY_PROFILE_PHOTO).cloned().unwrap_or_default()
    }

    pub fn set_profile_photo(&mut self, photo: String) {
        if photo.trim().is_empty() {
            self.labels.remove(keys::KEY_PROFILE_PHOTO);
        } else {
            self.labels.insert(keys::KEY_PROFILE_PHOTO.to_string(), photo);
        }
    }

    pub fn get_history(&self) -> String {
        self.labels.get(keys::KEY_HISTORY).cloned().unwrap_or_default()
    }

    pub fn set_history(&mut self, history: String) {
        if history.is_empty() {
            self.labels.remove(keys::KEY_HISTORY);
        } else {
            self.labels.insert(keys::KEY_HISTORY.to_string(), history);
        }
    }

    pub fn get_notes(&self) -> String {
        self.labels.get(keys::KEY_NOTES).cloned().unwrap_or_default()
    }

    pub fn set_notes(&mut self, notes: String) {
        if notes.is_empty() {
            self.labels.remove(keys::KEY_NOTES);
        } else {
            self.labels.insert(keys::KEY_NOTES.to_string(), notes);
        }
    }

    pub fn get_label(&self, key: &str) -> String {
        self.labels.get(key).cloned().unwrap_or_default()
    }

    pub fn set_label(&mut self, key: &str, val: String) {
        self.labels.insert(key.to_string(), val);
    }

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

    pub fn set_willpower_current(&mut self, current: i32) {
        let (total, _) = self.get_willpower();
        let val = current.clamp(0, total);
        self.attributes
            .entry(keys::KEY_WILLPOWER_CURRENT.to_string())
            .or_default()
            .level = val;
    }

    pub fn get_arete(&self) -> i32 {
        self.attributes
            .get(keys::KEY_ARETE)
            .map(|a| a.level)
            .unwrap_or(1)
            .clamp(1, 10)
    }

    pub fn set_arete(&mut self, val: i32) {
        self.attributes
            .entry(keys::KEY_ARETE.to_string())
            .or_default()
            .level = val.clamp(1, 10);
    }

    pub fn get_health(&self, index: usize) -> DamageType {
        let key = format!("{}{}", keys::HEALTH_KEY_PREFIX, index);
        let val = self.labels.get(&key).map(|s| s.as_str()).unwrap_or("none");
        DamageType::from_key(val)
    }

    pub fn set_health(&mut self, index: usize, dmg: DamageType) {
        let key = format!("{}{}", keys::HEALTH_KEY_PREFIX, index);
        self.labels.insert(key, dmg.to_key().to_string());
    }

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

    pub fn cycle_quintessence_paradox_box(&mut self, index: usize) {
        if index >= 20 {
            return;
        }
        let raw = self
            .labels
            .entry(keys::KEY_QUINTESSENCE_PARADOX.to_string())
            .or_insert_with(|| "0".repeat(20));
        let mut chars: Vec<char> = raw.chars().collect();
        while chars.len() < 20 {
            chars.push('0');
        }
        let current = chars[index];
        chars[index] = match current {
            '0' => '1',
            '1' => '2',
            _ => '0',
        };
        *raw = chars.into_iter().collect();
    }
}
