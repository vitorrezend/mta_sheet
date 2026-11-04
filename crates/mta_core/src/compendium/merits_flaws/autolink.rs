//! Links automáticos entre verbetes nos textos de Qualidades e Defeitos.
//!
//! O livro cita outros Traços sempre com uma expressão de referência: "the Flaw: Echoes",
//! "Merits: Twin Souls or Shattered Avatar", "Backgrounds like Cult, Demesne, and Past Life",
//! "Defeito Mental: Mente Feral", "Antecedentes como Culto". Só os nomes logo após essas
//! expressões viram links (`[nome](mta://merits_flaws/id)` ou `mta://backgrounds/id`), para não
//! transformar palavras comuns em links. Links que já existem no texto ficam como estão, e um
//! verbete nunca aponta para si mesmo. O resultado é calculado uma vez e guardado em cache.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use super::models::TraitType;
use super::ALL_MERITS_FLAWS;
use crate::compendium::backgrounds::ALL_BACKGROUNDS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Merit,
    Flaw,
    Background,
}

/// Nome (como aparece no texto) → destino. Ordenado do nome mais longo para o mais curto.
struct Entry {
    name: String,
    uri: String,
    id: &'static str,
    kind: Kind,
}

/// Formas alternativas usadas no texto do livro e das traduções.
const MERIT_FLAW_ALIASES: &[(&str, &str)] = &[
    ("Twin Soul", "twin_souls"),
    ("Nephilim", "nephilim_laham"),
    ("Nefilim", "nephilim_laham"),
    ("Jinx", "jinx_infernal_contraption"),
    ("Item Azarado", "jinx_infernal_contraption"),
    ("Hideaway", "hideaway_safehouse"),
    ("Safehouse", "hideaway_safehouse"),
    ("Quantum Voyager", "stormwarden"),
    ("Esoteric Discourse", "esoteric_discourse_technobabbler"),
    ("Technobabbler", "esoteric_discourse_technobabbler"),
    ("Scientific Mystic", "scientific_mystic_techgnosi"),
    ("Techgnosi", "scientific_mystic_techgnosi"),
    ("Insane", "insane_infamous_mentor"),
];

const BACKGROUND_ALIASES: &[(&str, &str)] = &[
    ("Enhancements", "enhancement"),
    ("Aprimoramentos", "enhancement"),
    ("Past Life", "past_lives"),
    ("Vida Passada", "past_lives"),
    ("Ally", "allies"),
    ("Aliado", "allies"),
    ("Secret Weapon", "secret_weapons"),
    ("Arma Secreta", "secret_weapons"),
    ("Wonders", "wonder"),
    ("Maravilhas", "wonder"),
    ("Retainer", "retainers"),
    ("Spy", "spies"),
];

fn push_names(out: &mut Vec<Entry>, full: &str, uri: String, id: &'static str, kind: Kind) {
    let clean = full.trim_matches(|c| c == '“' || c == '”' || c == '"').trim();
    if clean.is_empty() {
        return;
    }
    out.push(Entry { name: clean.to_string(), uri: uri.clone(), id, kind });
    // "Arcane / Cloaking", "Hideaway/Safehouse": cada parte também vale sozinha
    if clean.contains('/') {
        for part in clean.split('/') {
            let part = part.trim();
            if part.len() > 2 {
                out.push(Entry { name: part.to_string(), uri: uri.clone(), id, kind });
            }
        }
    }
}

static INDEX: LazyLock<Vec<Entry>> = LazyLock::new(|| {
    let mut out = Vec::new();
    for mf in ALL_MERITS_FLAWS.iter() {
        let kind = if mf.trait_type == TraitType::Merit { Kind::Merit } else { Kind::Flaw };
        let uri = format!("mta://merits_flaws/{}", mf.id);
        push_names(&mut out, mf.name, uri.clone(), mf.id, kind);
        push_names(&mut out, mf.name_pt, uri.clone(), mf.id, kind);
        if let Some(t) = mf.technocracy_name {
            push_names(&mut out, t, uri.clone(), mf.id, kind);
        }
    }
    for &(alias, id) in MERIT_FLAW_ALIASES {
        if let Some(mf) = ALL_MERITS_FLAWS.iter().find(|m| m.id == id) {
            let kind = if mf.trait_type == TraitType::Merit { Kind::Merit } else { Kind::Flaw };
            out.push(Entry { name: alias.to_string(), uri: format!("mta://merits_flaws/{}", mf.id), id: mf.id, kind });
        }
    }
    for bg in ALL_BACKGROUNDS.iter() {
        let uri = format!("mta://backgrounds/{}", bg.id);
        push_names(&mut out, bg.name, uri.clone(), bg.id, Kind::Background);
        push_names(&mut out, bg.name_pt, uri, bg.id, Kind::Background);
    }
    for &(alias, id) in BACKGROUND_ALIASES {
        if let Some(bg) = ALL_BACKGROUNDS.iter().find(|b| b.id == id) {
            out.push(Entry { name: alias.to_string(), uri: format!("mta://backgrounds/{}", bg.id), id: bg.id, kind: Kind::Background });
        }
    }
    out.sort_by(|a, b| b.name.chars().count().cmp(&a.name.chars().count()));
    out
});

