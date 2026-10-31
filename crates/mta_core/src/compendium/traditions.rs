//! As Nove Tradições do Conselho (M20, Capítulo Cinco: Ascension Warriors, pp. 147-165).
//!
//! Textos integrais bilíngues (inglês e português brasileiro) carregados de data/compendium/traditions/*.md.

use serde::Serialize;
use crate::i18n::Language;

/// Uma das Nove Tradições do Conselho.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TraditionDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub aliases: &'static [&'static str],
    pub page_ref: &'static str,
    /// IDs das Esferas de afinidade (`mta://spheres/<id>`), na ordem do livro.
    pub affinity_spheres: &'static [&'static str],
    pub affinity_label: &'static str,
    pub affinity_label_pt: &'static str,
    pub content: &'static str,
    pub content_pt: &'static str,
}

impl TraditionDefinition {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    pub fn affinity_label(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.affinity_label_pt,
            Language::EnUs => self.affinity_label,
        }
    }

    /// Texto integral no idioma.
    pub fn content(&self, lang: Language) -> Option<&'static str> {
        match lang {
            Language::EnUs => Some(self.content),
            Language::PtBr => Some(self.content_pt),
        }
    }

    /// Indica se já existe tradução própria em português (as Convenções ainda usam o texto em inglês).
    pub fn has_pt_text(&self) -> bool {
        self.content_pt != self.content
    }
}

/// Quadro introdutório "Tradition Descriptions" (p. 147).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TraditionArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub page_ref: &'static str,
    pub content: &'static str,
    pub content_pt: &'static str,
}

