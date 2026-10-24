//! Rolagem de d10 e interpretação do comando `/r` do chat das salas.

/// Limites aceitos para uma parada de dados de M20.
pub const MIN_DICE: i32 = 1;
pub const MAX_DICE: i32 = 30;
pub const MIN_DIFFICULTY: i32 = 3;
pub const MAX_DIFFICULTY: i32 = 10;

/// Pedido de rolagem, já com valores dentro dos limites.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RollRequest {
    pub dice_count: i32,
    pub difficulty: i32,
    pub specialty: bool,
    pub label: String,
}

impl RollRequest {
    pub fn new(dice_count: i32, difficulty: i32, specialty: bool, label: &str) -> Self {
        let dice_count = dice_count.clamp(MIN_DICE, MAX_DICE);
        let difficulty = difficulty.clamp(MIN_DIFFICULTY, MAX_DIFFICULTY);
        let label = match label.trim() {
            "" => format!("{}d10 vs Dif {}", dice_count, difficulty),
            text => text.chars().take(120).collect(),
        };
        Self { dice_count, difficulty, specialty, label }
    }
}

/// Interpreta `/r` ou `/roll`. Retorna `None` se o texto não for um comando de rolagem.
///
/// Formatos aceitos: `/r 5`, `/r 5d10`, `/r 5d10 dif 7`, `/r 4 esp Força + Briga`.
/// Palavras que não são parâmetros viram o rótulo da rolagem.
pub fn parse_roll_command(text: &str) -> Option<RollRequest> {
    let text = text.trim();
    let mut parts = text.split_whitespace();
    let command = parts.next()?.to_lowercase();
    if command != "/r" && command != "/roll" {
        return None;
    }

    let mut dice_count = 5;
    let mut difficulty = 6;
    let mut specialty = false;
    let mut label_words: Vec<&str> = Vec::new();
    let mut dice_set = false;

    while let Some(arg) = parts.next() {
        let lower = arg.to_lowercase();
        if !dice_set {
            let count = lower.strip_suffix("d10").or_else(|| lower.strip_suffix('d')).unwrap_or(&lower);
            if let Ok(n) = count.parse::<i32>() {
                dice_count = n;
                dice_set = true;
                continue;
            }
        }
        match lower.as_str() {
            "dif" | "diff" => {
                if let Some(n) = parts.next().and_then(|v| v.parse::<i32>().ok()) {
                    difficulty = n;
                }
            }
            "esp" | "spec" => specialty = true,
            _ => label_words.push(arg),
        }
    }

    Some(RollRequest::new(dice_count, difficulty, specialty, &label_words.join(" ")))
}

/// Rola `count` d10 com distribuição uniforme.
///
/// Usa amostragem por rejeição: `byte % 10` direto favoreceria 1–6 (256 não é múltiplo de 10).
pub fn roll_d10s(count: i32) -> Vec<i32> {
    let count = count.clamp(MIN_DICE, MAX_DICE) as usize;
    let mut results = Vec::with_capacity(count);
    let mut buf = [0u8; 32];
    while results.len() < count {
        if getrandom::getrandom(&mut buf).is_err() {
            break;
        }
        results.extend(
            buf.iter()
                .filter(|b| **b < 250)
                .map(|b| i32::from(b % 10) + 1)
                .take(count - results.len()),
        );
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_formats() {
        assert_eq!(parse_roll_command("/r 5d10 dif 7"), Some(RollRequest::new(5, 7, false, "")));
        assert_eq!(parse_roll_command("/roll 4 esp Força + Briga"), Some(RollRequest::new(4, 6, true, "Força + Briga")));
        assert_eq!(parse_roll_command("/R 3D"), Some(RollRequest::new(3, 6, false, "")));
        assert_eq!(parse_roll_command("/r"), Some(RollRequest::new(5, 6, false, "")));
    }

    #[test]
    fn ignores_regular_messages() {
        assert_eq!(parse_roll_command("oi pessoal"), None);
        assert_eq!(parse_roll_command("/rolar 5"), None);
        assert_eq!(parse_roll_command(""), None);
    }

    #[test]
    fn clamps_out_of_range_values() {
        let req = parse_roll_command("/r 99 dif 1").expect("roll");
        assert_eq!((req.dice_count, req.difficulty), (MAX_DICE, MIN_DIFFICULTY));
        assert_eq!(req.label, "30d10 vs Dif 3");
    }

    #[test]
    fn rolls_are_in_range_and_cover_all_faces() {
        let rolls = roll_d10s(30);
        assert_eq!(rolls.len(), 30);
        let mut seen = [false; 10];
        for _ in 0..200 {
            for die in roll_d10s(30) {
                assert!((1..=10).contains(&die));
                seen[(die - 1) as usize] = true;
            }
        }
        assert!(seen.iter().all(|s| *s), "todas as faces devem aparecer em 6000 dados");
    }
}
