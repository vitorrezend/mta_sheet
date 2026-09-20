//! Modelos de Dados para o Compêndio de Antecedentes (Backgrounds) de M20.
//!
//! Capítulo 6: Criação do Personagem (pp. 301-311).

use serde::Serialize;
use crate::i18n::Language;

/// Nível de pontuação de um Antecedente (de 0/X a 5, ou até 10 para os aplicáveis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BackgroundRating {
    pub dots: i32,
    pub description: &'static str,
    pub description_pt: &'static str,
}

impl BackgroundRating {
    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }
}

/// Amostra de equipe de suporte para o Antecedente Reforço (Backup).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BackupTeamSample {
    pub archetype: &'static str,
    pub archetype_pt: &'static str,
    pub members: &'static str,
    pub members_pt: &'static str,
}

impl BackupTeamSample {
    pub fn archetype(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.archetype_pt,
            Language::EnUs => self.archetype,
        }
    }

    pub fn members(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.members_pt,
            Language::EnUs => self.members,
        }
    }
}

/// Piscina de Pontos de Construção para Capelas / Constructos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ChantryPoolLevel {
    pub pool_range: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
}

impl ChantryPoolLevel {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }
}

/// Tabela de Dificuldade de Requisição conforme a Lealdade com os Superiores (M20, p. 316).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct RequisitionDifficulty {
    pub relationship: &'static str,
    pub relationship_pt: &'static str,
    pub difficulty: i32,
}

impl RequisitionDifficulty {
    pub fn relationship(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.relationship_pt,
            Language::EnUs => self.relationship,
        }
    }
}

/// Definição Canônica de um Antecedente M20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BackgroundDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub technocracy_name: Option<&'static str>,
    pub technocracy_name_pt: Option<&'static str>,
    pub page_ref: &'static str,
    pub max_dots: i32,
    pub description: &'static str,
    pub description_pt: &'static str,
    pub system: &'static str,
    pub system_pt: &'static str,
    pub ratings: &'static [BackgroundRating],
    pub backup_teams: Option<&'static [BackupTeamSample]>,
    pub chantry_pools: Option<&'static [ChantryPoolLevel]>,
    pub requisitions_chart: Option<&'static [RequisitionDifficulty]>,
}

impl BackgroundDefinition {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    pub fn secondary_name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name,
            Language::EnUs => self.name_pt,
        }
    }

    pub fn technocracy_name(&self, lang: Language) -> Option<&'static str> {
        match lang {
            Language::PtBr => self.technocracy_name_pt.or(self.technocracy_name),
            Language::EnUs => self.technocracy_name,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn system(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.system_pt,
            Language::EnUs => self.system,
        }
    }
}

/// Artigo Teórico de Regras Gerais de Antecedentes (M20, pp. 301-303).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BackgroundTheoryArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub page_ref: &'static str,
    pub content: &'static str,
    pub content_pt: &'static str,
}

impl BackgroundTheoryArticle {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn content(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.content_pt,
            Language::EnUs => self.content,
        }
    }
}
