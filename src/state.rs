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
    pub const HEALTH_KEY_PREFIX: &str = "health_";

    // Categories
    pub const CAT_TALENTOS: &str = "Talentos";
    pub const CAT_PERICIAS: &str = "Perícias";
    pub const CAT_CONHECIMENTOS: &str = "Conhecimentos";
    pub const CAT_ANTECEDENTES: &str = "Antecedentes";
    pub const CAT_RESONANCE: &str = "Resonance";
}

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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AttributeValue {
    pub level: i32,
    pub modifier: String,
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
            name: if name.trim().is_empty() { "Novo Personagem".to_string() } else { name },
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

    /// Set attribute/ability/sphere level and modifier
    pub fn set_attribute(&mut self, name: &str, level: Option<i32>, modifier: Option<String>) {
        let entry = self.attributes.entry(name.to_string()).or_default();
        if let Some(l) = level {
            entry.level = l;
        }
        if let Some(m) = modifier {
            entry.modifier = m;
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

#[server(GetSheets, "/api")]
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

#[server(GetSheet, "/api")]
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

#[server(CreateSheet, "/api")]
pub async fn create_sheet(name: String) -> Result<String, ServerFnError> {
    let clean_name = name.trim().to_string();
    let final_name = if clean_name.is_empty() { "Novo Personagem".to_string() } else { clean_name };

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

#[server(UpdateSheet, "/api")]
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

#[server(DeleteSheet, "/api")]
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
}
