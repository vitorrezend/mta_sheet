//! Lógica da lista do Compêndio de Qualidades e Defeitos: filtros, busca por relevância,
//! agrupamento por categoria e pares Qualidade/Defeito (Sterile, Cyclic Magick, Spirit Magnet).
//!
//! Funções puras, sem Leptos, para que a tela fique pequena e a lógica seja testável.

use crate::compendium::merits_flaws::{MeritFlawDefinition, TraitCategory, TraitType, ALL_MERITS_FLAWS};
use crate::i18n::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainTab {
    All,
    Merits,
    Flaws,
    Derangements,
}

/// Livro de origem do verbete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceFilter {
    All,
    M20,
    BoS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    /// Ordem em que aparecem nos livros.
    Book,
    Name,
    Cost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostFilter {
    Any,
    Exactly(i32),
    /// 7 pontos ou mais.
    SevenPlus,
}

impl CostFilter {
    fn accepts(self, mf: &MeritFlawDefinition) -> bool {
        match self {
            CostFilter::Any => true,
            CostFilter::Exactly(c) => mf.available_costs.contains(&c),
            CostFilter::SevenPlus => mf.available_costs.iter().any(|&c| c >= 7),
        }
    }
}

pub const CATEGORY_ORDER: [TraitCategory; 4] = [
    TraitCategory::Physical,
    TraitCategory::Mental,
    TraitCategory::Social,
    TraitCategory::Supernatural,
];

#[derive(Debug, Clone)]
pub struct ListFilters {
    pub query: String,
    pub tab: MainTab,
    pub category: Option<TraitCategory>,
    pub cost: CostFilter,
    pub source: SourceFilter,
    pub sort: SortMode,
    pub lang: Language,
}

/// Um bloco da lista: uma categoria e um tipo (ex.: Qualidades Físicas).
#[derive(Debug, Clone)]
pub struct ListGroup {
    pub category: TraitCategory,
    pub trait_type: TraitType,
    pub items: Vec<&'static MeritFlawDefinition>,
}

impl ListGroup {
    /// Chave estável para lembrar se o bloco está recolhido.
    pub fn key(&self) -> String {
        format!("{}_{}", self.category.id_str(), self.trait_type.id_str())
    }

    pub fn label(&self, lang: Language) -> &'static str {
        group_label(self.category, self.trait_type, lang)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListResult {
    /// Itens cujo nome corresponde à busca (ou todos, sem busca), agrupados.
    pub groups: Vec<ListGroup>,
    /// Itens que só citam o termo no texto (aparecem depois, em "Mencionado em").
    pub mentions: Vec<&'static MeritFlawDefinition>,
}

impl ListResult {
    pub fn total(&self) -> usize {
        self.groups.iter().map(|g| g.items.len()).sum::<usize>() + self.mentions.len()
    }
}

pub fn group_label(cat: TraitCategory, tt: TraitType, lang: Language) -> &'static str {
    match (lang, tt, cat) {
        (Language::PtBr, TraitType::Merit, TraitCategory::Physical) => "Qualidades Físicas",
        (Language::PtBr, TraitType::Merit, TraitCategory::Mental) => "Qualidades Mentais",
        (Language::PtBr, TraitType::Merit, TraitCategory::Social) => "Qualidades Sociais",
        (Language::PtBr, TraitType::Merit, TraitCategory::Supernatural) => "Qualidades Sobrenaturais",
        (Language::PtBr, TraitType::Flaw, TraitCategory::Physical) => "Defeitos Físicos",
        (Language::PtBr, TraitType::Flaw, TraitCategory::Mental) => "Defeitos Mentais",
        (Language::PtBr, TraitType::Flaw, TraitCategory::Social) => "Defeitos Sociais",
        (Language::PtBr, TraitType::Flaw, TraitCategory::Supernatural) => "Defeitos Sobrenaturais",
        (Language::EnUs, TraitType::Merit, TraitCategory::Physical) => "Physical Merits",
        (Language::EnUs, TraitType::Merit, TraitCategory::Mental) => "Mental Merits",
        (Language::EnUs, TraitType::Merit, TraitCategory::Social) => "Social Merits",
        (Language::EnUs, TraitType::Merit, TraitCategory::Supernatural) => "Supernatural Merits",
        (Language::EnUs, TraitType::Flaw, TraitCategory::Physical) => "Physical Flaws",
        (Language::EnUs, TraitType::Flaw, TraitCategory::Mental) => "Mental Flaws",
        (Language::EnUs, TraitType::Flaw, TraitCategory::Social) => "Social Flaws",
        (Language::EnUs, TraitType::Flaw, TraitCategory::Supernatural) => "Supernatural Flaws",
    }
}

