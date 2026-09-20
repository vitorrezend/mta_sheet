//! M20 Compêndio Modular de Armas, Manobras & Combate
//!
//! Organizado em submódulos coesos (< 1000 linhas cada):
//! - `models`: Tipos de dados, enums e contratos
//! - `rule_notes`: Notas de regras canônicas (#1-#10 Melee, #1-#12 Ranged)
//! - `melee`: 42 armas brancas e de impacto
//! - `ranged`: 40 armas de fogo, arcos e arremesso
//! - `maneuvers`: 44 manobras de combate corporais e marciais
//! - `do_canon`: Truque do Golpe Trovão, Oito Membros e Regras de Dô

pub mod models;
pub mod rule_notes;
pub mod melee;
pub mod ranged;
pub mod maneuvers;
pub mod do_canon;

pub use models::*;
pub use rule_notes::*;
pub use melee::*;
pub use ranged::*;
pub use maneuvers::*;
pub use do_canon::*;

// ============================================================================
// Catálogo Unificado Canônico de M20 (Exatamente 82 Armas)
// ============================================================================

pub const ALL_WEAPONS: &[WeaponDefinition] = &[
    // --- Melee (42) ---
    // Axes (5)
    HATCHET, TOMAHAWK, AXE, GREAT_AXE, POLEARM,
    // Blades (8)
    STILETTO, KNIFE, SHORT_SWORD, SWORD, KATANA, GREAT_SWORD, SAI, HOOK_SWORD,
    // Clubbing (9)
    RIOT_BATON, BASEBALL_BAT, CROWBAR, STAFF, IRON_STAFF, MACE, NUNCHAKU, SPIKED_CLUB, HUGE_SPIKED_CLUB,
    // Fist-Extension (8)
    SAP, BRASS_KNUCKLES, SPIKED_GAUNTLET, HAND_CLAWS_SMALL, HAND_CLAWS_LARGE, KATAR, WAR_FAN, WIND_AND_FIRE_WHEEL,
    // Improvised (4)
    BROKEN_BOTTLE, CHAIR, CHAINSAW, TABLE,
    // Whips and Chains (8)
    CHAIN, CHAIN_WHIP, KUSARIGAMA, MANRIKI_GUSARI, FLOGGER, BARBED_CAT, WHIP, BULLWHIP,

    // --- Ranged (34) ---
    // Conventional Guns (12)
    REVOLVER_LT, REVOLVER_HVY, PISTOL_LT, PISTOL_HVY, RIFLE_HUNTING,
    SMG_SMALL, SMG_LARGE, ASSAULT_RIFLE, SHOTGUN_SAWED_OFF, SHOTGUN_PUMP, SHOTGUN_SEMI_AUTO, SHOTGUN_ASSAULT,
    // Technocracy Sidearms (6)
    BIGGS_X5_MODEL_R, BIGGS_X5_MODEL_A, BIGGS_MJOLLNER_MK4, CASTLE_GRAVES_WW3, BOLAN_MK13_SYSTEM, HIT_MARK_CHAIN_GUN,
    // Bows & Crossbows (6)
    SHORT_BOW, HUNTING_BOW, LONG_BOW, CROSSBOW_COMMANDO, CROSSBOW, CROSSBOW_HVY,
    // Non-Lethal (3)
    TASER, TEAR_GAS, PACIFICATION_SPRAY,
    // Military Weapons (7)
    MACHINE_GUN_30, MACHINE_GUN_50, CANNON_30MM, GRENADE_LAUNCHER_M79, GRENADE_LAUNCHER_M19, FLAMETHROWER, ROCKET_LAUNCHER,

    // --- Thrown (6) ---
    THROWN_KNIFE, SHURIKEN, THROWN_SPEAR, STONE, STONE_LARGE, THROWN_TOMAHAWK,
];

pub fn find_weapon(query: &str) -> Option<&'static WeaponDefinition> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }

    // 1. Match exato por id
    if let Some(w) = ALL_WEAPONS.iter().find(|w| w.id.eq_ignore_ascii_case(&q)) {
        return Some(w);
    }

    // 2. Match exato por name ou name_pt
    if let Some(w) = ALL_WEAPONS.iter().find(|w| {
        w.name.to_lowercase() == q || w.name_pt.to_lowercase() == q
    }) {
        return Some(w);
    }

    // 3. Match em aliases
    if let Some(w) = ALL_WEAPONS.iter().find(|w| {
        w.aliases.iter().any(|alias| alias.to_lowercase() == q)
    }) {
        return Some(w);
    }

    // 4. Substring no nome
    ALL_WEAPONS.iter().find(|w| {
        w.name.to_lowercase().contains(&q)
            || w.name_pt.to_lowercase().contains(&q)
            || w.aliases.iter().any(|alias| alias.to_lowercase().contains(&q))
    })
}

pub fn get_weapons_by_category(cat: WeaponCategory) -> Vec<&'static WeaponDefinition> {
    ALL_WEAPONS.iter().filter(|w| w.category == cat).collect()
}

