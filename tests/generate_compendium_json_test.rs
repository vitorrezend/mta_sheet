use std::fs;
use std::path::Path;
use serde::Serialize;
use mta_sheet::compendium::{
    ALL_WEAPONS, ALL_RULE_NOTES, ALL_COMBAT_MANEUVERS, ALL_PRACTICES, BOX_LEFT_AND_RIGHT_HAND_PATHS,
    ALL_INSTRUMENTS, ALL_THEORY_ARTICLES, ALL_ARCHETYPES, ARCHETYPE_THEORY_RULES,
    ALL_ATTRIBUTES, ALL_BACKGROUNDS, BACKGROUND_THEORY_RULES,
};

#[derive(Serialize)]
struct WeaponsCompendiumPayload {
    weapons: &'static [mta_sheet::compendium::weapons::WeaponDefinition],
    notes: &'static [mta_sheet::compendium::weapons::WeaponRuleNote],
    maneuvers: &'static [mta_sheet::compendium::weapons::CombatManeuver],
}

#[derive(Serialize)]
struct PracticesCompendiumPayload {
    callout: &'static mta_sheet::compendium::practices::CalloutBox,
    practices: &'static [mta_sheet::compendium::practices::PracticeDefinition],
}

#[derive(Serialize)]
struct InstrumentsCompendiumPayload {
    articles: &'static [mta_sheet::compendium::instruments::InstrumentTheoryArticle],
    instruments: &'static [mta_sheet::compendium::instruments::InstrumentDefinition],
}

#[derive(Serialize)]
struct ArchetypesCompendiumPayload {
    theory: &'static mta_sheet::compendium::archetypes::ArchetypeTheoryArticle,
    archetypes: &'static [mta_sheet::compendium::archetypes::ArchetypeDefinition],
}

#[derive(Serialize)]
struct AttributesCompendiumPayload {
    attributes: &'static [mta_sheet::compendium::attributes::AttributeDefinition],
}

#[derive(Serialize)]
struct BackgroundsCompendiumPayload {
    theory: &'static mta_sheet::compendium::backgrounds::BackgroundTheoryArticle,
    backgrounds: &'static [mta_sheet::compendium::backgrounds::BackgroundDefinition],
}

#[test]
fn test_generate_compendium_json_files() {
    let out_dir = Path::new("data/compendium");
    fs::create_dir_all(out_dir).expect("Failed to create data/compendium directory");

    // 1. weapons.json
    let weapons_payload = WeaponsCompendiumPayload {
        weapons: ALL_WEAPONS,
        notes: ALL_RULE_NOTES,
        maneuvers: ALL_COMBAT_MANEUVERS,
    };
    let weapons_json = serde_json::to_string_pretty(&weapons_payload).expect("Failed to serialize weapons");
    fs::write(out_dir.join("weapons.json"), weapons_json).expect("Failed to write weapons.json");

    // 2. practices.json
    let practices_payload = PracticesCompendiumPayload {
        callout: &BOX_LEFT_AND_RIGHT_HAND_PATHS,
        practices: ALL_PRACTICES,
    };
    let practices_json = serde_json::to_string_pretty(&practices_payload).expect("Failed to serialize practices");
    fs::write(out_dir.join("practices.json"), practices_json).expect("Failed to write practices.json");

    // 3. instruments.json
    let instruments_payload = InstrumentsCompendiumPayload {
        articles: ALL_THEORY_ARTICLES,
        instruments: ALL_INSTRUMENTS,
    };
    let instruments_json = serde_json::to_string_pretty(&instruments_payload).expect("Failed to serialize instruments");
    fs::write(out_dir.join("instruments.json"), instruments_json).expect("Failed to write instruments.json");

    // 4. archetypes.json
    let archetypes_payload = ArchetypesCompendiumPayload {
        theory: &ARCHETYPE_THEORY_RULES,
        archetypes: ALL_ARCHETYPES,
    };
    let archetypes_json = serde_json::to_string_pretty(&archetypes_payload).expect("Failed to serialize archetypes");
    fs::write(out_dir.join("archetypes.json"), archetypes_json).expect("Failed to write archetypes.json");

    // 5. attributes.json
    let attributes_payload = AttributesCompendiumPayload {
        attributes: ALL_ATTRIBUTES,
    };
    let attributes_json = serde_json::to_string_pretty(&attributes_payload).expect("Failed to serialize attributes");
    fs::write(out_dir.join("attributes.json"), attributes_json).expect("Failed to write attributes.json");

    // 6. backgrounds.json
    let backgrounds_payload = BackgroundsCompendiumPayload {
        theory: &BACKGROUND_THEORY_RULES,
        backgrounds: ALL_BACKGROUNDS,
    };
    let backgrounds_json = serde_json::to_string_pretty(&backgrounds_payload).expect("Failed to serialize backgrounds");
    fs::write(out_dir.join("backgrounds.json"), backgrounds_json).expect("Failed to write backgrounds.json");

    assert!(out_dir.join("weapons.json").exists());
    assert!(out_dir.join("practices.json").exists());
    assert!(out_dir.join("instruments.json").exists());
    assert!(out_dir.join("archetypes.json").exists());
    assert!(out_dir.join("attributes.json").exists());
    assert!(out_dir.join("backgrounds.json").exists());
}
