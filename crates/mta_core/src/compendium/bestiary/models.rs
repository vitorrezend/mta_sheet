//! Tipos do Apêndice I (Allies and Antagonists): verbetes e leitura dos números de cada ficha.

use serde::Serialize;
use crate::i18n::Language;

/// Grupo do apêndice, na ordem do livro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BestiaryGroup {
    Intro,
    Animals,
    Masses,
    Technocracy,
    Awakened,
    Enemies,
    Spirits,
    Totems,
    Elementals,
    Minions,
    Paradox,
}

impl BestiaryGroup {
    pub const ALL: [BestiaryGroup; 11] = [
        BestiaryGroup::Intro,
        BestiaryGroup::Animals,
        BestiaryGroup::Masses,
        BestiaryGroup::Technocracy,
        BestiaryGroup::Awakened,
        BestiaryGroup::Enemies,
        BestiaryGroup::Spirits,
        BestiaryGroup::Totems,
        BestiaryGroup::Elementals,
        BestiaryGroup::Minions,
        BestiaryGroup::Paradox,
    ];

    pub fn label(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (BestiaryGroup::Intro, Language::EnUs) => "Supporting Cast",
            (BestiaryGroup::Intro, Language::PtBr) => "Elenco de Apoio",
            (BestiaryGroup::Animals, Language::EnUs) => "Bestiary",
            (BestiaryGroup::Animals, Language::PtBr) => "Bestiário",
            (BestiaryGroup::Masses, Language::EnUs) => "Among the Masses",
            (BestiaryGroup::Masses, Language::PtBr) => "Entre as Massas",
            (BestiaryGroup::Technocracy, Language::EnUs) => "Extraordinary Operatives & Technocratic Creations",
            (BestiaryGroup::Technocracy, Language::PtBr) => "Operativos Extraordinários & Criações Tecnocráticas",
            (BestiaryGroup::Awakened, Language::EnUs) => "Our Awakened Brethren",
            (BestiaryGroup::Awakened, Language::PtBr) => "Nossos Irmãos Despertos",
            (BestiaryGroup::Enemies, Language::EnUs) => "Awakened Enemies",
            (BestiaryGroup::Enemies, Language::PtBr) => "Inimigos Despertos",
            (BestiaryGroup::Spirits, Language::EnUs) => "Spirits",
            (BestiaryGroup::Spirits, Language::PtBr) => "Espíritos",
            (BestiaryGroup::Totems, Language::EnUs) => "Totem Spirits",
            (BestiaryGroup::Totems, Language::PtBr) => "Espíritos Totêmicos",
            (BestiaryGroup::Elementals, Language::EnUs) => "Elementals",
            (BestiaryGroup::Elementals, Language::PtBr) => "Elementais",
            (BestiaryGroup::Minions, Language::EnUs) => "Minions",
            (BestiaryGroup::Minions, Language::PtBr) => "Lacaios",
            (BestiaryGroup::Paradox, Language::EnUs) => "Paradox Manifestations",
            (BestiaryGroup::Paradox, Language::PtBr) => "Manifestações de Paradoxo",
        }
    }
}

/// Um verbete do Apêndice I: uma ficha pronta ou um texto de introdução/quadro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BestiaryEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub group: BestiaryGroup,
    pub is_article: bool,
    pub page_ref: &'static str,
    /// Texto integral em inglês; a ficha usa linhas `- **Rótulo:** valor`.
    pub content: &'static str,
    /// Texto integral traduzido para português (PT-BR).
    pub content_pt: &'static str,
}

/// Resumo numérico de uma ficha, lido do texto do livro.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct QuickStats {
    pub willpower: Option<i32>,
    pub health_levels: Vec<String>,
    pub arete: Option<i32>,
    pub rage: Option<i32>,
    pub gnosis: Option<i32>,
    pub essence: Option<i32>,
    /// (Atributo, valor) com o menor valor quando o livro dá uma faixa ("3-4" → 3).
    pub attributes: Vec<(String, i32)>,
}

impl QuickStats {
    pub fn attribute(&self, name: &str) -> Option<i32> {
        self.attributes.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|(_, v)| *v)
    }

    /// Base de iniciativa (Destreza + Raciocínio), quando a ficha tem os dois.
    pub fn initiative_base(&self) -> Option<i32> {
        Some(self.attribute("Dexterity")? + self.attribute("Wits")?)
    }
}