pub fn get_weapons_by_class(w_class: WeaponClass) -> Vec<&'static WeaponDefinition> {
    ALL_WEAPONS.iter().filter(|w| w.category.weapon_class() == w_class).collect()
}

pub fn get_weapons_by_main_group(group: WeaponMainGroup) -> Vec<&'static WeaponDefinition> {
    ALL_WEAPONS.iter().filter(|w| w.category.main_group() == group).collect()
}

pub fn get_categories_by_main_group(group: WeaponMainGroup) -> &'static [WeaponCategory] {
    match group {
        WeaponMainGroup::Melee => MELEE_CATEGORIES,
        WeaponMainGroup::Ranged => RANGED_CATEGORIES,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatEntity {
    Weapon(&'static WeaponDefinition),
    Maneuver(&'static CombatManeuver),
}

impl CombatEntity {
    pub fn id(&self) -> &'static str {
        match self {
            CombatEntity::Weapon(w) => w.id,
            CombatEntity::Maneuver(m) => m.id,
        }
    }

    pub fn name(&self, lang: crate::i18n::Language) -> &'static str {
        match self {
            CombatEntity::Weapon(w) => w.name(lang),
            CombatEntity::Maneuver(m) => m.name(lang),
        }
    }

    pub fn damage(&self, lang: crate::i18n::Language) -> &'static str {
        match self {
            CombatEntity::Weapon(w) => w.damage(lang),
            CombatEntity::Maneuver(m) => m.damage(lang),
        }
    }

    pub fn difficulty(&self, lang: crate::i18n::Language) -> String {
        match self {
            CombatEntity::Weapon(w) => w.difficulty.to_string(),
            CombatEntity::Maneuver(m) => m.difficulty(lang).to_string(),
        }
    }

    pub fn range(&self, lang: crate::i18n::Language) -> &'static str {
        match self {
            CombatEntity::Weapon(w) => {
                let r = w.range.trim();
                if r == "Close" || r == "C/C" {
                    match lang {
                        crate::i18n::Language::PtBr => "C/C",
                        crate::i18n::Language::EnUs => "Close",
                    }
                } else {
                    w.range
                }
            }
            CombatEntity::Maneuver(_) => match lang {
                crate::i18n::Language::PtBr => "C/C",
                crate::i18n::Language::EnUs => "Close",
            },
        }
    }

    pub fn notes(&self, lang: crate::i18n::Language) -> String {
        match self {
            CombatEntity::Weapon(w) => w.format_notes_str(lang),
            CombatEntity::Maneuver(m) => m.requirement(lang).to_string(),
        }
    }
}

