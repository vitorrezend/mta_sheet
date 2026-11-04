//! Textos Integrais M20 do Apêndice II: Qualidades, Defeitos, Perturbações e quadros.
//!
//! M20 pp. 642-651 e Book of Secrets pp. 34-96. Textos carregados de data/compendium/merits_flaws/**/*.en.md e *.pt.md.

use super::data::{BOS_FLAWS, BOS_MERITS};
use super::autolink::autolink_static;
use super::models::MeritFlawArticle;
use crate::i18n::Language;

/// Quadros e regras gerais do Apêndice II e Book of Secrets, na ordem do livro.
pub const MERIT_FLAW_ARTICLES: &[MeritFlawArticle] = &[
    MeritFlawArticle {
        id: "article_merits_flaws",
        title: "Merits and Flaws",
        title_pt: "Qualidades e Defeitos",
        page_ref: "M20, pp. 642-643",
        content: include_str!("../../../../../data/compendium/merits_flaws/articles/article_merits_flaws.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/articles/article_merits_flaws.pt.md"),
    },
    MeritFlawArticle {
        id: "article_powers_of_true_faith",
        title: "Optional Rule: Powers of True Faith",
        title_pt: "Regra Opcional: Poderes da Fé Verdadeira",
        page_ref: "M20, p. 645",
        content: include_str!("../../../../../data/compendium/merits_flaws/articles/article_powers_of_true_faith.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/articles/article_powers_of_true_faith.pt.md"),
    },
    MeritFlawArticle {
        id: "article_genetic_flaws",
        title: "Genetic Flaws",
        title_pt: "Defeitos Genéticos",
        page_ref: "M20, pp. 648-651",
        content: include_str!("../../../../../data/compendium/merits_flaws/articles/article_genetic_flaws.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/articles/article_genetic_flaws.pt.md"),
    },
    MeritFlawArticle {
        id: "article_derangements",
        title: "Derangements",
        title_pt: "Perturbações",
        page_ref: "M20, pp. 649-650",
        content: include_str!("../../../../../data/compendium/merits_flaws/articles/article_derangements.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/articles/article_derangements.pt.md"),
    },
    // Book of Secrets, Capítulo Um: quadros e regras de Merits and Flaws
    MeritFlawArticle {
        id: "article_bos_merits_flaws",
        title: "Merits and Flaws (Book of Secrets)",
        title_pt: "Qualidades e Defeitos (Book of Secrets)",
        page_ref: "BoS, pp. 34-81",
        content: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_merits_flaws.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_merits_flaws.pt.md"),
    },
    MeritFlawArticle {
        id: "article_bos_higher_limits_on_flaws",
        title: "Optional Rule: Higher Limits on Flaws",
        title_pt: "Regra Opcional: Limites Mais Altos para Defeitos",
        page_ref: "BoS, p. 35",
        content: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_higher_limits_on_flaws.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_higher_limits_on_flaws.pt.md"),
    },
    MeritFlawArticle {
        id: "article_bos_adversarial_backgrounds",
        title: "Adversarial Backgrounds",
        title_pt: "Antecedentes Adversariais",
        page_ref: "BoS, p. 35",
        content: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_adversarial_backgrounds.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_adversarial_backgrounds.pt.md"),
    },
    MeritFlawArticle {
        id: "article_bos_time_clarity_revisions",
        title: "Time, Clarity, and Revisions",
        title_pt: "Tempo, Clareza e Revisões",
        page_ref: "BoS, p. 35",
        content: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_time_clarity_revisions.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_time_clarity_revisions.pt.md"),
    },
    MeritFlawArticle {
        id: "article_bos_neurodiversity",
        title: "Neurodiversity and Mental Traits",
        title_pt: "Neurodiversidade e Características Mentais",
        page_ref: "BoS, p. 42",
        content: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_neurodiversity.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_neurodiversity.pt.md"),
    },
    MeritFlawArticle {
        id: "article_bos_blood_bond",
        title: "Mages, the Blood Bond, and the Effects of Vitae",
        title_pt: "Magos, o Laço de Sangue e os Efeitos da Vitae",
        page_ref: "BoS, p. 76",
        content: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_blood_bond.en.md"),
        content_pt: include_str!("../../../../../data/compendium/merits_flaws/bos/articles/article_bos_blood_bond.pt.md"),
    },
];

/// Busca um quadro do Apêndice II por ID.
pub fn find_merit_flaw_article(id: &str) -> Option<&'static MeritFlawArticle> {
    MERIT_FLAW_ARTICLES.iter().find(|a| a.id == id)
}

/// Texto integral de uma Qualidade ou Defeito, se disponível no idioma.
pub fn find_merit_flaw_unabridged_text(id: &str, lang: Language) -> Option<&'static str> {
    let base = id.strip_suffix("_flaw").unwrap_or(id);
    let paired = format!("{base}_flaw");
    // O verbete (e a outra metade do par Qualidade/Defeito) não aponta para si mesmo
    raw_merit_flaw_text(id, lang).map(|t| autolink_static(t, &[id, base, paired.as_str()]))
}

/// Conteúdo de um quadro do Apêndice II ou do Book of Secrets, com links entre verbetes.
pub fn merit_flaw_article_content(article: &MeritFlawArticle, lang: Language) -> &'static str {
    autolink_static(article.content(lang), &[])
}

