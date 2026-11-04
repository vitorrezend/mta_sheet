//! M20 Compêndio Modular de Práticas Mágikas (Practices)
//!
//! Capítulo 10: Foco e Artes (pp. 573-586).
//! Dividido em submódulos conforme a Clean Architecture (< 1.000 linhas por arquivo).

pub mod bos;
pub mod callouts;
pub mod data;

pub use bos::*;
pub use callouts::*;
pub use data::*;

use serde::Serialize;
use crate::i18n::Language;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticeDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub aliases: &'static [&'static str],
    pub page_ref: &'static str,
    pub quote: &'static str,
    pub quote_pt: &'static str,
    pub paradigm_affinity: &'static [&'static str],
    pub paradigm_affinity_pt: &'static [&'static str],
    pub description: &'static str,
    pub description_pt: &'static str,
    pub common_instruments: &'static [&'static str],
    pub common_instruments_pt: &'static [&'static str],
    pub associated_abilities: &'static [&'static str],
    pub associated_abilities_pt: &'static [&'static str],
    pub callouts: &'static [CalloutBox],
}

impl PracticeDefinition {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    pub fn quote(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.quote_pt,
            Language::EnUs => self.quote,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn paradigm_affinity(&self, lang: Language) -> &'static [&'static str] {
        match lang {
            Language::PtBr => self.paradigm_affinity_pt,
            Language::EnUs => self.paradigm_affinity,
        }
    }

    pub fn common_instruments(&self, lang: Language) -> &'static [&'static str] {
        match lang {
            Language::PtBr => self.common_instruments_pt,
            Language::EnUs => self.common_instruments,
        }
    }

    pub fn associated_abilities(&self, lang: Language) -> &'static [&'static str] {
        match lang {
            Language::PtBr => self.associated_abilities_pt,
            Language::EnUs => self.associated_abilities,
        }
    }
}

/// All registered practices: the 20 from M20 followed by the 7 from the Book of Secrets.
pub const ALL_PRACTICES: &[PracticeDefinition] = &[
    ALCHEMY,
    ART_OF_DESIRE,
    CHAOS_MAGICK,
    CRAFTWORK,
    CRAZY_WISDOM,
    CYBERNETICS,
    DOMINION,
    FAITH,
    GUTTER_MAGICK,
    HIGH_RITUAL_MAGICK,
    HYPERTECH,
    MALEFICIA,
    MARTIAL_ARTS,
    MEDICINE_WORK,
    REALITY_HACKING,
    SHAMANISM,
    VOUDOUN,
    WEIRD_SCIENCE,
    WITCHCRAFT,
    YOGA,
    ANIMALISM,
    BARDISM,
    ELEMENTALISM,
    GOD_BONDING,
    INVIGORATION,
    MEDIUMSHIP,
    PSIONICS,
];

/// Quadros de texto (sidebars) da seção de Práticas, endereçáveis por id.
pub const PRACTICE_BOXES: &[(&str, &CalloutBox)] = &[
    ("box_left_right", &BOX_LEFT_AND_RIGHT_HAND_PATHS),
    ("box_expanded_practices", &BOX_EXPANDED_PRACTICES),
    ("box_theosophy", &BOX_THEOSOPHY),
];

/// Busca um quadro (sidebar) da seção de Práticas pelo id.
pub fn find_practice_box(id: &str) -> Option<&'static CalloutBox> {
    PRACTICE_BOXES.iter().find(|(k, _)| *k == id).map(|(_, b)| *b)
}

/// Returns a slice of practice names for quick selection / autocomplete in the requested language.
pub fn get_practice_names(lang: Language) -> Vec<&'static str> {
    ALL_PRACTICES.iter().map(|p| p.name(lang)).collect()
}

