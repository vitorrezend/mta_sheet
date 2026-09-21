use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AttributeValue {
    #[serde(default, deserialize_with = "deserialize_flexible_i32")]
    pub level: i32,
    #[serde(default, skip_serializing_if = "is_empty_str")]
    pub modifier: String,
    #[serde(default, skip_serializing_if = "is_default_origins")]
    pub dot_origins: Vec<DotOrigin>,
    #[serde(default, skip_serializing_if = "is_false_bool")]
    pub is_supernatural: bool,
}

impl AttributeValue {
    pub fn new(level: i32, modifier: String) -> Self {
        let is_supernatural = level >= 6;
        Self {
            level,
            modifier,
            dot_origins: vec![DotOrigin::Base; level.max(0) as usize],
            is_supernatural,
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
        if new_level < 5 {
            self.is_supernatural = false;
        } else if new_level >= 6 {
            self.is_supernatural = true;
        }
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

impl super::character::CharacterData {
    /// Retorna a lista de IDs das 9 esferas ativas na ficha (para compêndio e visualização)
    pub fn get_active_spheres(&self) -> Vec<String> {
        let mut list = Vec::with_capacity(9);
        // 1. Correspondência / Dados
        let is_data = self.labels.get("sphere_slot_correspondence")
            .map(|v| v.eq_ignore_ascii_case("Dados") || v.eq_ignore_ascii_case("Data") || v.eq_ignore_ascii_case("techno"))
            .unwrap_or_else(|| {
                self.attributes.contains_key("Dados") || self.attributes.contains_key("Data")
            });
        if is_data {
            list.push("data".to_string());
        } else {
            list.push("correspondence".to_string());
        }

        list.push("entropy".to_string());
        list.push("forces".to_string());
        list.push("life".to_string());
        list.push("matter".to_string());
        list.push("mind".to_string());

        // 2. Primórdio / Utilidade Primordial
        let is_primal = self.labels.get("sphere_slot_prime")
            .map(|v| v.eq_ignore_ascii_case("Utilidade Primordial") || v.eq_ignore_ascii_case("Primal Utility") || v.eq_ignore_ascii_case("techno"))
            .unwrap_or_else(|| {
                self.attributes.contains_key("Utilidade Primordial") || self.attributes.contains_key("Primal Utility")
            });
        if is_primal {
            list.push("primal_utility".to_string());
        } else {
            list.push("prime".to_string());
        }

        // 3. Espírito / Ciência Dimensional
        let is_dim_sci = self.labels.get("sphere_slot_spirit")
            .map(|v| v.eq_ignore_ascii_case("Ciência Dimensional") || v.eq_ignore_ascii_case("Dimensional Science") || v.eq_ignore_ascii_case("techno"))
            .unwrap_or_else(|| {
                self.attributes.contains_key("Ciência Dimensional") || self.attributes.contains_key("Dimensional Science")
            });
        if is_dim_sci {
            list.push("dimensional_science".to_string());
        } else {
            list.push("spirit".to_string());
        }

        list.push("time".to_string());
        list
    }

    /// Altera a variante ativa de uma esfera (ex: Tradicional vs Tecnocrática)
    /// migrando atomicamente pontos, modificadores, origens de pontos e esfera de afinidade.
    pub fn swap_sphere_variant(&mut self, chosen_id: &str) -> bool {
        let swap_info: Option<(&'static str, &'static str, &[&'static str])> = match chosen_id.to_lowercase().as_str() {
            "correspondence" | "correspondência" => Some(("correspondence", "Correspondência", &["Dados", "Data"])),
            "data" | "dados" => Some(("correspondence", "Dados", &["Correspondência", "Correspondence"])),
            "spirit" | "espírito" => Some(("spirit", "Espírito", &["Ciência Dimensional", "Dimensional Science"])),
            "dimensional_science" | "ciência dimensional" | "ciencia dimensional" => Some(("spirit", "Ciência Dimensional", &["Espírito", "Spirit"])),
            "prime" | "primórdio" | "primordio" => Some(("prime", "Primórdio", &["Utilidade Primordial", "Primal Utility"])),
            "primal_utility" | "utilidade primordial" => Some(("prime", "Utilidade Primordial", &["Primórdio", "Prime"])),
            _ => None,
        };

        if let Some((slot, to_name, old_names)) = swap_info {
            // 1. Procura e transfere pontos de qualquer chave anterior para o novo nome canônico
            let mut found_attr = None;
            for &old in old_names {
                if let Some(attr) = self.attributes.remove(old) {
                    found_attr = Some(attr);
                    break;
                }
            }
            if let Some(attr) = found_attr {
                if !self.attributes.contains_key(to_name) || self.attributes.get(to_name).map(|a| a.level == 0).unwrap_or(true) {
                    self.attributes.insert(to_name.to_string(), attr);
                }
            }

            // 2. Se a Esfera de Afinidade estava associada a qualquer variante anterior, atualiza
            if let Some(ref aff) = self.get_affinity_sphere() {
                let is_old_aff = old_names.iter().any(|name| aff.eq_ignore_ascii_case(name));
                if is_old_aff {
                    self.set_affinity_sphere(Some(to_name.to_string()));
                }
            }

            // 3. Atualiza o label do slot da esfera com o nome canônico em português
            self.labels.insert(format!("sphere_slot_{}", slot), to_name.to_string());
            true
        } else {
            false
        }
    }
}