impl TraditionArticle {
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

pub const TRADITION_DESCRIPTIONS: TraditionArticle = TraditionArticle {
    id: "article_tradition_descriptions",
    title: "Tradition Descriptions",
    title_pt: "Descrição das Tradições",
    page_ref: "M20, p. 147",
    content: include_str!("../../../../data/compendium/traditions/article_tradition_descriptions.en.md"),
    content_pt: include_str!("../../../../data/compendium/traditions/article_tradition_descriptions.pt.md"),
};

pub const ALL_TRADITIONS: &[TraditionDefinition] = &[
    TraditionDefinition {
        id: "akashayana",
        name: "Akashayana",
        name_pt: "Akashayana",
        aliases: &["akashic brotherhood", "irmandade akashica", "akashicos", "akashics"],
        page_ref: "M20, pp. 148-149",
        affinity_spheres: &["mind", "life"],
        affinity_label: "Mind or Life",
        affinity_label_pt: "Mente ou Vida",
        content: include_str!("../../../../data/compendium/traditions/akashayana.en.md"),
        content_pt: include_str!("../../../../data/compendium/traditions/akashayana.pt.md"),
    },
    TraditionDefinition {
        id: "celestial_chorus",
        name: "Celestial Chorus",
        name_pt: "Coro Celestial",
        aliases: &["choristers", "coristas", "singers"],
        page_ref: "M20, pp. 150-151",
        affinity_spheres: &["prime", "forces", "spirit"],
        affinity_label: "Prime; Forces or Spirit",
        affinity_label_pt: "Primórdio; Forças ou Espírito",
        content: include_str!("../../../../data/compendium/traditions/celestial_chorus.en.md"),
        content_pt: include_str!("../../../../data/compendium/traditions/celestial_chorus.pt.md"),
    },
    TraditionDefinition {
        id: "cult_of_ecstasy",
        name: "Cult of Ecstasy",
        name_pt: "Culto do Êxtase",
        aliases: &["sahajiya", "ecstatics", "extaticos"],
        page_ref: "M20, pp. 152-153",
        affinity_spheres: &["time", "life", "mind"],
        affinity_label: "Time; Life or Mind",
        affinity_label_pt: "Tempo; Vida ou Mente",
        content: include_str!("../../../../data/compendium/traditions/cult_of_ecstasy.en.md"),
        content_pt: include_str!("../../../../data/compendium/traditions/cult_of_ecstasy.pt.md"),
    },
    TraditionDefinition {
        id: "dreamspeakers",
        name: "Dreamspeakers",
        name_pt: "Oradores dos Sonhos",
        aliases: &["kha'vadi", "khavadi", "society of dreams"],
        page_ref: "M20, pp. 154-155",
        affinity_spheres: &["spirit", "forces", "life", "matter"],
        affinity_label: "Spirit; Forces, Life, or Matter",
        affinity_label_pt: "Espírito; Forças, Vida ou Matéria",
        content: include_str!("../../../../data/compendium/traditions/dreamspeakers.en.md"),
        content_pt: include_str!("../../../../data/compendium/traditions/dreamspeakers.pt.md"),
    },
    TraditionDefinition {
        id: "euthanatos",
        name: "Euthanatos",
        name_pt: "Eutanatos",
        aliases: &["euthanatoi", "chakravanti", "thanatoics"],
        page_ref: "M20, pp. 156-157",
        affinity_spheres: &["entropy", "life", "spirit"],
        affinity_label: "Entropy; Life or Spirit",
        affinity_label_pt: "Entropia; Vida ou Espírito",
        content: include_str!("../../../../data/compendium/traditions/euthanatos.en.md"),
        content_pt: include_str!("../../../../data/compendium/traditions/euthanatos.pt.md"),
    },
    TraditionDefinition {
        id: "order_of_hermes",
        name: "Order of Hermes",
        name_pt: "Ordem de Hermes",
        aliases: &["hermetics", "hermeticos", "houses of hermes"],
        page_ref: "M20, pp. 158-159",
        affinity_spheres: &["forces"],
        affinity_label: "Forces",
        affinity_label_pt: "Forças",
        content: include_str!("../../../../data/compendium/traditions/order_of_hermes.en.md"),
        content_pt: include_str!("../../../../data/compendium/traditions/order_of_hermes.pt.md"),
    },
    TraditionDefinition {
        id: "society_of_ether",
        name: "Society of Ether",
        name_pt: "Sociedade do Éter",
        aliases: &["etherites", "eteritas", "sons of ether", "filhos do eter"],
        page_ref: "M20, pp. 160-161",
        affinity_spheres: &["matter", "forces", "prime"],
        affinity_label: "Matter; Forces or Prime",
        affinity_label_pt: "Matéria; Forças ou Primórdio",
        content: include_str!("../../../../data/compendium/traditions/society_of_ether.en.md"),
        content_pt: include_str!("../../../../data/compendium/traditions/society_of_ether.pt.md"),
    },
    TraditionDefinition {
        id: "verbena",
        name: "Verbena",
        name_pt: "Verbena",
        aliases: &["verbenae", "witch-folk"],
        page_ref: "M20, pp. 162-163",
        affinity_spheres: &["life", "forces"],
        affinity_label: "Life; Forces",
        affinity_label_pt: "Vida; Forças",
        content: include_str!("../../../../data/compendium/traditions/verbena.en.md"),
        content_pt: include_str!("../../../../data/compendium/traditions/verbena.pt.md"),
    },
    TraditionDefinition {
        id: "virtual_adepts",
        name: "Virtual Adepts",
        name_pt: "Adeptos Virtuais",
        aliases: &["mercurial elite", "elite mercurial", "difference engineers"],
        page_ref: "M20, pp. 164-165",
        affinity_spheres: &["correspondence", "data", "forces"],
        affinity_label: "Correspondence/ Data; Forces",
        affinity_label_pt: "Correspondência/ Dados; Forças",
        content: include_str!("../../../../data/compendium/traditions/virtual_adepts.en.md"),
        content_pt: include_str!("../../../../data/compendium/traditions/virtual_adepts.pt.md"),
    },
];

/// As cinco Convenções da União Tecnocrática (M20, pp. 186-195), no mesmo formato das Tradições.
pub const ALL_CONVENTIONS: &[TraditionDefinition] = &[
    TraditionDefinition {
        id: "iteration_x",
        name: "Iteration X",
        name_pt: "Iteração X",
        aliases: &["iterators", "iteradores", "clockwork convention", "convencao mecanica"],
        page_ref: "M20, pp. 186-187",
        affinity_spheres: &["forces", "matter", "time"],
        affinity_label: "Forces, Matter, or Time",
        affinity_label_pt: "Forças, Matéria ou Tempo",
        content: include_str!("../../../../data/compendium/technocracy/iteration_x.en.md"),
        content_pt: include_str!("../../../../data/compendium/technocracy/iteration_x.pt.md"),
    },
    TraditionDefinition {
        id: "new_world_order",
        name: "New World Order",
        name_pt: "Nova Ordem Mundial",
        aliases: &["nwo", "nom", "black suits", "homens de preto"],
        page_ref: "M20, pp. 188-189",
        affinity_spheres: &["mind", "correspondence", "data"],
        affinity_label: "Mind or Correspondence/ Data",
        affinity_label_pt: "Mente ou Correspondência/ Dados",
        content: include_str!("../../../../data/compendium/technocracy/new_world_order.en.md"),
        content_pt: include_str!("../../../../data/compendium/technocracy/new_world_order.pt.md"),
    },
    TraditionDefinition {
        id: "progenitors",
        name: "Progenitors",
        name_pt: "Progenitores",
        aliases: &["cosians", "cosian circle", "circulo cosiano"],
        page_ref: "M20, pp. 190-191",
        affinity_spheres: &["life", "prime"],
        affinity_label: "Life or Prime",
        affinity_label_pt: "Vida ou Primórdio",
        content: include_str!("../../../../data/compendium/technocracy/progenitors.en.md"),
        content_pt: include_str!("../../../../data/compendium/technocracy/progenitors.pt.md"),
    },
    TraditionDefinition {
        id: "syndicate",
        name: "Syndicate",
        name_pt: "Sindicato",
        aliases: &["high guild", "grande guilda"],
        page_ref: "M20, pp. 192-193",
        affinity_spheres: &["entropy", "mind", "primal_utility"],
        affinity_label: "Entropy, Mind or Primal Utility",
        affinity_label_pt: "Entropia, Mente ou Utilidade Primordial",
        content: include_str!("../../../../data/compendium/technocracy/syndicate.en.md"),
        content_pt: include_str!("../../../../data/compendium/technocracy/syndicate.pt.md"),
    },
    TraditionDefinition {
        id: "void_engineers",
        name: "Void Engineers",
        name_pt: "Engenheiros do Vazio",
        aliases: &["wanderers", "explorators", "void seekers", "andarilhos"],
        page_ref: "M20, pp. 194-195",
        affinity_spheres: &["dimensional_science", "spirit", "correspondence", "forces"],
        affinity_label: "Dimensional Science (Spirit), Correspondence, or Forces",
        affinity_label_pt: "Ciência Dimensional (Espírito), Correspondência ou Forças",
        content: include_str!("../../../../data/compendium/technocracy/void_engineers.en.md"),
        content_pt: include_str!("../../../../data/compendium/technocracy/void_engineers.pt.md"),
    },
];

/// As seitas da Aliança Disparate (M20, pp. 202-223), no mesmo formato das Tradições.
pub const ALL_DISPARATES: &[TraditionDefinition] = &[
    TraditionDefinition {
        id: "ahl_i_batin",
        name: "Ahl-i-Batin",
        name_pt: "Ahl-i-Batin",
        aliases: &["batini", "subtle ones", "sutis"],
        page_ref: "M20, pp. 202-203",
        affinity_spheres: &["correspondence", "mind"],
        affinity_label: "Correspondence or Mind — but never Entropy",
        affinity_label_pt: "Correspondência ou Mente — mas nunca Entropia",
        content: include_str!("../../../../data/compendium/disparates/ahl_i_batin.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/ahl_i_batin.pt.md"),
    },
    TraditionDefinition {
        id: "bataa",
        name: "Bata'a",
        name_pt: "Bata'a",
        aliases: &["bata’a", "bata a"],
        page_ref: "M20, pp. 204-205",
        affinity_spheres: &["spirit", "life"],
        affinity_label: "Spirit or Life",
        affinity_label_pt: "Espírito ou Vida",
        content: include_str!("../../../../data/compendium/disparates/bataa.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/bataa.pt.md"),
    },
    TraditionDefinition {
        id: "children_of_knowledge",
        name: "Children of Knowledge",
        name_pt: "Filhos do Conhecimento",
        aliases: &["solificati", "crowned ones"],
        page_ref: "M20, pp. 206-207",
        affinity_spheres: &["matter", "forces", "prime", "entropy"],
        affinity_label: "Traditionally Matter; some groups prefer Forces, Prime, or Entropy",
        affinity_label_pt: "Tradicionalmente Matéria; alguns grupos preferem Forças, Primórdio ou Entropia",
        content: include_str!("../../../../data/compendium/disparates/children_of_knowledge.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/children_of_knowledge.pt.md"),
    },
    TraditionDefinition {
        id: "hollow_ones",
        name: "Hollow Ones",
        name_pt: "Ocos",
        aliases: &["darklings", "hollowers", "os ocos"],
        page_ref: "M20, pp. 208-209",
        affinity_spheres: &[],
        affinity_label: "Any",
        affinity_label_pt: "Qualquer uma",
        content: include_str!("../../../../data/compendium/disparates/hollow_ones.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/hollow_ones.pt.md"),
    },
    TraditionDefinition {
        id: "kopa_loei",
        name: "Kopa Loei",
        name_pt: "Kopa Loei",
        aliases: &["kanakakahuna"],
        page_ref: "M20, pp. 210-211",
        affinity_spheres: &[],
        affinity_label: "None — Kopa Loei don’t believe in Spheres",
        affinity_label_pt: "Nenhuma — os Kopa Loei não acreditam em Esferas",
        content: include_str!("../../../../data/compendium/disparates/kopa_loei.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/kopa_loei.pt.md"),
    },
    TraditionDefinition {
        id: "ngoma",
        name: "Ngoma",
        name_pt: "Ngoma",
        aliases: &["ngo-ami"],
        page_ref: "M20, pp. 212-213",
        affinity_spheres: &["life", "mind", "prime", "spirit"],
        affinity_label: "Life, Mind, Prime, or Spirit",
        affinity_label_pt: "Vida, Mente, Primórdio ou Espírito",
        content: include_str!("../../../../data/compendium/disparates/ngoma.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/ngoma.pt.md"),
    },
    TraditionDefinition {
        id: "orphans",
        name: "Orphans",
        name_pt: "Órfãos",
        aliases: &["orphan", "orfao"],
        page_ref: "M20, pp. 214-215",
        affinity_spheres: &[],
        affinity_label: "Any",
        affinity_label_pt: "Qualquer uma",
        content: include_str!("../../../../data/compendium/disparates/orphans.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/orphans.pt.md"),
    },
    TraditionDefinition {
        id: "sisters_of_hippolyta",
        name: "Sisters of Hippolyta",
        name_pt: "Irmãs de Hipólita",
        aliases: &["hippolytoi", "sisters"],
        page_ref: "M20, pp. 216-217",
        affinity_spheres: &["life", "mind"],
        affinity_label: "Life or Mind",
        affinity_label_pt: "Vida ou Mente",
        content: include_str!("../../../../data/compendium/disparates/sisters_of_hippolyta.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/sisters_of_hippolyta.pt.md"),
    },
    TraditionDefinition {
        id: "taftani",
        name: "Taftâni",
        name_pt: "Taftâni",
        aliases: &["taftani", "weavers", "tecelões", "ashavanti"],
        page_ref: "M20, pp. 218-219",
        affinity_spheres: &["forces", "matter", "prime", "spirit"],
        affinity_label: "Forces, Matter, Prime, or Spirit",
        affinity_label_pt: "Forças, Matéria, Primórdio ou Espírito",
        content: include_str!("../../../../data/compendium/disparates/taftani.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/taftani.pt.md"),
    },
    TraditionDefinition {
        id: "templar_knights",
        name: "Templar Knights",
        name_pt: "Cavaleiros Templários",
        aliases: &["knights templar", "templars", "templarios"],
        page_ref: "M20, pp. 220-221",
        affinity_spheres: &["forces", "life", "mind", "prime"],
        affinity_label: "Forces, Life, Mind, or Prime",
        affinity_label_pt: "Forças, Vida, Mente ou Primórdio",
        content: include_str!("../../../../data/compendium/disparates/templar_knights.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/templar_knights.pt.md"),
    },
    TraditionDefinition {
        id: "wu_lung",
        name: "Wu Lung",
        name_pt: "Wu Lung",
        aliases: &["dragon wizards", "magos dragao"],
        page_ref: "M20, pp. 222-223",
        affinity_spheres: &["spirit", "forces", "matter", "life"],
        affinity_label: "Spirit, Forces, Matter, or Life",
        affinity_label_pt: "Espírito, Forças, Matéria ou Vida",
        content: include_str!("../../../../data/compendium/disparates/wu_lung.en.md"),
        content_pt: include_str!("../../../../data/compendium/disparates/wu_lung.pt.md"),
    },
];

fn normalize(q: &str) -> String {
    q.trim()
        .to_lowercase()
        .replace(['á', 'à', 'ã', 'â'], "a")
        .replace(['é', 'ê'], "e")
        .replace(['í'], "i")
        .replace(['ó', 'õ', 'ô'], "o")
        .replace(['ú'], "u")
        .replace(['ç'], "c")
        .replace('’', "'")
}

fn find_in(list: &'static [TraditionDefinition], query: &str) -> Option<&'static TraditionDefinition> {
    let q = normalize(query);
    if q.is_empty() {
        return None;
    }
    list.iter().find(|t| {
        t.id == q
            || normalize(t.name) == q
            || normalize(t.name_pt) == q
            || t.aliases.iter().any(|a| normalize(a) == q)
    })
}

/// Busca uma Tradição por ID, nome (EN ou PT) ou apelido.
pub fn find_tradition(query: &str) -> Option<&'static TraditionDefinition> {
    find_in(ALL_TRADITIONS, query)
}

/// Busca uma Convenção da União Tecnocrática por ID, nome (EN ou PT) ou apelido.
pub fn find_convention(query: &str) -> Option<&'static TraditionDefinition> {
    find_in(ALL_CONVENTIONS, query)
}

