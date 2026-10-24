use serde::{Deserialize, Serialize};
use super::traits::{
    deserialize_flexible_attribute_value, deserialize_flexible_i32, is_empty_str, is_zero_i32,
    AttributeValue,
};

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
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub notes: String,
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
pub struct ChantryEntry {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub location: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub description: String,
}

pub fn is_all_empty_merits(v: &Vec<MeritItem>) -> bool {
    v.is_empty() || v.iter().all(|m| m.name.trim().is_empty() && m.cost == 0)
}

pub fn is_all_empty_flaws(v: &Vec<FlawItem>) -> bool {
    v.is_empty() || v.iter().all(|f| f.name.trim().is_empty() && f.bonus == 0)
}

pub fn is_all_empty_wonders(v: &Vec<WonderItem>) -> bool {
    v.is_empty()
        || v.iter().all(|w| {
            w.name.trim().is_empty()
                && w.description.trim().is_empty()
                && w.quintessence_current == 0
                && w.points.level == 0
                && w.arete.level == 0
        })
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
    let filled: Vec<&WonderItem> = wonders
        .iter()
        .filter(|w| {
            !w.name.trim().is_empty()
                || !w.description.trim().is_empty()
                || w.quintessence_current > 0
                || w.points.level > 0
                || w.arete.level > 0
        })
        .collect();
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
    let filled: Vec<&ChantryEntry> = chantry
        .iter()
        .filter(|c| !c.location.trim().is_empty() || !c.description.trim().is_empty())
        .collect();
    let mut seq = serializer.serialize_seq(Some(filled.len()))?;
    for c in filled {
        seq.serialize_element(c)?;
    }
    seq.end()
}
