use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::keys::{self, STANDARD_SPHERES};
use super::traits::{
    is_empty_map, is_empty_str, is_false_bool, normalize_health_counts_for_total,
    AttributeValue, DamageType, DotOrigin,
};
use super::items::{
    is_all_empty_chantry, is_all_empty_flaws, is_all_empty_merits, is_all_empty_weapons,
    is_all_empty_wonders, is_default_armor, serialize_compact_chantry, serialize_compact_flaws,
    serialize_compact_merits, serialize_compact_weapons, serialize_compact_wonders, ArmorItem,
    ChantryEntry, FlawItem, MeritItem, WeaponItem, WonderItem,
};
use super::pages::{
    is_default_description, is_default_expanded_backgrounds, is_default_grimoire, is_default_history,
    is_default_notes, is_default_possessions, is_default_visuals, CharacterDescriptionData,
    CharacterHistoryData, CharacterNotesData, CharacterVisualsData, ExpandedBackgroundsData,
    GrimoireData, PossessionsData,
};
use super::dossier::{is_default_quiz, CharacterQuizData};
use super::summary::{default_sheet_type, CharacterSummary};

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

/// Identifica se uma string de nome representa um placeholder padrão/recuperado
pub fn is_placeholder_name(name: &str) -> bool {
    let t = name.trim();
    t.is_empty()
        || t == "Novo Mago"
        || t == "New Monster / Familiar"
        || t == "Sem Nome"
        || t == "Personagem Recuperado"
        || t == "Ficha Importada"
}

