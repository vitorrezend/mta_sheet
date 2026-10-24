//! Regras puras de Combate e Dano (Mago: A Ascensão M20 & Storyteller System)
//!
//! Contém modelos matemáticos e regras de rolagem de combate, dano corporal,
//! dano por armas de fogo, absorção (soak) e aplicação de penalidades de vitalidade.

use super::health::calculate_damage_penalty;

/// Tipos canônicos de dano no Storyteller System / M20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DamageType {
    /// Dano Contundente (socos, cassetetes, quedas leves).
    Bashing,
    /// Dano Letal (facas, espadas, balas para humanos comuns).
    Lethal,
    /// Dano Agravado (fogo, garras sobrenaturais, radiação, radiação paradoxal).
    Aggravated,
}

impl DamageType {
    pub fn label_pt(&self) -> &'static str {
        match self {
            DamageType::Bashing => "Contundente",
            DamageType::Lethal => "Letal",
            DamageType::Aggravated => "Agravado",
        }
    }

    pub fn label_en(&self) -> &'static str {
        match self {
            DamageType::Bashing => "Bashing",
            DamageType::Lethal => "Lethal",
            DamageType::Aggravated => "Aggravated",
        }
    }
}

/// Fórmulas de dano de armas no M20.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponDamageFormula {
    /// Dano baseado na Força do atacante mais um bônus da arma (ex: Força + 2 Letal).
    Melee {
        strength_bonus: i32,
        damage_type: DamageType,
    },
    /// Dano fixo de arma de fogo ou projétil (ex: Pistola 9mm = 4 Letal).
    Ranged {
        base_dice: i32,
        damage_type: DamageType,
    },
}

/// Calcula o total de dados de dano para um ataque corpo a corpo (Força + Bônus de Arma + Sucessos Extras).
/// O piso mínimo de dados de dano é sempre 1.
pub fn calculate_melee_damage_dice(strength: i32, weapon_bonus: i32, extra_attack_successes: i32) -> i32 {
    let raw = strength + weapon_bonus + extra_attack_successes.max(0);
    raw.max(1)
}

/// Calcula o total de dados de dano para um ataque à distância (Dano Base da Arma + Sucessos Extras).
/// O piso mínimo de dados de dano é sempre 1.
pub fn calculate_ranged_damage_dice(base_damage: i32, extra_attack_successes: i32) -> i32 {
    let raw = base_damage + extra_attack_successes.max(0);
    raw.max(1)
}

/// Calcula a parada de dados para teste de Absorção (Soak).
///
/// Regra canônica M20:
/// - **Dano Contundente (Bashing)**: Vigor + Armadura (qualquer personagem).
/// - **Dano Letal (Lethal)**: Apenas Armadura para humanos normais. Personagens sobrenaturais
///   ou despertos com magia de Vida ativa podem somar Vigor.
/// - **Dano Agravado (Aggravated)**: Não pode ser absorvido por humanos normais. Apenas efeitos
///   mágicos específicos (ex: Vida 3+) ou armaduras especiais permitem absorção.
pub fn calculate_soak_pool(
    stamina: i32,
    armor: i32,
    damage_type: DamageType,
    can_soak_lethal_with_stamina: bool,
    can_soak_aggravated: bool,
) -> i32 {
    let safe_stamina = stamina.max(0);
    let safe_armor = armor.max(0);

    match damage_type {
        DamageType::Bashing => safe_stamina + safe_armor,
        DamageType::Lethal => {
            if can_soak_lethal_with_stamina {
                safe_stamina + safe_armor
            } else {
                safe_armor
            }
        }
        DamageType::Aggravated => {
            if can_soak_aggravated {
                safe_stamina + safe_armor
            } else {
                0
            }
        }
    }
}

/// Calcula o dano líquido sofrido após a rolagem de absorção.
/// O dano resultante nunca é negativo.
pub fn calculate_net_damage(damage_successes: i32, soak_successes: i32) -> usize {
    damage_successes.saturating_sub(soak_successes).max(0) as usize
}

/// Modifica uma parada de dados de ação aplicando as penalidades de ferimento atuais.
/// O piso mínimo para qualquer ação é 1 dado (caso a parada não tenha sido reduzida a zero).
pub fn apply_wound_penalty_to_action_pool(raw_dice_pool: i32, health_penalty: i32) -> i32 {
    if raw_dice_pool <= 0 {
        0
    } else {
        (raw_dice_pool - health_penalty).max(0)
    }
}

/// Helper para obter a penalidade de ação baseada no estado de dano atual.
pub fn get_combat_health_penalty(agg: usize, lethal: usize, bashing: usize, extra_bruised: usize) -> i32 {
    calculate_damage_penalty(agg, lethal, bashing, extra_bruised)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_melee_damage_calculation() {
        // Força 3, Faca (bônus +1), 2 sucessos extras no ataque
        assert_eq!(calculate_melee_damage_dice(3, 1, 2), 6);
        // Força 1, sem bônus, 0 sucessos
        assert_eq!(calculate_melee_damage_dice(1, 0, 0), 1);
        // Piso mínimo 1 mesmo com valores negativos
        assert_eq!(calculate_melee_damage_dice(0, -2, 0), 1);
    }

    #[test]
    fn test_ranged_damage_calculation() {
        // Pistola 9mm (4 dados), 3 sucessos extras de mira/ataque
        assert_eq!(calculate_ranged_damage_dice(4, 3), 7);
        // Espingarda (8 dados), 0 sucessos extras
        assert_eq!(calculate_ranged_damage_dice(8, 0), 8);
    }

    #[test]
    fn test_soak_pool_rules() {
        let stamina = 3;
        let armor = 2;

        // Dano Contundente: Vigor (3) + Armadura (2) = 5
        assert_eq!(
            calculate_soak_pool(stamina, armor, DamageType::Bashing, false, false),
            5
        );

        // Dano Letal (humano comum): apenas Armadura = 2
        assert_eq!(
            calculate_soak_pool(stamina, armor, DamageType::Lethal, false, false),
            2
        );

        // Dano Letal (com proteção mágica de Vida): Vigor + Armadura = 5
        assert_eq!(
            calculate_soak_pool(stamina, armor, DamageType::Lethal, true, false),
            5
        );

        // Dano Agravado (sem efeito de absorção): 0
        assert_eq!(
            calculate_soak_pool(stamina, armor, DamageType::Aggravated, true, false),
            0
        );

        // Dano Agravado (com efeito mágico de absorção): Vigor + Armadura = 5
        assert_eq!(
            calculate_soak_pool(stamina, armor, DamageType::Aggravated, true, true),
            5
        );
    }

    #[test]
    fn test_net_damage_and_wound_penalty() {
        assert_eq!(calculate_net_damage(5, 3), 2);
        assert_eq!(calculate_net_damage(2, 4), 0);

        // Parada 7 de dados com ferimento -2 => 5 dados
        assert_eq!(apply_wound_penalty_to_action_pool(7, 2), 5);
        // Parada 2 com ferimento -5 => 0 dados (incapacitado para agir)
        assert_eq!(apply_wound_penalty_to_action_pool(2, 5), 0);
    }
}
