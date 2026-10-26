//! Regras puras de Esteira de Quintessência e Paradoxo (Mago: A Ascensão M20)

pub const TOTAL_TRACK_SLOTS: usize = 20;

/// Analisa a esteira de Quintessência e Paradoxo, retornando (quintessência, paradoxo, esteira normalizada).
pub fn parse_quintessence_paradox(raw_track: &str) -> (i32, i32, String) {
    let mut normalized = String::with_capacity(TOTAL_TRACK_SLOTS);
    for c in raw_track.chars().take(TOTAL_TRACK_SLOTS) {
        if c == '1' || c == '2' {
            normalized.push(c);
        } else {
            normalized.push('0');
        }
    }
    while normalized.len() < TOTAL_TRACK_SLOTS {
        normalized.push('0');
    }

    let quint = normalized.chars().filter(|&c| c == '1').count() as i32;
    let paradox = normalized.chars().filter(|&c| c == '2').count() as i32;

    (quint, paradox, normalized)
}

/// Formata a esteira com base em quantidades solicitadas de quintessência e paradoxo.
/// No M20, a Quintessência ocupa slots a partir da esquerda ('1') e o Paradoxo a partir da direita ('2').
/// Se a soma ultrapassar 20, o Paradoxo tem prioridade e expulsa a Quintessência (Paradox Overload).
pub fn format_track(quint: usize, paradox: usize) -> String {
    let safe_paradox = paradox.min(TOTAL_TRACK_SLOTS);
    let max_quint = TOTAL_TRACK_SLOTS.saturating_sub(safe_paradox);
    let safe_quint = quint.min(max_quint);
    let empty_slots = TOTAL_TRACK_SLOTS.saturating_sub(safe_quint + safe_paradox);

    let mut track = String::with_capacity(TOTAL_TRACK_SLOTS);
    for _ in 0..safe_quint {
        track.push('1');
    }
    for _ in 0..empty_slots {
        track.push('0');
    }
    for _ in 0..safe_paradox {
        track.push('2');
    }

    track
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_format_track() {
        let (q, p, track) = parse_quintessence_paradox("11100000000000000022");
        assert_eq!(q, 3);
        assert_eq!(p, 2);
        assert_eq!(track.len(), 20);

        let formatted = format_track(5, 3);
        assert_eq!(formatted, "11111000000000000222");

        // Conflito: Paradoxo expulsa Quintessência
        let overflow = format_track(15, 10);
        let (q_ov, p_ov, _) = parse_quintessence_paradox(&overflow);
        assert_eq!(p_ov, 10);
        assert_eq!(q_ov, 10);
    }
}
