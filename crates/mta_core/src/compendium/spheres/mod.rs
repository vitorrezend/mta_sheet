//! M20 Compêndio Modular de Esferas da Mágika (Spheres of Magick)
//!
//! Capítulo 10: O Livro das Esferas (pp. 504-534).
//!
//! Organizado em submódulos coesos (< 500 linhas cada):
//! - `models`: Tipos de dados, enums e contratos
//! - `data`: Catálogo unificado das 13 Esferas da Mágika e variantes
//! - `unabridged`: Textos completos carregados sob demanda via Markdown

pub mod models;
pub mod data;
pub mod unabridged;

pub use models::*;
pub use data::*;
pub use unabridged::find_unabridged_text;

use crate::i18n::Language;

// ============================================================================
// Catálogo Unificado Canônico de M20 (13 Esferas e Variantes)
// ============================================================================

pub const ALL_SPHERES: &[SphereDefinition] = &[
    CORRESPONDENCE,
    ENTROPY,
    FORCES,
    LIFE,
    MATTER,
    MIND,
    PRIME,
    SPIRIT,
    TIME,
    DATA,
    DIMENSIONAL_SCIENCE,
    PRIMAL_UTILITY,
    WILD_TALENT,
];

/// Retorna todas as esferas e variantes canônicas catalogadas.
pub fn get_all_spheres() -> &'static [SphereDefinition] {
    ALL_SPHERES
}

/// Normaliza uma string para comparação case-insensitive sem acentos básicos.
fn normalize_query(q: &str) -> String {
    q.trim()
        .to_lowercase()
        .replace(['á', 'à', 'ã', 'â'], "a")
        .replace(['é', 'ê'], "e")
        .replace(['í'], "i")
        .replace(['ó', 'õ', 'ô'], "o")
        .replace(['ú'], "u")
        .replace(['ç'], "c")
}

/// Busca inteligente e resiliente por uma Esfera no Compêndio.
/// Suporta IDs, nomes em inglês, nomes em português, variantes da Tecnocracia e buscas parciais.
pub fn find_sphere(query: &str) -> Option<&'static SphereDefinition> {
    let q_norm = normalize_query(query);
    if q_norm.is_empty() {
        return None;
    }

    // 1. Match exato por ID
    if let Some(s) = ALL_SPHERES.iter().find(|s| s.id == q_norm) {
        return Some(s);
    }

    // 2. Match exato por nome normalizado (EN ou PT)
    if let Some(s) = ALL_SPHERES.iter().find(|s| {
        normalize_query(s.name) == q_norm || normalize_query(s.name_pt) == q_norm
    }) {
        return Some(s);
    }

    // 3. Match em equivalentes da Tecnocracia ou Místicos
    if let Some(s) = ALL_SPHERES.iter().find(|s| {
        s.technocracy_equivalent.map(|t| normalize_query(t) == q_norm).unwrap_or(false)
            || s.technocracy_equivalent_pt.map(|t| normalize_query(t) == q_norm).unwrap_or(false)
            || s.mystic_equivalent.map(|m| normalize_query(m) == q_norm).unwrap_or(false)
            || s.mystic_equivalent_pt.map(|m| normalize_query(m) == q_norm).unwrap_or(false)
    }) {
        return Some(s);
    }

    // 4. Substring nos nomes principais ou equivalentes
    ALL_SPHERES.iter().find(|s| {
        normalize_query(s.name).contains(&q_norm)
            || normalize_query(s.name_pt).contains(&q_norm)
            || s.technocracy_equivalent.map(|t| normalize_query(t).contains(&q_norm)).unwrap_or(false)
            || s.technocracy_equivalent_pt.map(|t| normalize_query(t).contains(&q_norm)).unwrap_or(false)
            || s.mystic_equivalent.map(|m| normalize_query(m).contains(&q_norm)).unwrap_or(false)
            || s.mystic_equivalent_pt.map(|m| normalize_query(m).contains(&q_norm)).unwrap_or(false)
    })
}

