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
    Base,        // Criação de Ficha (Preto clássico)
    Bonus,       // Pontos de Bônus / Freebies (Roxo Ametista)
    Experience,  // Experiência / XP (Verde Esmeralda)
    Temporary,   // Feitiço / Wonder / Buff (Dourado Solar)
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AttributeValue {
    pub level: i32,
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
pub struct CharacterData {
    pub id: String,
    pub name: String,
    pub attributes: HashMap<String, AttributeValue>,
    pub labels: HashMap<String, String>,
    pub custom_lists: HashMap<String, Vec<String>>,
}

impl CharacterData {
    pub fn new(id: String, name: String) -> Self {
        let mut sheet = Self {
            id,
            name: if name.trim().is_empty() { "Novo Mago".to_string() } else { name },
            attributes: HashMap::new(),
            labels: HashMap::new(),
            custom_lists: HashMap::new(),
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
        let mut traits_to_process: Vec<(String, String, String, bool, bool, bool, bool)> = Vec::new();

        // 1. Atributos
        for &attr_name in &STANDARD_ATTRIBUTES {
            visited_keys.insert(attr_name.to_string());
            traits_to_process.push((attr_name.to_string(), attr_name.to_string(), "Atributo".to_string(), false, false, false, false));
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
                    traits_to_process.push((id.clone(), label, cat_label.to_string(), false, false, false, false));
                }
            }
        }

        // 3. Esferas
        for &sphere_name in &STANDARD_SPHERES {
            visited_keys.insert(sphere_name.to_string());
            traits_to_process.push((sphere_name.to_string(), sphere_name.to_string(), "Esfera".to_string(), true, false, false, false));
        }

        // 4. Arete
        let arete_val = self.get_attribute_level(keys::KEY_ARETE, 1);
        visited_keys.insert(keys::KEY_ARETE.to_string());
        traits_to_process.push((keys::KEY_ARETE.to_string(), "Arete".to_string(), "Vantagem".to_string(), false, true, false, false));

        // 5. Força de Vontade
        visited_keys.insert(keys::KEY_WILLPOWER_TOTAL.to_string());
        traits_to_process.push((keys::KEY_WILLPOWER_TOTAL.to_string(), "Força de Vontade".to_string(), "Vantagem".to_string(), false, false, true, false));

        // 6. Antecedentes
        if let Some(list) = self.custom_lists.get(keys::CAT_ANTECEDENTES) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Antecedente".to_string(), false, false, false, true));
            }
        }

        // 7. Ressonância
        if let Some(list) = self.custom_lists.get(keys::CAT_RESONANCE) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Ressonância".to_string(), false, false, false, true));
            }
        }

        // 8. Quaisquer outros atributos personalizados
        for (id, attr) in &self.attributes {
            if !visited_keys.contains(id) && attr.level > 0 {
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Outro".to_string(), false, false, false, false));
            }
        }

        for (id, display_name, category, is_sphere, is_arete, is_willpower, is_background) in traits_to_process {
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

                if trait_bonus_cost > 0 || trait_xp_cost > 0 {
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
        log::error!("Database pool not found in request context");
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let rows = sqlx::query("SELECT id, name, updated_at FROM character_sheets ORDER BY updated_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            log::error!("Failed to fetch sheets from DB: {:?}", e);
            ServerFnError::new(format!("Falha ao consultar fichas: {}", e))
        })?;

    let summaries = rows.into_iter().map(|row| CharacterSummary {
        id: row.get("id"),
        name: row.get("name"),
        updated_at: row.get("updated_at"),
    }).collect();

    Ok(summaries)
}

#[server(endpoint = "get_sheet")]
pub async fn get_sheet(id: String) -> Result<CharacterData, ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não fornecido"));
    }

    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        log::error!("Database pool not found in request context");
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let row = sqlx::query("SELECT data FROM character_sheets WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            log::error!("Error querying sheet {}: {:?}", id, e);
            ServerFnError::new(format!("Erro ao buscar ficha no banco: {}", e))
        })?
        .ok_or_else(|| {
            log::warn!("Sheet with id {} not found", id);
            ServerFnError::new(format!("Ficha com ID '{}' não encontrada", id))
        })?;

    let data_json: String = row.get("data");
    let mut data: CharacterData = serde_json::from_str(&data_json).map_err(|e: serde_json::Error| {
        log::error!("Corrupted JSON in DB for sheet {}: {:?}", id, e);
        ServerFnError::new(format!("Dados da ficha corrompidos: {}", e))
    })?;

    data.sanitize();
    Ok(data)
}

#[server(endpoint = "create_sheet")]
pub async fn create_sheet(name: String) -> Result<String, ServerFnError> {
    let clean_name = name.trim().to_string();
    let final_name = if clean_name.is_empty() { "Novo Mago".to_string() } else { clean_name };

    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        log::error!("Database pool not found in request context");
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let id = Uuid::new_v4().to_string();
    let initial_data = CharacterData::new(id.clone(), final_name.clone());

    let data_json = serde_json::to_string(&initial_data).map_err(|e: serde_json::Error| {
        log::error!("Serialization error creating sheet: {:?}", e);
        ServerFnError::new(format!("Falha ao serializar dados iniciais: {}", e))
    })?;

    sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&final_name)
        .bind(data_json)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            log::error!("Failed to insert new sheet {}: {:?}", id, e);
            ServerFnError::new(format!("Falha ao salvar nova ficha no banco: {}", e))
        })?;

    log::info!("Created new character sheet: id={}, name={}", id, final_name);
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
        log::error!("Database pool not found in request context");
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let data_json = serde_json::to_string(&data).map_err(|e: serde_json::Error| {
        log::error!("Serialization error updating sheet {}: {:?}", id, e);
        ServerFnError::new(format!("Falha ao serializar dados da ficha: {}", e))
    })?;

    let result = sqlx::query("UPDATE character_sheets SET name = ?, data = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(&data.name)
        .bind(data_json)
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            log::error!("Failed to update sheet {}: {:?}", id, e);
            ServerFnError::new(format!("Falha ao atualizar ficha no banco: {}", e))
        })?;

    if result.rows_affected() == 0 {
        log::warn!("UpdateSheet: No rows updated for id {}", id);
        return Err(ServerFnError::new("Ficha não encontrada para atualização"));
    }

    Ok(())
}

#[server(endpoint = "delete_sheet")]
pub async fn delete_sheet(id: String) -> Result<(), ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não fornecido"));
    }

    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        log::error!("Database pool not found in request context");
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    sqlx::query("DELETE FROM character_sheets WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            log::error!("Failed to delete sheet {}: {:?}", id, e);
            ServerFnError::new(format!("Falha ao excluir ficha do banco: {}", e))
        })?;

    log::info!("Deleted character sheet id={}", id);
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
}
