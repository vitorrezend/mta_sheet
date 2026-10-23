//! Compêndio aberto a partir da ficha: o que cada seção abre e o que acontece ao escolher um item.

use leptos::*;
use crate::compendium::abilities::{AbilityCategory, ALL_ABILITIES};
use crate::compendium::weapons::{CombatManeuver, WeaponDefinition};
use crate::components::mta_sheet::page5::{ArchetypeTarget, CompendiumSection, PracticeCompendiumContext, PracticeCompendiumModal};
use crate::components::mta_sheet::sheet::ActiveDotOriginContext;
use crate::components::Callback;
use crate::state::{keys, CharacterData, WeaponItem};

#[derive(Clone, Debug, PartialEq)]
pub enum CompendiumTarget {
    Practice(Option<usize>),
    Instrument(Option<usize>),
    Archetype(Option<ArchetypeTarget>),
    Attribute,
    Background(Option<usize>),
    Sphere(Option<usize>),
    Weapon(Option<usize>),
    Ability(Option<usize>),
    MeritFlaw(Option<usize>),
}

impl CompendiumTarget {
    /// Posição na lista da ficha que o item escolhido deve ocupar (None = nova linha)
    fn slot(&self) -> Option<usize> {
        match self {
            Self::Practice(s) | Self::Instrument(s) | Self::Background(s) | Self::Sphere(s)
            | Self::Weapon(s) | Self::Ability(s) | Self::MeritFlaw(s) => *s,
            Self::Archetype(_) | Self::Attribute => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompendiumModalState {
    pub section: CompendiumSection,
    pub query: String,
    pub target: CompendiumTarget,
}

impl Default for CompendiumModalState {
    fn default() -> Self {
        Self {
            section: CompendiumSection::Practices,
            query: String::new(),
            target: CompendiumTarget::Practice(None),
        }
    }
}

/// Id da linha `slot` da lista; cria uma linha nova quando o slot não existe.
fn row_id(list: &mut Vec<String>, slot: Option<usize>, prefix: &str) -> String {
    if let Some(id) = slot.and_then(|i| list.get(i)) {
        return id.clone();
    }
    let id = format!("{}_{}", prefix, uuid::Uuid::new_v4());
    list.push(id.clone());
    id
}

/// Grava `value` na posição `idx` de uma lista de nomes (Práticas, Instrumentos),
/// que começa com três linhas vazias.
fn set_name_slot(list: &mut Vec<String>, idx: usize, value: String) {
    if list.is_empty() {
        *list = vec![String::new(); 3];
    }
    if list.len() <= idx {
        list.resize(idx + 1, String::new());
    }
    list[idx] = value;
}

/// Categoria da habilidade escolhida e o prefixo dos ids dessa categoria.
fn ability_category(name: &str) -> (&'static str, &'static str) {
    let category = ALL_ABILITIES
        .iter()
        .find(|a| name.starts_with(a.name_pt) || name.starts_with(a.name) || name.starts_with(a.id))
        .map(|a| a.category);
    match category {
        Some(AbilityCategory::Skills) => ("Perícias", "per"),
        Some(AbilityCategory::Knowledges) => ("Conhecimentos", "con"),
        Some(AbilityCategory::Talents) | None => ("Talentos", "tal"),
    }
}

/// Posição da arma: a escolhida, ou a primeira linha vazia, ou uma nova.
fn weapon_slot(weapons: &mut Vec<WeaponItem>, slot: Option<usize>) -> usize {
    let idx = slot.unwrap_or_else(|| weapons.iter().position(|w| w.name.trim().is_empty()).unwrap_or(weapons.len()));
    if weapons.len() <= idx {
        weapons.resize(idx + 1, WeaponItem::default());
    }
    idx
}

/// Seção aberta e se a janela está visível.
#[derive(Clone, Copy)]
pub struct SheetCompendium {
    pub state: RwSignal<CompendiumModalState>,
    pub show: ReadSignal<bool>,
    pub set_show: WriteSignal<bool>,
}

impl SheetCompendium {
    fn open(&self, section: CompendiumSection, query: String, target: CompendiumTarget) {
        self.state.set(CompendiumModalState { section, query, target });
        self.set_show.set(true);
    }
}

/// Abre o compêndio na seção certa; as páginas da ficha recebem os atalhos por contexto.
pub fn provide_sheet_compendium() -> SheetCompendium {
    let (show, set_show) = create_signal(false);
    let compendium = SheetCompendium { state: create_rw_signal(CompendiumModalState::default()), show, set_show };
    let opener = move |section: CompendiumSection, target: fn(Option<usize>) -> CompendiumTarget| {
        Callback::new(move |(slot, query): (Option<usize>, String)| compendium.open(section, query, target(slot)))
    };
    let open_practice = opener(CompendiumSection::Practices, CompendiumTarget::Practice);
    provide_context(PracticeCompendiumContext {
        open: open_practice.clone(),
        open_practice,
        open_instrument: opener(CompendiumSection::Instruments, CompendiumTarget::Instrument),
        open_archetype: Callback::new(move |(target, query): (Option<ArchetypeTarget>, String)| {
            compendium.open(CompendiumSection::Archetypes, query, CompendiumTarget::Archetype(target))
        }),
        open_attribute: opener(CompendiumSection::Attributes, |_| CompendiumTarget::Attribute),
        open_weapon: opener(CompendiumSection::Weapons, CompendiumTarget::Weapon),
        open_background: opener(CompendiumSection::Backgrounds, CompendiumTarget::Background),
        open_sphere: opener(CompendiumSection::Spheres, CompendiumTarget::Sphere),
        open_ability: opener(CompendiumSection::Abilities, CompendiumTarget::Ability),
        open_merit_flaw: opener(CompendiumSection::MeritsFlaws, CompendiumTarget::MeritFlaw),
    });
    compendium
}

/// Janela do compêndio; aplica na ficha o item escolhido.
#[component]
pub fn SheetCompendiumModal(compendium: SheetCompendium) -> impl IntoView {
    let state = compendium.state;
    let data = expect_context::<ReadSignal<CharacterData>>();
    let set_data = expect_context::<WriteSignal<CharacterData>>();
    let active_origin = expect_context::<ActiveDotOriginContext>().origin;
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get_untracked()).unwrap_or_default();

    let target = move || state.with_untracked(|s| s.target.clone());

    let on_practice = Callback::new(move |name: String| {
        if let CompendiumTarget::Practice(Some(idx)) = target() {
            set_data.update(|s| set_name_slot(&mut s.grimoire.practices, idx, name));
        }
    });
    let on_instrument = Callback::new(move |name: String| {
        if let CompendiumTarget::Instrument(Some(idx)) = target() {
            set_data.update(|s| set_name_slot(&mut s.grimoire.instruments, idx, name));
        }
    });
    let on_archetype = Callback::new(move |(target, name): (ArchetypeTarget, String)| {
        let key = match target {
            ArchetypeTarget::Nature => "Natureza",
            ArchetypeTarget::Demeanor => "Comportamento",
        };
        set_data.update(|s| s.set_label(key, name));
    });
    let on_weapon = Callback::new(move |(slot, w): (Option<usize>, &'static WeaponDefinition)| {
        let lang = lang();
        set_data.update(|s| {
            let idx = weapon_slot(&mut s.weapons, slot);
            s.weapons[idx] = WeaponItem {
                name: w.name(lang).to_string(),
                diff: w.difficulty.to_string(),
                damage: w.damage(lang).to_string(),
                range: w.range.to_string(),
                rate: w.rate.to_string(),
                clip: w.clip.to_string(),
                conceal: w.conceal.to_string(),
                notes: w.format_notes_str(lang),
            };
        });
    });
    let on_maneuver = Callback::new(move |(slot, m): (Option<usize>, &'static CombatManeuver)| {
        let lang = lang();
        set_data.update(|s| {
            let idx = weapon_slot(&mut s.weapons, slot);
            s.weapons[idx] = m.to_weapon_item(lang);
        });
    });
    let on_background = Callback::new(move |(slot, name, level): (Option<usize>, String, i32)| {
        // A posição de onde o compêndio foi aberto só vale se foi aberto para um antecedente
        let slot = slot.or(match target() { CompendiumTarget::Background(s) => s, _ => None });
        let origin = active_origin.get_untracked();
        set_data.update(|s| {
            let id = row_id(s.custom_lists.entry("Antecedentes".to_string()).or_default(), slot, "bg");
            s.labels.insert(id.clone(), name);
            s.set_attribute_with_origin(&id, Some(level), None, origin);
        });
    });
    let on_merit_flaw = Callback::new(move |(slot, name, cost, is_flaw): (Option<usize>, String, i32, bool)| {
        let slot = slot.or(match target() { CompendiumTarget::MeritFlaw(s) => s, _ => None });
        let (category, prefix) = if is_flaw { (keys::CAT_FLAWS, "flaw") } else { (keys::CAT_MERITS, "merit") };
        let origin = active_origin.get_untracked();
        set_data.update(|s| {
            let id = row_id(s.custom_lists.entry(category.to_string()).or_default(), slot, prefix);
            s.labels.insert(id.clone(), name);
            s.set_attribute_with_origin(&id, Some(cost), None, origin);
        });
    });
    let on_ability = Callback::new(move |(slot, name): (Option<usize>, String)| {
        let slot = slot.or(match target() { CompendiumTarget::Ability(s) => s, _ => None });
        let (category, prefix) = ability_category(&name);
        set_data.update(|s| {
            let id = row_id(s.custom_lists.entry(category.to_string()).or_default(), slot, &format!("ab_{}", prefix));
            s.labels.insert(id, name);
        });
    });
    let on_sphere = Callback::new(move |chosen_id: String| {
        set_data.update(|s| {
            s.swap_sphere_variant(&chosen_id);
        });
    });

    view! {
        <PracticeCompendiumModal
            show_modal=compendium.show
            set_show_modal=compendium.set_show
            initial_query=Signal::derive(move || state.with(|s| s.query.clone()))
            initial_section=Some(Signal::derive(move || state.with(|s| s.section)))
            on_select_practice=on_practice
            on_select_instrument=on_instrument
            on_select_archetype=on_archetype
            initial_archetype_target=Some(Signal::derive(move || match state.with(|s| s.target.clone()) {
                CompendiumTarget::Archetype(t) => t,
                _ => None,
            }))
            target_slot=Some(Signal::derive(move || match state.with(|s| s.target.clone()) {
                t @ (CompendiumTarget::Weapon(_) | CompendiumTarget::Background(_) | CompendiumTarget::Ability(_) | CompendiumTarget::MeritFlaw(_)) => t.slot(),
                _ => None,
            }))
            on_select_weapon=on_weapon
            on_select_maneuver=on_maneuver
            on_select_background=on_background
            on_select_sphere=on_sphere
            on_select_ability=on_ability
            on_select_merit_flaw=on_merit_flaw
            active_spheres=Signal::derive(move || data.with(|s| s.get_active_spheres()))
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_id_reuses_existing_rows_and_appends_new_ones() {
        let mut list = vec!["bg_a".to_string()];
        assert_eq!(row_id(&mut list, Some(0), "bg"), "bg_a");
        let new_id = row_id(&mut list, Some(5), "bg");
        assert!(new_id.starts_with("bg_"));
        assert_eq!(list.len(), 2);
        row_id(&mut list, None, "bg");
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn name_slots_start_with_three_rows_and_grow() {
        let mut list = Vec::new();
        set_name_slot(&mut list, 1, "Feitiçaria".into());
        assert_eq!(list, vec!["", "Feitiçaria", ""]);
        set_name_slot(&mut list, 4, "Alquimia".into());
        assert_eq!(list.len(), 5);
        assert_eq!(list[4], "Alquimia");
    }

    #[test]
    fn weapon_slot_prefers_the_first_empty_row() {
        let mut weapons = vec![WeaponItem { name: "Faca".into(), ..Default::default() }, WeaponItem::default()];
        assert_eq!(weapon_slot(&mut weapons, None), 1);
        assert_eq!(weapon_slot(&mut weapons, Some(3)), 3);
        assert_eq!(weapons.len(), 4);
    }

    #[test]
    fn unknown_abilities_fall_back_to_talents() {
        assert_eq!(ability_category("Habilidade inventada"), ("Talentos", "tal"));
    }
}
