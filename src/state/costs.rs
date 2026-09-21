pub use crate::rules::costs::*;

use super::models::{
    keys, CharacterData, CostBreakdownItem, CostSummary, CreationPointsSummary, DotOrigin,
    STANDARD_ATTRIBUTES, STANDARD_KNOWLEDGES, STANDARD_SKILLS, STANDARD_SPHERES, STANDARD_TALENTS,
};

impl CharacterData {
    /// Calculates the character creation base points distribution and checks for budget overflows
    pub fn calculate_creation_points(&self) -> CreationPointsSummary {
        let affinity_sphere = self.get_affinity_sphere();

        // 1. Atributos (Físicos, Sociais, Mentais - Orçamento: 7 / 5 / 3, Total: 15)
        let mut attr_physical = 0;
        let mut attr_social = 0;
        let mut attr_mental = 0;

        for &attr_name in &["Força", "Destreza", "Vigor"] {
            let (base, _, _, _) = self.attributes.get(attr_name).map(|a| a.count_origins()).unwrap_or((1, 0, 0, 0));
            attr_physical += if base > 1 { base - 1 } else { 0 };
        }
        for &attr_name in &["Carisma", "Manipulação", "Aparência"] {
            let (base, _, _, _) = self.attributes.get(attr_name).map(|a| a.count_origins()).unwrap_or((1, 0, 0, 0));
            attr_social += if base > 1 { base - 1 } else { 0 };
        }
        for &attr_name in &["Percepção", "Inteligência", "Raciocínio"] {
            let (base, _, _, _) = self.attributes.get(attr_name).map(|a| a.count_origins()).unwrap_or((1, 0, 0, 0));
            attr_mental += if base > 1 { base - 1 } else { 0 };
        }

        let attr_total_spent = attr_physical + attr_social + attr_mental;
        let attr_spread_valid = is_valid_spread(
            attr_physical,
            attr_social,
            attr_mental,
            ATTR_CREATION_SPREAD[0],
            ATTR_CREATION_SPREAD[1],
            ATTR_CREATION_SPREAD[2],
        );
        let attr_exceeded = !attr_spread_valid || attr_total_spent > ATTR_CREATION_BUDGET;

        // 2. Habilidades (Talentos, Perícias, Conhecimentos - Orçamento: 13 / 9 / 5, Total: 27, Cap: 3)
        let mut ab_talents = 0;
        let mut ab_skills = 0;
        let mut ab_knowledges = 0;
        let mut ab_cap_violations = Vec::new();

        let check_abilities = |standard: &[&str], cat_key: &str, violations: &mut Vec<String>| -> usize {
            let mut total = 0;
            // 1. Standard abilities
            for &name in standard {
                if let Some(attr) = self.attributes.get(name) {
                    let (base, _, _, _) = attr.count_origins();
                    if !is_within_creation_ability_cap(base) {
                        violations.push(format!("{} ({} pts base)", name, base));
                    }
                    total += base;
                }
            }
            // 2. Custom abilities
            if let Some(list) = self.custom_lists.get(cat_key) {
                for id in list {
                    if let Some(attr) = self.attributes.get(id) {
                        let (base, _, _, _) = attr.count_origins();
                        if !is_within_creation_ability_cap(base) {
                            let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                            violations.push(format!("{} ({} pts base)", label, base));
                        }
                        total += base;
                    }
                }
            }
            total
        };

        ab_talents += check_abilities(&STANDARD_TALENTS, keys::CAT_TALENTOS, &mut ab_cap_violations);
        ab_skills += check_abilities(&STANDARD_SKILLS, keys::CAT_PERICIAS, &mut ab_cap_violations);
        ab_knowledges += check_abilities(&STANDARD_KNOWLEDGES, keys::CAT_CONHECIMENTOS, &mut ab_cap_violations);

        let ab_total_spent = ab_talents + ab_skills + ab_knowledges;
        let ab_spread_valid = is_valid_spread(
            ab_talents,
            ab_skills,
            ab_knowledges,
            ABILITY_CREATION_SPREAD[0],
            ABILITY_CREATION_SPREAD[1],
            ABILITY_CREATION_SPREAD[2],
        );
        let ab_exceeded = !ab_spread_valid || ab_total_spent > ABILITY_CREATION_BUDGET || !ab_cap_violations.is_empty();

        // 3. Esferas (Orçamento: 6 pontos, +1 grátis de afinidade)
        let mut spheres_spent = 0;
        for &sph in &STANDARD_SPHERES {
            let attr_opt = self.attributes.get(sph).or_else(|| {
                match sph {
                    "Correspondência" => self.attributes.get("Dados").or_else(|| self.attributes.get("Data")),
                    "Primórdio" => self.attributes.get("Utilidade Primordial").or_else(|| self.attributes.get("Primal Utility")),
                    "Espírito" => self.attributes.get("Ciência Dimensional").or_else(|| self.attributes.get("Dimensional Science")),
                    _ => None,
                }
            });
            if let Some(attr) = attr_opt {
                let (base, _, _, _) = attr.count_origins();
                let is_affinity = affinity_sphere.as_ref().map(|s| {
                    s.eq_ignore_ascii_case(sph)
                        || (sph == "Correspondência" && (s.eq_ignore_ascii_case("Dados") || s.eq_ignore_ascii_case("Data")))
                        || (sph == "Primórdio" && (s.eq_ignore_ascii_case("Utilidade Primordial") || s.eq_ignore_ascii_case("Primal Utility")))
                        || (sph == "Espírito" && (s.eq_ignore_ascii_case("Ciência Dimensional") || s.eq_ignore_ascii_case("Dimensional Science")))
                }).unwrap_or(false);
                if is_affinity && base > 0 {
                    spheres_spent += base - 1;
                } else {
                    spheres_spent += base;
                }
            }
        }
        let spheres_budget = SPHERES_CREATION_BUDGET;
        let spheres_exceeded = spheres_spent > spheres_budget;

        // 4. Arete (1 grátis)
        let (arete_base, _, _, _) = self.attributes.get(keys::KEY_ARETE).map(|a| a.count_origins()).unwrap_or((ARETE_CREATION_BASE, 0, 0, 0));
        let arete_exceeded = arete_base > ARETE_CREATION_BASE;

        // 5. Antecedentes (Orçamento: 7 pontos)
        let mut backgrounds_spent = 0;
        if let Some(list) = self.custom_lists.get(keys::CAT_ANTECEDENTES) {
            for id in list {
                if let Some(attr) = self.attributes.get(id) {
                    let (base, _, _, _) = attr.count_origins();
                    backgrounds_spent += base;
                }
            }
        }
        let backgrounds_budget = BACKGROUNDS_CREATION_BUDGET;
        let backgrounds_exceeded = backgrounds_spent > backgrounds_budget;

        // 6. Força de Vontade (5 grátis)
        let (willpower_base, _, _, _) = self.attributes.get(keys::KEY_WILLPOWER_TOTAL).map(|a| a.count_origins()).unwrap_or((WILLPOWER_CREATION_BASE, 0, 0, 0));
        let willpower_exceeded = willpower_base > WILLPOWER_CREATION_BASE;

        // 7. Ressonância (Orçamento: 1 ponto)
        let mut resonance_spent = 0;
        if let Some(list) = self.custom_lists.get(keys::CAT_RESONANCE) {
            for id in list {
                if let Some(attr) = self.attributes.get(id) {
                    let (base, _, _, _) = attr.count_origins();
                    resonance_spent += base;
                }
            }
        }
        let resonance_budget = RESONANCE_CREATION_BUDGET;
        let resonance_exceeded = resonance_spent > resonance_budget;

        // 8. Warnings e Consolidação
        let mut warnings = Vec::new();

        if attr_exceeded {
            if attr_total_spent > 15 {
                warnings.push(format!(
                    "Atributos: {}/15 pontos de criação gastos (Físicos: {}, Sociais: {}, Mentais: {}). Excedente: {} pts.",
                    attr_total_spent, attr_physical, attr_social, attr_mental, attr_total_spent - 15
                ));
            } else {
                warnings.push(format!(
                    "Atributos: distribuição atual (Físicos: {}, Sociais: {}, Mentais: {}) não encaixa no teto 7 / 5 / 3.",
                    attr_physical, attr_social, attr_mental
                ));
            }
        }

        if ab_exceeded {
            if ab_total_spent > 27 {
                warnings.push(format!(
                    "Habilidades: {}/27 pontos de criação gastos (Talentos: {}, Perícias: {}, Conhecimentos: {}). Excedente: {} pts.",
                    ab_total_spent, ab_talents, ab_skills, ab_knowledges, ab_total_spent - 27
                ));
            } else if !ab_spread_valid {
                warnings.push(format!(
                    "Habilidades: distribuição atual (Talentos: {}, Perícias: {}, Conhecimentos: {}) não encaixa no teto 13 / 9 / 5.",
                    ab_talents, ab_skills, ab_knowledges
                ));
            }
            for viol in &ab_cap_violations {
                warnings.push(format!(
                    "Habilidade '{}': na criação básica o teto é 3 pontos (acima de 3 deve ser marcado com Bônus ou XP).",
                    viol
                ));
            }
        }

        if spheres_exceeded {
            warnings.push(format!(
                "Esferas: {}/6 pontos de criação gastos (descontando 1 grátis de afinidade). Excedente: {} pts.",
                spheres_spent, spheres_spent - 6
            ));
        }

        if arete_exceeded {
            warnings.push(format!(
                "Arete: nível base {} ultrapassa o 1 ponto grátis inicial (comprar Arete 2 ou 3 na criação custa 4 Pontos de Bônus/dot).",
                arete_base
            ));
        }

        if backgrounds_exceeded {
            warnings.push(format!(
                "Antecedentes: {}/7 pontos de criação gastos. Excedente: {} pts.",
                backgrounds_spent, backgrounds_spent - 7
            ));
        }

        if willpower_exceeded {
            warnings.push(format!(
                "Força de Vontade: nível base {} ultrapassa os 5 pontos grátis iniciais (pontos adicionais devem ser marcados com Bônus).",
                willpower_base
            ));
        }

        if resonance_exceeded {
            warnings.push(format!(
                "Ressonância: {}/1 ponto de criação gasto (pontos adicionais devem ser marcados com Bônus).",
                resonance_spent
            ));
        }

        let has_any_overflow = attr_exceeded
            || ab_exceeded
            || spheres_exceeded
            || arete_exceeded
            || backgrounds_exceeded
            || willpower_exceeded
            || resonance_exceeded;

        CreationPointsSummary {
            attr_physical,
            attr_social,
            attr_mental,
            attr_total_spent,
            attr_budget_total: ATTR_CREATION_BUDGET,
            attr_spread_valid,
            attr_exceeded,

            ab_talents,
            ab_skills,
            ab_knowledges,
            ab_total_spent,
            ab_budget_total: ABILITY_CREATION_BUDGET,
            ab_spread_valid,
            ab_exceeded,
            ab_cap_violations,

            spheres_spent,
            spheres_budget,
            spheres_exceeded,

            arete_base,
            arete_exceeded,

            backgrounds_spent,
            backgrounds_budget,
            backgrounds_exceeded,

            willpower_base,
            willpower_exceeded,

            resonance_spent,
            resonance_budget,
            resonance_exceeded,

            has_any_overflow,
            warnings,
        }
    }

