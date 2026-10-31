//! Mini ficha descartável de NPC: os números de uma ficha pronta do Apêndice I
//! (ou de um NPC em branco) com vitalidade, Força de Vontade e Essência para marcar
//! durante a cena. Não vai para o banco; o app guarda só no navegador.

use serde::{Deserialize, Serialize};
use crate::compendium::bestiary::BestiaryEntry;
use crate::i18n::Language;
use crate::rules::{apply_health_penalty_to_initiative, normalize_health_counts};
use super::traits::DamageType;

/// Os nove Atributos, na ordem da ficha (Físicos, Sociais, Mentais).
pub const NPC_ATTRIBUTES: [&str; 9] = [
    "Strength", "Dexterity", "Stamina",
    "Charisma", "Manipulation", "Appearance",
    "Perception", "Intelligence", "Wits",
];

/// Nível de vitalidade padrão de um mortal no M20.
const STANDARD_HEALTH: [&str; 7] = ["OK", "-1", "-1", "-2", "-2", "-5", "Incapacitated"];

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NpcAttribute {
    pub name: String,
    pub value: i32,
    /// Faixa do livro quando ela existe ("3-4").
    #[serde(default)]
    pub range: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NpcCard {
    pub uid: String,
    /// ID do verbete do Apêndice I de onde a ficha saiu (None = NPC em branco).
    #[serde(default)]
    pub source_id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub attributes: Vec<NpcAttribute>,
    pub willpower_max: i32,
    pub willpower: i32,
    /// Rótulos do livro, um por nível ("OK", "-1", ..., "Incapacitated").
    #[serde(default)]
    pub health_labels: Vec<String>,
    /// Dano marcado: (agravado, letal, contundente), sempre nessa ordem nas caixas.
    #[serde(default)]
    pub damage: (usize, usize, usize),
    #[serde(default)]
    pub arete: Option<i32>,
    #[serde(default)]
    pub rage: Option<i32>,
    #[serde(default)]
    pub gnosis: Option<i32>,
    #[serde(default)]
    pub essence_max: Option<i32>,
    #[serde(default)]
    pub essence: i32,
    #[serde(default)]
    pub initiative_base: Option<i32>,
    #[serde(default)]
    pub notes: String,
}

impl NpcCard {
    /// NPC genérico: Atributos 2, Força de Vontade 5 e vitalidade de mortal.
    pub fn blank(uid: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            uid: uid.into(),
            source_id: None,
            name: name.into(),
            attributes: NPC_ATTRIBUTES.iter().map(|n| NpcAttribute { name: n.to_string(), value: 2, range: None }).collect(),
            willpower_max: 5,
            willpower: 5,
            health_labels: STANDARD_HEALTH.iter().map(|s| s.to_string()).collect(),
            damage: (0, 0, 0),
            arete: None,
            rage: None,
            gnosis: None,
            essence_max: None,
            essence: 0,
            initiative_base: Some(4),
            notes: String::new(),
        }
    }

    /// Monta a mini ficha com os números do verbete. Espíritos sem níveis de
    /// vitalidade usam a Essência como "vida".
    pub fn from_bestiary(entry: &BestiaryEntry, uid: impl Into<String>, name: impl Into<String>) -> Self {
        let q = entry.quick_stats();
        let attributes = entry
            .attribute_ranges()
            .into_iter()
            .map(|(name, lo, hi)| NpcAttribute { name, value: lo, range: (hi > lo).then(|| format!("{}-{}", lo, hi)) })
            .collect();
        let willpower_max = q.willpower.unwrap_or(0);
        Self {
            uid: uid.into(),
            source_id: Some(entry.id.to_string()),
            name: name.into(),
            attributes,
            willpower_max,
            willpower: willpower_max,
            health_labels: q.health_levels.clone(),
            damage: (0, 0, 0),
            arete: q.arete,
            rage: q.rage,
            gnosis: q.gnosis,
            essence_max: q.essence,
            essence: q.essence.unwrap_or(0),
            initiative_base: q.initiative_base(),
            notes: String::new(),
        }
    }

    pub fn attribute(&self, name: &str) -> Option<i32> {
        self.attributes.iter().find(|a| a.name.eq_ignore_ascii_case(name)).map(|a| a.value)
    }

    pub fn set_attribute(&mut self, name: &str, value: i32) {
        let value = value.clamp(0, 10);
        match self.attributes.iter_mut().find(|a| a.name.eq_ignore_ascii_case(name)) {
            Some(a) => a.value = value,
            None => self.attributes.push(NpcAttribute { name: name.to_string(), value, range: None }),
        }
        if let (Some(d), Some(w)) = (self.attribute("Dexterity"), self.attribute("Wits")) {
            self.initiative_base = Some(d + w);
        }
    }

    pub fn has_health_track(&self) -> bool {
        !self.health_labels.is_empty()
    }

    pub fn health_boxes(&self) -> usize {
        self.health_labels.len()
    }

    pub fn total_damage(&self) -> usize {
        self.damage.0 + self.damage.1 + self.damage.2
    }

    pub fn damage_at(&self, i: usize) -> DamageType {
        let (a, l, b) = self.damage;
        if i < a {
            DamageType::Aggravated
        } else if i < a + l {
            DamageType::Lethal
        } else if i < a + l + b {
            DamageType::Bashing
        } else {
            DamageType::None
        }
    }

    fn set_damage(&mut self, a: usize, l: usize, b: usize) {
        self.damage = normalize_health_counts(a, l, b, self.health_boxes());
    }

    /// Clique numa caixa: igual à ficha (vazia → contundente até ela; / → ✕ → ✳).
    pub fn click_box(&mut self, index: usize) {
        if index >= self.health_boxes() {
            return;
        }
        let (mut a, mut l, mut b) = self.damage;
        let total = a + l + b;
        match self.damage_at(index) {
            DamageType::None => b += index + 1 - total,
            DamageType::Bashing => {
                b -= 1;
                l += 1;
            }
            DamageType::Lethal => {
                l -= 1;
                a += 1;
            }
            DamageType::Aggravated => {
                if total == 1 {
                    a = 0;
                    b = 1;
                } else {
                    a -= 1;
                }
            }
        }
        self.set_damage(a, l, b);
    }

    /// Marca `amount` pontos de dano. Contundente que passa do fim da trilha vira
    /// letal; o resto que não cabe se perde.
    pub fn apply_damage(&mut self, kind: DamageType, amount: usize) {
        let n = self.health_boxes();
        let (mut a, mut l, mut b) = self.damage;
        match kind {
            DamageType::Aggravated => a += amount,
            DamageType::Lethal => l += amount,
            DamageType::Bashing => b += amount,
            DamageType::None => return,
        }
        while a + l + b > n {
            if b > 0 {
                b -= 1;
                if b > 0 {
                    b -= 1;
                    l += 1;
                }
            } else if l > 0 {
                l -= 1;
            } else {
                a -= 1;
            }
        }
        self.set_damage(a, l, b);
    }

    /// Cura `amount` pontos, do dano mais leve para o mais grave.
    pub fn heal(&mut self, amount: usize) {
        let (mut a, mut l, mut b) = self.damage;
        for _ in 0..amount {
            if b > 0 {
                b -= 1;
            } else if l > 0 {
                l -= 1;
            } else if a > 0 {
                a -= 1;
            }
        }
        self.set_damage(a, l, b);
    }

    pub fn clear_damage(&mut self) {
        self.damage = (0, 0, 0);
    }

    /// Fora de combate: a última caixa da trilha está marcada.
    pub fn is_down(&self) -> bool {
        self.has_health_track() && self.total_damage() >= self.health_boxes()
    }

    /// Penalidade de dados do nível atual (0, 1, 2, 3 ou 5).
    pub fn wound_penalty(&self) -> i32 {
        let d = self.total_damage();
        if d == 0 || self.is_down() {
            return 0;
        }
        self.health_labels
            .get(d - 1)
            .and_then(|l| l.trim().strip_prefix('-'))
            .and_then(|n| n.trim().parse::<i32>().ok())
            .unwrap_or(0)
    }

    /// Estado em palavras: "Ferido (-1)", "Incapacitado"...
    pub fn health_status(&self, lang: Language) -> String {
        let pt = lang == Language::PtBr;
        if !self.has_health_track() {
            return match (self.essence_max, pt) {
                (Some(max), true) => format!("Essência {}/{}", self.essence, max),
                (Some(max), false) => format!("Essence {}/{}", self.essence, max),
                (None, true) => "Sem vitalidade".into(),
                (None, false) => "No health track".into(),
            };
        }
        if self.total_damage() == 0 {
            return if pt { "Ileso".into() } else { "Unhurt".into() };
        }
        if self.is_down() {
            let last = self.health_labels.last().map(String::as_str).unwrap_or("Incapacitated");
            return if pt { final_label_pt(last).to_string() } else { last.to_string() };
        }
        let name = match (self.wound_penalty(), pt) {
            (0, true) => "Escoriado",
            (0, false) => "Bruised",
            (1, true) => "Ferido",
            (1, false) => "Hurt",
            (2, true) => "Espancado",
            (2, false) => "Mauled",
            (_, true) => "Aleijado",
            (_, false) => "Crippled",
        };
        match self.wound_penalty() {
            0 => name.to_string(),
            p => format!("{} (-{})", name, p),
        }
    }

    /// Base de iniciativa já com a penalidade de ferimento (piso 2, como na ficha).
    pub fn initiative_now(&self) -> Option<i32> {
        self.initiative_base.map(|b| apply_health_penalty_to_initiative(b, self.wound_penalty()))
    }

    pub fn set_willpower(&mut self, value: i32) {
        self.willpower = value.clamp(0, self.willpower_max);
    }

    pub fn adjust_essence(&mut self, delta: i32) {
        let max = self.essence_max.unwrap_or(i32::MAX);
        self.essence = (self.essence + delta).clamp(0, max);
    }
}