/// Expressões de referência e o tipo de verbete que elas anunciam.
/// `adjective` = aceita uma palavra entre o termo e os dois-pontos ("Defeito Mental:").
const KEYWORDS: &[(&str, Kind)] = &[
    ("Merits", Kind::Merit),
    ("Merit", Kind::Merit),
    ("Flaws", Kind::Flaw),
    ("Flaw", Kind::Flaw),
    ("Backgrounds", Kind::Background),
    ("Background", Kind::Background),
    ("Qualidades", Kind::Merit),
    ("Qualidade", Kind::Merit),
    ("Defeitos", Kind::Flaw),
    ("Defeito", Kind::Flaw),
    ("Antecedentes", Kind::Background),
    ("Antecedente", Kind::Background),
];

const SEPARATORS: &[&str] = &[", and ", ", or ", ", e ", ", ou ", ", ", " and ", " or ", " e ", " ou ", " / ", "/"];

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '\''
}

/// Depois da palavra-chave, onde começa a lista de nomes (ou None se não for uma referência).
fn after_keyword(rest: &str) -> Option<usize> {
    for lead in [": ", " like ", " como "] {
        if rest.starts_with(lead) {
            return Some(lead.len());
        }
    }
    // Uma palavra entre o termo e os dois-pontos: "Defeito Mental: ", "Qualidades Sobrenaturais: "
    let word: String = rest.strip_prefix(' ')?.chars().take_while(|c| c.is_alphabetic()).collect();
    if word.is_empty() {
        return None;
    }
    let skip = 1 + word.len();
    rest[skip..].starts_with(": ").then_some(skip + 2)
}

