//! Regras puras de vitalidade e dano para World of Darkness / Mago: A Ascensão

/// Normaliza contagens de dano (Agravado, Letal, Contundente) para caberem exatamente no total de caixas.
pub fn normalize_health_counts(agg: usize, lethal: usize, bashing: usize, total: usize) -> (usize, usize, usize) {
    let safe_agg = agg.min(total);
    let rem_after_agg = total.saturating_sub(safe_agg);
    let safe_lethal = lethal.min(rem_after_agg);
    let rem_after_lethal = rem_after_agg.saturating_sub(safe_lethal);
    let mut safe_bashing = bashing.min(rem_after_lethal);

    if safe_agg + safe_lethal + safe_bashing > total {
        safe_bashing = total.saturating_sub(safe_agg + safe_lethal);
    }

    (safe_agg, safe_lethal, safe_bashing)
}

/// Calcula a penalidade de dados decorrente do nível de dano acumulado.
/// 
/// Tabela de Vitalidade Padrão WoD:
/// - 0 de dano: Íntegro (Penalidade 0)
/// - 1 de dano: Escoriado (Penalidade 0)
/// - 2 de dano: Ferido (Penalidade -1)
/// - 3 de dano: Ferido Gravemente (Penalidade -1)
/// - 4 de dano: Espancado (Penalidade -2)
/// - 5 de dano: Aleijado (Penalidade -5)
/// - 6+ de dano: Incapacitado
pub fn calculate_damage_penalty(agg: usize, lethal: usize, bashing: usize, extra_bruised: usize) -> i32 {
    let total_damage = agg + lethal + bashing;
    if total_damage == 0 {
        return 0;
    }

    // Caixas extras de Escoriado absorvem dano antes de começar a penalidade de Ferido
    let adjusted_damage = total_damage.saturating_sub(extra_bruised);

    match adjusted_damage {
        0 | 1 => 0,  // Íntegro / Escoriado
        2 => 1,      // Ferido (-1)
        3 => 1,      // Ferido Gravemente (-1)
        4 => 2,      // Espancado (-2)
        5 => 5,      // Aleijado (-5)
        _ => 5,      // Incapacitado / Além
    }
}

/// Aplica a penalidade de dano à base de iniciativa, garantindo o piso mínimo oficial de 2.
pub fn apply_health_penalty_to_initiative(raw_initiative: i32, health_penalty: i32) -> i32 {
    (raw_initiative - health_penalty).max(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_health() {
        let (a, l, b) = normalize_health_counts(5, 5, 5, 7);
        assert_eq!(a, 5);
        assert_eq!(l, 2);
        assert_eq!(b, 0);
        assert_eq!(a + l + b, 7);
    }

    #[test]
    fn test_penalties_and_floor() {
        assert_eq!(calculate_damage_penalty(0, 0, 0, 0), 0);
        assert_eq!(calculate_damage_penalty(0, 0, 1, 0), 0);
        assert_eq!(calculate_damage_penalty(0, 1, 1, 0), 1);
        assert_eq!(calculate_damage_penalty(0, 2, 2, 0), 2);
        assert_eq!(calculate_damage_penalty(2, 2, 1, 0), 5);

        // Piso mínimo 2 em iniciativa
        assert_eq!(apply_health_penalty_to_initiative(6, 0), 6);
        assert_eq!(apply_health_penalty_to_initiative(6, 1), 5);
        assert_eq!(apply_health_penalty_to_initiative(6, 2), 4);
        assert_eq!(apply_health_penalty_to_initiative(6, 5), 2);
        assert_eq!(apply_health_penalty_to_initiative(2, 5), 2);
    }
}
