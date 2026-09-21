//! Resolvedor e Sincronizador de Consultas Iniciais do Compêndio M20.

use crate::compendium::archetypes::find_archetype;
use crate::compendium::attributes::find_attribute;
use crate::compendium::backgrounds::find_background;
use crate::compendium::instruments::{find_instrument, find_theory_article};
use crate::compendium::practices::find_practice;
use crate::compendium::spheres::find_sphere;
use crate::compendium::weapons::{find_combat_entity, CombatEntity, CombatSubTab};
use crate::compendium::abilities::{find_ability, find_ability_theory_rule};
use crate::compendium::merits_flaws::{find_derangement, find_merit_flaw};
use crate::components::compendium::CompendiumSection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCompendiumLocation {
    pub section: CompendiumSection,
    pub item_id: String,
    pub combat_subtab: Option<CombatSubTab>,
}

/// Tenta resolver uma query textual para uma seção e ID de verbete do Compêndio M20.
pub fn resolve_compendium_query(
    preferred_section: CompendiumSection,
    query: &str,
) -> Option<ResolvedCompendiumLocation> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Tenta primeiro casar com a seção indicada como preferida
    if let Some(loc) = match_section(preferred_section, trimmed) {
        return Some(loc);
    }

    // Se não encontrou na seção indicada, tenta localizar em qualquer outra seção
    const ALL_SECTIONS: &[CompendiumSection] = &[
        CompendiumSection::Abilities,
        CompendiumSection::MeritsFlaws,
        CompendiumSection::Backgrounds,
        CompendiumSection::Spheres,
        CompendiumSection::Weapons,
        CompendiumSection::Practices,
        CompendiumSection::Instruments,
        CompendiumSection::Attributes,
        CompendiumSection::Archetypes,
    ];

    for &sec in ALL_SECTIONS {
        if sec != preferred_section {
            if let Some(loc) = match_section(sec, trimmed) {
                return Some(loc);
            }
        }
    }

    None
}

fn match_section(section: CompendiumSection, query: &str) -> Option<ResolvedCompendiumLocation> {
    match section {
        CompendiumSection::Abilities => {
            if let Some(ab) = find_ability(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Abilities,
                    item_id: ab.id.to_string(),
                    combat_subtab: None,
                })
            } else if let Some(rule) = find_ability_theory_rule(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Abilities,
                    item_id: rule.id.to_string(),
                    combat_subtab: None,
                })
            } else {
                None
            }
        }
        CompendiumSection::MeritsFlaws => {
            if let Some(mf) = find_merit_flaw(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::MeritsFlaws,
                    item_id: mf.id.to_string(),
                    combat_subtab: None,
                })
            } else if let Some(d) = find_derangement(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::MeritsFlaws,
                    item_id: d.id.to_string(),
                    combat_subtab: None,
                })
            } else {
                None
            }
        }
        CompendiumSection::Weapons => {
            if let Some(entity) = find_combat_entity(query) {
                match entity {
                    CombatEntity::Weapon(w) => Some(ResolvedCompendiumLocation {
                        section: CompendiumSection::Weapons,
                        item_id: w.id.to_string(),
                        combat_subtab: Some(CombatSubTab::Weapons),
                    }),
                    CombatEntity::Maneuver(m) => Some(ResolvedCompendiumLocation {
                        section: CompendiumSection::Weapons,
                        item_id: m.id.to_string(),
                        combat_subtab: Some(CombatSubTab::Maneuvers),
                    }),
                }
            } else {
                None
            }
        }
        CompendiumSection::Attributes => {
            if let Some(matched_attr) = find_attribute(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Attributes,
                    item_id: matched_attr.id.to_string(),
                    combat_subtab: None,
                })
            } else if query.eq_ignore_ascii_case("rule")
                || query.eq_ignore_ascii_case("regras")
                || query.eq_ignore_ascii_case("especialidades")
                || query.eq_ignore_ascii_case("specialties")
            {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Attributes,
                    item_id: "rule_specialties".to_string(),
                    combat_subtab: None,
                })
            } else {
                None
            }
        }
        CompendiumSection::Archetypes => {
            if let Some(matched_a) = find_archetype(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Archetypes,
                    item_id: matched_a.id.to_string(),
                    combat_subtab: None,
                })
            } else if query.eq_ignore_ascii_case("theory")
                || query.eq_ignore_ascii_case("regras")
                || query.eq_ignore_ascii_case("natureza")
                || query.eq_ignore_ascii_case("comportamento")
            {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Archetypes,
                    item_id: "theory_nature_demeanor".to_string(),
                    combat_subtab: None,
                })
            } else {
                None
            }
        }
        CompendiumSection::Instruments => {
            if let Some(matched_inst) = find_instrument(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Instruments,
                    item_id: matched_inst.id.to_string(),
                    combat_subtab: None,
                })
            } else if let Some(matched_art) = find_theory_article(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Instruments,
                    item_id: matched_art.id.to_string(),
                    combat_subtab: None,
                })
            } else {
                None
            }
        }
        CompendiumSection::Practices => {
            if let Some(matched_p) = find_practice(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Practices,
                    item_id: matched_p.id.to_string(),
                    combat_subtab: None,
                })
            } else {
                None
            }
        }
        CompendiumSection::Backgrounds => {
            if let Some(matched_bg) = find_background(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Backgrounds,
                    item_id: matched_bg.id.to_string(),
                    combat_subtab: None,
                })
            } else if query.eq_ignore_ascii_case("theory")
                || query.eq_ignore_ascii_case("regras")
                || query.eq_ignore_ascii_case("cabala")
            {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Backgrounds,
                    item_id: "theory_background_rules".to_string(),
                    combat_subtab: None,
                })
            } else {
                None
            }
        }
        CompendiumSection::Spheres => {
            if let Some(matched_s) = find_sphere(query) {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Spheres,
                    item_id: matched_s.id.to_string(),
                    combat_subtab: None,
                })
            } else if matches!(query.to_ascii_lowercase().as_str(), "theory" | "regras" | "rules" | "metafisica" | "metaphysics") {
                Some(ResolvedCompendiumLocation {
                    section: CompendiumSection::Spheres,
                    item_id: "theory_sphere_rules".to_string(),
                    combat_subtab: None,
                })
            } else {
                None
            }
        }
    }
}