/// Nome do último nível de vitalidade em português.
pub fn final_label_pt(label: &str) -> &'static str {
    match label {
        "Terminated" => "Terminado",
        "Vaporized" => "Vaporizado",
        "Dead" => "Morto",
        _ => "Incapacitado",
    }
}

/// Próximo nome livre: "Valentão", "Valentão 2", "Valentão 3"...
pub fn next_npc_name(existing: &[NpcCard], base: &str) -> String {
    if !existing.iter().any(|c| c.name == base) {
        return base.to_string();
    }
    (2..).map(|n| format!("{} {}", base, n)).find(|cand| !existing.iter().any(|c| &c.name == cand)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compendium::bestiary::find_bestiary_entry;

    fn thug() -> NpcCard {
        let e = find_bestiary_entry("thug").expect("thug");
        NpcCard::from_bestiary(e, "u1", "Thug")
    }

    #[test]
    fn test_from_bestiary_reads_book_numbers() {
        let c = thug();
        assert_eq!(c.willpower, 3);
        assert_eq!(c.health_boxes(), 7);
        assert_eq!(c.attribute("Strength"), Some(3));
        assert_eq!(c.initiative_base, Some(4));

        let cop = NpcCard::from_bestiary(find_bestiary_entry("wolf").expect("wolf"), "u2", "Wolf");
        assert_eq!(cop.attributes.iter().find(|a| a.name == "Strength").and_then(|a| a.range.clone()), Some("3-4".into()));

        let air = NpcCard::from_bestiary(find_bestiary_entry("air_elemental").expect("air"), "u3", "Air");
        assert!(!air.has_health_track());
        assert_eq!((air.essence, air.essence_max), (40, Some(40)));
    }

    #[test]
    fn test_damage_penalty_and_status() {
        let mut c = thug();
        c.apply_damage(DamageType::Bashing, 2);
        assert_eq!(c.wound_penalty(), 1);
        assert_eq!(c.health_status(Language::PtBr), "Ferido (-1)");
        assert_eq!(c.initiative_now(), Some(3));

        c.apply_damage(DamageType::Lethal, 1);
        assert_eq!(c.damage, (0, 1, 2));
        assert_eq!(c.damage_at(0), DamageType::Lethal);

        c.heal(2);
        assert_eq!(c.damage, (0, 1, 0));

        c.apply_damage(DamageType::Aggravated, 10);
        assert!(c.is_down());
        assert_eq!(c.health_status(Language::PtBr), "Incapacitado");
    }

    #[test]
    fn test_bashing_overflow_becomes_lethal() {
        let mut c = thug();
        c.apply_damage(DamageType::Bashing, 7);
        assert_eq!(c.damage, (0, 0, 7));
        c.apply_damage(DamageType::Bashing, 1);
        assert_eq!(c.damage, (0, 1, 6));
    }

    #[test]
    fn test_click_box_cycles_like_the_sheet() {
        let mut c = thug();
        c.click_box(2);
        assert_eq!(c.damage, (0, 0, 3));
        c.click_box(0);
        assert_eq!(c.damage_at(0), DamageType::Lethal);
    }

    #[test]
    fn test_next_name_numbers_copies() {
        let cards = vec![NpcCard::blank("a", "Thug"), NpcCard::blank("b", "Thug 2")];
        assert_eq!(next_npc_name(&cards, "Thug"), "Thug 3");
        assert_eq!(next_npc_name(&cards, "Wolf"), "Wolf");
    }
}
