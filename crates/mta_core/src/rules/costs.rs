//! Regras puras de Custos, Orçamentos e Distribuição de Pontos (Mago: A Ascensão M20)
//!
//! Tabela oficial de criação de personagens, pontos de bônus (Freebies) e experiência (XP),
//! conforme M20 (pp. 64-67, 335-337).

// ============================================================================
// CONSTANTES DE CRIAÇÃO BÁSICA
// ============================================================================

/// Orçamento total de pontos em Atributos na criação (7 / 5 / 3 = 15).
pub const ATTR_CREATION_BUDGET: usize = 15;
/// Distribuição canônica de Atributos por prioridade (Primário, Secundário, Terciário).
pub const ATTR_CREATION_SPREAD: [usize; 3] = [7, 5, 3];

/// Orçamento total de pontos em Habilidades na criação (13 / 9 / 5 = 27).
pub const ABILITY_CREATION_BUDGET: usize = 27;
/// Distribuição canônica de Habilidades por prioridade (Primário, Secundário, Terciário).
pub const ABILITY_CREATION_SPREAD: [usize; 3] = [13, 9, 5];
/// Teto máximo de bolinhas em qualquer Habilidade durante a criação básica (M20 p. 65).
pub const ABILITY_CREATION_CAP: usize = 3;

/// Orçamento de pontos em Esferas na criação (6 pontos livres + 1 de afinidade).
pub const SPHERES_CREATION_BUDGET: usize = 6;
/// Orçamento de pontos em Antecedentes na criação.
pub const BACKGROUNDS_CREATION_BUDGET: usize = 7;
/// Pontos base iniciais gratuitos de Força de Vontade.
pub const WILLPOWER_CREATION_BASE: usize = 5;
/// Pontos base iniciais gratuitos de Arete.
pub const ARETE_CREATION_BASE: usize = 1;
/// Pontos base de Ressonância permitidos na criação inicial.
pub const RESONANCE_CREATION_BUDGET: usize = 1;

/// Orçamento padrão de Pontos de Bônus (Freebie Points) na criação.
pub const FREEBIE_INITIAL_BUDGET: i32 = 15;

// ============================================================================
// CATEGORIAS DE TRAÇOS
// ============================================================================

/// Categorias canônicas de traços para cálculo de custos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitCategory {
    Attribute,
    Ability,
    Sphere,
    Arete,
    Willpower,
    Background,
    Merit,
    Flaw,
    Other,
}

// ============================================================================
// FUNÇÕES PURAS DE VALIDAÇÃO DE DISTRIBUIÇÃO
// ============================================================================

/// Verifica se uma distribuição em três grupos (ex: Físicos, Sociais, Mentais)
/// respeita a hierarquia de tetos [Primário, Secundário, Terciário] em qualquer ordem.
pub fn is_valid_spread(spent1: usize, spent2: usize, spent3: usize, tier1: usize, tier2: usize, tier3: usize) -> bool {
    let mut sorted = [spent1, spent2, spent3];
    sorted.sort_by(|a, b| b.cmp(a));
    sorted[0] <= tier1 && sorted[1] <= tier2 && sorted[2] <= tier3
}

/// Verifica se uma pontuação de habilidade respeita o teto de 3 bolinhas da criação básica.
pub fn is_within_creation_ability_cap(rating: usize) -> bool {
    rating <= ABILITY_CREATION_CAP
}

// ============================================================================
// CUSTOS DE PONTOS DE BÔNUS (FREEBIE POINTS)
// ============================================================================

/// Retorna o custo unitário em Pontos de Bônus para elevar um traço em 1 bolinha.
/// Conforme M20 p. 67.
pub fn freebie_cost_per_dot(category: TraitCategory) -> i32 {
    match category {
        TraitCategory::Attribute => 5,
        TraitCategory::Ability => 2,
        TraitCategory::Sphere => 7,
        TraitCategory::Arete => 4,
        TraitCategory::Willpower => 1,
        TraitCategory::Background => 1,
        TraitCategory::Merit => 1,
        TraitCategory::Flaw => -1, // Defeitos devolvem pontos de bônus
        TraitCategory::Other => 1,
    }
}

// ============================================================================
// CUSTOS DE PONTOS DE EXPERIÊNCIA (XP)
// ============================================================================

/// Retorna o custo de XP para comprar ou avançar um traço para o próximo nível.
///
/// Parâmetros:
/// - `category`: tipo do traço.
/// - `current_rating`: nível atual (0 para compra de novo traço, 1..=9 para avanço).
/// - `is_affinity_sphere`: verdadeiro se for a Esfera de Afinidade do mago.
///
/// Conforme tabela de custos de XP de M20 (p. 336):
/// - Atributo: Nível Atual × 4
/// - Nova Habilidade: 3 XP
/// - Habilidade: Nível Atual × 2
/// - Nova Esfera: 10 XP
/// - Esfera de Afinidade: Nível Atual × 7
/// - Outra Esfera: Nível Atual × 8
/// - Arete: Nível Atual × 8
/// - Força de Vontade: Nível Atual × 1
/// - Antecedente / Qualidade / Outros: 3 XP (novo) ou Nível Atual × 3
pub fn xp_cost_for_dot(category: TraitCategory, current_rating: usize, is_affinity_sphere: bool) -> i32 {
    let curr = current_rating as i32;
    match category {
        TraitCategory::Attribute => curr * 4,
        TraitCategory::Ability => {
            if current_rating == 0 {
                3
            } else {
                curr * 2
            }
        }
        TraitCategory::Sphere => {
            if current_rating == 0 {
                10
            } else if is_affinity_sphere {
                curr * 7
            } else {
                curr * 8
            }
        }
        TraitCategory::Arete => curr * 8,
        TraitCategory::Willpower => curr * 1,
        TraitCategory::Background | TraitCategory::Merit | TraitCategory::Other => {
            if current_rating == 0 {
                3
            } else {
                curr * 3
            }
        }
        TraitCategory::Flaw => 0,
    }
}

