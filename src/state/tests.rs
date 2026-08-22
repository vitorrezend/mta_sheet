#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::state::*;
    use std::collections::HashMap;

    #[test]
    fn test_damage_type_cycle() {
        let dmg = DamageType::None;
        assert_eq!(dmg.cycle(), DamageType::Bashing);
        assert_eq!(dmg.cycle().cycle(), DamageType::Lethal);
        assert_eq!(dmg.cycle().cycle().cycle(), DamageType::Aggravated);
        assert_eq!(dmg.cycle().cycle().cycle().cycle(), DamageType::None);
    }

    #[test]
    fn test_willpower_clamp_and_current_adjustment() {
        let mut char_data = CharacterData::new("123".to_string(), "Hermes".to_string());
        char_data.set_willpower_total(8);
        char_data.set_willpower_current(7);

        let (total, current) = char_data.get_willpower();
        assert_eq!(total, 8);
        assert_eq!(current, 7);

        // Lowering total below current must clamp current
        char_data.set_willpower_total(5);
        let (total, current) = char_data.get_willpower();
        assert_eq!(total, 5);
        assert_eq!(current, 5);
    }

    #[test]
    fn test_arete_bounds() {
        let mut char_data = CharacterData::new("123".to_string(), "Hermes".to_string());
        assert_eq!(char_data.get_arete(), 1);

        char_data.set_arete(4);
        assert_eq!(char_data.get_arete(), 4);

        char_data.set_arete(15);
        assert_eq!(char_data.get_arete(), 10);
    }

    #[test]
    fn test_quintessence_paradox_counting() {
        let mut char_data = CharacterData::new("123".to_string(), "Hermes".to_string());
        let (q, p, _) = char_data.get_quintessence_paradox();
        assert_eq!(q, 0);
        assert_eq!(p, 0);

        char_data.cycle_quintessence_paradox_box(0); // -> '1' (Quintessence)
        char_data.cycle_quintessence_paradox_box(1); // -> '1'
        char_data.cycle_quintessence_paradox_box(1); // -> '2' (Paradox)

        let (q, p, _) = char_data.get_quintessence_paradox();
        assert_eq!(q, 1);
        assert_eq!(p, 1);
    }

    #[test]
    fn test_sanitize_fixes_corrupt_data() {
        let mut char_data = CharacterData {
            id: "test".to_string(),
            name: "   ".to_string(),
            attributes: HashMap::new(),
            labels: HashMap::new(),
            custom_lists: HashMap::new(),
            ..Default::default()
        };

        char_data.sanitize();
        assert_eq!(char_data.name, "Sem Nome");
        assert_eq!(char_data.get_arete(), 1);
        assert_eq!(char_data.get_willpower(), (5, 5));
    }

    #[test]
    fn test_dot_origins_tracking_and_counting() {
        let mut attr = AttributeValue::new(3, "Espadas".to_string());
        assert_eq!(attr.dot_origins.len(), 3);
        assert_eq!(attr.count_origins(), (3, 0, 0, 0));

        // Add 1 dot with Bonus origin
        attr.set_level_with_origin(4, DotOrigin::Bonus);
        assert_eq!(attr.level, 4);
        assert_eq!(attr.count_origins(), (3, 1, 0, 0));

        // Add 1 dot with XP origin
        attr.set_level_with_origin(5, DotOrigin::Experience);
        assert_eq!(attr.level, 5);
        assert_eq!(attr.count_origins(), (3, 1, 1, 0));

        // Manually toggle dot 0 to Temporary
        attr.set_dot_origin(0, DotOrigin::Temporary);
        assert_eq!(attr.count_origins(), (2, 1, 1, 1));

        // Lower level to 3 truncates the last dots (leaving Temporary at 0 and Base at 1, 2)
        attr.set_level_with_origin(3, DotOrigin::Base);
        assert_eq!(attr.level, 3);
        assert_eq!(attr.count_origins(), (2, 0, 0, 1));
    }

    #[test]
    fn test_legacy_empty_origins_upgrade_preserves_base_dots() {
        // Simulates an existing attribute from DB with level 3 and empty dot_origins
        let mut attr = AttributeValue {
            level: 3,
            modifier: String::new(),
            dot_origins: Vec::new(),
        };

        // Adding 4th dot with Bonus mode must keep first 3 as Base and only 4th as Bonus
        attr.set_level_with_origin(4, DotOrigin::Bonus);
        assert_eq!(attr.level, 4);
        assert_eq!(attr.dot_origins, vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Bonus]);
        assert_eq!(attr.count_origins(), (3, 1, 0, 0));

        // Adding 5th dot with XP mode must add 5th as XP
        attr.set_level_with_origin(5, DotOrigin::Experience);
        assert_eq!(attr.level, 5);
        assert_eq!(attr.dot_origins, vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Experience]);
        assert_eq!(attr.count_origins(), (3, 1, 1, 0));
    }

    #[test]
    fn test_calculate_costs_freebies_and_xp() {
        let mut char_data = CharacterData::new("c1".to_string(), "Mago Teste".to_string());

        // 1. Força (Attribute): Level 4 -> [Base, Base, Bonus, Experience]
        // Bonus at idx 2: 5 pts
        // XP at idx 3: idx 3 * 4 = 12 XP
        char_data.attributes.insert("Força".to_string(), AttributeValue {
            level: 4,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Experience],
        });

        // 2. Custom Ability (Talento): Level 3 -> [Base, Bonus, Experience]
        // Bonus at idx 1: 2 pts
        // XP at idx 2: idx 2 * 2 = 4 XP
        char_data.custom_lists.insert(keys::CAT_TALENTOS.to_string(), vec!["tal_1".to_string()]);
        char_data.labels.insert("tal_1".to_string(), "Prontidão".to_string());
        char_data.attributes.insert("tal_1".to_string(), AttributeValue {
            level: 3,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Experience],
        });

        // 3. Forças (Sphere): Level 2 -> [Base, Experience] (Affinity Sphere)
        // XP at idx 1 (Affinity): 1 * 7 = 7 XP
        char_data.set_affinity_sphere(Some("Forças".to_string()));
        char_data.attributes.insert("Forças".to_string(), AttributeValue {
            level: 2,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Experience],
        });

        // 4. Correspondência (Sphere): Level 2 -> [Base, Experience] (Other Sphere)
        // XP at idx 1 (Non-affinity): 1 * 8 = 8 XP
        char_data.attributes.insert("Correspondência".to_string(), AttributeValue {
            level: 2,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Experience],
        });

        // 5. Arete: Level 3 -> [Base, Bonus, Experience]
        // Bonus at idx 1: 4 pts
        // XP at idx 2: 2 * 8 = 16 XP
        char_data.attributes.insert(keys::KEY_ARETE.to_string(), AttributeValue {
            level: 3,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Experience],
        });

        // 6. Willpower Total: Level 6 -> [Base x 5, Bonus]
        // Bonus at idx 5: 1 pt
        char_data.attributes.insert(keys::KEY_WILLPOWER_TOTAL.to_string(), AttributeValue {
            level: 6,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Base, DotOrigin::Bonus],
        });

        let summary = char_data.calculate_costs();

        // Total Bonus: 5 (Força) + 2 (Talento) + 4 (Arete) + 1 (Willpower) = 12 pts
        assert_eq!(summary.total_bonus_spent, 12);
        assert_eq!(summary.bonus_limit, 15);
        assert_eq!(summary.arete_warning, false); // Arete is 3 (<= 3)

        // Total XP: 12 (Força) + 4 (Talento) + 7 (Forças Afinidade) + 8 (Correspondência) + 16 (Arete) = 47 XP
        assert_eq!(summary.total_xp_spent, 47);
    }

    #[test]
    fn test_calculate_costs_with_merits_and_flaws() {
        let mut char_data = CharacterData::new("c2".to_string(), "Mago Merits Test".to_string());

        // Qualidade: Level 3 com 2 Bônus -> +2 pontos de bônus
        char_data.custom_lists.insert(keys::CAT_MERITS.to_string(), vec!["merit_1".to_string()]);
        char_data.labels.insert("merit_1".to_string(), "Avatar Focado".to_string());
        char_data.attributes.insert("merit_1".to_string(), AttributeValue {
            level: 3,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Bonus],
        });

        // Defeito: Level 3 -> concede 3 pontos de bônus (-3 no total_bonus_spent)
        char_data.custom_lists.insert(keys::CAT_FLAWS.to_string(), vec!["flaw_1".to_string()]);
        char_data.labels.insert("flaw_1".to_string(), "Inimigo Jurado".to_string());
        char_data.attributes.insert("flaw_1".to_string(), AttributeValue {
            level: 3,
            modifier: String::new(),
            dot_origins: vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Base],
        });

        let summary = char_data.calculate_costs();
        // Total Bonus: +2 (Qualidade) - 3 (Defeito) = -1 pts
        assert_eq!(summary.total_bonus_spent, -1);
    }

    #[test]
    fn test_calculate_costs_with_wonders() {
        let mut char_data = CharacterData::new("c3".to_string(), "Mago Wonder Test".to_string());

        // Maravilha: Nível 3 (1 Base, 1 Bonus, 1 XP)
        // Bonus at idx 1: 1 pt
        // XP at idx 2: 2 * 3 = 6 XP
        char_data.wonders = vec![WonderItem {
            id: "w1".to_string(),
            name: "Anel do Éter".to_string(),
            image_url: String::new(),
            points: AttributeValue {
                level: 3,
                modifier: String::new(),
                dot_origins: vec![DotOrigin::Base, DotOrigin::Bonus, DotOrigin::Experience],
            },
            arete: AttributeValue {
                level: 2,
                modifier: String::new(),
                dot_origins: vec![DotOrigin::Base, DotOrigin::Base],
            },
            quintessence_max: 10,
            quintessence_current: 7,
            description: "Armazena quintessência e manipula forças".to_string(),
        }];

        let summary = char_data.calculate_costs();
        assert_eq!(summary.total_bonus_spent, 1);
        assert_eq!(summary.total_xp_spent, 6);
    }

    #[test]
    fn test_legacy_string_wonders_deserialization() {
        let legacy_json = r#"{
            "id": "legacy_char_1",
            "name": "Mago do Passado",
            "attributes": {},
            "labels": {},
            "custom_lists": {},
            "wonders": [
                {
                    "name": "Grimório Antigo",
                    "points": "5",
                    "arete": "3",
                    "quintessence": "4",
                    "description": "Livro de couro antigo"
                }
            ]
        }"#;

        let res: Result<CharacterData, _> = serde_json::from_str(legacy_json);
        assert!(res.is_ok(), "Failed to deserialize legacy CharacterData: {:?}", res.err());
        
        let mut char_data = res.unwrap();
        char_data.sanitize();

        assert_eq!(char_data.wonders.len(), 4);
        let wonder = &char_data.wonders[0];
        assert_eq!(wonder.name, "Grimório Antigo");
        assert_eq!(wonder.points.level, 5);
        assert_eq!(wonder.arete.level, 3);
        assert_eq!(wonder.quintessence_max, 5);
        assert_eq!(wonder.quintessence_current, 4);
    }

    #[test]
    fn test_schema_evolution_fuzzing_all_field_type_permutations() {
        let permutations = vec![
            // 1. All strings format
            r#"{"id":"fuzz_1","name":"Fuzz 1","attributes":{"Força":{"level":"4","modifier":"","dot_origins":[]}},"labels":{},"custom_lists":{},"wonders":[{"name":"W1","points":"3","arete":"2","quintessence":"5","description":""}]}"#,
            // 2. All numbers format
            r#"{"id":"fuzz_2","name":"Fuzz 2","attributes":{"Força":{"level":4,"modifier":"","dot_origins":[]}},"labels":{},"custom_lists":{},"wonders":[{"name":"W2","points":{"level":3,"modifier":"","dot_origins":[]},"arete":{"level":2,"modifier":"","dot_origins":[]},"quintessence_max":10,"quintessence_current":5,"description":""}]}"#,
            // 3. Mixed empty strings and defaults
            r#"{"id":"fuzz_3","name":"","attributes":{},"labels":{},"custom_lists":{},"wonders":[{"name":"W3","points":"","arete":"","quintessence":"","description":""}]}"#,
            // 4. Missing fields entirely
            r#"{"id":"fuzz_4","name":"Fuzz 4","attributes":{}}"#,
        ];

        for (idx, json_str) in permutations.into_iter().enumerate() {
            let res: Result<CharacterData, _> = serde_json::from_str(json_str);
            assert!(res.is_ok(), "Permutation {} failed to deserialize: {:?}", idx + 1, res.err());
            let mut char_data = res.unwrap();
            char_data.sanitize();
            assert!(!char_data.name.is_empty(), "Sanitize should ensure non-empty name");
            assert_eq!(char_data.wonders.len(), 4, "Sanitize should ensure 4 wonder slots");
        }
    }

    #[test]
    fn test_resilient_recovery_from_heavily_corrupted_json() {
        let broken_json = r#"{
            "id": "broken_123",
            "name": "Mago Sobrevivente",
            "attributes": {
                "Força": "4",
                "Destreza": 3,
                "Vigor": { "level": "5", "modifier": "Resistente", "dot_origins": ["bonus"] }
            },
            "labels": {
                "Conceito": "Sobrevivente",
                "profile_history": "Histórico longo..."
            },
            "custom_lists": {
                "Talentos": ["tal_1", "tal_2"]
            },
            "wonders": [
                { "name": "Amuleto", "points": "2", "arete": 1, "quintessence": "5" }
            ],
            "rotes": "Fórmulas de sobrevivência"
        }"#;

        let recovered = CharacterData::from_raw_json_resilient("broken_123", broken_json);
        assert!(recovered.is_some(), "Resilient recovery must succeed on salvageable JSON");
        let data = recovered.unwrap();

        assert_eq!(data.name, "Mago Sobrevivente");
        assert_eq!(data.get_attribute_level("Força", 0), 4);
        assert_eq!(data.get_attribute_level("Destreza", 0), 3);
        assert_eq!(data.get_attribute_level("Vigor", 0), 5);
        assert_eq!(data.get_label("Conceito"), "Sobrevivente");
        assert_eq!(data.wonders.len(), 4);
        assert_eq!(data.wonders[0].name, "Amuleto");
        assert_eq!(data.wonders[0].points.level, 2);
    }

    #[test]
    fn test_page3_serialization_and_recovery() {
        let mut char_data = CharacterData::new("page3_test".to_string(), "Mago Arcano".to_string());
        char_data.expanded_backgrounds.allies = "Conselho das Nove".to_string();
        char_data.expanded_backgrounds.contacts = "Informante da Interpol".to_string();
        char_data.expanded_backgrounds.other_title = "Avatar Secreto".to_string();
        char_data.expanded_backgrounds.other_text = "Manifestação ancestral".to_string();

        char_data.possessions.gear_carried = "Mochila tática, celular descartável".to_string();
        char_data.possessions.foci = "Varinha de teixo, anel hermético".to_string();
        char_data.possessions.familiar = "Corvo espiritual de nome Huginn".to_string();

        char_data.chantry = vec![
            ChantryEntry {
                location: "Capela Concordia".to_string(),
                description: "Refúgio da Tradição Hermética".to_string(),
            }
        ];

        char_data.sanitize();
        assert!(char_data.chantry.len() >= 3, "Sanitize must ensure at least 3 chantry rows");

        // Serialize and Deserialize roundtrip
        let json = serde_json::to_string(&char_data).unwrap();
        let recovered: CharacterData = serde_json::from_str(&json).unwrap();

        assert_eq!(recovered.expanded_backgrounds.allies, "Conselho das Nove");
        assert_eq!(recovered.expanded_backgrounds.other_title, "Avatar Secreto");
        assert_eq!(recovered.possessions.gear_carried, "Mochila tática, celular descartável");
        assert_eq!(recovered.possessions.familiar, "Corvo espiritual de nome Huginn");
        assert_eq!(recovered.chantry[0].location, "Capela Concordia");
    }

    #[test]
    fn test_page4_history_description_and_visuals_serialization() {
        let mut char_data = CharacterData::new("page4_test".to_string(), "Mago Hermético".to_string());
        char_data.history_data.history = "Nascido em Praga, despertou em 2017...".to_string();
        char_data.history_data.goals_destiny = "Alcançar Arete 10 e restaurar a Biblioteca de Alexandria".to_string();
        char_data.history_data.seekings = "Busca pelo Olho de Hermes".to_string();
        char_data.history_data.quiets = "Silêncio de Delírio Estático".to_string();

        char_data.description_data.age = "32".to_string();
        char_data.description_data.hair = "Negros".to_string();
        char_data.description_data.avatar_nature = "Primordial e Incandescente".to_string();
        char_data.visuals.cabal_chart_url = "/uploads/cabal_concordia.png".to_string();
        char_data.visuals.character_sketch_url = "/uploads/portrait_32.webp".to_string();

        char_data.sanitize();

        // Roundtrip serialization
        let json = serde_json::to_string(&char_data).unwrap();
        let recovered: CharacterData = serde_json::from_str(&json).unwrap();

        assert_eq!(recovered.history_data.history, "Nascido em Praga, despertou em 2017...");
        assert_eq!(recovered.history_data.goals_destiny, "Alcançar Arete 10 e restaurar a Biblioteca de Alexandria");
        assert_eq!(recovered.description_data.age, "32");
        assert_eq!(recovered.description_data.avatar_nature, "Primordial e Incandescente");
        assert_eq!(recovered.visuals.cabal_chart_url, "/uploads/cabal_concordia.png");
        assert_eq!(recovered.visuals.character_sketch_url, "/uploads/portrait_32.webp");
    }

    #[test]
    fn test_validate_image_magic_bytes_security() {
        use crate::state::server_fns::validate_image_magic_bytes;

        // Valid PNG
        let valid_png = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR...";
        assert_eq!(validate_image_magic_bytes(valid_png).unwrap(), ("image/png", "png"));

        // Valid JPEG
        let valid_jpeg = b"\xFF\xD8\xFF\xE0\x00\x10JFIF...";
        assert_eq!(validate_image_magic_bytes(valid_jpeg).unwrap(), ("image/jpeg", "jpg"));

        // Valid WebP
        let valid_webp = b"RIFF\x00\x00\x00\x00WEBPVP8 ...";
        assert_eq!(validate_image_magic_bytes(valid_webp).unwrap(), ("image/webp", "webp"));

        // Valid GIF
        let valid_gif = b"GIF89a\x01\x00\x01\x00...";
        assert_eq!(validate_image_magic_bytes(valid_gif).unwrap(), ("image/gif", "gif"));

        // Valid SVG
        let valid_svg = b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\"></svg>";
        assert_eq!(validate_image_magic_bytes(valid_svg).unwrap(), ("image/svg+xml", "svg"));

        // MALICIOUS / INVALID: Fake PNG with shell script or ELF binary
        let fake_png_script = b"#!/bin/bash\necho 'malicious script'";
        assert!(validate_image_magic_bytes(fake_png_script).is_err());

        // MALICIOUS / INVALID: Fake Windows EXE
        let fake_exe = b"MZ\x90\x00\x03\x00\x00\x00\x04\x00\x00\x00\xff\xff";
        assert!(validate_image_magic_bytes(fake_exe).is_err());

        // MALICIOUS / INVALID: Random garbage
        let garbage = b"random string without image signature";
        assert!(validate_image_magic_bytes(garbage).is_err());
    }

    #[test]
    fn test_character_summary_enriched_serialization() {
        use crate::state::models::CharacterSummary;

        let summary = CharacterSummary {
            id: "mago_777".to_string(),
            name: "John Constantine".to_string(),
            tradition: "Culto do Êxtase".to_string(),
            essence: "Dinâmica".to_string(),
            arete: 4,
            willpower: 8,
            photo_url: "/uploads/john.webp".to_string(),
            spheres: vec![("Tempo".to_string(), 3), ("Mente".to_string(), 2)],
            sheet_type: "mage".to_string(),
            is_public: true,
            is_owner: true,
            updated_at: "2026-08-21 17:00:00".to_string(),
        };

        let json = serde_json::to_string(&summary).unwrap();
        let recovered: CharacterSummary = serde_json::from_str(&json).unwrap();

        assert_eq!(recovered.id, "mago_777");
        assert_eq!(recovered.name, "John Constantine");
        assert_eq!(recovered.tradition, "Culto do Êxtase");
        assert_eq!(recovered.essence, "Dinâmica");
        assert_eq!(recovered.arete, 4);
        assert_eq!(recovered.willpower, 8);
        assert_eq!(recovered.photo_url, "/uploads/john.webp");
        assert_eq!(recovered.spheres, vec![("Tempo".to_string(), 3), ("Mente".to_string(), 2)]);
        assert!(recovered.is_public);
        assert!(recovered.is_owner);

        // Backward compatibility: old JSON without the new fields
        let legacy_json = r#"{"id":"legacy_1","name":"Mago Antigo","updated_at":"2026-08-20"}"#;
        let legacy_recovered: CharacterSummary = serde_json::from_str(legacy_json).unwrap();
        assert_eq!(legacy_recovered.name, "Mago Antigo");
        assert_eq!(legacy_recovered.arete, 1);
        assert_eq!(legacy_recovered.willpower, 5);
        assert!(!legacy_recovered.is_public);
        assert!(!legacy_recovered.is_owner);
        assert!(legacy_recovered.photo_url.is_empty());
    }

    #[test]
    fn test_wod_health_track_sorting_and_overflow() {
        use crate::state::models::{CharacterData, DamageType};

        let mut data = CharacterData::new("test_char".to_string(), "Test Mage".to_string());
        assert_eq!(data.get_health_counts(), (0, 0, 0));

        // 1. Click on index 2 (Ferido / 3rd box) on clean sheet -> marks 3 Bashing
        data.click_health_box(2);
        assert_eq!(data.get_health_counts(), (0, 0, 3));
        assert_eq!(data.get_health(0), DamageType::Bashing);
        assert_eq!(data.get_health(1), DamageType::Bashing);
        assert_eq!(data.get_health(2), DamageType::Bashing);
        assert_eq!(data.get_health(3), DamageType::None);

        // 2. Click on index 1 (Machucado / 2nd box with Bashing) -> marks 2 Lethal, pushing 3 Bashing down
        data.click_health_box(1);
        assert_eq!(data.get_health_counts(), (0, 2, 3));
        assert_eq!(data.get_health(0), DamageType::Lethal);
        assert_eq!(data.get_health(1), DamageType::Lethal);
        assert_eq!(data.get_health(2), DamageType::Bashing);
        assert_eq!(data.get_health(3), DamageType::Bashing);
        assert_eq!(data.get_health(4), DamageType::Bashing);
        assert_eq!(data.get_health(5), DamageType::None);

        // 3. Click on index 0 (Lethal) -> marks 1 Aggravated, pushing Lethal and Bashing down
        data.click_health_box(0);
        assert_eq!(data.get_health_counts(), (1, 2, 3));
        assert_eq!(data.get_health(0), DamageType::Aggravated);
        assert_eq!(data.get_health(1), DamageType::Lethal);
        assert_eq!(data.get_health(2), DamageType::Lethal);
        assert_eq!(data.get_health(3), DamageType::Bashing);
        assert_eq!(data.get_health(4), DamageType::Bashing);
        assert_eq!(data.get_health(5), DamageType::Bashing);
        assert_eq!(data.get_health(6), DamageType::None);

        // 4. Click index 0 (Aggravated) -> heals 1 Aggravated, shifting damage up
        data.click_health_box(0);
        assert_eq!(data.get_health_counts(), (0, 2, 3));
        assert_eq!(data.get_health(0), DamageType::Lethal);
        assert_eq!(data.get_health(1), DamageType::Lethal);
        assert_eq!(data.get_health(2), DamageType::Bashing);

        // 5. Overflow test: Fill track with 4 Lethal and 3 Bashing (total 7 boxes full)
        data.set_health_counts(0, 4, 3);
        assert_eq!(data.get_health_counts(), (0, 4, 3));

        // Add 1 excess bashing: (0, 4, 4) -> should upgrade 1 bashing to lethal, resulting in (0, 5, 2)
        data.set_health_counts(0, 4, 4);
        assert_eq!(data.get_health_counts(), (0, 5, 2));

        // Add excess lethal to full lethal track: (0, 8, 0) -> upgrades 1 lethal to aggravated -> (1, 6, 0)
        data.set_health_counts(0, 8, 0);
        assert_eq!(data.get_health_counts(), (1, 6, 0));

        // 6. Right click / heal box
        data.heal_health_box(0); // Heals the aggravated box
        assert_eq!(data.get_health_counts(), (0, 6, 0));

        // Clear all
        data.clear_health();
        assert_eq!(data.get_health_counts(), (0, 0, 0));
        assert_eq!(data.get_health(0), DamageType::None);
    }

    #[test]
    fn test_gods_and_monsters_creation_and_pools() {
        let mut gm = CharacterData::new_gods_and_monsters("gm_01".to_string(), "Quimera de Hermes".to_string());
        assert_eq!(gm.sheet_type, "gods_and_monsters");
        assert!(gm.is_gods_and_monsters());
        assert_eq!(gm.labels.get("Type").unwrap(), "Familiar");

        // Gnosis test (10 dots + 10 boxes)
        assert_eq!(gm.get_gnosis(), (0, "0".repeat(10)));
        gm.set_gnosis_dots(4);
        assert_eq!(gm.get_gnosis().0, 4);
        gm.cycle_gnosis_box(0);
        gm.cycle_gnosis_box(1);
        assert_eq!(gm.get_gnosis().1, "1100000000");

        // Essence Pool test (50 boxes - progressive like dots)
        assert_eq!(gm.get_essence_pool(), (0, "0".repeat(50)));
        gm.click_essence_box(4); // Fills 5 boxes (indices 0..=4)
        assert_eq!(gm.get_essence_pool().0, 5);
        assert_eq!(&gm.get_essence_pool().1[0..5], "11111");
        assert_eq!(&gm.get_essence_pool().1[5..10], "00000");

        gm.click_essence_box(4); // Clicking same box reduces by 1 -> 4 boxes
        assert_eq!(gm.get_essence_pool().0, 4);

        gm.clear_essence(); // Clears all
        assert_eq!(gm.get_essence_pool().0, 0);

        gm.set_essence_spent(25);
        assert_eq!(gm.get_essence_pool().0, 25);

        // Paradox Pool test (20 boxes)
        assert_eq!(gm.get_paradox_pool(), (0, "0".repeat(20)));
        gm.cycle_paradox_box(0);
        gm.cycle_paradox_box(19);
        assert_eq!(gm.get_paradox_pool().0, 2);

        // Extra Bruised Health Levels (Gods & Monsters / Large Bygones)
        assert_eq!(gm.get_extra_bruised(), 0);
        assert_eq!(gm.get_total_health_boxes(), 7);
        gm.add_extra_bruised();
        gm.add_extra_bruised();
        assert_eq!(gm.get_extra_bruised(), 2);
        assert_eq!(gm.get_total_health_boxes(), 9);

        // Click box 8 (Incapacitado) with Bashing -> fills all 9 boxes with Bashing
        gm.click_health_box(8);
        assert_eq!(gm.get_health_counts(), (0, 0, 9));

        // Excess damage overflow on 9 boxes:
        gm.set_health_counts(0, 5, 5); // 10 total -> upgrades 1 to lethal -> (0, 6, 3)
        assert_eq!(gm.get_health_counts(), (0, 6, 3));

        gm.remove_extra_bruised();
        assert_eq!(gm.get_extra_bruised(), 1);
        assert_eq!(gm.get_total_health_boxes(), 8);

        // Roundtrip JSON serialization preserves all pools and sheet_type
        let json = serde_json::to_string(&gm).unwrap();
        let recovered: CharacterData = serde_json::from_str(&json).unwrap();
        assert_eq!(recovered.sheet_type, "gods_and_monsters");
        assert!(recovered.is_gods_and_monsters());
        assert_eq!(recovered.get_gnosis().0, 4);
        assert_eq!(recovered.get_essence_pool().0, 25);
        assert_eq!(recovered.get_paradox_pool().0, 2);
        assert_eq!(recovered.get_extra_bruised(), 1);
        assert_eq!(recovered.get_total_health_boxes(), 8);
    }
}