fn first_int(s: &str) -> Option<i32> {
    let digits: String = s
        .trim_start_matches(|c: char| !c.is_ascii_digit())
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().ok()
}

fn is_attribute_label(label: &str) -> bool {
    label.ends_with("Attributes") || label.starts_with("Suggested Attributes") || label.starts_with("Materialized Attributes")
}

/// Lê "Strength 3-4, Dexterity 3; use Gnosis..." como (Atributo, mínimo, máximo).
fn parse_attribute_ranges(value: &str) -> Vec<(String, i32, i32)> {
    let mut out: Vec<(String, i32, i32)> = Vec::new();
    for part in value.split([',', ';']) {
        let part = part.trim().replace(':', "");
        let mut words = part.split_whitespace();
        let (Some(name), Some(val)) = (words.next(), words.next()) else { continue };
        let Some(lo) = first_int(val) else { continue };
        let hi = val.split_once('-').and_then(|(_, h)| first_int(h)).unwrap_or(lo).max(lo);
        if !out.iter().any(|(n, _, _)| n == name) {
            out.push((name.to_string(), lo, hi));
        }
    }
    out
}

impl BestiaryEntry {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    /// Retorna o texto integral no idioma selecionado.
    pub fn content(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.content_pt,
            Language::EnUs => self.content,
        }
    }

    /// Indica se o verbete possui tradução em português.
    pub fn has_pt_text(&self) -> bool {
        !self.content_pt.is_empty()
    }

    /// Linhas `- **Rótulo:** valor` da ficha em inglês, na ordem do livro.
    pub fn stat_lines(&self) -> Vec<(&'static str, &'static str)> {
        self.content
            .lines()
            .filter_map(|l| {
                let rest = l.trim().strip_prefix("- **")?;
                let (label, value) = rest.split_once(":** ")?;
                Some((label, value.trim()))
            })
            .collect()
    }

    /// Linhas `- **Rótulo:** valor` da ficha em português.
    pub fn stat_lines_pt(&self) -> Vec<(&'static str, &'static str)> {
        self.content_pt
            .lines()
            .filter_map(|l| {
                let rest = l.trim().strip_prefix("- **")?;
                let (label, value) = rest.split_once(":** ")?;
                Some((label, value.trim()))
            })
            .collect()
    }

    /// Atributos com a faixa que o livro sugere ("Strength 3-4" → ("Strength", 3, 4)).
    pub fn attribute_ranges(&self) -> Vec<(String, i32, i32)> {
        let mut out: Vec<(String, i32, i32)> = Vec::new();
        for (label, value) in self.stat_lines() {
            if is_attribute_label(label) {
                for r in parse_attribute_ranges(value) {
                    if !out.iter().any(|(n, _, _)| *n == r.0) {
                        out.push(r);
                    }
                }
            }
        }
        out
    }

    pub fn quick_stats(&self) -> QuickStats {
        let mut q = QuickStats::default();
        for (label, value) in self.stat_lines() {
            match label {
                "Willpower" => q.willpower = q.willpower.or(first_int(value)),
                "Arete" | "Suggested Arete" => q.arete = first_int(value),
                l if l.starts_with("Health Levels") || l == "Materialized Health Levels" => {
                    q.health_levels = if value.contains(',') {
                        value.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
                    } else {
                        let n = first_int(value).unwrap_or(0).max(0) as usize;
                        vec!["OK".to_string(); n]
                    };
                }
                l if is_attribute_label(l) => {
                    for (name, lo, _) in parse_attribute_ranges(value) {
                        if q.attribute(&name).is_none() {
                            q.attributes.push((name, lo));
                        }
                    }
                }
                "Traits" => {
                    for part in value.split(',') {
                        let part = part.trim();
                        if let Some(v) = part.strip_prefix("Willpower ") {
                            q.willpower = first_int(v);
                        } else if let Some(v) = part.strip_prefix("Rage ") {
                            q.rage = first_int(v);
                        } else if let Some(v) = part.strip_prefix("Gnosis ") {
                            q.gnosis = first_int(v);
                        } else if let Some(v) = part.strip_prefix("Essence ") {
                            q.essence = first_int(v);
                        }
                    }
                }
                _ => {}
            }
        }
        q
    }
}
