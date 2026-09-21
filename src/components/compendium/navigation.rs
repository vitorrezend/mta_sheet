//! Sistema de Navegação Interna e Hyperlinks do Compêndio M20.
//!
//! Fornece o protocolo `mta://` para vinculação cruzada entre verbetes,
//! histórico de navegação ("⬅ Voltar") e parsing de destinos.

use crate::compendium::archetypes::find_archetype;
use crate::compendium::attributes::find_attribute;
use crate::compendium::backgrounds::find_background;
use crate::compendium::instruments::{find_instrument, find_theory_article};
use crate::compendium::practices::find_practice;
use crate::compendium::spheres::find_sphere;
use crate::compendium::weapons::{find_combat_entity, CombatEntity, CombatSubTab};
use crate::components::compendium::CompendiumSection;
use crate::i18n::Language;

use crate::compendium::abilities::{find_ability, find_ability_theory_rule};
use crate::compendium::merits_flaws::{find_derangement, find_merit_flaw};

/// Destino de navegação dentro do Compêndio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompendiumTarget {
    Background(String),
    Sphere(String),
    Weapon(String),
    Maneuver(String),
    Practice(String),
    Instrument(String),
    Attribute(String),
    Archetype(String),
    Ability(String),
    MeritFlaw(String),
    Derangement(String),
}