/// Busca inteligente e resiliente para encontrar Arma ou Manobra de Combate
/// a partir de um nome digitado ou equipado na ficha (incluindo variações com parênteses,
/// prefixos ou sufixos personalizados).
pub fn find_combat_entity(query: &str) -> Option<CombatEntity> {
    let q = query.trim();
    if q.is_empty() {
        return None;
    }

    let q_lower = q.to_lowercase();

    // 1. Match exato por ID em Armas e Manobras
    if let Some(w) = ALL_WEAPONS.iter().find(|w| w.id.eq_ignore_ascii_case(q)) {
        return Some(CombatEntity::Weapon(w));
    }
    if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| m.id.eq_ignore_ascii_case(q)) {
        return Some(CombatEntity::Maneuver(m));
    }

    // 2. Match exato por Nome (EN ou PT) em Armas e Manobras
    // Garante que "Soco" bata na manobra Soco (punch) e não na arma "Soco Inglês"
    if let Some(w) = ALL_WEAPONS.iter().find(|w| {
        w.name.to_lowercase() == q_lower || w.name_pt.to_lowercase() == q_lower
    }) {
        return Some(CombatEntity::Weapon(w));
    }
    if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| {
        m.name.to_lowercase() == q_lower || m.name_pt.to_lowercase() == q_lower
    }) {
        return Some(CombatEntity::Maneuver(m));
    }

    // 3. Match em aliases conhecidos
    if let Some(w) = ALL_WEAPONS.iter().find(|w| {
        w.aliases.iter().any(|alias| alias.eq_ignore_ascii_case(q))
    }) {
        return Some(CombatEntity::Weapon(w));
    }
    match q_lower.as_str() {
        "thunder punch" | "soco trovão" | "soco trovao" => {
            if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "punch") {
                return Some(CombatEntity::Maneuver(m));
            }
        }
        "chute trovão" | "chute do trovao" | "chute trovao" => {
            if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "thunder_kick") {
                return Some(CombatEntity::Maneuver(m));
            }
        }
        "strike vital point" | "golpe em ponto vital" | "ponto vital" => {
            if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "vital_strike") {
                return Some(CombatEntity::Maneuver(m));
            }
        }
        "roundhouse" | "roundhouse kick" | "chute circular" => {
            if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "spinning_kick") {
                return Some(CombatEntity::Maneuver(m));
            }
        }
        "nerve strike" | "pressure point strike" | "pressure point" | "golpe no nervo" => {
            if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "nerve_strike") {
                return Some(CombatEntity::Maneuver(m));
            }
        }
        "elbow strike" | "knee strike" | "golpe de cotovelo" | "joelhada" => {
            if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "elbow_knee_strike") {
                return Some(CombatEntity::Maneuver(m));
            }
        }
        "hard strike" | "soft strike" | "golpe duro" | "golpe suave" => {
            if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "hard_soft_strike") {
                return Some(CombatEntity::Maneuver(m));
            }
        }
        _ => {}
    }

    // 4. Se contiver parênteses (ex: "Kiaijutsu (Grito de Ferro)" ou "Pistola Leve (Glock)"),
    // testa a parte antes dos parênteses e o conteúdo dentro
    if let Some((before, after)) = q.split_once('(') {
        let clean_before = before.trim();
        if !clean_before.is_empty() {
            if let Some(entity) = find_combat_entity(clean_before) {
                return Some(entity);
            }
        }
        let inside = after.trim_end_matches(')').trim();
        if !inside.is_empty() {
            if let Some(entity) = find_combat_entity(inside) {
                return Some(entity);
            }
        }
    }

    // 5. Match por inclusão da arma ou manobra dentro da query (preferindo o match de maior comprimento)
    // Ex: "Pistola Pesada (Glock 9mm)" contém "Pistola Pesada" (14 chars) vs "Pistola" (7 chars)
    // "Espada Longa +1" contém "Espada Longa"
    let mut best_weapon: Option<&'static WeaponDefinition> = None;
    let mut max_w_len = 0;

    for w in ALL_WEAPONS {
        let n_en = w.name.to_lowercase();
        let n_pt = w.name_pt.to_lowercase();
        if q_lower.contains(&n_en) && n_en.len() > max_w_len {
            max_w_len = n_en.len();
            best_weapon = Some(w);
        }
        if q_lower.contains(&n_pt) && n_pt.len() > max_w_len {
            max_w_len = n_pt.len();
            best_weapon = Some(w);
        }
        for alias in w.aliases {
            let a = alias.to_lowercase();
            if q_lower.contains(&a) && a.len() > max_w_len {
                max_w_len = a.len();
                best_weapon = Some(w);
            }
        }
    }

    let mut best_maneuver: Option<&'static CombatManeuver> = None;
    let mut max_m_len = 0;

    for m in ALL_COMBAT_MANEUVERS {
        let n_en = m.name.to_lowercase();
        let n_pt = m.name_pt.to_lowercase();
        if q_lower.contains(&n_en) && n_en.len() > max_m_len {
            max_m_len = n_en.len();
            best_maneuver = Some(m);
        }
        if q_lower.contains(&n_pt) && n_pt.len() > max_m_len {
            max_m_len = n_pt.len();
            best_maneuver = Some(m);
        }
    }

    if max_w_len >= max_m_len && max_w_len > 0 {
        if let Some(w) = best_weapon {
            return Some(CombatEntity::Weapon(w));
        }
    } else if max_m_len > 0 {
        if let Some(m) = best_maneuver {
            return Some(CombatEntity::Maneuver(m));
        }
    }

    // 6. Substring padrão de fallback nas armas e manobras
    if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| {
        m.name.to_lowercase().contains(&q_lower) || m.name_pt.to_lowercase().contains(&q_lower)
    }) {
        return Some(CombatEntity::Maneuver(m));
    }

    if let Some(w) = ALL_WEAPONS.iter().find(|w| {
        w.name.to_lowercase().contains(&q_lower) || w.name_pt.to_lowercase().contains(&q_lower)
    }) {
        return Some(CombatEntity::Weapon(w));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thunder_punch_trick_integrity() {
        assert_eq!(THUNDER_PUNCH_TRICK.id, "thunder_punch");
        assert_eq!(THUNDER_PUNCH_TRICK.page_ref, "M20, p. 449");
        assert_eq!(THUNDER_PUNCH_TRICK.paragraphs.len(), 3);
        assert_eq!(THUNDER_PUNCH_TRICK.paragraphs_pt.len(), 3);
        assert_eq!(THUNDER_PUNCH_TRICK.sphere_tags.len(), 8);
        assert_eq!(THUNDER_PUNCH_TRICK.sphere_tags_pt.len(), 8);

        // Termos canônicos em inglês
        assert!(THUNDER_PUNCH_TRICK.paragraphs[0].contains("Correspondence 1"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs[0].contains("maximum adjustment of -3"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs[1].contains("Life 3 or Prime 3 Pattern assault"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs[2].contains("If the target manages to soak every level of damage"));

        // Termos canônicos em português
        assert!(THUNDER_PUNCH_TRICK.paragraphs_pt[0].contains("Correspondência 1"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs_pt[0].contains("ajuste máximo de -3"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs_pt[1].contains("Vida 3 ou Primórdio 3"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs_pt[2].contains("Se o alvo conseguir absorver cada nível de dano"));
    }
}
