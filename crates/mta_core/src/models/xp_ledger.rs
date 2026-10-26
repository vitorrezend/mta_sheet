use serde::{Deserialize, Serialize};
use super::character::CharacterData;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct XpLedgerEntry {
    pub id: String,
    pub date: String,
    pub description: String,
    pub amount: i32,
    #[serde(default)]
    pub created_at: String,
}

impl XpLedgerEntry {
    pub fn new(description: String, amount: i32, date: Option<String>) -> Self {
        let entry_date = date.unwrap_or_else(|| "Sessão Atual".to_string());
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            date: entry_date,
            description,
            amount,
            created_at: String::new(),
        }
    }
}

impl CharacterData {
    /// Soma todos os pontos de XP concedidos no histórico do personagem
    pub fn total_xp_awarded(&self) -> i32 {
        self.xp_ledger.iter().map(|e| e.amount).sum()
    }

    /// Adiciona uma nova entrada de concessão de XP
    pub fn add_xp_entry(&mut self, description: String, amount: i32, date: Option<String>) {
        self.xp_ledger.push(XpLedgerEntry::new(description, amount, date));
    }

    /// Remove uma entrada de XP pelo ID
    pub fn remove_xp_entry(&mut self, id: &str) {
        self.xp_ledger.retain(|e| e.id != id);
    }
}