impl CompendiumTarget {
    /// Faz o parse de uma URI no formato `mta://<secao>/<id>` ou `mta://weapons/<subtab>/<id>`.
    ///
    /// Exemplos válidos:
    /// - `mta://backgrounds/allies`
    /// - `mta://backgrounds/theory_background_rules`
    /// - `mta://weapons/maneuvers/do` ou `mta://weapons/maneuver/do`
    /// - `mta://weapons/weapons/katana` ou `mta://weapons/katana`
    /// - `mta://practices/alchemy`
    /// - `mta://instruments/tools_of_focus`
    /// - `mta://attributes/strength`
    /// - `mta://attributes/rule_specialties`
    /// - `mta://archetypes/activist`
    pub fn from_uri(uri: &str) -> Option<Self> {
        let path = uri.strip_prefix("mta://")?;
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "backgrounds" | "background" | "antecedentes" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::Background(id.to_string()))
            }
            "spheres" | "sphere" | "esferas" | "esfera" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::Sphere(id.to_string()))
            }
            "weapons" | "armas" | "combat" | "combate" => {
                if parts.len() >= 3 {
                    match parts[1] {
                        "maneuvers" | "maneuver" | "manobras" | "manobra" => {
                            Some(CompendiumTarget::Maneuver(parts[2].to_string()))
                        }
                        "weapons" | "weapon" | "armas" | "arma" => {
                            Some(CompendiumTarget::Weapon(parts[2].to_string()))
                        }
                        _ => {
                            // Tenta encontrar a entidade pelo ID
                            if let Some(entity) = find_combat_entity(parts[2]) {
                                match entity {
                                    CombatEntity::Weapon(_) => Some(CompendiumTarget::Weapon(parts[2].to_string())),
                                    CombatEntity::Maneuver(_) => Some(CompendiumTarget::Maneuver(parts[2].to_string())),
                                }
                            } else {
                                Some(CompendiumTarget::Weapon(parts[2].to_string()))
                            }
                        }
                    }
                } else if parts.len() == 2 {
                    // mta://weapons/<id> -> descobre se é arma ou manobra
                    let id = parts[1];
                    if let Some(entity) = find_combat_entity(id) {
                        match entity {
                            CombatEntity::Weapon(_) => Some(CompendiumTarget::Weapon(id.to_string())),
                            CombatEntity::Maneuver(_) => Some(CompendiumTarget::Maneuver(id.to_string())),
                        }
                    } else {
                        Some(CompendiumTarget::Weapon(id.to_string()))
                    }
                } else {
                    None
                }
            }
            "maneuvers" | "maneuver" | "manobras" | "manobra" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::Maneuver(id.to_string()))
            }
            "practices" | "practice" | "praticas" | "pratica" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::Practice(id.to_string()))
            }
            "instruments" | "instrument" | "instrumentos" | "instrumento" | "focus" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::Instrument(id.to_string()))
            }
            "attributes" | "attribute" | "atributos" | "atributo" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::Attribute(id.to_string()))
            }
            "archetypes" | "archetype" | "arquetipos" | "arquetipo" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::Archetype(id.to_string()))
            }
            "abilities" | "ability" | "habilidades" | "habilidade" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::Ability(id.to_string()))
            }
            "merits_flaws" | "merits" | "flaws" | "qualidades_defeitos" | "qualidades" | "defeitos" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::MeritFlaw(id.to_string()))
            }
            "derangements" | "derangement" | "perturbacoes" | "perturbacao" => {
                let id = parts.get(1)?;
                Some(CompendiumTarget::Derangement(id.to_string()))
            }
            _ => None,
        }
    }

    /// Retorna a seção do compêndio correspondente a este destino.
    pub fn section(&self) -> CompendiumSection {
        match self {
            CompendiumTarget::Background(_) => CompendiumSection::Backgrounds,
            CompendiumTarget::Sphere(_) => CompendiumSection::Spheres,
            CompendiumTarget::Weapon(_) | CompendiumTarget::Maneuver(_) => CompendiumSection::Weapons,
            CompendiumTarget::Practice(_) => CompendiumSection::Practices,
            CompendiumTarget::Instrument(_) => CompendiumSection::Instruments,
            CompendiumTarget::Attribute(_) => CompendiumSection::Attributes,
            CompendiumTarget::Archetype(_) => CompendiumSection::Archetypes,
            CompendiumTarget::Ability(_) => CompendiumSection::Abilities,
            CompendiumTarget::MeritFlaw(_) | CompendiumTarget::Derangement(_) => CompendiumSection::MeritsFlaws,
        }
    }

    /// Retorna o ID do item.
    pub fn item_id(&self) -> &str {
        match self {
            CompendiumTarget::Background(id) => id,
            CompendiumTarget::Sphere(id) => id,
            CompendiumTarget::Weapon(id) => id,
            CompendiumTarget::Maneuver(id) => id,
            CompendiumTarget::Practice(id) => id,
            CompendiumTarget::Instrument(id) => id,
            CompendiumTarget::Attribute(id) => id,
            CompendiumTarget::Archetype(id) => id,
            CompendiumTarget::Ability(id) => id,
            CompendiumTarget::MeritFlaw(id) => id,
            CompendiumTarget::Derangement(id) => id,
        }
    }

    /// Retorna a sub-aba de combate, caso aplicável.
    pub fn combat_subtab(&self) -> Option<CombatSubTab> {
        match self {
            CompendiumTarget::Weapon(_) => Some(CombatSubTab::Weapons),
            CompendiumTarget::Maneuver(_) => Some(CombatSubTab::Maneuvers),
            _ => None,
        }
    }

    /// Retorna um rótulo legível para o item no idioma especificado.
    pub fn label(&self, lang: Language) -> String {
        match self {
            CompendiumTarget::Background(id) => {
                if id == "theory_background_rules" {
                    match lang {
                        Language::PtBr => "Regras de Antecedentes".to_string(),
                        Language::EnUs => "Background Rules".to_string(),
                    }
                } else if let Some(bg) = find_background(id) {
                    bg.name(lang).to_string()
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::Sphere(id) => {
                if id == "theory_sphere_rules" {
                    match lang {
                        Language::PtBr => "Regras Gerais das Esferas".to_string(),
                        Language::EnUs => "General Sphere Rules".to_string(),
                    }
                } else if let Some(s) = find_sphere(id) {
                    s.name(lang).to_string()
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::Weapon(id) => {
                if let Some(entity) = find_combat_entity(id) {
                    match entity {
                        CombatEntity::Weapon(w) => w.name(lang).to_string(),
                        CombatEntity::Maneuver(m) => m.name(lang).to_string(),
                    }
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::Maneuver(id) => {
                if let Some(entity) = find_combat_entity(id) {
                    match entity {
                        CombatEntity::Weapon(w) => w.name(lang).to_string(),
                        CombatEntity::Maneuver(m) => m.name(lang).to_string(),
                    }
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::Practice(id) => {
                if let Some(p) = find_practice(id) {
                    p.name(lang).to_string()
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::Instrument(id) => {
                if let Some(inst) = find_instrument(id) {
                    inst.name(lang).to_string()
                } else if let Some(art) = find_theory_article(id) {
                    art.title(lang).to_string()
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::Attribute(id) => {
                if id == "rule_specialties" {
                    match lang {
                        Language::PtBr => "Regras de Especialidades".to_string(),
                        Language::EnUs => "Specialties Rules".to_string(),
                    }
                } else if let Some(attr) = find_attribute(id) {
                    attr.name(lang).to_string()
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::Archetype(id) => {
                if id == "theory_nature_demeanor" {
                    match lang {
                        Language::PtBr => "Teoria de Arquétipos".to_string(),
                        Language::EnUs => "Archetype Theory".to_string(),
                    }
                } else if let Some(arch) = find_archetype(id) {
                    arch.name(lang).to_string()
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::Ability(id) => {
                if let Some(art) = find_ability_theory_rule(id) {
                    art.title(lang).to_string()
                } else if let Some(ab) = find_ability(id) {
                    ab.name(lang).to_string()
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::MeritFlaw(id) => {
                if let Some(mf) = find_merit_flaw(id) {
                    mf.name(lang).to_string()
                } else {
                    id.clone()
                }
            }
            CompendiumTarget::Derangement(id) => {
                if let Some(d) = find_derangement(id) {
                    d.name(lang).to_string()
                } else {
                    id.clone()
                }
            }
        }
    }
}

/// Registro no histórico de navegação do compêndio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompendiumHistoryEntry {
    pub section: CompendiumSection,
    pub item_id: String,
    pub combat_subtab: Option<CombatSubTab>,
    pub label: String,
}
