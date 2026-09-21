//! M20 Compêndio de Habilidades (Abilities).
//!
//! Capítulo 6: Criação do Personagem (pp. 275-301).

pub mod models;
pub mod data;

pub use models::*;
pub use data::*;

use std::sync::LazyLock;

/// Coleção consolidada de todas as habilidades (básicas e secundárias).
pub static ALL_ABILITIES: LazyLock<Vec<AbilityDefinition>> = LazyLock::new(|| {
    let mut list = Vec::new();
    list.extend_from_slice(CORE_TALENTS);
    list.extend_from_slice(CORE_SKILLS);
    list.extend_from_slice(CORE_KNOWLEDGES);
    list.extend_from_slice(SECONDARY_ABILITIES);
    list
});

/// Busca uma habilidade por ID ou nome (em inglês ou português).
pub fn find_ability(query: &str) -> Option<&'static AbilityDefinition> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }
    ALL_ABILITIES.iter().find(|a| {
        a.id.eq_ignore_ascii_case(&q)
            || a.name.to_lowercase() == q
            || a.name_pt.to_lowercase() == q
    })
}

/// Busca um artigo de regra teórica opcional por ID.
pub fn find_ability_theory_rule(id: &str) -> Option<&'static AbilityTheoryArticle> {
    ABILITY_THEORY_RULES.iter().find(|r| r.id == id)
}