    /// Calculate full cost summary of Freebie Points (Bonus: 15) and Experience Points (XP)
    pub fn calculate_costs(&self) -> CostSummary {
        let affinity_sphere = self.get_affinity_sphere();
        let creation_points = self.calculate_creation_points();
        let mut total_bonus_spent = 0;
        let mut total_xp_spent = 0;
        let mut items = Vec::new();
        let mut visited_keys = std::collections::HashSet::new();
        let mut traits_to_process: Vec<(String, String, String, bool, bool, bool, bool, bool, bool)> = Vec::new();

        // 1. Atributos
        for &attr_name in &STANDARD_ATTRIBUTES {
            visited_keys.insert(attr_name.to_string());
            traits_to_process.push((attr_name.to_string(), attr_name.to_string(), "Atributo".to_string(), false, false, false, false, false, false));
        }

        // 2. Habilidades (Talentos, Perícias, Conhecimentos)
        for (standard_list, cat_key, cat_label) in [
            (&STANDARD_TALENTS[..], keys::CAT_TALENTOS, "Talento"),
            (&STANDARD_SKILLS[..], keys::CAT_PERICIAS, "Perícia"),
            (&STANDARD_KNOWLEDGES[..], keys::CAT_CONHECIMENTOS, "Conhecimento"),
        ] {
            for &id in standard_list {
                visited_keys.insert(id.to_string());
                traits_to_process.push((id.to_string(), id.to_string(), cat_label.to_string(), false, false, false, false, false, false));
            }
            if let Some(list) = self.custom_lists.get(cat_key) {
                for id in list {
                    visited_keys.insert(id.clone());
                    let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                    traits_to_process.push((id.clone(), label, cat_label.to_string(), false, false, false, false, false, false));
                }
            }
        }

        // 3. Esferas
        for &sphere_name in &STANDARD_SPHERES {
            let active_name = match sphere_name {
                "Correspondência" => {
                    let is_techno = self.attributes.contains_key("Dados")
                        || self.attributes.contains_key("Data")
                        || self.labels.get("sphere_slot_correspondence")
                            .map(|v| v.eq_ignore_ascii_case("Dados") || v.eq_ignore_ascii_case("Data") || v.eq_ignore_ascii_case("techno"))
                            .unwrap_or(false);
                    if is_techno {
                        "Dados"
                    } else {
                        "Correspondência"
                    }
                }
                "Primórdio" => {
                    let is_techno = self.attributes.contains_key("Utilidade Primordial")
                        || self.attributes.contains_key("Primal Utility")
                        || self.labels.get("sphere_slot_prime")
                            .map(|v| v.eq_ignore_ascii_case("Utilidade Primordial") || v.eq_ignore_ascii_case("Primal Utility") || v.eq_ignore_ascii_case("techno"))
                            .unwrap_or(false);
                    if is_techno {
                        "Utilidade Primordial"
                    } else {
                        "Primórdio"
                    }
                }
                "Espírito" => {
                    let is_techno = self.attributes.contains_key("Ciência Dimensional")
                        || self.attributes.contains_key("Dimensional Science")
                        || self.labels.get("sphere_slot_spirit")
                            .map(|v| v.eq_ignore_ascii_case("Ciência Dimensional") || v.eq_ignore_ascii_case("Dimensional Science") || v.eq_ignore_ascii_case("techno"))
                            .unwrap_or(false);
                    if is_techno {
                        "Ciência Dimensional"
                    } else {
                        "Espírito"
                    }
                }
                _ => sphere_name,
            };
            visited_keys.insert(sphere_name.to_string());
            if active_name != sphere_name {
                visited_keys.insert(active_name.to_string());
            }
            traits_to_process.push((active_name.to_string(), active_name.to_string(), "Esfera".to_string(), true, false, false, false, false, false));
        }

        // 4. Arete
        let arete_val = self.get_attribute_level(keys::KEY_ARETE, 1);
        visited_keys.insert(keys::KEY_ARETE.to_string());
        traits_to_process.push((keys::KEY_ARETE.to_string(), "Arete".to_string(), "Vantagem".to_string(), false, true, false, false, false, false));

        // 5. Força de Vontade
        visited_keys.insert(keys::KEY_WILLPOWER_TOTAL.to_string());
        traits_to_process.push((keys::KEY_WILLPOWER_TOTAL.to_string(), "Força de Vontade".to_string(), "Vantagem".to_string(), false, false, true, false, false, false));

        // 6. Antecedentes
        if let Some(list) = self.custom_lists.get(keys::CAT_ANTECEDENTES) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Antecedente".to_string(), false, false, false, true, false, false));
            }
        }

        // 7. Ressonância
        if let Some(list) = self.custom_lists.get(keys::CAT_RESONANCE) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Ressonância".to_string(), false, false, false, true, false, false));
            }
        }

        // 8. Qualidades (Merits)
        if let Some(list) = self.custom_lists.get(keys::CAT_MERITS) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Qualidade".to_string(), false, false, false, false, true, false));
            }
        }

        // 9. Defeitos (Flaws)
        if let Some(list) = self.custom_lists.get(keys::CAT_FLAWS) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Defeito".to_string(), false, false, false, false, false, true));
            }
        }

        // 10. Outros Traços (Other Traits)
        if let Some(list) = self.custom_lists.get(keys::CAT_OTHER_TRAITS) {
            for id in list {
                visited_keys.insert(id.clone());
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Outro Traço".to_string(), false, false, false, true, false, false));
            }
        }

        // 11. Quaisquer outros atributos personalizados
        for (id, attr) in &self.attributes {
            if !visited_keys.contains(id) && attr.level > 0 {
                let label = self.labels.get(id).cloned().unwrap_or_else(|| id.clone());
                traits_to_process.push((id.clone(), label, "Outro".to_string(), false, false, false, true, false, false));
            }
        }

        for (id, display_name, category, is_sphere, is_arete, is_willpower, is_background, is_merit, is_flaw) in traits_to_process {
            if let Some(attr) = self.attributes.get(&id) {
                let lvl = attr.level.max(0) as usize;
                if lvl == 0 {
                    continue;
                }
                let origins = attr.get_origins(lvl);
                let mut trait_bonus_cost = 0;
                let mut trait_xp_cost = 0;
                let mut bonus_dots = 0;
                let mut xp_dots = 0;

                let is_affinity = is_sphere && affinity_sphere.as_ref().map(|s| s.eq_ignore_ascii_case(&id)).unwrap_or(false);

                if is_flaw {
                    // Defeitos concedem pontos de bônus (1 pt de bônus negativo para subtrair do total de bônus)
                    let flaw_pts = lvl as i32;
                    trait_bonus_cost = -flaw_pts;
                    bonus_dots = lvl;
                } else {
                    let cat = if is_arete {
                        TraitCategory::Arete
                    } else if is_sphere {
                        TraitCategory::Sphere
                    } else if is_willpower {
                        TraitCategory::Willpower
                    } else if is_background || is_merit {
                        TraitCategory::Background
                    } else if STANDARD_ATTRIBUTES.contains(&id.as_str()) {
                        TraitCategory::Attribute
                    } else {
                        TraitCategory::Ability
                    };

                    for (idx, &origin) in origins.iter().take(lvl).enumerate() {
                        match origin {
                            DotOrigin::Bonus => {
                                bonus_dots += 1;
                                trait_bonus_cost += freebie_cost_per_dot(cat);
                            }
                            DotOrigin::Experience => {
                                xp_dots += 1;
                                trait_xp_cost += xp_cost_for_dot(cat, idx, is_affinity);
                            }
                            _ => {}
                        }
                    }
                }

                if trait_bonus_cost != 0 || trait_xp_cost > 0 {
                    total_bonus_spent += trait_bonus_cost;
                    total_xp_spent += trait_xp_cost;
                    items.push(CostBreakdownItem {
                        id,
                        name: display_name,
                        category: if is_affinity { "Esfera de Afinidade".to_string() } else { category },
                        level: attr.level,
                        bonus_dots,
                        bonus_cost: trait_bonus_cost,
                        xp_dots,
                        xp_cost: trait_xp_cost,
                    });
                }
            }
        }

        // 12. Maravilhas (Wonders)
        for (w_idx, wonder) in self.wonders.iter().enumerate() {
            if wonder.points.level > 0 {
                let display_name = if wonder.name.trim().is_empty() {
                    format!("Maravilha {}", w_idx + 1)
                } else {
                    format!("Maravilha ({})", wonder.name.trim())
                };
                let id = if !wonder.id.is_empty() { wonder.id.clone() } else { format!("wonder_{}", w_idx) };
                let lvl = wonder.points.level.max(0) as usize;
                let origins = wonder.points.get_origins(lvl);
                let mut trait_bonus_cost = 0;
                let mut trait_xp_cost = 0;
                let mut bonus_dots = 0;
                let mut xp_dots = 0;

                for (idx, &origin) in origins.iter().take(lvl).enumerate() {
                    match origin {
                        DotOrigin::Bonus => {
                            bonus_dots += 1;
                            trait_bonus_cost += freebie_cost_per_dot(TraitCategory::Background);
                        }
                        DotOrigin::Experience => {
                            xp_dots += 1;
                            trait_xp_cost += xp_cost_for_dot(TraitCategory::Background, idx, false);
                        }
                        _ => {}
                    }
                }

                if trait_bonus_cost > 0 || trait_xp_cost > 0 {
                    total_bonus_spent += trait_bonus_cost;
                    total_xp_spent += trait_xp_cost;
                    items.push(CostBreakdownItem {
                        id,
                        name: display_name,
                        category: "Maravilha (Antecedente)".to_string(),
                        level: wonder.points.level,
                        bonus_dots,
                        bonus_cost: trait_bonus_cost,
                        xp_dots,
                        xp_cost: trait_xp_cost,
                    });
                }
            }
        }

        let arete_warning = arete_val > 3;

        CostSummary {
            total_bonus_spent,
            bonus_limit: FREEBIE_INITIAL_BUDGET,
            total_xp_spent,
            items,
            arete_warning,
            arete_total: arete_val,
            affinity_sphere,
            creation_points,
        }
    }

    /// Helper to get single dot cost and explanation for tooltips
    pub fn get_dot_cost_description(trait_name: &str, dot_idx: usize, origin: DotOrigin, is_affinity: bool) -> (i32, String) {
        let cat = if trait_name == keys::KEY_ARETE {
            TraitCategory::Arete
        } else if STANDARD_SPHERES.contains(&trait_name)
            || trait_name == "Dados" || trait_name == "Data"
            || trait_name == "Utilidade Primordial" || trait_name == "Primal Utility"
            || trait_name == "Ciência Dimensional" || trait_name == "Dimensional Science" {
            TraitCategory::Sphere
        } else if trait_name == keys::KEY_WILLPOWER_TOTAL {
            TraitCategory::Willpower
        } else if STANDARD_ATTRIBUTES.contains(&trait_name) {
            TraitCategory::Attribute
        } else if trait_name.starts_with("bg_") {
            TraitCategory::Background
        } else {
            TraitCategory::Ability
        };
        format_dot_cost_description(
            cat,
            dot_idx,
            is_affinity,
            matches!(origin, DotOrigin::Bonus),
            matches!(origin, DotOrigin::Experience),
        )
    }
}