/// Retorna a lista de especialidades sugeridas para a esfera especificada no idioma solicitado.
pub fn get_suggested_specialties(name_or_id: &str, lang: Language) -> Vec<&'static str> {
    if let Some(s) = find_sphere(name_or_id) {
        s.suggested_specialties(lang)
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_all_spheres_count_and_uniqueness() {
        assert_eq!(ALL_SPHERES.len(), 13, "M20 deve conter exatamente 13 esferas e variantes");
        let mut ids = HashSet::new();
        for s in ALL_SPHERES {
            assert!(ids.insert(s.id), "ID duplicado encontrado: {}", s.id);
            assert!(s.page_ref.starts_with("M20, p"), "Referência de página inválida para {}: {}", s.name, s.page_ref);
            assert!(!s.name_pt.is_empty(), "Nome em PT não pode ser vazio para {}", s.name);
            assert!(!s.description.is_empty(), "Descrição em EN não pode ser vazia para {}", s.name);
            assert!(!s.description_pt.is_empty(), "Descrição em PT não pode ser vazia para {}", s.name_pt);
            if !s.is_optional_rule {
                assert_eq!(s.ranks.len(), 5, "Esfera {} deve conter exatamente 5 postos", s.name);
            }
        }
    }

    #[test]
    fn test_find_sphere_resolves_bilingual() {
        // Testes em inglês
        assert_eq!(find_sphere("Correspondence").unwrap().id, "correspondence");
        assert_eq!(find_sphere("Entropy").unwrap().id, "entropy");
        assert_eq!(find_sphere("Forces").unwrap().id, "forces");
        assert_eq!(find_sphere("Life").unwrap().id, "life");
        assert_eq!(find_sphere("Matter").unwrap().id, "matter");
        assert_eq!(find_sphere("Mind").unwrap().id, "mind");
        assert_eq!(find_sphere("Prime").unwrap().id, "prime");
        assert_eq!(find_sphere("Spirit").unwrap().id, "spirit");
        assert_eq!(find_sphere("Time").unwrap().id, "time");
        assert_eq!(find_sphere("Data").unwrap().id, "data");
        assert_eq!(find_sphere("Dimensional Science").unwrap().id, "dimensional_science");
        assert_eq!(find_sphere("Primal Utility").unwrap().id, "primal_utility");
        assert_eq!(find_sphere("Wild Talent").unwrap().id, "wild_talent");

        // Testes em português com ou sem acentos
        assert_eq!(find_sphere("Correspondência").unwrap().id, "correspondence");
        assert_eq!(find_sphere("correspondencia").unwrap().id, "correspondence");
        assert_eq!(find_sphere("Entropia").unwrap().id, "entropy");
        assert_eq!(find_sphere("Forças").unwrap().id, "forces");
        assert_eq!(find_sphere("forcas").unwrap().id, "forces");
        assert_eq!(find_sphere("Vida").unwrap().id, "life");
        assert_eq!(find_sphere("Matéria").unwrap().id, "matter");
        assert_eq!(find_sphere("materia").unwrap().id, "matter");
        assert_eq!(find_sphere("Mente").unwrap().id, "mind");
        assert_eq!(find_sphere("Primórdio").unwrap().id, "prime");
        assert_eq!(find_sphere("primordio").unwrap().id, "prime");
        assert_eq!(find_sphere("Espírito").unwrap().id, "spirit");
        assert_eq!(find_sphere("espirito").unwrap().id, "spirit");
        assert_eq!(find_sphere("Tempo").unwrap().id, "time");
        assert_eq!(find_sphere("Dados").unwrap().id, "data");
        assert_eq!(find_sphere("Ciência Dimensional").unwrap().id, "dimensional_science");
        assert_eq!(find_sphere("ciencia dimensional").unwrap().id, "dimensional_science");
        assert_eq!(find_sphere("Utilidade Primordial").unwrap().id, "primal_utility");
        assert_eq!(find_sphere("Talento Selvagem").unwrap().id, "wild_talent");
    }
}