/// Look up a practice by case-insensitive name, ID, or alias.
pub fn find_practice(query: &str) -> Option<&'static PracticeDefinition> {
    let clean = query.trim();
    if clean.is_empty() {
        return None;
    }

    // 1. Direct ID match
    if let Some(found) = ALL_PRACTICES.iter().find(|p| p.id.eq_ignore_ascii_case(clean)) {
        return Some(found);
    }

    // 2. Direct name match (EN)
    if let Some(found) = ALL_PRACTICES.iter().find(|p| p.name.eq_ignore_ascii_case(clean)) {
        return Some(found);
    }

    // 3. Direct name match (PT)
    if let Some(found) = ALL_PRACTICES.iter().find(|p| p.name_pt.eq_ignore_ascii_case(clean)) {
        return Some(found);
    }

    // 4. Alias / Portuguese translation match
    if let Some(found) = ALL_PRACTICES.iter().find(|p| {
        p.aliases.iter().any(|alias| alias.eq_ignore_ascii_case(clean))
    }) {
        return Some(found);
    }

    // 5. Substring / partial match fallback
    let lower = clean.to_lowercase();
    ALL_PRACTICES.iter().find(|p| {
        p.name.to_lowercase().contains(&lower)
            || p.name_pt.to_lowercase().contains(&lower)
            || p.aliases.iter().any(|a| a.to_lowercase().contains(&lower))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_practices_count_and_uniqueness() {
        assert_eq!(ALL_PRACTICES.len(), 27);
        let mut ids = std::collections::HashSet::new();
        for p in ALL_PRACTICES {
            assert!(ids.insert(p.id), "Duplicate practice ID: {}", p.id);
            assert!(!p.page_ref.is_empty(), "Missing page_ref for {}", p.name);
            assert!(!p.description.is_empty(), "Missing EN description for {}", p.name);
            assert!(!p.description_pt.is_empty(), "Missing PT description for {}", p.name_pt);
            assert_eq!(p.quote.is_empty(), p.quote_pt.is_empty(), "Quote EN/PT mismatch for {}", p.name_pt);
            assert_eq!(p.paradigm_affinity.len(), p.paradigm_affinity_pt.len(), "{}", p.id);
            assert_eq!(p.common_instruments.len(), p.common_instruments_pt.len(), "{}", p.id);
            assert_eq!(p.associated_abilities.len(), p.associated_abilities_pt.len(), "{}", p.id);
        }
    }

    #[test]
    fn test_bilingual_accessor_methods() {
        let p = &WITCHCRAFT;
        assert_eq!(p.name(Language::EnUs), "Witchcraft");
        assert_eq!(p.name(Language::PtBr), "Bruxaria");
        assert!(p.description(Language::PtBr).contains("Bruxa. Uma das palavras"));
        assert!(p.description(Language::EnUs).contains("Witch. One of the more"));
    }

    #[test]
    fn test_find_practice_exact_and_case_insensitive() {
        assert_eq!(find_practice("Witchcraft").unwrap().id, "witchcraft");
        assert_eq!(find_practice("witchcraft").unwrap().id, "witchcraft");
        assert_eq!(find_practice("Bruxaria").unwrap().id, "witchcraft");
        assert_eq!(find_practice("bruxaria").unwrap().id, "witchcraft");
        assert_eq!(find_practice("Alchemy").unwrap().id, "alchemy");
        assert_eq!(find_practice("Alquimia").unwrap().id, "alchemy");
        assert_eq!(find_practice("Yoga").unwrap().id, "yoga");
        assert_eq!(find_practice("Ioga").unwrap().id, "yoga");
    }

    #[test]
    fn test_bos_practices_lookup() {
        assert_eq!(find_practice("Psionics").map(|p| p.id), Some("psionics"));
        assert_eq!(find_practice("Mediunidade").map(|p| p.id), Some("mediumship"));
        assert_eq!(find_practice("God-Bonding").map(|p| p.id), Some("god_bonding"));
        assert_eq!(PSIONICS.callouts.first().map(|c| c.title), Some("Theosophy"));
        assert!(find_practice_box("box_expanded_practices").is_some());
        assert!(ALL_PRACTICES.iter().filter(|p| p.page_ref.starts_with("BoS")).count() == 7);
    }

    #[test]
    fn test_left_and_right_hand_paths_bilingual() {
        let b = &BOX_LEFT_AND_RIGHT_HAND_PATHS;
        assert_eq!(b.title(Language::EnUs), "Left- and Right-Hand Paths");
        assert_eq!(b.title(Language::PtBr), "Caminhos da Mão Esquerda e da Mão Direita");
        assert!(b.content(Language::PtBr).contains("Caminho da Mão Esquerda e Caminho da Mão Direita"));
    }

    #[test]
    fn test_paragraph_parity_between_en_and_pt() {
        for p in ALL_PRACTICES {
            let en_paras: Vec<&str> = p.description.split("\n\n").map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            let pt_paras: Vec<&str> = p.description_pt.split("\n\n").map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            assert_eq!(
                en_paras.len(),
                pt_paras.len(),
                "Practice '{}' paragraph count mismatch! EN has {}, PT has {}",
                p.id,
                en_paras.len(),
                pt_paras.len()
            );
        }
    }
}

