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

/// WoD / Mage: The Ascension Health Track Normalization & Overflow Resolution
pub fn normalize_health_counts_for_total(mut agg: usize, mut lethal: usize, mut bashing: usize, total: usize) -> (usize, usize, usize) {
    if total == 0 {
        return (0, 0, 0);
    }
    // 1. Resolve bashing overflow: each excess point beyond total upgrades 1 existing bashing to lethal
    while agg + lethal + bashing > total && bashing > 0 {
        if bashing >= 2 {
            bashing -= 2;
            lethal += 1;
        } else {
            bashing -= 1;
            if lethal > 0 {
                lethal -= 1;
                agg += 1;
            } else {
                agg += 1;
            }
        }
    }

    // 2. Resolve lethal overflow: each excess point beyond total upgrades 1 existing lethal to aggravated
    while agg + lethal + bashing > total && lethal > 0 {
        if lethal >= 2 {
            lethal -= 2;
            agg += 1;
        } else {
            lethal -= 1;
            agg += 1;
        }
    }

    // 3. Cap aggravated at total max
    if agg >= total {
        return (total, 0, 0);
    }

    // 4. Ensure sum fits in total boxes
    let remaining = total - agg;
    if lethal > remaining {
        lethal = remaining;
        bashing = 0;
    } else {
        let remaining_bashing = remaining - lethal;
        if bashing > remaining_bashing {
            bashing = remaining_bashing;
        }
    }

    (agg, lethal, bashing)
}

