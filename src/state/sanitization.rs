use super::models::{
    keys, ArmorItem, AttributeValue, ChantryEntry, CharacterData, CharacterDescriptionData,
    CharacterHistoryData, CharacterVisualsData, DotOrigin, ExpandedBackgroundsData, FlawItem, MeritItem,
    PossessionsData, WeaponItem, WonderItem, GrimoireData,
};

impl CharacterData {
    /// Sanitize data: clamp attributes, ensure valid bounds, fix name
    pub fn sanitize(&mut self) {
        if self.name.trim().is_empty() {
            self.name = "Sem Nome".to_string();
        }

        // Ensure Arete is at least 1 and at most 10
        let arete = self.attributes.entry(keys::KEY_ARETE.to_string()).or_default();
        arete.level = arete.level.clamp(1, 10);

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

        // Initialize default Resonance items if empty
        let res_list = self.custom_lists.entry(keys::CAT_RESONANCE.to_string()).or_default();
        if res_list.is_empty() {
            let defaults = [
                ("res_entropic", "Entrópico"),
                ("res_static", "Estático"),
                ("res_dynamic", "Dinâmico"),
            ];
            for (id, label) in defaults {
                res_list.push(id.to_string());
                self.labels.entry(id.to_string()).or_insert_with(|| label.to_string());
            }
        }

        // Ensure minimum slots for Merits (7), Flaws (7), Wonders (4) and Weapons (4)
        while self.merits.len() < 7 {
            self.merits.push(MeritItem::default());
        }
        while self.flaws.len() < 7 {
            self.flaws.push(FlawItem::default());
        }
        while self.wonders.len() < 4 {
            self.wonders.push(WonderItem::default());
        }
        for wonder in &mut self.wonders {
            if wonder.id.is_empty() {
                wonder.id = format!("wonder_{}", uuid::Uuid::new_v4());
            }
            if wonder.quintessence_max < 0 {
                wonder.quintessence_max = 0;
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

        // Ensure minimum slots for Chantry (3)
        while self.chantry.len() < 3 {
            self.chantry.push(ChantryEntry::default());
        }

        // Ensure Grimoire rotes have valid UUIDs
        for rote in &mut self.grimoire.rotes {
            if rote.id.is_empty() {
                rote.id = format!("rote_{}", uuid::Uuid::new_v4());
            }
        }

        // Ensure minimum slots for practices and instruments (3)
        while self.grimoire.practices.len() < 3 {
            self.grimoire.practices.push(String::new());
        }
        while self.grimoire.instruments.len() < 3 {
            self.grimoire.instruments.push(String::new());
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

        // Page 3 Recovery
        if let Some(exp_bg) = val.get("expanded_backgrounds") {
            if let Ok(bg) = serde_json::from_value::<ExpandedBackgroundsData>(exp_bg.clone()) {
                char_data.expanded_backgrounds = bg;
            }
        }

        if let Some(poss) = val.get("possessions") {
            if let Ok(p) = serde_json::from_value::<PossessionsData>(poss.clone()) {
                char_data.possessions = p;
            }
        }

        if let Some(chantry_arr) = val.get("chantry").and_then(|v| v.as_array()) {
            char_data.chantry.clear();
            for c in chantry_arr {
                if let Ok(entry) = serde_json::from_value::<ChantryEntry>(c.clone()) {
                    char_data.chantry.push(entry);
                }
            }
        }

        // Page 4 Recovery
        if let Some(hist) = val.get("history_data") {
            if let Ok(h) = serde_json::from_value::<CharacterHistoryData>(hist.clone()) {
                char_data.history_data = h;
            }
        }

        if let Some(desc) = val.get("description_data") {
            if let Ok(d) = serde_json::from_value::<CharacterDescriptionData>(desc.clone()) {
                char_data.description_data = d;
            }
        }

        if let Some(vis) = val.get("visuals") {
            if let Ok(v) = serde_json::from_value::<CharacterVisualsData>(vis.clone()) {
                char_data.visuals = v;
            }
        }

        // Page 5 Grimoire Recovery
        if let Some(grim) = val.get("grimoire") {
            if let Ok(g) = serde_json::from_value::<GrimoireData>(grim.clone()) {
                char_data.grimoire = g;
            }
        }

        char_data.sanitize();
        Some(char_data)
    }
}