/// Minúsculas e sem acentos, para comparar "fe" com "Fé" e "imune" com "Imune".
pub fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ã' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'õ' | 'ô' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            '’' => '\'',
            _ => c,
        })
        .collect()
}

/// Outra metade de um Traço que é ao mesmo tempo Qualidade e Defeito
/// (`sterile` ↔ `sterile_flaw`). Exige o mesmo nome e tipos opostos.
pub fn twin(mf: &MeritFlawDefinition) -> Option<&'static MeritFlawDefinition> {
    let other_id = match mf.id.strip_suffix("_flaw") {
        Some(base) => base.to_string(),
        None => format!("{}_flaw", mf.id),
    };
    ALL_MERITS_FLAWS
        .iter()
        .find(|o| o.id == other_id && o.name == mf.name && o.trait_type != mf.trait_type)
}

fn name_matches(mf: &MeritFlawDefinition, q: &str) -> bool {
    normalize(mf.name).contains(q)
        || normalize(mf.name_pt).contains(q)
        || mf.technocracy_name.is_some_and(|t| normalize(t).contains(q))
        || mf.technocracy_name_pt.is_some_and(|t| normalize(t).contains(q))
}

fn text_matches(mf: &MeritFlawDefinition, q: &str) -> bool {
    normalize(mf.description).contains(q) || normalize(mf.description_pt).contains(q)
}