impl CharacterData {
    pub fn new(id: String, name: String) -> Self {
        let clean_name = name.trim();
        let initial_name = if clean_name.is_empty() { "Novo Mago" } else { clean_name };
        let mut sheet = Self {
            id,
            name: initial_name.to_string(),
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
        sheet.labels.insert(keys::HEADER_NOME.to_string(), initial_name.to_string());
        sheet.sanitize();
        sheet
    }

    pub fn new_gods_and_monsters(id: String, name: String) -> Self {
        let clean_name = name.trim();
        let initial_name = if clean_name.is_empty() { "New Monster / Familiar" } else { clean_name };
        let mut sheet = Self {
            id,
            name: initial_name.to_string(),
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
        sheet.labels.insert(keys::HEADER_NOME.to_string(), initial_name.to_string());
        sheet.labels.insert("Type".to_string(), "Familiar".to_string());
        sheet.labels.insert("Concept".to_string(), "".to_string());
        sheet.labels.insert("essence_pool".to_string(), "0".repeat(50));
        sheet.labels.insert("gnosis_temp".to_string(), "0".repeat(10));
        sheet.labels.insert("paradox_pool".to_string(), "0".repeat(20));
        sheet.sanitize();
        sheet
    }

    /// Retorna o nome de exibição do personagem, priorizando o campo "Nome" / "Name" dos rótulos (labels)
    /// preenchido na ficha e caindo de forma resiliente para self.name ou para o padrão do tipo da ficha.
    pub fn get_display_name(&self) -> String {
        let label_name = self.labels.get(keys::HEADER_NOME)
            .or_else(|| self.labels.get("Name"))
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());

        let root_name = self.name.trim();

        // 1. Se o rótulo tiver um nome personalizado (não placeholder), tem prioridade máxima
        if let Some(lbl) = label_name {
            if !is_placeholder_name(lbl) {
                return lbl.to_string();
            }
            // Se o rótulo for placeholder padrão, mas root_name tiver um nome personalizado:
            if !is_placeholder_name(root_name) {
                return root_name.to_string();
            }
            return lbl.to_string();
        }

        // 2. Se o rótulo não estiver presente, usa root_name se não for vazio e nem placeholder "Sem Nome"
        if !root_name.is_empty() && root_name != "Sem Nome" {
            return root_name.to_string();
        }

        // 3. Fallback padrão de acordo com o tipo da ficha
        if self.is_gods_and_monsters() {
            "New Monster / Familiar".to_string()
        } else {
            "Novo Mago".to_string()
        }
    }

    /// Atualiza sincronizadamente tanto self.name quanto self.labels[keys::HEADER_NOME]
    pub fn set_display_name(&mut self, new_name: &str) {
        let trimmed = new_name.trim();
        let default_name = if self.is_gods_and_monsters() {
            "New Monster / Familiar"
        } else {
            "Novo Mago"
        };
        if trimmed.is_empty() {
            self.name = default_name.to_string();
            self.labels.insert(keys::HEADER_NOME.to_string(), String::new());
        } else {
            self.name = trimmed.to_string();
            self.labels.insert(keys::HEADER_NOME.to_string(), trimmed.to_string());
        }
    }

    pub fn is_placeholder_name(name: &str) -> bool {
        is_placeholder_name(name)
    }

    pub fn is_gods_and_monsters(&self) -> bool {
        self.sheet_type == "gods_and_monsters"
    }

    pub fn parse_from_db(id: &str, data_json: &str) -> Option<Self> {
        serde_json::from_str::<CharacterData>(data_json)
            .ok()
            .or_else(|| CharacterData::from_raw_json_resilient(id, data_json))
    }

    pub fn to_summary(&self, updated_at: String, is_public: bool, is_owner: bool) -> CharacterSummary {
        let (tradition, essence, arete) = if self.is_gods_and_monsters() {
            (
                self.labels.get("Type").cloned().unwrap_or_else(|| "Familiar / Bygone".to_string()),
                self.labels.get("Concept").cloned().unwrap_or_default(),
                self.get_attribute_level("Gnosis", 0),
            )
        } else {
            (
                self.labels.get("Tradição").cloned().unwrap_or_default(),
                self.labels.get("Essência").cloned().unwrap_or_default(),
                self.get_attribute_level(keys::KEY_ARETE, 1),
            )
        };

        let willpower = self.get_attribute_level(keys::KEY_WILLPOWER_TOTAL, 5);
        let photo_url = if !self.visuals.character_sketch_url.is_empty() {
            self.visuals.character_sketch_url.clone()
        } else {
            self.get_profile_photo()
        };
        let (photo_focus_x, photo_focus_y) = self.get_photo_focus();

        let mut spheres = Vec::with_capacity(STANDARD_SPHERES.len());
        for sphere in STANDARD_SPHERES {
            let lvl = self.get_attribute_level(sphere, 0);
            spheres.push((sphere.to_string(), lvl));
        }

        let sheet_type = if self.sheet_type.is_empty() {
            "mage".to_string()
        } else {
            self.sheet_type.clone()
        };

        CharacterSummary {
            id: self.id.clone(),
            name: self.get_display_name(),
            tradition,
            essence,
            arete,
            willpower,
            photo_url,
            photo_focus_y,
            photo_focus_x,
            spheres,
            sheet_type,
            is_public,
            is_owner,
            updated_at,
        }
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

    pub fn is_attribute_supernatural(&self, name: &str) -> bool {
        self.attributes
            .get(name)
            .map(|a| a.is_supernatural || a.level >= 6)
            .unwrap_or(false)
    }

    pub fn toggle_attribute_supernatural(&mut self, name: &str) {
        let entry = self.attributes.entry(name.to_string()).or_default();
        let current = entry.is_supernatural || entry.level >= 6;
        entry.is_supernatural = !current;
        if !entry.is_supernatural && entry.level > 5 {
            entry.set_level_with_origin(5, DotOrigin::Base);
        } else if entry.is_supernatural && entry.level < 5 {
            entry.set_level_with_origin(5, DotOrigin::Base);
        }
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

    pub fn get_photo_focus(&self) -> (i32, i32) {
        let y = self.labels.get("photo_focus_y")
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(self.visuals.photo_focus_y);

        let x = self.labels.get("photo_focus_x")
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(self.visuals.photo_focus_x);

        (x.clamp(0, 100), y.clamp(0, 100))
    }

    pub fn set_photo_focus(&mut self, x: i32, y: i32) {
        let cx = x.clamp(0, 100);
        let cy = y.clamp(0, 100);
        self.visuals.photo_focus_x = cx;
        self.visuals.photo_focus_y = cy;
        self.labels.insert("photo_focus_x".to_string(), cx.to_string());
        self.labels.insert("photo_focus_y".to_string(), cy.to_string());
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
        if key == "Nome" || key == keys::HEADER_NOME {
            self.get_display_name()
        } else {
            self.labels.get(key).cloned().unwrap_or_default()
        }
    }

    pub fn set_label(&mut self, key: &str, val: String) {
        if key == "Nome" || key == keys::HEADER_NOME {
            self.set_display_name(&val);
        } else {
            self.labels.insert(key.to_string(), val);
        }
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

    pub fn set_quintessence_paradox_box(&mut self, index: usize, state: char) {
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
        chars[index] = state;
        *raw = chars.into_iter().collect();
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

    /// Adiciona 1 ponto de Quintessência no sentido horário (para cima a partir das 9h: 0, 1, 2... 19)
    pub fn add_quintessence(&mut self) {
        let raw = self
            .labels
            .entry(keys::KEY_QUINTESSENCE_PARADOX.to_string())
            .or_insert_with(|| "0".repeat(20));
        let mut chars: Vec<char> = raw.chars().collect();
        while chars.len() < 20 {
            chars.push('0');
        }

        // Procura o primeiro slot livre no sentido horário a partir de 0
        for i in 0..20 {
            if chars[i] == '0' {
                chars[i] = '1';
                *raw = chars.into_iter().collect();
                return;
            }
        }
    }

    /// Remove 1 ponto de Quintessência (do ponto mais distante no sentido horário)
    pub fn remove_quintessence(&mut self) {
        let raw = self
            .labels
            .entry(keys::KEY_QUINTESSENCE_PARADOX.to_string())
            .or_insert_with(|| "0".repeat(20));
        let mut chars: Vec<char> = raw.chars().collect();
        while chars.len() < 20 {
            chars.push('0');
        }

        for i in (0..20).rev() {
            if chars[i] == '1' {
                chars[i] = '0';
                *raw = chars.into_iter().collect();
                return;
            }
        }
    }

    /// Adiciona 1 ponto de Paradoxo no sentido anti-horário (para baixo a partir das 9h: 19, 18, 17... 0)
    /// Se encontrar Quintessência no caminho, o Paradoxo consome/sobrepõe a Quintessência conforme as regras do Mago.
    pub fn add_paradox(&mut self) {
        let raw = self
            .labels
            .entry(keys::KEY_QUINTESSENCE_PARADOX.to_string())
            .or_insert_with(|| "0".repeat(20));
        let mut chars: Vec<char> = raw.chars().collect();
        while chars.len() < 20 {
            chars.push('0');
        }

        // Percorre de 19 descendo até 0
        for i in (0..20).rev() {
            if chars[i] != '2' {
                chars[i] = '2';
                *raw = chars.into_iter().collect();
                return;
            }
        }
    }

    /// Remove 1 ponto de Paradoxo (o ponto mais recente adicionado, subindo em direção às 9h)
    pub fn remove_paradox(&mut self) {
        let raw = self
            .labels
            .entry(keys::KEY_QUINTESSENCE_PARADOX.to_string())
            .or_insert_with(|| "0".repeat(20));
        let mut chars: Vec<char> = raw.chars().collect();
        while chars.len() < 20 {
            chars.push('0');
        }

        // Procura a partir de 0 subindo para 19
        for i in 0..20 {
            if chars[i] == '2' {
                chars[i] = '0';
                *raw = chars.into_iter().collect();
                return;
            }
        }
    }

    /// Limpa todos os pontos de Quintessência da roda
    pub fn clear_quintessence(&mut self) {
        let raw = self
            .labels
            .entry(keys::KEY_QUINTESSENCE_PARADOX.to_string())
            .or_insert_with(|| "0".repeat(20));
        let mut chars: Vec<char> = raw.chars().collect();
        for c in chars.iter_mut() {
            if *c == '1' {
                *c = '0';
            }
        }
        *raw = chars.into_iter().collect();
    }

    /// Limpa todos os pontos de Paradoxo da roda
    pub fn clear_paradox(&mut self) {
        let raw = self
            .labels
            .entry(keys::KEY_QUINTESSENCE_PARADOX.to_string())
            .or_insert_with(|| "0".repeat(20));
        let mut chars: Vec<char> = raw.chars().collect();
        for c in chars.iter_mut() {
            if *c == '2' {
                *c = '0';
            }
        }
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

    /// Verifica se uma página possui conteúdo real preenchido pelo jogador
    pub fn is_page_has_content(&self, page_index: usize) -> bool {
        match page_index {
            0 => true, // Página 1 (Principal): sempre ativa
            1 => {
                // Página 2 (Mágika & Combate): Méritos, Defeitos, Armas, Maravilhas, Focos/Paradigmas
                let has_merits = self.merits.iter().any(|m| !m.name.trim().is_empty());
                let has_flaws = self.flaws.iter().any(|f| !f.name.trim().is_empty());
                let has_weapons = self.weapons.iter().any(|w| !w.name.trim().is_empty());
                let has_wonders = self.wonders.iter().any(|w| !w.name.trim().is_empty());
                let has_focus = self.labels.iter().any(|(k, v)| {
                    (k.starts_with("focus_") || k.starts_with("paradigm_") || k.starts_with("practice_") || k.starts_with("instrument_") || k.starts_with("other_trait_"))
                        && !v.trim().is_empty()
                });
                let has_armor = !self.armor.class_name.trim().is_empty() || !self.armor.rating.trim().is_empty();
                has_merits || has_flaws || has_weapons || has_wonders || has_focus || has_armor
            }
            2 => {
                // Página 3 (Antecedentes Expandidos & Posses)
                let has_chantry = self.chantry.iter().any(|c| !c.location.trim().is_empty() || !c.description.trim().is_empty());
                let has_possessions = !self.possessions.gear_carried.trim().is_empty() || !self.possessions.equipment_owned.trim().is_empty() || !self.possessions.grimoire.trim().is_empty();
                let has_expanded_bg = self.custom_lists.iter().any(|(k, list)| k.starts_with("exp_bg_") && !list.is_empty());
                has_chantry || has_possessions || has_expanded_bg
            }
            3 => {
                // Página 4 (História & Visual)
                let has_history = !self.history_data.history.trim().is_empty() || !self.history_data.goals_destiny.trim().is_empty();
                let has_desc = !self.description_data.apparent_age.trim().is_empty() || !self.description_data.hair.trim().is_empty() || !self.description_data.eyes.trim().is_empty();
                let has_visuals = !self.visuals.cabal_chart_url.trim().is_empty() || !self.visuals.character_sketch_url.trim().is_empty();
                has_history || has_desc || has_visuals
            }
            4 => {
                // Página 5 (Grimório)
                !self.grimoire.rotes.is_empty() || !self.grimoire.paradigm.trim().is_empty() || !self.grimoire.general_notes.trim().is_empty()
            }
            5 => {
                // Página 6 (Notas)
                !self.notes_data.session_notes.trim().is_empty() || !self.notes_data.campaign_journal.trim().is_empty()
            }
            _ => false,
        }
    }
}
