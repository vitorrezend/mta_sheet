//! Apêndice I: Allies and Antagonists (M20, pp. 618-641).
//!
//! - `models`: tipos e leitura dos números de cada ficha (`QuickStats`)
//! - `data`: catálogo gerado a partir de data/compendium/bestiary/manifest.json

pub mod data;
pub mod models;

pub use data::ALL_BESTIARY;
pub use models::*;

/// Busca um verbete por ID ou nome (EN ou PT), sem diferenciar maiúsculas.
pub fn find_bestiary_entry(query: &str) -> Option<&'static BestiaryEntry> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }
    ALL_BESTIARY.iter().find(|e| e.id == q || e.name.to_lowercase() == q || e.name_pt.to_lowercase() == q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_has_every_entry_with_text() {
        assert_eq!(ALL_BESTIARY.iter().filter(|e| !e.is_article).count(), 61);
        let mut ids = std::collections::HashSet::new();
        for e in ALL_BESTIARY {
            assert!(ids.insert(e.id), "ID duplicado: {}", e.id);
            assert!(e.content.contains("**Source:** M20"), "{} sem fonte", e.id);
            assert!(e.content_pt.contains("**Fonte:** Livro Básico M20"), "{} sem fonte em PT", e.id);
            assert!(e.has_pt_text(), "{} sem texto PT", e.id);
            if !e.is_article && e.group != BestiaryGroup::Totems && e.id != "sendings" && e.id != "memophores" {
                assert!(!e.stat_lines().is_empty(), "{} sem ficha", e.id);
                assert!(!e.stat_lines_pt().is_empty(), "{} sem ficha em PT", e.id);
            }
        }
    }

    #[test]
    fn test_quick_stats_reads_people_animals_and_spirits() {
        let thug = find_bestiary_entry("thug").unwrap().quick_stats();
        assert_eq!(thug.willpower, Some(3));
        assert_eq!(thug.health_levels.len(), 7);
        assert_eq!(thug.initiative_base(), Some(4));

        let wolf = find_bestiary_entry("Wolf").unwrap().quick_stats();
        assert_eq!(wolf.attribute("Strength"), Some(3));

        let horse = find_bestiary_entry("horse").unwrap().quick_stats();
        assert_eq!(horse.attribute("Strength"), Some(5));

        let air = find_bestiary_entry("air_elemental").unwrap().quick_stats();
        assert_eq!((air.willpower, air.rage, air.gnosis, air.essence), (Some(3), Some(8), Some(7), Some(40)));

        let judgment = find_bestiary_entry("judgment").unwrap().quick_stats();
        assert_eq!(judgment.health_levels.len(), 20);
        assert_eq!(judgment.attribute("Strength"), Some(6));

        let pillory = find_bestiary_entry("pillory").unwrap().quick_stats();
        assert_eq!(pillory.arete, Some(6));
    }
}
