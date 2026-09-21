use std::fs;
use std::path::Path;
use mta_sheet::state::{CharacterData, DamageType};

#[test]
fn test_vitality_component_and_svg_markings() {
    let vitality_rs = Path::new("src/components/mta_sheet/page1/advantages/vitality.rs");
    assert!(vitality_rs.exists(), "src/components/mta_sheet/page1/advantages/vitality.rs deve existir!");

    let content = fs::read_to_string(vitality_rs).expect("Falha ao ler vitality.rs");

    // Verifica se o componente HealthDamageIcon foi criado
    assert!(
        content.contains("fn HealthDamageIcon"),
        "❌ HealthDamageIcon deve estar presente para renderizar as marcas de dano!"
    );

    // Verifica traçado canônico SVG para Contundente (barra diagonal /)
    assert!(
        content.contains("dmg-bashing-svg") && content.contains(r#"line x1="3" y1="13" x2="13" y2="3""#),
        "❌ Dano contundente deve possuir traçado diagonal SVG preciso (3, 13) -> (13, 3)!"
    );

    // Verifica traçado canônico SVG para Letal (cruz X)
    assert!(
        content.contains("dmg-lethal-svg")
            && content.contains(r#"line x1="3" y1="13" x2="13" y2="3""#)
            && content.contains(r#"line x1="3" y1="3" x2="13" y2="13""#),
        "❌ Dano letal deve possuir as duas diagonais do X cruzado no SVG!"
    );

    // Verifica traçado canônico SVG para Agravado (asterisco de 4 eixos / 8 pontas da White Wolf)
    assert!(
        content.contains("dmg-agg-svg")
            && content.contains(r#"line x1="8" y1="2" x2="8" y2="14""#)
            && content.contains(r#"line x1="2" y1="8" x2="14" y2="8""#),
        "❌ Dano agravado deve possuir o asterisco estelar com as linhas vertical e horizontal cruzadas!"
    );

    // Verifica indicador de linha de ferimento ativo
    assert!(
        content.contains("is-active-wound") && content.contains("is_active_wound"),
        "❌ A linha do nível de ferimento ativo deve ter a classe reativa is-active-wound!"
    );

    // Verifica se os termos oficiais de M20 (Contundente, Letal, Agravado) estão presentes
    assert!(
        content.contains("Contundente") && content.contains("Letal") && content.contains("Agravado"),
        "❌ As pílulas de contagem devem usar a nomenclatura oficial M20 (Contundente, Letal, Agravado)!"
    );
}

#[test]
fn test_vitality_css_styles() {
    let css_file = Path::new("styles/04-page1-main.css");
    let content = fs::read_to_string(css_file).expect("Falha ao ler 04-page1-main.css");

    // Estilos das caixas de dano por estado
    assert!(
        content.contains(".health-box.damage-bashing")
            && content.contains(".health-box.damage-lethal")
            && content.contains(".health-box.damage-aggravated"),
        "❌ styles/04-page1-main.css deve conter classes com cores temáticas para cada tipo de dano!"
    );

    // Linha de ferimento ativo
    assert!(
        content.contains(".health-row.is-active-wound"),
        "❌ styles/04-page1-main.css deve conter destaque para a linha de ferimento ativo (.health-row.is-active-wound)!"
    );

    // SVGs com cores e sombras
    assert!(
        content.contains(".dmg-bashing-svg")
            && content.contains(".dmg-lethal-svg")
            && content.contains(".dmg-agg-svg"),
        "❌ styles/04-page1-main.css deve conter classes específicas para os SVGs de dano!"
    );
}

#[test]
fn test_party_cards_use_refined_damage_symbols() {
    let desktop_rs = Path::new("src/components/rooms/party_card_desktop.rs");
    let desktop_content = fs::read_to_string(desktop_rs).expect("Falha ao ler party_card_desktop.rs");
    assert!(
        desktop_content.contains(r#""lethal" => "✕""#) && desktop_content.contains(r#""aggravated" => "✳""#),
        "❌ party_card_desktop.rs deve usar os glifos refinados ✕ e ✳ para o HUD da Cabala!"
    );

    let mobile_rs = Path::new("src/components/rooms/party_card_mobile.rs");
    let mobile_content = fs::read_to_string(mobile_rs).expect("Falha ao ler party_card_mobile.rs");
    assert!(
        mobile_content.contains(r#""lethal" => "✕""#) && mobile_content.contains(r#""aggravated" => "✳""#),
        "❌ party_card_mobile.rs deve usar os glifos refinados ✕ e ✳ para o HUD móvel da Cabala!"
    );
}

#[test]
fn test_character_data_health_counts_and_cycling() {
    let mut data = CharacterData::default();
    assert_eq!(data.get_health_counts(), (0, 0, 0));
    assert_eq!(data.get_health(0), DamageType::None);

    // Marca Contundente
    data.click_health_box(0);
    assert_eq!(data.get_health(0), DamageType::Bashing);
    assert_eq!(data.get_health_counts(), (0, 0, 1));

    // Evolui para Letal
    data.click_health_box(0);
    assert_eq!(data.get_health(0), DamageType::Lethal);
    assert_eq!(data.get_health_counts(), (0, 1, 0));

    // Evolui para Agravado
    data.click_health_box(0);
    assert_eq!(data.get_health(0), DamageType::Aggravated);
    assert_eq!(data.get_health_counts(), (1, 0, 0));

    // Cura
    data.heal_health_box(0);
    assert_eq!(data.get_health(0), DamageType::None);
    assert_eq!(data.get_health_counts(), (0, 0, 0));
}
