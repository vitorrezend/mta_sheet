//! Regras puras de Especialidades (Mago: A Ascensão M20 & Storyteller System)
//!
//! Conforme M20 (p. 273, Capítulo 6: Creating the Character):
//! "When a character has 4+ dots in an Attribute or Ability Trait, the player may pick
//! a specialty. On rolls related to that specialty, every 10 rolled counts as TWO successes."

/// Nível mínimo de bolinhas (4) necessário para desbloquear uma especialidade canônica de M20.
pub const MIN_DOTS_FOR_SPECIALTY: i32 = 4;

/// Estado de validação de especialidade para um traço.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialtyStatus {
    /// O traço tem nível menor que 4; especialidades são desnecessárias ou inativas.
    Inactive,
    /// O traço tem nível 4+ e nenhuma especialidade foi definida (sugestão/aviso).
    UnlockedMissing,
    /// O traço tem nível 4+ e possui uma ou mais especialidades ativas.
    Active(String),
}

/// Verifica se um traço atingiu o limiar de 4+ bolinhas que desbloqueia/exige especialidade.
pub fn requires_specialty(rating: i32) -> bool {
    rating >= MIN_DOTS_FOR_SPECIALTY
}

/// Avalia o status da especialidade de acordo com a pontuação do traço e o texto preenchido.
pub fn evaluate_specialty(rating: i32, current_specialty: &str) -> SpecialtyStatus {
    if !requires_specialty(rating) {
        SpecialtyStatus::Inactive
    } else {
        let trimmed = current_specialty.trim();
        if trimmed.is_empty() {
            SpecialtyStatus::UnlockedMissing
        } else {
            SpecialtyStatus::Active(trimmed.to_string())
        }
    }
}

/// Calcula a quantidade de sucessos gerados por um dado individual segundo a regra canônica de M20.
///
/// - Se `die_result == 1`: gera -1 sucesso (cancela um sucesso).
/// - Se `die_result >= difficulty`:
///   - Se `die_result == 10` e a especialidade estiver ativa: conta como **2 sucessos**.
///   - Caso contrário: conta como **1 sucesso**.
/// - Se `die_result < difficulty`: 0 sucessos.
pub fn count_successes_for_die(die_result: i32, difficulty: i32, specialty_active: bool) -> i32 {
    let clamped_diff = difficulty.clamp(2, 10);
    if die_result == 1 {
        -1
    } else if die_result >= clamped_diff {
        if die_result == 10 && specialty_active {
            2
        } else {
            1
        }
    } else {
        0
    }
}

/// Avalia uma rolagem completa de d10 no sistema Storyteller (M20).
/// Retorna `(sucessos_liquidos, falha_critica_ou_botch)`.
pub fn evaluate_roll(dice: &[i32], difficulty: i32, specialty_active: bool) -> (i32, bool) {
    let mut total_successes = 0;
    let mut ones = 0;

    for &die in dice {
        if die == 1 {
            ones += 1;
        } else if die >= difficulty {
            if die == 10 && specialty_active {
                total_successes += 2;
            } else {
                total_successes += 1;
            }
        }
    }

    if total_successes == 0 && ones > 0 {
        // Falha crítica (Botch): nenhum sucesso e pelo menos um resultado 1
        (0, true)
    } else {
        let net = (total_successes - ones).max(0);
        (net, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requires_specialty() {
        assert!(!requires_specialty(1));
        assert!(!requires_specialty(2));
        assert!(!requires_specialty(3));
        assert!(requires_specialty(4));
        assert!(requires_specialty(5));
    }

    #[test]
    fn test_evaluate_specialty() {
        assert_eq!(evaluate_specialty(3, ""), SpecialtyStatus::Inactive);
        assert_eq!(evaluate_specialty(4, ""), SpecialtyStatus::UnlockedMissing);
        assert_eq!(evaluate_specialty(4, "   "), SpecialtyStatus::UnlockedMissing);
        assert_eq!(
            evaluate_specialty(4, "Reflexos Rápidos"),
            SpecialtyStatus::Active("Reflexos Rápidos".to_string())
        );
        assert_eq!(
            evaluate_specialty(5, "Programação Segura"),
            SpecialtyStatus::Active("Programação Segura".to_string())
        );
    }

    #[test]
    fn test_count_successes_for_die() {
        // Dificuldade padrão 6
        assert_eq!(count_successes_for_die(1, 6, false), -1);
        assert_eq!(count_successes_for_die(5, 6, false), 0);
        assert_eq!(count_successes_for_die(6, 6, false), 1);
        assert_eq!(count_successes_for_die(9, 6, false), 1);

        // Sem especialidade: 10 gera 1 sucesso
        assert_eq!(count_successes_for_die(10, 6, false), 1);

        // Com especialidade ativa: 10 gera 2 sucessos
        assert_eq!(count_successes_for_die(10, 6, true), 2);
    }

    #[test]
    fn test_evaluate_roll() {
        // Rolagem simples sem especialidade: [10, 8, 5, 2] contra dif 6 => 2 sucessos
        let (net, botch) = evaluate_roll(&[10, 8, 5, 2], 6, false);
        assert_eq!(net, 2);
        assert!(!botch);

        // Mesma rolagem COM especialidade ativa => 10 gera 2 sucessos + 1 do 8 = 3
        let (net, botch) = evaluate_roll(&[10, 8, 5, 2], 6, true);
        assert_eq!(net, 3);
        assert!(!botch);

        // Cancelamento por '1': [10, 1] com especialidade => 2 - 1 = 1
        let (net, botch) = evaluate_roll(&[10, 1], 6, true);
        assert_eq!(net, 1);
        assert!(!botch);

        // Botch: [5, 4, 1] contra dif 6 => 0 sucessos e um 1
        let (net, botch) = evaluate_roll(&[5, 4, 1], 6, false);
        assert_eq!(net, 0);
        assert!(botch);
    }
}