fn sort_items(items: &mut [&'static MeritFlawDefinition], sort: SortMode, lang: Language) {
    let book_pos = |mf: &MeritFlawDefinition| ALL_MERITS_FLAWS.iter().position(|o| o.id == mf.id).unwrap_or(usize::MAX);
    match sort {
        SortMode::Book => items.sort_by_key(|mf| book_pos(mf)),
        SortMode::Name => items.sort_by_key(|mf| normalize(mf.name(lang))),
        SortMode::Cost => items.sort_by_key(|mf| {
            (mf.available_costs.iter().copied().min().unwrap_or(0), normalize(mf.name(lang)))
        }),
    }
}

/// Monta a lista: aplica aba, categoria, custo e livro; separa quem tem o termo no nome
/// de quem só o cita no texto; junta os pares Qualidade/Defeito numa linha só (na aba Todas).
pub fn build_list(f: &ListFilters) -> ListResult {
    if f.tab == MainTab::Derangements {
        return ListResult::default();
    }
    let q = normalize(f.query.trim());

    let candidates = ALL_MERITS_FLAWS.iter().filter(|mf| {
        match f.tab {
            MainTab::Merits if mf.trait_type != TraitType::Merit => return false,
            MainTab::Flaws if mf.trait_type != TraitType::Flaw => return false,
            // Na aba Todas, o par aparece uma vez só, pela metade Qualidade
            MainTab::All if mf.trait_type == TraitType::Flaw && twin(mf).is_some() => return false,
            _ => {}
        }
        if f.category.is_some_and(|c| c != mf.category) {
            return false;
        }
        match f.source {
            SourceFilter::M20 if mf.source_book() != "M20" => return false,
            SourceFilter::BoS if mf.source_book() != "BoS" => return false,
            _ => {}
        }
        f.cost.accepts(mf)
    });

    let mut named = Vec::new();
    let mut mentions = Vec::new();
    for mf in candidates {
        if q.is_empty() || name_matches(mf, &q) {
            named.push(mf);
        } else if text_matches(mf, &q) {
            mentions.push(mf);
        }
    }

    let mut groups = Vec::new();
    for cat in CATEGORY_ORDER {
        for tt in [TraitType::Merit, TraitType::Flaw] {
            let mut items: Vec<_> = named.iter().copied().filter(|mf| mf.category == cat && mf.trait_type == tt).collect();
            if items.is_empty() {
                continue;
            }
            sort_items(&mut items, f.sort, f.lang);
            groups.push(ListGroup { category: cat, trait_type: tt, items });
        }
    }
    sort_items(&mut mentions, SortMode::Name, f.lang);
    ListResult { groups, mentions }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filters(query: &str) -> ListFilters {
        ListFilters {
            query: query.to_string(),
            tab: MainTab::All,
            category: None,
            cost: CostFilter::Any,
            source: SourceFilter::All,
            sort: SortMode::Book,
            lang: Language::PtBr,
        }
    }

    fn ids(r: &ListResult) -> Vec<&'static str> {
        r.groups.iter().flat_map(|g| g.items.iter().map(|m| m.id)).collect()
    }

    #[test]
    fn search_puts_name_matches_first_and_text_mentions_apart() {
        let r = build_list(&filters("hyperfocus"));
        assert_eq!(ids(&r), vec!["hyperfocus"]);
        // Mental Lock cita Hyperfocus no texto: vai para "Mencionado em"
        assert!(r.mentions.iter().any(|m| m.id == "mental_lock"));
        assert!(!r.mentions.iter().any(|m| m.id == "hyperfocus"));
    }

    #[test]
    fn search_ignores_accents_and_finds_portuguese_names() {
        assert!(ids(&build_list(&filters("fe verdadeira"))).contains(&"true_faith"));
        assert!(ids(&build_list(&filters("FILACTERIO"))).contains(&"phylactery"));
    }

    #[test]
    fn merit_flaw_pairs_show_once_in_all_tab() {
        let r = build_list(&filters(""));
        let all = ids(&r);
        assert!(all.contains(&"sterile") && !all.contains(&"sterile_flaw"));
        assert!(all.contains(&"cyclic_magick") && !all.contains(&"cyclic_magick_flaw"));
        // Na aba Defeitos a metade Defeito aparece
        let flaws = build_list(&ListFilters { tab: MainTab::Flaws, ..filters("") });
        assert!(ids(&flaws).contains(&"sterile_flaw"));
        // Amnesia (Defeito) não tem par: o "amnesia" é uma Perturbação, não uma Qualidade
        assert!(twin(crate::compendium::merits_flaws::find_merit_flaw("amnesia_flaw").unwrap()).is_none());
    }

    #[test]
    fn groups_follow_book_categories() {
        let r = build_list(&filters(""));
        let keys: Vec<String> = r.groups.iter().map(|g| g.key()).collect();
        assert_eq!(keys.first().map(String::as_str), Some("physical_merits"));
        assert_eq!(keys.last().map(String::as_str), Some("supernatural_flaws"));
        assert_eq!(keys.len(), 8);
        // Nenhum item fica de fora: total = todos menos as 3 metades Defeito agrupadas
        assert_eq!(r.total(), ALL_MERITS_FLAWS.len() - 3);
    }

    #[test]
    fn cost_source_and_sort_filters() {
        let seven = build_list(&ListFilters { cost: CostFilter::SevenPlus, ..filters("") });
        assert!(seven.groups.iter().flat_map(|g| &g.items).all(|m| m.available_costs.iter().any(|&c| c >= 7)));
        let m20 = build_list(&ListFilters { source: SourceFilter::M20, ..filters("") });
        assert!(m20.groups.iter().flat_map(|g| &g.items).all(|m| m.page_ref.starts_with("M20")));
        let by_cost = build_list(&ListFilters { sort: SortMode::Cost, category: Some(TraitCategory::Supernatural), ..filters("") });
        let costs: Vec<i32> = by_cost.groups[0].items.iter().map(|m| m.available_costs[0]).collect();
        assert!(costs.windows(2).all(|w| w[0] <= w[1]), "{costs:?}");
    }
}