/// Busca uma Tradição ou uma Convenção.
/// Busca uma seita da Aliança Disparate por ID, nome (EN ou PT) ou apelido.
pub fn find_disparate(query: &str) -> Option<&'static TraditionDefinition> {
    find_in(ALL_DISPARATES, query)
}

/// Busca uma Tradição, uma Convenção ou uma seita Disparate.
pub fn find_faction(query: &str) -> Option<&'static TraditionDefinition> {
    find_tradition(query)
        .or_else(|| find_convention(query))
        .or_else(|| find_disparate(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nine_traditions_with_full_text() {
        assert_eq!(ALL_TRADITIONS.len(), 9);
        for t in ALL_TRADITIONS {
            assert!(t.content.contains("#### Organization"), "{} sem Organization", t.id);
            assert!(t.content.contains("#### Initiation"), "{} sem Initiation", t.id);
            assert!(t.content.contains("#### Focus"), "{} sem Focus", t.id);
            assert!(t.content.contains("#### Stereotypes"), "{} sem Stereotypes", t.id);
            assert!(t.content_pt.contains("#### Organização"), "{} sem Organização em PT", t.id);
            assert!(t.content_pt.contains("#### Iniciação"), "{} sem Iniciação em PT", t.id);
            assert!(t.content_pt.contains("#### Foco"), "{} sem Foco em PT", t.id);
            assert!(t.content_pt.contains("#### Estereótipos"), "{} sem Estereótipos em PT", t.id);
            assert!(!t.affinity_spheres.is_empty());
            assert!(!t.affinity_label_pt.is_empty());
            assert!(t.content(Language::EnUs).is_some());
            assert!(t.content(Language::PtBr).is_some());
        }
    }

    #[test]
    fn test_find_tradition_by_name_and_alias() {
        assert_eq!(find_tradition("Order of Hermes").unwrap().id, "order_of_hermes");
        assert_eq!(find_tradition("ordem de hermes").unwrap().id, "order_of_hermes");
        assert_eq!(find_tradition("Chakravanti").unwrap().id, "euthanatos");
        assert_eq!(find_tradition("eutanatos").unwrap().id, "euthanatos");
        assert_eq!(find_tradition("sociedade do eter").unwrap().id, "society_of_ether");
        assert_eq!(find_tradition("adeptos virtuais").unwrap().id, "virtual_adepts");
        assert_eq!(find_tradition("coro celestial").unwrap().id, "celestial_chorus");
        assert_eq!(find_tradition("culto do extase").unwrap().id, "cult_of_ecstasy");
        assert_eq!(find_tradition("oradores dos sonhos").unwrap().id, "dreamspeakers");
        assert!(find_tradition("technocracy").is_none());
    }

    #[test]
    fn test_five_conventions_with_full_text() {
        assert_eq!(ALL_CONVENTIONS.len(), 5);
        for c in ALL_CONVENTIONS {
            assert!(c.content.contains("#### Organization"), "{} sem Organization", c.id);
            assert!(c.content.contains("#### Initiation"), "{} sem Initiation", c.id);
            assert!(c.content.contains("#### Focus"), "{} sem Focus", c.id);
            assert!(c.content.contains("#### Stereotypes"), "{} sem Stereotypes", c.id);
            assert!(c.content_pt.contains("#### Organização"), "{} sem Organização em PT", c.id);
            assert!(c.content_pt.contains("#### Iniciação"), "{} sem Iniciação em PT", c.id);
            assert!(c.content_pt.contains("#### Foco"), "{} sem Foco em PT", c.id);
            assert!(c.content_pt.contains("#### Estereótipos"), "{} sem Estereótipos em PT", c.id);
            assert!(!c.affinity_spheres.is_empty());
            assert!(!c.affinity_label_pt.is_empty());
            assert!(c.content(Language::EnUs).is_some());
            assert!(c.content(Language::PtBr).is_some());
            assert!(c.has_pt_text(), "Convenção {} deve ter texto em português", c.id);
        }
    }

    #[test]
    fn test_eleven_disparates_with_full_text() {
        assert_eq!(ALL_DISPARATES.len(), 11);
        for d in ALL_DISPARATES {
            assert!(d.content.contains("#### Organization"), "{} sem Organization", d.id);
            assert!(d.content.contains("#### Initiation"), "{} sem Initiation", d.id);
            assert!(d.content.contains("#### Focus"), "{} sem Focus", d.id);
            assert!(d.content.contains("#### Stereotypes"), "{} sem Stereotypes", d.id);
            assert!(d.content_pt.contains("#### Organização"), "{} sem Organização em PT", d.id);
            assert!(d.content_pt.contains("#### Iniciação"), "{} sem Iniciação em PT", d.id);
            assert!(d.content_pt.contains("#### Foco"), "{} sem Foco em PT", d.id);
            assert!(d.content_pt.contains("#### Estereótipos"), "{} sem Estereótipos em PT", d.id);
            assert!(!d.affinity_label_pt.is_empty());
            assert!(d.content(Language::EnUs).is_some());
            assert!(d.content(Language::PtBr).is_some());
            assert!(d.has_pt_text(), "Disparate {} deve ter texto em português", d.id);
        }
    }
}