fn raw_merit_flaw_text(id: &str, lang: Language) -> Option<&'static str> {
    match lang {
        Language::EnUs => match id {
            "acute_senses" => Some(include_str!("../../../../../data/compendium/merits_flaws/acute_senses.en.md")),
            "language" => Some(include_str!("../../../../../data/compendium/merits_flaws/language.en.md")),
            "dark_triad" => Some(include_str!("../../../../../data/compendium/merits_flaws/dark_triad.en.md")),
            "stormwarden" => Some(include_str!("../../../../../data/compendium/merits_flaws/stormwarden.en.md")),
            "ties" => Some(include_str!("../../../../../data/compendium/merits_flaws/ties.en.md")),
            "berserker" => Some(include_str!("../../../../../data/compendium/merits_flaws/berserker.en.md")),
            "umbral_affinity" => Some(include_str!("../../../../../data/compendium/merits_flaws/umbral_affinity.en.md")),
            "too_tough_to_die" => Some(include_str!("../../../../../data/compendium/merits_flaws/too_tough_to_die.en.md")),
            "true_faith" => Some(include_str!("../../../../../data/compendium/merits_flaws/true_faith.en.md")),
            "addiction" => Some(include_str!("../../../../../data/compendium/merits_flaws/addiction.en.md")),
            "construct" => Some(include_str!("../../../../../data/compendium/merits_flaws/construct.en.md")),
            "cursed" => Some(include_str!("../../../../../data/compendium/merits_flaws/cursed.en.md")),
            "deranged" => Some(include_str!("../../../../../data/compendium/merits_flaws/deranged.en.md")),
            "echoes" => Some(include_str!("../../../../../data/compendium/merits_flaws/echoes.en.md")),
            "enemy" => Some(include_str!("../../../../../data/compendium/merits_flaws/enemy.en.md")),
            "ptsd" => Some(include_str!("../../../../../data/compendium/merits_flaws/ptsd.en.md")),
            "stress_atavism" => Some(include_str!("../../../../../data/compendium/merits_flaws/berserker.en.md")),
            _ => bos_text(id, Language::EnUs),
        },
        Language::PtBr => match id {
            "acute_senses" => Some(include_str!("../../../../../data/compendium/merits_flaws/acute_senses.pt.md")),
            "language" => Some(include_str!("../../../../../data/compendium/merits_flaws/language.pt.md")),
            "dark_triad" => Some(include_str!("../../../../../data/compendium/merits_flaws/dark_triad.pt.md")),
            "stormwarden" => Some(include_str!("../../../../../data/compendium/merits_flaws/stormwarden.pt.md")),
            "ties" => Some(include_str!("../../../../../data/compendium/merits_flaws/ties.pt.md")),
            "berserker" => Some(include_str!("../../../../../data/compendium/merits_flaws/berserker.pt.md")),
            "umbral_affinity" => Some(include_str!("../../../../../data/compendium/merits_flaws/umbral_affinity.pt.md")),
            "too_tough_to_die" => Some(include_str!("../../../../../data/compendium/merits_flaws/too_tough_to_die.pt.md")),
            "true_faith" => Some(include_str!("../../../../../data/compendium/merits_flaws/true_faith.pt.md")),
            "addiction" => Some(include_str!("../../../../../data/compendium/merits_flaws/addiction.pt.md")),
            "construct" => Some(include_str!("../../../../../data/compendium/merits_flaws/construct.pt.md")),
            "cursed" => Some(include_str!("../../../../../data/compendium/merits_flaws/cursed.pt.md")),
            "deranged" => Some(include_str!("../../../../../data/compendium/merits_flaws/deranged.pt.md")),
            "echoes" => Some(include_str!("../../../../../data/compendium/merits_flaws/echoes.pt.md")),
            "enemy" => Some(include_str!("../../../../../data/compendium/merits_flaws/enemy.pt.md")),
            "ptsd" => Some(include_str!("../../../../../data/compendium/merits_flaws/ptsd.pt.md")),
            "stress_atavism" => Some(include_str!("../../../../../data/compendium/merits_flaws/berserker.pt.md")),
            _ => bos_text(id, Language::PtBr),
        },
    }
}

/// Texto do Book of Secrets.
fn bos_text(id: &str, lang: Language) -> Option<&'static str> {
    BOS_MERITS.iter().chain(BOS_FLAWS.iter()).find(|mf| mf.id == id).map(|mf| mf.description(lang))
}

/// Texto integral de uma Perturbação, se disponível no idioma.
pub fn find_derangement_unabridged_text(id: &str, lang: Language) -> Option<&'static str> {
    raw_derangement_text(id, lang).map(|t| autolink_static(t, &[id]))
}

fn raw_derangement_text(id: &str, lang: Language) -> Option<&'static str> {
    match lang {
        Language::EnUs => match id {
            "amnesia" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/amnesia.en.md")),
            "aspd" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/aspd.en.md")),
            "bad" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/bad.en.md")),
            "dementia" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/dementia.en.md")),
            "fugue" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/fugue.en.md")),
            "megalomania" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/megalomania.en.md")),
            "mpd" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/mpd.en.md")),
            "ocd" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/ocd.en.md")),
            "paranoia" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/paranoia.en.md")),
            "schizophrenia" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/schizophrenia.en.md")),
            _ => None,
        },
        Language::PtBr => match id {
            "amnesia" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/amnesia.pt.md")),
            "aspd" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/aspd.pt.md")),
            "bad" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/bad.pt.md")),
            "dementia" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/dementia.pt.md")),
            "fugue" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/fugue.pt.md")),
            "megalomania" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/megalomania.pt.md")),
            "mpd" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/mpd.pt.md")),
            "ocd" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/ocd.pt.md")),
            "paranoia" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/paranoia.pt.md")),
            "schizophrenia" => Some(include_str!("../../../../../data/compendium/merits_flaws/derangements/schizophrenia.pt.md")),
            _ => None,
        },
    }
}