/// Nome conhecido no início de `s`, respeitando o tipo anunciado. Devolve (início, fim, destino) em bytes:
/// as aspas “…” entram no link quando fecham logo após o nome; senão o link começa depois delas.
fn match_name(s: &str, wanted: Kind, self_ids: &[&str]) -> Option<(usize, usize, &'static Entry)> {
    let (open, body) = match s.strip_prefix('“') {
        Some(b) => ('“'.len_utf8(), b),
        None => (0, s),
    };
    let candidates = INDEX.iter().filter(|e| match wanted {
        Kind::Background => e.kind == Kind::Background,
        _ => e.kind != Kind::Background,
    });
    let mut best: Option<(usize, usize, &'static Entry)> = None;
    for e in candidates {
        if !body.starts_with(e.name.as_str()) {
            continue;
        }
        let end = open + e.name.len();
        if s[end..].chars().next().is_some_and(is_word_char) {
            continue;
        }
        let (start, end) = match (open > 0, s[end..].starts_with('”')) {
            (true, true) => (0, end + '”'.len_utf8()),
            (true, false) => (open, end),
            _ => (0, end),
        };
        // Entre homônimos (Sterile, Cyclic Magick…), fica o do tipo anunciado
        let better = match best {
            None => true,
            Some((_, best_end, b)) => end > best_end || (end == best_end && b.kind != wanted && e.kind == wanted),
        };
        if better {
            best = Some((start, end, e));
        }
    }
    best.filter(|(_, _, e)| !self_ids.contains(&e.id))
}

/// Liga os nomes de uma lista após a palavra-chave. Devolve o texto com links e quantos bytes consumiu.
fn link_list(s: &str, wanted: Kind, self_ids: &[&str], out: &mut String) -> usize {
    let mut pos = 0;
    let Some((start, end, entry)) = match_name(s, wanted, self_ids) else { return 0 };
    out.push_str(&s[..start]);
    out.push_str(&format!("[{}]({})", &s[start..end], entry.uri));
    pos += end;
    loop {
        let rest = &s[pos..];
        let Some(sep) = SEPARATORS.iter().find(|sep| rest.starts_with(**sep)) else { break };
        let after = &rest[sep.len()..];
        let Some((start, end, entry)) = match_name(after, wanted, self_ids) else { break };
        out.push_str(sep);
        out.push_str(&after[..start]);
        out.push_str(&format!("[{}]({})", &after[start..end], entry.uri));
        pos += sep.len() + end;
    }
    pos
}

/// Aplica os links num trecho sem links existentes.
fn link_plain(text: &str, self_ids: &[&str], out: &mut String) {
    let mut i = 0;
    while i < text.len() {
        let rest = &text[i..];
        let at_word_start = i == 0 || !text[..i].chars().next_back().is_some_and(is_word_char);
        let hit = if at_word_start {
            KEYWORDS.iter().find_map(|&(kw, kind)| {
                let after = rest.strip_prefix(kw)?;
                let lead = after_keyword(after)?;
                Some((kw.len() + lead, kind))
            })
        } else {
            None
        };
        if let Some((skip, kind)) = hit {
            out.push_str(&rest[..skip]);
            let consumed = link_list(&rest[skip..], kind, self_ids, out);
            i += skip + consumed;
            continue;
        }
        let Some(c) = rest.chars().next() else { break };
        out.push(c);
        i += c.len_utf8();
    }
}

/// Texto com links automáticos (sem mexer nos links que já existem).
pub fn autolink(text: &str, self_ids: &[&str]) -> String {
    let mut out = String::with_capacity(text.len() + 256);
    let mut i = 0;
    while let Some(rel) = text[i..].find('[') {
        let start = i + rel;
        // Link markdown existente: [texto](destino)
        let existing = text[start..].find("](").and_then(|close| {
            let after = start + close + 2;
            text[after..].find(')').map(|end| after + end + 1)
        });
        match existing {
            Some(end) if !text[start + 1..end].contains('\n') => {
                link_plain(&text[i..start], self_ids, &mut out);
                out.push_str(&text[start..end]);
                i = end;
            }
            _ => {
                link_plain(&text[i..start + 1], self_ids, &mut out);
                i = start + 1;
            }
        }
    }
    link_plain(&text[i..], self_ids, &mut out);
    out
}

static CACHE: LazyLock<Mutex<HashMap<(usize, usize), &'static str>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Versão em cache de [`autolink`] para textos estáticos (cada texto é processado uma vez só).
pub fn autolink_static(text: &'static str, self_ids: &[&str]) -> &'static str {
    let key = (text.as_ptr() as usize, text.len());
    let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(done) = cache.get(&key) {
        return done;
    }
    let linked: &'static str = Box::leak(autolink(text, self_ids).into_boxed_str());
    cache.insert(key, linked);
    linked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_references_after_keywords() {
        let t = autolink("see the Flaw: Echoes, and the Background: Wonder.", &[]);
        assert_eq!(t, "see the Flaw: [Echoes](mta://merits_flaws/echoes), and the Background: [Wonder](mta://backgrounds/wonder).");
    }

    #[test]
    fn links_lists_with_separators_and_like() {
        let t = autolink("Merits like Legendary Attributes, Nine Lives, and Too Tough to Die, Flaws like", &[]);
        assert!(t.contains("[Legendary Attributes](mta://merits_flaws/legendary_attributes)"));
        assert!(t.contains("[Nine Lives](mta://merits_flaws/nine_lives)"));
        assert!(t.contains(", and [Too Tough to Die](mta://merits_flaws/too_tough_to_die)"));
        let b = autolink("Backgrounds like Cult, Demesne, and Past Life", &[]);
        assert!(b.contains("[Past Life](mta://backgrounds/past_lives)"), "{b}");
    }

    #[test]
    fn portuguese_keywords_with_adjective() {
        let t = autolink("veja o Defeito Mental: Mente Feral (p. 52) e a Qualidade: “Imortal” no nível", &[]);
        assert!(t.contains("[Mente Feral](mta://merits_flaws/feral_mind)"), "{t}");
        assert!(t.contains("[“Imortal”](mta://merits_flaws/immortal)"), "{t}");
        // Vírgula dentro das aspas: o link fica só no nome
        let q = autolink("the Merit: “Immortal,” (above)", &[]);
        assert_eq!(q, "the Merit: “[Immortal](mta://merits_flaws/immortal),” (above)");
        let a = autolink("os Antecedentes: Aliado, Familiar, Mentor ou Totem", &[]);
        assert!(a.contains("[Totem](mta://backgrounds/totem)"), "{a}");
    }

    #[test]
    fn keyword_type_picks_the_right_half_of_a_pair() {
        assert!(autolink("the Flaw: Cyclic Magick", &[]).contains("mta://merits_flaws/cyclic_magick_flaw"));
        assert!(autolink("the Merit: Cyclic Magick", &[]).contains("mta://merits_flaws/cyclic_magick)"));
    }

    #[test]
    fn keeps_existing_links_skips_self_and_plain_words() {
        let src = "the [Echoes](mta://merits_flaws/echoes) and the Flaw: Echoes";
        assert_eq!(autolink(src, &["echoes"]), src);
        // Sem expressão de referência, nada vira link
        assert_eq!(autolink("Merits and Flaws fall into categories", &[]), "Merits and Flaws fall into categories");
        assert_eq!(autolink("a Flaw: nothing here", &[]), "a Flaw: nothing here");
    }
}