/// Helper puro que gera o par (custo, descrição) para tooltips de bolinhas.
pub fn format_dot_cost_description(
    category: TraitCategory,
    dot_idx: usize,
    is_affinity: bool,
    is_bonus: bool,
    is_xp: bool,
) -> (i32, String) {
    if is_bonus {
        let cost = freebie_cost_per_dot(category);
        (cost, format!("{} pts de Bônus", cost))
    } else if is_xp {
        let cost = xp_cost_for_dot(category, dot_idx, is_affinity);
        let desc = match category {
            TraitCategory::Sphere if dot_idx == 0 => "10 XP (Nova Esfera)".to_string(),
            TraitCategory::Sphere if is_affinity => {
                format!("{} XP (Afinidade Nível {} -> {})", cost, dot_idx, dot_idx + 1)
            }
            TraitCategory::Sphere => {
                format!("{} XP (Nível {} -> {})", cost, dot_idx, dot_idx + 1)
            }
            TraitCategory::Ability if dot_idx == 0 => "3 XP (Nova Habilidade)".to_string(),
            _ => format!("{} XP (Nível {} -> {})", cost, dot_idx, dot_idx + 1),
        };
        (cost, desc)
    } else {
        (0, "Criação Base (Grátis)".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spread_validation() {
        // Atributos 7 / 5 / 3 válidos em qualquer ordem
        assert!(is_valid_spread(7, 5, 3, 7, 5, 3));
        assert!(is_valid_spread(3, 7, 5, 7, 5, 3));
        assert!(is_valid_spread(5, 3, 7, 7, 5, 3));
        assert!(is_valid_spread(6, 4, 2, 7, 5, 3)); // Abaixo do teto é válido

        // Inválido: ultrapassa algum teto
        assert!(!is_valid_spread(8, 4, 3, 7, 5, 3));
        assert!(!is_valid_spread(7, 6, 2, 7, 5, 3));
        assert!(!is_valid_spread(6, 6, 3, 7, 5, 3));

        // Habilidades 13 / 9 / 5
        assert!(is_valid_spread(13, 9, 5, 13, 9, 5));
        assert!(is_valid_spread(9, 13, 5, 13, 9, 5));
        assert!(!is_valid_spread(14, 8, 5, 13, 9, 5));
        assert!(!is_valid_spread(11, 10, 5, 13, 9, 5));
    }

    #[test]
    fn test_ability_creation_cap() {
        assert!(is_within_creation_ability_cap(0));
        assert!(is_within_creation_ability_cap(1));
        assert!(is_within_creation_ability_cap(2));
        assert!(is_within_creation_ability_cap(3));
        assert!(!is_within_creation_ability_cap(4));
        assert!(!is_within_creation_ability_cap(5));
    }

    #[test]
    fn test_freebie_costs() {
        assert_eq!(freebie_cost_per_dot(TraitCategory::Attribute), 5);
        assert_eq!(freebie_cost_per_dot(TraitCategory::Ability), 2);
        assert_eq!(freebie_cost_per_dot(TraitCategory::Sphere), 7);
        assert_eq!(freebie_cost_per_dot(TraitCategory::Arete), 4);
        assert_eq!(freebie_cost_per_dot(TraitCategory::Willpower), 1);
        assert_eq!(freebie_cost_per_dot(TraitCategory::Background), 1);
        assert_eq!(freebie_cost_per_dot(TraitCategory::Merit), 1);
        assert_eq!(freebie_cost_per_dot(TraitCategory::Flaw), -1);
    }

    #[test]
    fn test_xp_costs() {
        // Atributo: atual * 4 (ex: subir de 2 para 3 custa 8 XP)
        assert_eq!(xp_cost_for_dot(TraitCategory::Attribute, 2, false), 8);
        assert_eq!(xp_cost_for_dot(TraitCategory::Attribute, 4, false), 16);

        // Habilidade: nova = 3, atual * 2 (ex: subir de 3 para 4 custa 6 XP)
        assert_eq!(xp_cost_for_dot(TraitCategory::Ability, 0, false), 3);
        assert_eq!(xp_cost_for_dot(TraitCategory::Ability, 3, false), 6);

        // Esferas:
        // Nova Esfera: 10 XP
        assert_eq!(xp_cost_for_dot(TraitCategory::Sphere, 0, false), 10);
        // Afinidade: atual * 7 (ex: subir de 2 para 3 custa 14 XP)
        assert_eq!(xp_cost_for_dot(TraitCategory::Sphere, 2, true), 14);
        // Não-afinidade: atual * 8 (ex: subir de 2 para 3 custa 16 XP)
        assert_eq!(xp_cost_for_dot(TraitCategory::Sphere, 2, false), 16);

        // Arete: atual * 8 (ex: subir de 3 para 4 custa 24 XP)
        assert_eq!(xp_cost_for_dot(TraitCategory::Arete, 3, false), 24);

        // Força de Vontade: atual * 1
        assert_eq!(xp_cost_for_dot(TraitCategory::Willpower, 6, false), 6);
    }

    #[test]
    fn test_dot_cost_description_formatting() {
        let (cost, desc) = format_dot_cost_description(TraitCategory::Attribute, 2, false, true, false);
        assert_eq!(cost, 5);
        assert_eq!(desc, "5 pts de Bônus");

        let (cost, desc) = format_dot_cost_description(TraitCategory::Sphere, 2, true, false, true);
        assert_eq!(cost, 14);
        assert_eq!(desc, "14 XP (Afinidade Nível 2 -> 3)");
    }
}
