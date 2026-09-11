//! Regras puras de Iniciativa para o Storyteller System / World of Darkness

use super::health::apply_health_penalty_to_initiative;

/// Calcula a base passiva de iniciativa: Destreza + Raciocínio menos penalidades de dano (piso mínimo 2).
pub fn calculate_initiative_base(dexterity: i32, wits: i32, health_penalty: i32) -> i32 {
    let raw_sum = dexterity + wits;
    apply_health_penalty_to_initiative(raw_sum, health_penalty)
}

/// Calcula o valor final de iniciativa somando o resultado do dado d10 (1 a 10).
pub fn calculate_initiative_roll(base: i32, d10_roll: i32) -> i32 {
    base + d10_roll.clamp(1, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initiative_base() {
        assert_eq!(calculate_initiative_base(3, 3, 0), 6);
        assert_eq!(calculate_initiative_base(3, 3, 1), 5);
        assert_eq!(calculate_initiative_base(3, 3, 5), 2); // Piso mínimo 2
        assert_eq!(calculate_initiative_base(1, 1, 5), 2); // Piso mínimo 2
    }

    #[test]
    fn test_initiative_roll() {
        let base = calculate_initiative_base(3, 2, 0); // 5
        assert_eq!(calculate_initiative_roll(base, 7), 12);
        assert_eq!(calculate_initiative_roll(base, 1), 6);
    }
}
