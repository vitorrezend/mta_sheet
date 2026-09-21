//! Compêndio de Qualidades, Defeitos e Perturbações (M20).
//!
//! Apêndice II: Qualidades e Defeitos (pp. 642-650).

pub mod models;
pub mod data;

pub use models::*;
pub use data::*;

use std::sync::LazyLock;

/// Coleção consolidada de todas as qualidades e defeitos.
pub static ALL_MERITS_FLAWS: LazyLock<Vec<MeritFlawDefinition>> = LazyLock::new(|| {
    let mut list = Vec::new();
    list.extend_from_slice(ALL_MERITS);
    list.extend_from_slice(ALL_FLAWS);
    list
});

/// Busca uma qualidade ou defeito por ID ou nome (em inglês, português ou nome tecnocrata).
pub fn find_merit_flaw(query: &str) -> Option<&'static MeritFlawDefinition> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }
    ALL_MERITS_FLAWS.iter().find(|mf| {
        mf.id.eq_ignore_ascii_case(&q)
            || mf.name.to_lowercase() == q
            || mf.name_pt.to_lowercase() == q
            || mf.technocracy_name.is_some_and(|t| t.to_lowercase() == q)
            || mf.technocracy_name_pt.is_some_and(|t| t.to_lowercase() == q)
    })
}

/// Busca uma perturbação mental por ID ou nome (em inglês ou português).
pub fn find_derangement(query: &str) -> Option<&'static DerangementDefinition> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }
    ALL_DERANGEMENTS.iter().find(|d| {
        d.id.eq_ignore_ascii_case(&q)
            || d.name.to_lowercase() == q
            || d.name_pt.to_lowercase() == q
    })
}
