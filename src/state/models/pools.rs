use super::character::CharacterData;

impl CharacterData {
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
