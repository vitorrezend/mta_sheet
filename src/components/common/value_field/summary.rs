//! Regras puras do campo de característica: textos de dica e cores das bolinhas.

use crate::state::DotOrigin;

/// Corta o nome em `max` caracteres, com reticências.
pub fn truncate_label(label: String, max: Option<usize>) -> String {
    match max {
        Some(max) if label.chars().count() > max => format!("{}...", label.chars().take(max).collect::<String>()),
        _ => label,
    }
}

fn origin_at(origins: &[DotOrigin], idx: usize) -> DotOrigin {
    origins.get(idx).copied().unwrap_or(DotOrigin::Base)
}

/// Dica do nome: nível e de onde vieram os pontos, por exemplo "Força (Nível 3: 2 Base • 1 XP)".
pub fn tooltip_label(label: &str, level: i32, origins: Option<&[DotOrigin]>) -> String {
    if level <= 0 {
        return label.to_string();
    }
    let Some(origins) = origins else {
        return format!("{} (Nível {})", label, level);
    };
    let mut counts = [0; 4];
    for i in 0..level as usize {
        counts[match origin_at(origins, i) {
            DotOrigin::Base => 0,
            DotOrigin::Bonus => 1,
            DotOrigin::Experience => 2,
            DotOrigin::Temporary => 3,
        }] += 1;
    }
    let parts: Vec<String> = counts
        .iter()
        .zip(["Base", "Bônus", "XP", "Buff"])
        .filter(|(n, _)| **n > 0)
        .map(|(n, name)| format!("{} {}", n, name))
        .collect();
    format!("{} (Nível {}: {})", label, level, parts.join(" • "))
}

/// Classe de cor da bolinha `dot` (começando em 1); vazia quando a bolinha não está marcada.
pub fn dot_color(level: i32, dot: i32, origins: Option<&[DotOrigin]>) -> &'static str {
    if level < dot {
        return "";
    }
    origins.map_or("dot-base", |o| origin_at(o, (dot - 1) as usize).color_class())
}

/// Cor do selo "+N" da escala sobrenatural: a origem mais "cara" entre os pontos acima de 5.
pub fn supernatural_badge_class(level: i32, origins: Option<&[DotOrigin]>) -> &'static str {
    let Some(origins) = origins.filter(|_| level > 5) else {
        return "badge-origin-base";
    };
    let extra: Vec<DotOrigin> = (5..level.min(10) as usize).filter_map(|i| origins.get(i).copied()).collect();
    if extra.contains(&DotOrigin::Temporary) {
        "badge-origin-temp"
    } else if extra.contains(&DotOrigin::Experience) {
        "badge-origin-xp"
    } else if extra.contains(&DotOrigin::Bonus) {
        "badge-origin-bonus"
    } else {
        "badge-origin-base"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DotOrigin::*;

    #[test]
    fn truncates_long_labels_only() {
        assert_eq!(truncate_label("Correspondência".into(), Some(5)), "Corre...");
        assert_eq!(truncate_label("Força".into(), Some(5)), "Força");
        assert_eq!(truncate_label("Força".into(), None), "Força");
    }

    #[test]
    fn tooltip_lists_where_points_came_from() {
        assert_eq!(tooltip_label("Força", 0, Some(&[])), "Força");
        assert_eq!(tooltip_label("Força", 2, None), "Força (Nível 2)");
        assert_eq!(tooltip_label("Força", 3, Some(&[Base, Experience])), "Força (Nível 3: 2 Base • 1 XP)");
    }

    #[test]
    fn dots_take_the_color_of_their_origin() {
        assert_eq!(dot_color(2, 3, None), "");
        assert_eq!(dot_color(3, 1, None), "dot-base");
        assert_eq!(dot_color(3, 2, Some(&[Base, Bonus])), Bonus.color_class());
        assert_eq!(dot_color(3, 3, Some(&[Base, Bonus])), "dot-base");
    }

    #[test]
    fn supernatural_badge_uses_the_most_expensive_origin() {
        let origins = [Base, Base, Base, Base, Base, Bonus, Experience];
        assert_eq!(supernatural_badge_class(5, Some(&origins)), "badge-origin-base");
        assert_eq!(supernatural_badge_class(6, Some(&origins)), "badge-origin-bonus");
        assert_eq!(supernatural_badge_class(7, Some(&origins)), "badge-origin-xp");
        assert_eq!(supernatural_badge_class(7, None), "badge-origin-base");
    }
}
