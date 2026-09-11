use serde::{Deserialize, Serialize};
use super::keys::STANDARD_SPHERES;

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
pub struct CreationCategoryBreakdown {
    pub name: String,
    pub spent: usize,
    pub budget: usize,
    pub is_exceeded: bool,
    pub details: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CreationPointsSummary {
    // Atributos (Físicos, Sociais, Mentais: 7 / 5 / 3)
    pub attr_physical: usize,
    pub attr_social: usize,
    pub attr_mental: usize,
    pub attr_total_spent: usize,
    pub attr_budget_total: usize, // 15
    pub attr_spread_valid: bool,  // se distribuição cabe em {7, 5, 3}
    pub attr_exceeded: bool,

    // Habilidades (Talentos, Perícias, Conhecimentos: 13 / 9 / 5, max 3 por trait)
    pub ab_talents: usize,
    pub ab_skills: usize,
    pub ab_knowledges: usize,
    pub ab_total_spent: usize,
    pub ab_budget_total: usize, // 27
    pub ab_spread_valid: bool,  // se distribuição cabe em {13, 9, 5}
    pub ab_exceeded: bool,
    pub ab_cap_violations: Vec<String>, // Nomes das habilidades com >3 bolinhas base

    // Esferas (6 pontos)
    pub spheres_spent: usize,
    pub spheres_budget: usize, // 6
    pub spheres_exceeded: bool,

    // Arete (1 grátis)
    pub arete_base: usize,
    pub arete_exceeded: bool, // arete_base > 1

    // Antecedentes (7 pontos)
    pub backgrounds_spent: usize,
    pub backgrounds_budget: usize, // 7
    pub backgrounds_exceeded: bool,

    // Força de Vontade (5 grátis)
    pub willpower_base: usize,
    pub willpower_exceeded: bool, // willpower_base > 5

    // Ressonância (1 ponto)
    pub resonance_spent: usize,
    pub resonance_budget: usize, // 1
    pub resonance_exceeded: bool,

    // Alertas e consolidação
    pub has_any_overflow: bool,
    pub warnings: Vec<String>,
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
    pub creation_points: CreationPointsSummary,
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
    #[serde(default = "default_photo_focus")]
    pub photo_focus_y: i32,
    #[serde(default = "default_photo_focus")]
    pub photo_focus_x: i32,
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

pub fn default_sheet_type() -> String {
    "mage".to_string()
}
fn default_arete() -> i32 {
    1
}
fn default_willpower() -> i32 {
    5
}
pub fn default_photo_focus() -> i32 {
    50
}

impl CharacterSummary {
    pub fn fallback(
        id: String,
        name: String,
        updated_at: String,
        is_public: bool,
        is_owner: bool,
    ) -> Self {
        let spheres = STANDARD_SPHERES.iter().map(|s| (s.to_string(), 0)).collect();
        Self {
            id,
            name,
            tradition: String::new(),
            essence: String::new(),
            arete: 1,
            willpower: 5,
            photo_url: String::new(),
            photo_focus_y: 50,
            photo_focus_x: 50,
            spheres,
            sheet_type: "mage".to_string(),
            is_public,
            is_owner,
            updated_at,
        }
    }
}
