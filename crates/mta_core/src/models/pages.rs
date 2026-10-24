use serde::{Deserialize, Serialize};
use super::traits::{is_empty_str, is_empty_vec};
use super::summary::default_photo_focus;

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CharacterVisualsData {
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub cabal_chart_url: String,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub character_sketch_url: String,
    #[serde(default = "default_photo_focus", skip_serializing_if = "is_default_focus")]
    pub photo_focus_y: i32,
    #[serde(default = "default_photo_focus", skip_serializing_if = "is_default_focus")]
    pub photo_focus_x: i32,
}

impl Default for CharacterVisualsData {
    fn default() -> Self {
        Self {
            cabal_chart_url: String::new(),
            character_sketch_url: String::new(),
            photo_focus_y: 50,
            photo_focus_x: 50,
        }
    }
}

pub fn is_default_focus(f: &i32) -> bool {
    *f == 50
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