pub fn normalize_health_counts(agg: usize, lethal: usize, bashing: usize) -> (usize, usize, usize) {
    normalize_health_counts_for_total(agg, lethal, bashing, 7)
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

pub fn is_empty_str(s: &str) -> bool {
    s.trim().is_empty()
}

pub fn is_zero_i32(n: &i32) -> bool {
    *n == 0
}

pub fn is_false_bool(b: &bool) -> bool {
    !*b
}

pub fn is_default_origins(v: &Vec<DotOrigin>) -> bool {
    v.is_empty() || v.iter().all(|&o| o == DotOrigin::Base)
}

pub fn is_empty_vec<T>(v: &Vec<T>) -> bool {
    v.is_empty()
}

pub fn is_empty_map<K, V>(m: &HashMap<K, V>) -> bool {
    m.is_empty()
}

pub fn is_all_empty_merits(v: &Vec<MeritItem>) -> bool {
    v.is_empty() || v.iter().all(|m| m.name.trim().is_empty() && m.cost == 0)
}

pub fn is_all_empty_flaws(v: &Vec<FlawItem>) -> bool {
    v.is_empty() || v.iter().all(|f| f.name.trim().is_empty() && f.bonus == 0)
}

pub fn is_all_empty_wonders(v: &Vec<WonderItem>) -> bool {
    v.is_empty() || v.iter().all(|w| w.name.trim().is_empty() && w.description.trim().is_empty() && w.quintessence_current == 0 && w.points.level == 0 && w.arete.level == 0)
}

pub fn is_all_empty_weapons(v: &Vec<WeaponItem>) -> bool {
    v.is_empty() || v.iter().all(|w| w.name.trim().is_empty())
}

pub fn is_all_empty_chantry(v: &Vec<ChantryEntry>) -> bool {
    v.is_empty() || v.iter().all(|c| c.location.trim().is_empty() && c.description.trim().is_empty())
}

pub fn serialize_compact_merits<S>(merits: &Vec<MeritItem>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let filled: Vec<&MeritItem> = merits.iter().filter(|m| !m.name.trim().is_empty() || m.cost > 0).collect();
    let mut seq = serializer.serialize_seq(Some(filled.len()))?;
    for m in filled {
        seq.serialize_element(m)?;
    }
    seq.end()
}

pub fn serialize_compact_flaws<S>(flaws: &Vec<FlawItem>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let filled: Vec<&FlawItem> = flaws.iter().filter(|f| !f.name.trim().is_empty() || f.bonus > 0).collect();
    let mut seq = serializer.serialize_seq(Some(filled.len()))?;
    for f in filled {
        seq.serialize_element(f)?;
    }
    seq.end()
}

pub fn serialize_compact_weapons<S>(weapons: &Vec<WeaponItem>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let filled: Vec<&WeaponItem> = weapons.iter().filter(|w| !w.name.trim().is_empty()).collect();
    let mut seq = serializer.serialize_seq(Some(filled.len()))?;
    for w in filled {
        seq.serialize_element(w)?;
    }
    seq.end()
}

pub fn serialize_compact_wonders<S>(wonders: &Vec<WonderItem>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let filled: Vec<&WonderItem> = wonders.iter().filter(|w| !w.name.trim().is_empty() || !w.description.trim().is_empty() || w.quintessence_current > 0 || w.points.level > 0 || w.arete.level > 0).collect();
    let mut seq = serializer.serialize_seq(Some(filled.len()))?;
    for w in filled {
        seq.serialize_element(w)?;
    }
    seq.end()
}

pub fn serialize_compact_chantry<S>(chantry: &Vec<ChantryEntry>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let filled: Vec<&ChantryEntry> = chantry.iter().filter(|c| !c.location.trim().is_empty() || !c.description.trim().is_empty()).collect();
    let mut seq = serializer.serialize_seq(Some(filled.len()))?;
    for c in filled {
        seq.serialize_element(c)?;
    }
    seq.end()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AttributeValue {
    #[serde(default, deserialize_with = "deserialize_flexible_i32")]
    pub level: i32,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub modifier: String,
    #[serde(default, skip_serializing_if = "is_default_origins")]
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
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub name: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub merit_type: String,
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub cost: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlawItem {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub name: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub flaw_type: String,
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub bonus: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WonderItem {
    #[serde(default = "default_wonder_id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub name: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub image_url: String,
    #[serde(default, deserialize_with = "deserialize_flexible_attribute_value")]
    pub points: AttributeValue,
    #[serde(default, deserialize_with = "deserialize_flexible_attribute_value")]
    pub arete: AttributeValue,
    #[serde(default = "default_wonder_quint_max", deserialize_with = "deserialize_flexible_i32")]
    pub quintessence_max: i32,
    #[serde(default, alias = "quintessence", deserialize_with = "deserialize_flexible_i32", skip_serializing_if = "is_zero_i32")]
    pub quintessence_current: i32,
    #[serde(default, skip_serializing_if = "is_empty_str")]
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
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub name: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub diff: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub damage: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub range: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub rate: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub clip: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub conceal: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ArmorItem {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub class_name: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub rating: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub penalty: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub description: String,
}

pub fn is_default_armor(a: &ArmorItem) -> bool {
    *a == ArmorItem::default()
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
    #[serde(default = "default_sheet_type")]
    pub sheet_type: String,
    #[serde(default)]
    pub is_public: bool,
    #[serde(default)]
    pub is_owner: bool,
    pub updated_at: String,
}

pub fn default_sheet_type() -> String { "mage".to_string() }
fn default_arete() -> i32 { 1 }
fn default_willpower() -> i32 { 5 }

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExpandedBackgroundsData {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub allies: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub contacts: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub fame: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub influence: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub library: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub node: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub resources: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub retainers: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub sanctum: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub other_title: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub other_text: String,
}

pub fn is_default_expanded_backgrounds(b: &ExpandedBackgroundsData) -> bool {
    *b == ExpandedBackgroundsData::default()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PossessionsData {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub gear_carried: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub equipment_owned: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub foci: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub familiar: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub grimoire: String,
}

pub fn is_default_possessions(p: &PossessionsData) -> bool {
    *p == PossessionsData::default()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ChantryEntry {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub location: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterHistoryData {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub history: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub goals_destiny: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub seekings: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub quiets: String,
}

pub fn is_default_history(h: &CharacterHistoryData) -> bool {
    *h == CharacterHistoryData::default()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterDescriptionData {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub age: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub apparent_age: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub date_of_birth: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub age_of_awakening: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub hair: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub eyes: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub race: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub nationality: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub height: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub weight: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub sex: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub physical_description: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub avatar_nature: String,
}

pub fn is_default_description(d: &CharacterDescriptionData) -> bool {
    *d == CharacterDescriptionData::default()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterVisualsData {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub cabal_chart_url: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub character_sketch_url: String,
}

pub fn is_default_visuals(v: &CharacterVisualsData) -> bool {
    *v == CharacterVisualsData::default()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RoteSphereRequirement {
    #[serde(default)]
    pub sphere: String,
    #[serde(default = "default_sphere_level")]
    pub level: i32,
}

fn default_sphere_level() -> i32 {
    1
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrimoireRoteItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub spheres: String,
    #[serde(default)]
    pub sphere_list: Vec<RoteSphereRequirement>,
    #[serde(default)]
    pub highest_sphere: i32,
    #[serde(default)]
    pub enhancing_ability: String,
    #[serde(default)]
    pub focus: String,
    #[serde(default)]
    pub practice: String,
    #[serde(default)]
    pub instrument: String,
    #[serde(default)]
    pub description: String,
}

impl GrimoireRoteItem {
    /// Retorna o nível da maior esfera usada (da lista estruturada, do campo highest_sphere ou do texto)
    pub fn get_highest_sphere_level(&self) -> i32 {
        if self.highest_sphere > 0 {
            return self.highest_sphere.clamp(1, 10);
        }
        if !self.sphere_list.is_empty() {
            let max = self.sphere_list.iter().map(|s| s.level).max().unwrap_or(1);
            return max.clamp(1, 10);
        }
        let mut max_lvl = 1;
        for word in self.spheres.split(|c: char| !c.is_numeric()) {
            if let Ok(num) = word.parse::<i32>() {
                if (1..=10).contains(&num) && num > max_lvl {
                    max_lvl = num;
                }
            }
        }
        max_lvl
    }

    /// Retorna a tupla de dificuldades (Coincidente: +2, Vulgar: +3, Vulgar com Testemunha: +4)
    pub fn calculate_difficulties(&self) -> (i32, i32, i32) {
        let max_sphere = self.get_highest_sphere_level();
        (max_sphere + 2, max_sphere + 3, max_sphere + 4)
    }

    /// Sincroniza a string de esferas com base na lista de esferas
    pub fn sync_spheres_string(&mut self) {
        if !self.sphere_list.is_empty() {
            self.spheres = self.sphere_list
                .iter()
                .map(|s| format!("{} {}", s.sphere, s.level))
                .collect::<Vec<_>>()
                .join(", ");
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterNotesData {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub session_notes: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub campaign_journal: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub attachment_image_url: String,
}

pub fn is_default_notes(n: &CharacterNotesData) -> bool {
    *n == CharacterNotesData::default()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuizQuestionEntry {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub title: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub prompt: String,
    #[serde(default)]
    pub answer: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub category: String, // "character" ou "player"
}

pub fn default_quiz_questions() -> Vec<QuizQuestionEntry> {
    vec![
        // === SEÇÃO 1: PERGUNTAS PARA O PERSONAGEM ===
        QuizQuestionEntry {
            id: "q_char_age".to_string(),
            title: "Qual É A Sua Idade?".to_string(),
            prompt: "Quantos anos têm o seu personagem? Por quanto tempo ele estudou mágika? A sua aparência reflete a sua idade? Que acontecimentos foram importantes para o seu personagem (se a crônica se passa em 1996 e o seu personagem tem 25 anos, acontecimentos como a queda do Muro de Berlim terão efeito direto; se tem 40 anos, a Guerra do Vietnã terá influência marcante)?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_different".to_string(),
            title: "Quando Você Percebeu Que Era... Diferente?".to_string(),
            prompt: "A maioria dos magos tem infâncias estranhas com eventos inexplicáveis e coincidências bizarras. Esses acontecimentos passaram despercebidos ou tiveram consequências tremendas? Uma infância feliz pode induzir no mago um sentimento de dever com os menos favorecidos, enquanto outro que passou sua infância marginalizado pode ter dificuldade de se relacionar com os outros.".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_skills".to_string(),
            title: "Como Você Desenvolveu Suas Habilidades?".to_string(),
            prompt: "Ninguém aparece do nada, completo com perícias e um lugar na sociedade. Quem era você? Onde você cresceu? Como você aprendeu aquilo que sabe? Estas perguntas dão mais profundidade e sugerem linhas de enredo que o Narrador possa tecer na crônica.".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_important_people".to_string(),
            title: "Quem É Importante Para Você?".to_string(),
            prompt: "Nenhum homem é uma ilha. Ele tem amigos superficiais ou um pequeno círculo bem unido? Como se relaciona com sua família? Alguma dessas pessoas sabe sobre suas habilidades de balançar os pilares do Céu? Como reagem a isso? O seu Despertar o afastou de todos que eram importantes? Quem é importante para ele agora?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_first_magick".to_string(),
            title: "Quando Você Encontrou A Mágika Pela Primeira Vez?".to_string(),
            prompt: "Os humanos têm uma grande capacidade de ignorar ou desprezar coisas que não se encaixam nos seus modelos de mundo. Que acontecimentos superaram essa habilidade? Como descobriu que havia mistérios que não podiam ser solucionados? Ficou com medo, surpreso, louco pelo poder ou teve um colapso? Ou simplesmente nunca perdeu a habilidade infantil de acreditar em tudo?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_mentor".to_string(),
            title: "Quem Era O Seu Mentor?".to_string(),
            prompt: "Magos da Tradição normalmente começam com algum tipo de mentor. Como conheceu essa pessoa? Você o procurou ou ele veio até você? Ele foi atencioso ou duro? Ele explicou tudo, ou simplesmente fez perguntas e observou? O mentor agiu como professor, pai, irmão mais velho ou força fundamental na personalidade?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_cabala".to_string(),
            title: "Como Você Conheceu Os Outros na Sua Cabala?".to_string(),
            prompt: "A cabala normalmente se refere aos personagens dos outros jogadores. Como você os conheceu e interagiu com eles? Preveniram alguma catástrofe na Teia, salvaram alguém de um ataque Nefandi ou foram reunidos por uma causa maior?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },
        QuizQuestionEntry {
            id: "q_char_mundane_life".to_string(),
            title: "Você Mantém Uma Vida Comum?".to_string(),
            prompt: "Você tem uma 'identidade secreta'? Você continua a interagir com os Adormecidos, escondendo os aspectos sobrenaturais da sua existência, ou você deixou sua vida antiga para trás?".to_string(),
            answer: String::new(),
            category: "character".to_string(),
        },

        // === SEÇÃO 2: DICAS & PERGUNTAS PARA OS JOGADORES SOBRE O CAMINHO ===
        QuizQuestionEntry {
            id: "q_player_what_to_do".to_string(),
            title: "O Que Você, o Jogador, Quer Fazer?".to_string(),
            prompt: "As motivações são muito importantes, tanto as suas quanto as do seu personagem. Que tipo de pessoa você quer representar (cientista louco, mago enigmático, socialite)? Que tipo de coisas você gostaria de realizar no jogo (alimentar famintos, lutar, ficar rico, vingar alguém)? Lembre-se de que Mago é sobre encontrar verdades maiores.".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_destiny_pursued".to_string(),
            title: "Que Destino Ele Persegue?".to_string(),
            prompt: "O que o místiko vê no seu destino? Morrer por um bem maior? Derrubar a Tecnocracia? Acabar com o mal, aperfeiçoar seu eu interior ou escrever os evangelhos do século XXI? Que visões guiam a busca do destino?".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_destiny_view".to_string(),
            title: "Como o Mago Vê o Seu Destino?".to_string(),
            prompt: "A maioria dos magos, especialmente os novos, não querem pensar sobre o fim do caminho. No entanto, todo mago tem alguma ideia do que o destino reservou para ele. Isso o assusta ou intriga? O que ele sente sobre isso, e o que fará para persegui-lo... ou evitá-lo?".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_avatar_nature".to_string(),
            title: "Como é o Avatar do Personagem?".to_string(),
            prompt: "O que está nos planos do Avatar (tornar-se uno com todos, devolver a mágika ao mundo, eliminar preconceitos, completar negócios inacabados de vidas passadas)? Como ele aparece (um amigo imaginário, um surto de inspiração, um anjo com as mãos sangrentas)?".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_avatar_relation".to_string(),
            title: "Como o Mago se Relaciona Com o seu Avatar?".to_string(),
            prompt: "O seu mago está em conflito com o seu eu mágiko? Como eles se relacionam? O Avatar atormenta o místiko com poder/conhecimento, senta num canto ou o arrasta através das Procuras até abrir seus olhos ou ficar louco? O mago quer ser um mago ou preferiria voltar à vida antiga? Lembre-se: harmonia perfeita é tediosa!".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
        QuizQuestionEntry {
            id: "q_player_conflicts".to_string(),
            title: "Que Conflitos Podem Surgir ao Longo do Caminho?".to_string(),
            prompt: "O Caminho da Ascensão verdadeira nunca foi suave. Que tipos de distrações podem tirar o mago do seu caminho (amor verdadeiro, desilusões, vingança, traição, insanidade, orgulho)? Descobrindo isso, você terá uma ideia de como seu personagem reagirá quando tais coisas acontecerem.".to_string(),
            answer: String::new(),
            category: "player".to_string(),
        },
    ]
}

pub fn deserialize_quiz_entries<'de, D>(deserializer: D) -> Result<Vec<QuizQuestionEntry>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct PartialQuizEntry {
        id: String,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        prompt: Option<String>,
        #[serde(default)]
        answer: String,
        #[serde(default)]
        category: Option<String>,
    }

    let parsed_entries = Vec::<PartialQuizEntry>::deserialize(deserializer)?;
    let mut default_entries = default_quiz_questions();

    for parsed in parsed_entries {
        if let Some(existing) = default_entries.iter_mut().find(|e| e.id == parsed.id) {
            existing.answer = parsed.answer;
            if let Some(t) = parsed.title { if !t.is_empty() { existing.title = t; } }
            if let Some(p) = parsed.prompt { if !p.is_empty() { existing.prompt = p; } }
            if let Some(c) = parsed.category { if !c.is_empty() { existing.category = c; } }
        } else {
            default_entries.push(QuizQuestionEntry {
                id: parsed.id,
                title: parsed.title.unwrap_or_default(),
                prompt: parsed.prompt.unwrap_or_default(),
                answer: parsed.answer,
                category: parsed.category.unwrap_or_else(|| "custom".to_string()),
            });
        }
    }

    Ok(default_entries)
}

pub fn serialize_compact_quiz_entries<S>(entries: &Vec<QuizQuestionEntry>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let answered: Vec<&QuizQuestionEntry> = entries.iter().filter(|e| !e.answer.trim().is_empty()).collect();
    let mut seq = serializer.serialize_seq(Some(answered.len()))?;
    for entry in answered {
        seq.serialize_element(entry)?;
    }
    seq.end()
}

pub fn is_default_quiz(q: &CharacterQuizData) -> bool {
    q.entries.iter().all(|e| e.answer.trim().is_empty())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CharacterQuizData {
    #[serde(default = "default_quiz_questions", deserialize_with = "deserialize_quiz_entries", serialize_with = "serialize_compact_quiz_entries")]
    pub entries: Vec<QuizQuestionEntry>,
}

impl Default for CharacterQuizData {
    fn default() -> Self {
        Self {
            entries: default_quiz_questions(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrimoireData {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub paradigm: String,
    #[serde(default, skip_serializing_if = "is_empty_vec")]
    pub practices: Vec<String>,
    #[serde(default, skip_serializing_if = "is_empty_vec")]
    pub instruments: Vec<String>,
    #[serde(default, skip_serializing_if = "is_empty_vec")]
    pub rotes: Vec<GrimoireRoteItem>,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub general_notes: String,
}

pub fn is_default_grimoire(g: &GrimoireData) -> bool {
    *g == GrimoireData::default()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterData {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_sheet_type")]
    pub sheet_type: String,
    #[serde(default, skip_serializing_if = "is_false_bool")]
    pub is_public: bool,
    #[serde(default, skip_serializing_if = "is_empty_map")]
    pub attributes: HashMap<String, AttributeValue>,
    #[serde(default, skip_serializing_if = "is_empty_map")]
    pub labels: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "is_empty_map")]
    pub custom_lists: HashMap<String, Vec<String>>,
    
    // Page 2: Magic & Combat
    #[serde(default, skip_serializing_if = "is_all_empty_merits", serialize_with = "serialize_compact_merits")]
    pub merits: Vec<MeritItem>,
    #[serde(default, skip_serializing_if = "is_all_empty_flaws", serialize_with = "serialize_compact_flaws")]
    pub flaws: Vec<FlawItem>,
    #[serde(default, skip_serializing_if = "is_all_empty_wonders", serialize_with = "serialize_compact_wonders")]
    pub wonders: Vec<WonderItem>,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub rotes: String,
    #[serde(default, skip_serializing_if = "is_all_empty_weapons", serialize_with = "serialize_compact_weapons")]
    pub weapons: Vec<WeaponItem>,
    #[serde(default, skip_serializing_if = "is_default_armor")]
    pub armor: ArmorItem,

    // Page 3: Expanded Backgrounds, Possessions & Chantry
    #[serde(default, skip_serializing_if = "is_default_expanded_backgrounds")]
    pub expanded_backgrounds: ExpandedBackgroundsData,
    #[serde(default, skip_serializing_if = "is_default_possessions")]
    pub possessions: PossessionsData,
    #[serde(default, skip_serializing_if = "is_all_empty_chantry", serialize_with = "serialize_compact_chantry")]
    pub chantry: Vec<ChantryEntry>,

    // Page 4: History, Description & Visuals
    #[serde(default, skip_serializing_if = "is_default_history")]
    pub history_data: CharacterHistoryData,
    #[serde(default, skip_serializing_if = "is_default_description")]
    pub description_data: CharacterDescriptionData,
    #[serde(default, skip_serializing_if = "is_default_visuals")]
    pub visuals: CharacterVisualsData,

    // Page 5: Grimoire
    #[serde(default, skip_serializing_if = "is_default_grimoire")]
    pub grimoire: GrimoireData,
    #[serde(default, skip_serializing_if = "is_default_notes")]
    pub notes_data: CharacterNotesData,

    // Annex: Character Creation Questionnaire / Dossier
    #[serde(default, skip_serializing_if = "is_default_quiz")]
    pub quiz_data: CharacterQuizData,
}

impl CharacterData {
    pub fn new(id: String, name: String) -> Self {
        let mut sheet = Self {
            id,
            name: if name.trim().is_empty() { "Novo Mago".to_string() } else { name },
            sheet_type: "mage".to_string(),
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
            grimoire: GrimoireData::default(),
            notes_data: CharacterNotesData::default(),
            quiz_data: CharacterQuizData::default(),
        };
        sheet.sanitize();
        sheet
    }

    pub fn new_gods_and_monsters(id: String, name: String) -> Self {
        let mut sheet = Self {
            id,
            name: if name.trim().is_empty() { "New Monster / Familiar".to_string() } else { name },
            sheet_type: "gods_and_monsters".to_string(),
            is_public: false,
            attributes: HashMap::new(),
            labels: HashMap::new(),
            custom_lists: HashMap::new(),
            merits: vec![MeritItem::default(); 7],
            flaws: vec![FlawItem::default(); 7],
            wonders: Vec::new(),
            rotes: String::new(),
            weapons: vec![WeaponItem::default(); 6],
            armor: ArmorItem::default(),
            expanded_backgrounds: ExpandedBackgroundsData::default(),
            possessions: PossessionsData::default(),
            chantry: Vec::new(),
            history_data: CharacterHistoryData::default(),
            description_data: CharacterDescriptionData::default(),
            visuals: CharacterVisualsData::default(),
            grimoire: GrimoireData::default(),
            notes_data: CharacterNotesData::default(),
            quiz_data: CharacterQuizData::default(),
        };
        sheet.labels.insert("Type".to_string(), "Familiar".to_string());
        sheet.labels.insert("Concept".to_string(), "".to_string());
        sheet.labels.insert("essence_pool".to_string(), "0".repeat(50));
        sheet.labels.insert("gnosis_temp".to_string(), "0".repeat(10));
        sheet.labels.insert("paradox_pool".to_string(), "0".repeat(20));
        sheet.sanitize();
        sheet
    }

    pub fn is_gods_and_monsters(&self) -> bool {
        self.sheet_type == "gods_and_monsters"
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

    pub const TOTAL_HEALTH_BOXES: usize = 7;

    pub fn get_extra_bruised(&self) -> usize {
        self.labels.get("extra_bruised_levels").and_then(|s| s.parse().ok()).unwrap_or(0)
    }

    pub fn set_extra_bruised(&mut self, val: usize) {
        self.labels.insert("extra_bruised_levels".to_string(), val.to_string());
    }

    pub fn add_extra_bruised(&mut self) {
        let cur = self.get_extra_bruised();
        self.set_extra_bruised(cur + 1);
    }

    pub fn remove_extra_bruised(&mut self) {
        let cur = self.get_extra_bruised();
        if cur > 0 {
            self.set_extra_bruised(cur - 1);
            let (agg, lethal, bashing) = self.get_health_counts();
            self.set_health_counts(agg, lethal, bashing);
        }
    }

    pub fn get_total_health_boxes(&self) -> usize {
        7 + self.get_extra_bruised()
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

    pub fn get_health_counts(&self) -> (usize, usize, usize) {
        let mut agg = 0;
        let mut lethal = 0;
        let mut bashing = 0;
        let total = self.get_total_health_boxes();
        for i in 0..total {
            match self.get_health(i) {
                DamageType::Aggravated => agg += 1,
                DamageType::Lethal => lethal += 1,
                DamageType::Bashing => bashing += 1,
                DamageType::None => {}
            }
        }
        (agg, lethal, bashing)
    }

    pub fn set_health_counts(&mut self, agg: usize, lethal: usize, bashing: usize) {
        let total = self.get_total_health_boxes();
        let (agg, lethal, bashing) = normalize_health_counts_for_total(agg, lethal, bashing, total);
        for i in 0..total {
            let dmg = if i < agg {
                DamageType::Aggravated
            } else if i < agg + lethal {
                DamageType::Lethal
            } else if i < agg + lethal + bashing {
                DamageType::Bashing
            } else {
                DamageType::None
            };
            self.set_health(i, dmg);
        }
    }

    pub fn click_health_box(&mut self, index: usize) {
        let total = self.get_total_health_boxes();
        if index >= total {
            return;
        }

        let (mut agg, mut lethal, mut bashing) = self.get_health_counts();
        let current_dmg = self.get_health(index);

        match current_dmg {
            DamageType::None => {
                let total_current = agg + lethal + bashing;
                let target_total = index + 1;
                if target_total > total_current {
                    bashing += target_total - total_current;
                }
            }
            DamageType::Bashing => {
                let new_lethal = (index + 1).saturating_sub(agg).max(lethal + 1);
                lethal = new_lethal;
            }
            DamageType::Lethal => {
                let new_agg = (index + 1).max(agg + 1);
                agg = new_agg;
            }
            DamageType::Aggravated => {
                agg = agg.saturating_sub(1);
            }
        }

        self.set_health_counts(agg, lethal, bashing);
    }

    pub fn heal_health_box(&mut self, index: usize) {
        let total = self.get_total_health_boxes();
        if index >= total {
            return;
        }

        let (mut agg, mut lethal, mut bashing) = self.get_health_counts();
        let current_dmg = self.get_health(index);

        match current_dmg {
            DamageType::Bashing => bashing = bashing.saturating_sub(1),
            DamageType::Lethal => lethal = lethal.saturating_sub(1),
            DamageType::Aggravated => agg = agg.saturating_sub(1),
            DamageType::None => {}
        }

        self.set_health_counts(agg, lethal, bashing);
    }

    pub fn clear_health(&mut self) {
        self.set_health_counts(0, 0, 0);
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

    // Gods & Monsters: Gnosis (10 dots + 10 temp boxes)
    pub fn get_gnosis(&self) -> (i32, String) {
        let dots = self.get_attribute_level("Gnosis", 0);
        let temp_raw = self.labels.get("gnosis_temp").cloned().unwrap_or_else(|| "0".repeat(10));
        let normalized = if temp_raw.len() == 10 { temp_raw } else { "0".repeat(10) };
        (dots, normalized)
    }

    pub fn set_gnosis_dots(&mut self, dots: i32) {
        self.set_attribute("Gnosis", Some(dots.clamp(0, 10)), None);
    }

    pub fn cycle_gnosis_box(&mut self, index: usize) {
        if index >= 10 {
            return;
        }
        let raw = self.labels.entry("gnosis_temp".to_string()).or_insert_with(|| "0".repeat(10));
        let mut chars: Vec<char> = raw.chars().collect();
        while chars.len() < 10 {
            chars.push('0');
        }
        chars[index] = if chars[index] == '1' { '0' } else { '1' };
        *raw = chars.into_iter().collect();
    }

    // Gods & Monsters: Essence Pool (50 boxes / 5 rows of 10)
    pub fn get_essence_pool(&self) -> (i32, String) {
        let raw = self.labels.get("essence_pool").cloned().unwrap_or_else(|| "0".repeat(50));
        let normalized = if raw.len() == 50 { raw } else { "0".repeat(50) };
        let spent = normalized.chars().filter(|&c| c == '1').count() as i32;
        (spent, normalized)
    }

    pub fn set_essence_spent(&mut self, amount: usize) {
        let count = amount.min(50);
        let mut chars = vec!['0'; 50];
        for i in 0..count {
            chars[i] = '1';
        }
        let pool_str: String = chars.into_iter().collect();
        self.labels.insert("essence_pool".to_string(), pool_str);
    }

    pub fn click_essence_box(&mut self, index: usize) {
        if index >= 50 {
            return;
        }
        let (current_spent, _) = self.get_essence_pool();
        let target = (index + 1) as i32;
        if current_spent == target {
            self.set_essence_spent(index);
        } else {
            self.set_essence_spent(index + 1);
        }
    }

    pub fn clear_essence(&mut self) {
        self.set_essence_spent(0);
    }

    pub fn cycle_essence_box(&mut self, index: usize) {
        self.click_essence_box(index);
    }

    // Gods & Monsters: Paradox Pool (20 boxes / 2 rows of 10)
    pub fn get_paradox_pool(&self) -> (i32, String) {
        let raw = self.labels.get("paradox_pool").cloned().unwrap_or_else(|| "0".repeat(20));
        let normalized = if raw.len() == 20 { raw } else { "0".repeat(20) };
        let active = normalized.chars().filter(|&c| c == '1').count() as i32;
        (active, normalized)
    }

    pub fn cycle_paradox_box(&mut self, index: usize) {
        if index >= 20 {
            return;
        }
        let raw = self.labels.entry("paradox_pool".to_string()).or_insert_with(|| "0".repeat(20));
        let mut chars: Vec<char> = raw.chars().collect();
        while chars.len() < 20 {
            chars.push('0');
        }
        chars[index] = if chars[index] == '1' { '0' } else { '1' };
        *raw = chars.into_iter().collect();
    }
}
