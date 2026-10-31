//! Mini fichas de NPC: um "encontro" descartável que vive só no navegador.
//!
//! - `NpcTray`: contexto global com as mini fichas e os ganchos da sala
//! - `NpcTrayDock`: painel lateral com as fichas, a busca no bestiário e o botão flutuante
//! - `MiniSheet`: a mini ficha (vitalidade, Força de Vontade, Atributos)

mod dock;
mod mini_sheet;

pub use dock::NpcTrayDock;
pub use mini_sheet::MiniSheet;

use leptos::*;
use crate::compendium::bestiary::BestiaryEntry;
use crate::i18n::Language;
use crate::state::{next_npc_name, NpcCard};

const STORAGE_KEY: &str = "mta_npc_tray";

/// Pedido de token de NPC para o mapa da sala aberta.
#[derive(Clone, Debug, PartialEq)]
pub struct NpcTokenRequest {
    pub name: String,
    pub color: String,
}

/// Pedido de entrada na iniciativa da sala aberta.
#[derive(Clone, Debug, PartialEq)]
pub struct NpcInitiativeRequest {
    pub name: String,
    pub base: i32,
    pub penalty: i32,
}

#[derive(Clone, Copy)]
pub struct NpcTray {
    pub cards: RwSignal<Vec<NpcCard>>,
    pub open: RwSignal<bool>,
    /// Mini ficha aberta em detalhe (Atributos, Habilidades, notas)
    pub expanded: RwSignal<Option<String>>,
    /// O mapa da sala está aberto e quem usa é o mestre
    pub map_ready: RwSignal<bool>,
    /// O rastreador de iniciativa da sala está montado e quem usa é o mestre
    pub initiative_ready: RwSignal<bool>,
    pub token_request: RwSignal<Option<NpcTokenRequest>>,
    pub initiative_request: RwSignal<Option<NpcInitiativeRequest>>,
}

impl NpcTray {
    fn new() -> Self {
        Self {
            cards: create_rw_signal(Vec::new()),
            open: create_rw_signal(false),
            expanded: create_rw_signal(None),
            map_ready: create_rw_signal(false),
            initiative_ready: create_rw_signal(false),
            token_request: create_rw_signal(None),
            initiative_request: create_rw_signal(None),
        }
    }

    /// Cria uma mini ficha a partir do verbete, abre o painel e mostra a ficha nova.
    pub fn add_from_bestiary(&self, entry: &BestiaryEntry, lang: Language) {
        let name = self.cards.with_untracked(|c| next_npc_name(c, entry.name(lang)));
        self.push(NpcCard::from_bestiary(entry, new_uid(), name));
    }

    pub fn add_blank(&self) {
        let name = self.cards.with_untracked(|c| next_npc_name(c, "NPC"));
        self.push(NpcCard::blank(new_uid(), name));
    }

    pub fn duplicate(&self, uid: &str) {
        let Some(mut copy) = self.cards.with_untracked(|c| c.iter().find(|x| x.uid == uid).cloned()) else { return };
        let base = copy.name.trim_end_matches(|c: char| c.is_ascii_digit()).trim_end().to_string();
        copy.name = self.cards.with_untracked(|c| next_npc_name(c, &base));
        copy.uid = new_uid();
        copy.clear_damage();
        copy.willpower = copy.willpower_max;
        copy.essence = copy.essence_max.unwrap_or(copy.essence);
        self.push(copy);
    }

    fn push(&self, card: NpcCard) {
        let uid = card.uid.clone();
        self.cards.update(|c| c.push(card));
        self.expanded.set(Some(uid));
        self.open.set(true);
    }

    pub fn remove(&self, uid: &str) {
        self.cards.update(|c| c.retain(|x| x.uid != uid));
    }

    pub fn update_card(&self, uid: &str, f: impl FnOnce(&mut NpcCard)) {
        self.cards.update(|c| {
            if let Some(card) = c.iter_mut().find(|x| x.uid == uid) {
                f(card);
            }
        });
    }
}

fn new_uid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Cria o contexto das mini fichas e liga a lista ao navegador (só no cliente).
pub fn provide_npc_tray() -> NpcTray {
    let tray = NpcTray::new();
    let loaded = store_value(false);
    create_effect(move |_| {
        let json = tray.cards.with(|c| serde_json::to_string(c).unwrap_or_default());
        if !loaded.get_value() {
            loaded.set_value(true);
            if let Some(saved) = load().and_then(|s| serde_json::from_str::<Vec<NpcCard>>(&s).ok()) {
                tray.cards.set(saved);
            }
            return;
        }
        save(&json);
    });
    provide_context(tray);
    tray
}

pub fn use_npc_tray() -> Option<NpcTray> {
    use_context::<NpcTray>()
}

fn load() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_sys::window().and_then(|w| w.local_storage().ok()).flatten()?;
        return storage.get_item(STORAGE_KEY).ok().flatten();
    }
    #[cfg(not(target_arch = "wasm32"))]
    None
}

fn save(json: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.set_item(STORAGE_KEY, json);
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (json, STORAGE_KEY);
}

/// Verbetes com números para virar mini ficha (fora ficam totens, memóforos e Sendings).
pub fn has_mini_sheet(entry: &BestiaryEntry) -> bool {
    if entry.is_article {
        return false;
    }
    let q = entry.quick_stats();
    q.willpower.is_some() || !q.health_levels.is_empty() || q.essence.is_some()
}

/// Linhas da ficha que a mini ficha e o bloco do compêndio já desenham (Atributos,
/// Força de Vontade, vitalidade, traços de espírito) ou que ficam no texto (Imagem, Notas).
pub fn is_drawn_stat_label(label: &str) -> bool {
    label.contains("Attributes")
        || label.contains("Atributos")
        || label == "Willpower"
        || label == "Força de Vontade"
        || label.contains("Health Levels")
        || label.contains("Níveis de Vitalidade")
        || label == "Traits"
        || label == "Traços"
}

pub fn is_flavor_label(label: &str) -> bool {
    matches!(label, "Image" | "Imagem" | "Roleplaying Notes" | "Notas de Interpretação")
}

/// Linhas da ficha no idioma da tela.
pub fn stat_lines_for(entry: &BestiaryEntry, lang: Language) -> Vec<(&'static str, &'static str)> {
    match lang {
        Language::PtBr if entry.has_pt_text() => entry.stat_lines_pt(),
        _ => entry.stat_lines(),
    }
}

/// Nome do Atributo no idioma da tela.
pub fn attribute_label(name: &str, lang: Language) -> &str {
    if lang == Language::EnUs {
        return name;
    }
    match name {
        "Strength" => "Força",
        "Dexterity" => "Destreza",
        "Stamina" => "Vigor",
        "Charisma" => "Carisma",
        "Manipulation" => "Manipulação",
        "Appearance" => "Aparência",
        "Perception" => "Percepção",
        "Intelligence" => "Inteligência",
        "Wits" => "Raciocínio",
        other => other,
    }
}
