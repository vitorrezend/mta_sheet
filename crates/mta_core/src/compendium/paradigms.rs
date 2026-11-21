//! Paradigmas Expandidos do M20 Book of Secrets, Capítulo Três: Matters of Focus
//! (BoS, pp. 188-196).
//!
//! Texto integral em inglês em data/compendium/paradigms/*.en.md e tradução integral
//! em *.pt.md. Cada paradigma traz as Práticas Associadas sugeridas pelo livro.

use serde::Serialize;
use crate::i18n::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ParadigmDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub page_ref: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
    pub associated_practices: &'static [&'static str],
    pub associated_practices_pt: &'static [&'static str],
}

impl ParadigmDefinition {
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

    pub fn associated_practices(&self, lang: Language) -> &'static [&'static str] {
        match lang {
            Language::PtBr => self.associated_practices_pt,
            Language::EnUs => self.associated_practices,
        }
    }
}

/// Artigo introdutório de uma seção do compêndio (texto corrido, sem estatísticas).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ParadigmArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub page_ref: &'static str,
    pub content: &'static str,
    pub content_pt: &'static str,
}

impl ParadigmArticle {
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

/// Introdução "Expanded Paradigms" (BoS, p. 188).
pub const ARTICLE_EXPANDED_PARADIGMS: ParadigmArticle = ParadigmArticle {
    id: "article_expanded_paradigms",
    title: "Expanded Paradigms",
    title_pt: "Paradigmas Expandidos",
    page_ref: "BoS, p. 188",
    content: include_str!("../../../../data/compendium/paradigms/expanded_paradigms.en.md"),
    content_pt: include_str!("../../../../data/compendium/paradigms/expanded_paradigms.pt.md"),
};

/// Os 13 paradigmas do Book of Secrets, na ordem alfabética do sumário do capítulo.
pub const ALL_PARADIGMS: &[ParadigmDefinition] = &[
    ParadigmDefinition {
        id: "aliens_make_us_what_we_are",
        name: "Aliens Make Us What We Are",
        name_pt: "Os Alienígenas Nos Fazem o Que Somos",
        page_ref: "BoS, pp. 188-189",
        description: include_str!("../../../../data/compendium/paradigms/aliens_make_us_what_we_are.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/aliens_make_us_what_we_are.pt.md"),
        associated_practices: &[
            r#"Chaos magick (which has plenty of weird ties to Lovecraft and UFOs)"#,
            r#"craftwork (replicating alien manufacture, of course!)"#,
            r#"crazy wisdom"#,
            r#"cybernetics"#,
            r#"faith (in godlike aliens and/or alien gods)"#,
            r#"god-bonding (likewise)"#,
            r#"hypertech"#,
            r#"invigoration"#,
            r#"maleficia (especially of the “secrets from the Void” variety)"#,
            r#"martial arts (alien fighting techniques)"#,
            r#"mediumship (channeling alien entities)"#,
            r#"psionics"#,
            r#"reality hacking"#,
            r#"weird science"#,
            r#"postmodern variations on yoga"#,
        ],
        associated_practices_pt: &[
            r#"Magia do caos (que tem um monte de ligações estranhas com Lovecraft e OVNIs)"#,
            r#"artesanato (reproduzindo a manufatura alienígena, é claro!)"#,
            r#"sabedoria louca"#,
            r#"cibernética"#,
            r#"fé (em alienígenas divinos e/ou deuses alienígenas)"#,
            r#"vínculo divino (idem)"#,
            r#"hipertecnologia"#,
            r#"revigoramento"#,
            r#"malefício (especialmente da variedade “segredos do Vazio”)"#,
            r#"artes marciais (técnicas de luta alienígenas)"#,
            r#"mediunidade (canalizando entidades alienígenas)"#,
            r#"psiônica"#,
            r#"hacking da realidade"#,
            r#"ciência estranha"#,
            r#"variações pós-modernas da ioga"#,
        ],
    },
    ParadigmDefinition {
        id: "all_power_comes_from_gods",
        name: "All Power Comes from God(s)",
        name_pt: "Todo Poder Vem de Deus(es)",
        page_ref: "BoS, p. 189",
        description: include_str!("../../../../data/compendium/paradigms/all_power_comes_from_gods.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/all_power_comes_from_gods.pt.md"),
        associated_practices: &[
            r#"Dominion"#,
            r#"faith"#,
            r#"god-bonding (obviously)"#,
            r#"gutter magick (those in the gutter are often those with the most faith in their divinity)"#,
            r#"High Ritual Magick (which often demands obedience to God as part of the ritual requirements)"#,
            r#"maleficia (so about those bloody-minded gods…)"#,
            r#"martial arts (“I kick ass for the Lord!”)"#,
            r#"medicine-work (often tied to faith in the Creator)"#,
            r#"Voudoun (in which most power flows from your connection to the Loa)"#,
            r#"witchcraft (the Old Gods)"#,
        ],
        associated_practices_pt: &[
            r#"Domínio"#,
            r#"fé"#,
            r#"vínculo divino (obviamente)"#,
            r#"magia de sarjeta (quem está na sarjeta muitas vezes é quem tem mais fé em sua divindade)"#,
            r#"Alta Magia Ritual (que muitas vezes exige obediência a Deus como parte dos requisitos do ritual)"#,
            r#"malefício (então, sobre aqueles deuses sanguinários…)"#,
            r#"artes marciais (“Eu chuto bundas pelo Senhor!”)"#,
            r#"medicina sagrada (muitas vezes ligada à fé no Criador)"#,
            r#"Vodu (no qual a maior parte do poder flui da sua ligação com os Loa)"#,
            r#"bruxaria (os Velhos Deuses)"#,
        ],
    },
    ParadigmDefinition {
        id: "all_the_worlds_a_stage",
        name: "All the World’s a Stage",
        name_pt: "O Mundo Inteiro é um Palco",
        page_ref: "BoS, pp. 189-190",
        description: include_str!("../../../../data/compendium/paradigms/all_the_worlds_a_stage.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/all_the_worlds_a_stage.pt.md"),
        associated_practices: &[
            r#"The Art of Desire"#,
            r#"bardism"#,
            r#"crazy wisdom (once you’ve seen the truth, you’re crazy)"#,
            r#"dominion"#,
            r#"gutter magick (this puts the senselessness of life into perspective)"#,
            r#"hypertech"#,
            r#"invigoration (“act well your part – there all the honor lies!”)"#,
            r#"mediumship (in connection with the real audience)"#,
            r#"psionics"#,
            r#"reality hacking (“because I’ve got the script, motherfuckers!”)"#,
            r#"weird science"#,
        ],
        associated_practices_pt: &[
            r#"A Arte do Desejo"#,
            r#"bardismo"#,
            r#"sabedoria louca (depois que você viu a verdade, você é louco)"#,
            r#"domínio"#,
            r#"magia de sarjeta (isso coloca em perspectiva a falta de sentido da vida)"#,
            r#"hipertecnologia"#,
            r#"revigoramento (“desempenhe bem o seu papel – nisso reside toda a honra!”)"#,
            r#"mediunidade (em conexão com a plateia de verdade)"#,
            r#"psiônica"#,
            r#"hacking da realidade (“porque eu tenho o roteiro, seus filhos da puta!”)"#,
            r#"ciência estranha"#,
        ],
    },
    ParadigmDefinition {
        id: "ancestor_veneration",
        name: "Ancestor Veneration",
        name_pt: "Veneração aos Ancestrais",
        page_ref: "BoS, p. 193",
        description: include_str!("../../../../data/compendium/paradigms/ancestor_veneration.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/ancestor_veneration.pt.md"),
        associated_practices: &[
            r#"Bardism"#,
            r#"faith"#,
            r#"medicine-work"#,
            r#"mediumship"#,
            r#"shamanism"#,
            r#"Voudoun"#,
            r#"witchcraft"#,
        ],
        associated_practices_pt: &[
            r#"Bardismo"#,
            r#"fé"#,
            r#"medicina sagrada"#,
            r#"mediunidade"#,
            r#"xamanismo"#,
            r#"Vodu"#,
            r#"bruxaria"#,
        ],
    },
    ParadigmDefinition {
        id: "ancient_wisdom_is_the_key",
        name: "Ancient Wisdom is the Key",
        name_pt: "A Sabedoria Antiga é a Chave",
        page_ref: "BoS, p. 190",
        description: include_str!("../../../../data/compendium/paradigms/ancient_wisdom_is_the_key.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/ancient_wisdom_is_the_key.pt.md"),
        associated_practices: &[
            r#"Alchemy"#,
            r#"animalism (“the Oldest Ways are best!”)"#,
            r#"bardism (Atlantean, Greek, and Chinese musicology)"#,
            r#"craftwork"#,
            r#"crazy wisdom"#,
            r#"dominion"#,
            r#"elementalism"#,
            r#"god-bonding"#,
            r#"High Ritual Magick"#,
            r#"invigoration"#,
            r#"maleficia (those Old Gods could be pretty nasty…)"#,
            r#"medicine-work (“Your ‘modern medicine’ is lies and profits!”)"#,
            r#"mediumship"#,
            r#"psionics (“ancient secrets of the mind”)"#,
            r#"shamanism"#,
            r#"witchcraft"#,
            r#"yoga"#,
        ],
        associated_practices_pt: &[
            r#"Alquimia"#,
            r#"animalismo (“os Caminhos Mais Antigos são os melhores!”)"#,
            r#"bardismo (musicologia atlante, grega e chinesa)"#,
            r#"artesanato"#,
            r#"sabedoria louca"#,
            r#"domínio"#,
            r#"elementalismo"#,
            r#"vínculo divino"#,
            r#"Alta Magia Ritual"#,
            r#"revigoramento"#,
            r#"malefício (aqueles Velhos Deuses podiam ser bem cruéis…)"#,
            r#"medicina sagrada (“Sua ‘medicina moderna’ é mentira e lucro!”)"#,
            r#"mediunidade"#,
            r#"psiônica (“segredos antigos da mente”)"#,
            r#"xamanismo"#,
            r#"bruxaria"#,
            r#"ioga"#,
        ],
    },
    ParadigmDefinition {
        id: "consciousness_is_the_only_true_reality",
        name: "Consciousness is the Only True Reality",
        name_pt: "A Consciência é a Única Realidade Verdadeira",
        page_ref: "BoS, pp. 190-191",
        description: include_str!("../../../../data/compendium/paradigms/consciousness_is_the_only_true_reality.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/consciousness_is_the_only_true_reality.pt.md"),
        associated_practices: &[
            r#"Alchemy (especially among all types of Solificati)"#,
            r#"Art of Desire/hypereconimics (“It’s all about what you think it’s worth…”)"#,
            r#"chaos magick"#,
            r#"crazy wisdom (obviously)"#,
            r#"dominion (ditto that)"#,
            r#"High Ritual Magick (which is often all about changing one’s perceptions of “What is” into “What I want it to become”)"#,
            r#"hypertech"#,
            r#"invigoration"#,
            r#"martial arts"#,
            r#"psionics"#,
            r#"reality hacking (for which this paradigm is kind of a foundation)"#,
            r#"yoga (going back to the roots of the idea in Hindu and Buddhist philosophy)"#,
        ],
        associated_practices_pt: &[
            r#"Alquimia (especialmente entre todos os tipos de Solificati)"#,
            r#"Arte do Desejo/hipereconomia (“Tudo depende de quanto você acha que vale…”)"#,
            r#"magia do caos"#,
            r#"sabedoria louca (obviamente)"#,
            r#"domínio (idem)"#,
            r#"Alta Magia Ritual (que muitas vezes gira em torno de transformar a percepção de “O que é” em “O que eu quero que se torne”)"#,
            r#"hipertecnologia"#,
            r#"revigoramento"#,
            r#"artes marciais"#,
            r#"psiônica"#,
            r#"hacking da realidade (para o qual esse paradigma é uma espécie de alicerce)"#,
            r#"ioga (voltando às raízes da ideia na filosofia hindu e budista)"#,
        ],
    },
    ParadigmDefinition {
        id: "embrace_the_threshold",
        name: "Embrace the Threshold",
        name_pt: "Abrace o Limiar",
        page_ref: "BoS, pp. 191-192",
        description: include_str!("../../../../data/compendium/paradigms/embrace_the_threshold.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/embrace_the_threshold.pt.md"),
        associated_practices: &[
            r#"Alchemy"#,
            r#"chaos magick"#,
            r#"crazy wisdom"#,
            r#"cybernetics"#,
            r#"gutter magick (for obvious reasons)"#,
            r#"faith"#,
            r#"maleficia (working to bring about that ending)"#,
            r#"psionics"#,
            r#"reality hacking"#,
            r#"yoga"#,
        ],
        associated_practices_pt: &[
            r#"Alquimia"#,
            r#"magia do caos"#,
            r#"sabedoria louca"#,
            r#"cibernética"#,
            r#"magia de sarjeta (por razões óbvias)"#,
            r#"fé"#,
            r#"malefício (trabalhando para provocar esse fim)"#,
            r#"psiônica"#,
            r#"hacking da realidade"#,
            r#"ioga"#,
        ],
    },
    ParadigmDefinition {
        id: "a_holographic_reality",
        name: "A Holographic Reality",
        name_pt: "Uma Realidade Holográfica",
        page_ref: "BoS, p. 192",
        description: include_str!("../../../../data/compendium/paradigms/a_holographic_reality.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/a_holographic_reality.pt.md"),
        associated_practices: &[
            r#"Chaos magick"#,
            r#"crazy wisdom"#,
            r#"cybernetics"#,
            r#"hypertech"#,
            r#"reality hacking"#,
            r#"weird science"#,
            r#"witchcraft (in its newest forms)"#,
            r#"yoga (likewise)"#,
        ],
        associated_practices_pt: &[
            r#"Magia do caos"#,
            r#"sabedoria louca"#,
            r#"cibernética"#,
            r#"hipertecnologia"#,
            r#"hacking da realidade"#,
            r#"ciência estranha"#,
            r#"bruxaria (em suas formas mais novas)"#,
            r#"ioga (idem)"#,
        ],
    },
    ParadigmDefinition {
        id: "transcend_your_limits",
        name: "Transcend Your Limits",
        name_pt: "Transcenda Seus Limites",
        page_ref: "BoS, pp. 192-194",
        description: include_str!("../../../../data/compendium/paradigms/transcend_your_limits.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/transcend_your_limits.pt.md"),
        associated_practices: &[
            r#"The Art of Desire/hypereconomics"#,
            r#"bardism"#,
            r#"crazy wisdom"#,
            r#"dominion"#,
            r#"High Ritual Magick"#,
            r#"invigoration"#,
            r#"maleficia"#,
            r#"psionics"#,
            r#"reality hacking"#,
            r#"Voudoun"#,
            r#"witchcraft (shadow-work in particular)"#,
            r#"yoga (Left-Hand Path varieties)"#,
        ],
        associated_practices_pt: &[
            r#"A Arte do Desejo/hipereconomia"#,
            r#"bardismo"#,
            r#"sabedoria louca"#,
            r#"domínio"#,
            r#"Alta Magia Ritual"#,
            r#"revigoramento"#,
            r#"malefício"#,
            r#"psiônica"#,
            r#"hacking da realidade"#,
            r#"Vodu"#,
            r#"bruxaria (o trabalho com a sombra, em particular)"#,
            r#"ioga (as variedades do Caminho da Mão Esquerda)"#,
        ],
    },
    ParadigmDefinition {
        id: "turning_the_keys_to_reality",
        name: "Turning the Keys to Reality",
        name_pt: "Girando as Chaves da Realidade",
        page_ref: "BoS, p. 194",
        description: include_str!("../../../../data/compendium/paradigms/turning_the_keys_to_reality.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/turning_the_keys_to_reality.pt.md"),
        associated_practices: &[
            r#"Alchemy"#,
            r#"the Art of Desire"#,
            r#"craftwork"#,
            r#"dominion"#,
            r#"elementalism"#,
            r#"god-bonding"#,
            r#"High Ritual Magick"#,
            r#"invigoration"#,
            r#"medicine-work"#,
            r#"psionics"#,
            r#"reality hacking"#,
            r#"yoga"#,
        ],
        associated_practices_pt: &[
            r#"Alquimia"#,
            r#"a Arte do Desejo"#,
            r#"artesanato"#,
            r#"domínio"#,
            r#"elementalismo"#,
            r#"vínculo divino"#,
            r#"Alta Magia Ritual"#,
            r#"revigoramento"#,
            r#"medicina sagrada"#,
            r#"psiônica"#,
            r#"hacking da realidade"#,
            r#"ioga"#,
        ],
    },
    ParadigmDefinition {
        id: "we_are_meant_to_be_wild",
        name: "We are Meant to be Wild",
        name_pt: "Nascemos para Ser Selvagens",
        page_ref: "BoS, p. 195",
        description: include_str!("../../../../data/compendium/paradigms/we_are_meant_to_be_wild.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/we_are_meant_to_be_wild.pt.md"),
        associated_practices: &[
            r#"Animalism"#,
            r#"crazy wisdom"#,
            r#"dominion"#,
            r#"elementalism"#,
            r#"invigoration"#,
            r#"shamanism"#,
            r#"witchcraft"#,
            r#"yoga"#,
        ],
        associated_practices_pt: &[
            r#"Animalismo"#,
            r#"sabedoria louca"#,
            r#"domínio"#,
            r#"elementalismo"#,
            r#"revigoramento"#,
            r#"xamanismo"#,
            r#"bruxaria"#,
            r#"ioga"#,
        ],
    },
    ParadigmDefinition {
        id: "we_are_not_men",
        name: "We Are NOT Men!",
        name_pt: "Nós NÃO Somos Homens!",
        page_ref: "BoS, pp. 195-196",
        description: include_str!("../../../../data/compendium/paradigms/we_are_not_men.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/we_are_not_men.pt.md"),
        associated_practices: &[
            r#"The Art of Desire/hypereconomics"#,
            r#"cybernetics"#,
            r#"dominion"#,
            r#"High Ritual Magick"#,
            r#"hypertech"#,
            r#"invigoration"#,
            r#"martial arts"#,
            r#"psionics"#,
            r#"reality hacking"#,
            r#"weird science"#,
        ],
        associated_practices_pt: &[
            r#"A Arte do Desejo/hipereconomia"#,
            r#"cibernética"#,
            r#"domínio"#,
            r#"Alta Magia Ritual"#,
            r#"hipertecnologia"#,
            r#"revigoramento"#,
            r#"artes marciais"#,
            r#"psiônica"#,
            r#"hacking da realidade"#,
            r#"ciência estranha"#,
        ],
    },
    ParadigmDefinition {
        id: "were_all_gods_in_disguise",
        name: "We’re All God(s) in Disguise",
        name_pt: "Somos Todos Deus(es) Disfarçados",
        page_ref: "BoS, p. 196",
        description: include_str!("../../../../data/compendium/paradigms/were_all_gods_in_disguise.en.md"),
        description_pt: include_str!("../../../../data/compendium/paradigms/were_all_gods_in_disguise.pt.md"),
        associated_practices: &[
            r#"Alchemy"#,
            r#"crazy wisdom"#,
            r#"faith"#,
            r#"High Ritual Magick"#,
            r#"invigoration"#,
            r#"martial arts"#,
            r#"psionics"#,
            r#"witchcraft"#,
            r#"yoga"#,
        ],
        associated_practices_pt: &[
            r#"Alquimia"#,
            r#"sabedoria louca"#,
            r#"fé"#,
            r#"Alta Magia Ritual"#,
            r#"revigoramento"#,
            r#"artes marciais"#,
            r#"psiônica"#,
            r#"bruxaria"#,
            r#"ioga"#,
        ],
    },
];

/// Busca um paradigma por id, nome EN/PT ou substring.
pub fn find_paradigm(query: &str) -> Option<&'static ParadigmDefinition> {
    let clean = query.trim();
    if clean.is_empty() {
        return None;
    }
    let lower = clean.to_lowercase();
    ALL_PARADIGMS
        .iter()
        .find(|p| {
            p.id.eq_ignore_ascii_case(clean)
                || p.name.eq_ignore_ascii_case(clean)
                || p.name_pt.to_lowercase() == lower
        })
        .or_else(|| {
            ALL_PARADIGMS.iter().find(|p| {
                p.name.to_lowercase().contains(&lower) || p.name_pt.to_lowercase().contains(&lower)
            })
        })
}

/// Busca o artigo introdutório da seção por id.
pub fn find_paradigm_article(query: &str) -> Option<&'static ParadigmArticle> {
    let clean = query.trim();
    (ARTICLE_EXPANDED_PARADIGMS.id.eq_ignore_ascii_case(clean)).then_some(&ARTICLE_EXPANDED_PARADIGMS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_paradigms_integrity() {
        assert_eq!(ALL_PARADIGMS.len(), 13);
        let mut ids = std::collections::HashSet::new();
        for p in ALL_PARADIGMS {
            assert!(ids.insert(p.id), "Duplicate paradigm ID: {}", p.id);
            assert!(p.page_ref.starts_with("BoS, p"), "Invalid page_ref for {}", p.id);
            let en = p.description.split("\n\n").count();
            let pt = p.description_pt.split("\n\n").count();
            assert_eq!(en, pt, "Paragraph mismatch in {}", p.id);
            assert_eq!(p.associated_practices.len(), p.associated_practices_pt.len(), "{}", p.id);
            assert!(!p.associated_practices.is_empty(), "{}", p.id);
        }
    }

    #[test]
    fn test_find_paradigm() {
        assert_eq!(find_paradigm("We Are NOT Men!").map(|p| p.id), Some("we_are_not_men"));
        assert_eq!(find_paradigm("Abrace o Limiar").map(|p| p.id), Some("embrace_the_threshold"));
        assert_eq!(find_paradigm("ancestor_veneration").map(|p| p.id), Some("ancestor_veneration"));
        assert!(find_paradigm_article("article_expanded_paradigms").is_some());
    }
}
