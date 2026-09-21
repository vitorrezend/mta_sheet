use std::fs;
use std::path::Path;

#[test]
fn test_robots_txt_content() {
    let seo_rs = Path::new("src/server/handlers/seo.rs");
    assert!(seo_rs.exists(), "src/server/handlers/seo.rs deve existir!");

    let content = fs::read_to_string(seo_rs).expect("Falha ao ler seo.rs");
    assert!(
        content.contains("User-agent: *"),
        "❌ robots.txt deve permitir rastreamento para todos os User-agents!"
    );
    assert!(
        content.contains("Sitemap: /sitemap.xml"),
        "❌ robots.txt deve apontar para o sitemap.xml!"
    );
}

#[test]
fn test_sitemap_xml_structure() {
    let seo_rs = Path::new("src/server/handlers/seo.rs");
    let content = fs::read_to_string(seo_rs).expect("Falha ao ler seo.rs");

    assert!(
        content.contains("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"),
        "❌ sitemap.xml deve conter o schema padrão sitemaps.org!"
    );
    assert!(
        content.contains("<loc>/</loc>"),
        "❌ sitemap.xml deve conter a URL raiz (/)!"
    );
    assert!(
        content.contains("<loc>/rooms</loc>"),
        "❌ sitemap.xml deve conter a rota pública de salas (/rooms)!"
    );
}

#[test]
fn test_favicon_svg_validity() {
    let seo_rs = Path::new("src/server/handlers/seo.rs");
    let content = fs::read_to_string(seo_rs).expect("Falha ao ler seo.rs");

    assert!(
        content.contains("<svg") && content.contains("</svg>"),
        "❌ favicon.svg deve ser um arquivo SVG vetorial válido!"
    );
    assert!(
        content.contains("viewBox=\"0 0 64 64\""),
        "❌ favicon.svg deve conter viewBox quadrado (64x64)!"
    );
}

#[test]
fn test_seo_meta_tags_in_html_shell() {
    let lib_rs = Path::new("src/lib.rs");
    let content = fs::read_to_string(lib_rs).expect("Falha ao ler src/lib.rs");

    // Title
    assert!(
        content.contains("<Title text=\"MTA Sheet"),
        "❌ Title tag com nome e propósito semântico deve estar presente!"
    );

    // Meta Description & Keywords
    assert!(
        content.contains(r#"<Meta name="description""#),
        "❌ Meta description deve estar presente para indexação no Google!"
    );
    assert!(
        content.contains(r#"<Meta name="keywords""#),
        "❌ Meta keywords deve estar presente com termos de Mago M20 e Gods & Monsters!"
    );

    // Robots & Theme-color
    assert!(
        content.contains(r#"<Meta name="robots" content="index, follow"/>"#),
        "❌ Meta robots index, follow deve estar presente!"
    );
    assert!(
        content.contains(r#"<Meta name="theme-color""#),
        "❌ Meta theme-color deve estar presente para navegadores móveis!"
    );

    // Open Graph
    assert!(
        content.contains(r#"<Meta property="og:title""#),
        "❌ Open Graph og:title deve estar presente para compartilhamento em redes!"
    );
    assert!(
        content.contains(r#"<Meta property="og:description""#),
        "❌ Open Graph og:description deve estar presente!"
    );
    assert!(
        content.contains(r#"<Meta property="og:type" content="website"/>"#),
        "❌ Open Graph og:type deve ser 'website'!"
    );

    // Twitter Card
    assert!(
        content.contains(r#"<Meta name="twitter:card" content="summary"/>"#)
            || content.contains(r#"<Meta name="twitter:card" content="summary_large_image"/>"#),
        "❌ Twitter Card deve estar presente!"
    );

    // Favicon Link & Canonical
    assert!(
        content.contains(r#"<Link rel="icon" type_="image/svg+xml" href="/favicon.svg"/>"#),
        "❌ Link do Favicon vetorial deve estar configurado no HTML raiz!"
    );
    assert!(
        content.contains(r#"<Link rel="canonical""#),
        "❌ Link canonical deve estar presente para SEO!"
    );

    // Schema.org JSON-LD
    assert!(
        content.contains(r#"<Script type_="application/ld+json">"#),
        "❌ Dados estruturados JSON-LD (Schema.org WebApplication) devem estar presentes!"
    );
    assert!(
        content.contains("WebApplication") && content.contains("GameApplication"),
        "❌ O Schema JSON-LD deve declarar a aplicação como WebApplication / GameApplication!"
    );
}

#[test]
fn test_home_showcase_component_and_styles() {
    let showcase_rs = Path::new("src/components/views/home/showcase.rs");
    assert!(showcase_rs.exists(), "src/components/views/home/showcase.rs deve existir!");

    let showcase_content = fs::read_to_string(showcase_rs).expect("Falha ao ler showcase.rs");
    assert!(
        showcase_content.contains("home-hero-section"),
        "❌ O componente HomeShowcase deve renderizar a hero section (.home-hero-section)!"
    );
    assert!(
        showcase_content.contains("home-features-showcase"),
        "❌ O componente HomeShowcase deve renderizar a vitrine de recursos (.home-features-showcase)!"
    );
    assert!(
        showcase_content.contains("showcase-card-sheet")
            && showcase_content.contains("showcase-card-rules")
            && showcase_content.contains("showcase-card-combat")
            && showcase_content.contains("showcase-card-rooms"),
        "❌ Os 4 cards de recursos canônicos (Ficha, Regras, Combate/Dô e Salas) devem estar presentes!"
    );

    let home_css = Path::new("styles/home.css");
    let css_content = fs::read_to_string(home_css).expect("Falha ao ler styles/home.css");
    assert!(
        css_content.contains(".home-hero-section") && css_content.contains(".showcase-cards-grid"),
        "❌ styles/home.css deve conter estilos para hero section e showcase cards grid!"
    );
}

#[test]
fn test_home_layout_and_tabs_architecture() {
    let home_mod_rs = Path::new("src/components/views/home/mod.rs");
    let content = fs::read_to_string(home_mod_rs).expect("Falha ao ler home/mod.rs");

    // O showcase no topo da página deve ser exclusivo para visitantes não autenticados
    assert!(
        content.contains("if user.get().is_none()") && content.contains("<HomeShowcase"),
        "❌ O HomeShowcase deve ser renderizado no topo exclusivamente para visitantes!"
    );

    // A seção de criação de ficha deve estar no topo para usuários logados
    assert!(
        content.contains(r#"<section class="create-section">"#),
        "❌ A seção de criação de ficha (<section class=\"create-section\">) deve estar no topo para usuários logados!"
    );

    // As abas de navegação de fichas devem estar limpas e focadas
    assert!(
        content.contains("home_tab_my_sheets") && content.contains("home_tab_public_sheets"),
        "❌ As abas de fichas (Minhas Fichas e Fichas Públicas) devem estar presentes!"
    );

    // As traduções da aba devem estar registradas
    let i18n_rs = Path::new("src/i18n.rs");
    let i18n_content = fs::read_to_string(i18n_rs).expect("Falha ao ler i18n.rs");
    assert!(
        i18n_content.contains(r#""home_tab_showcase" => "✨ Apresentação & Recursos""#),
        "❌ A tradução pt-BR de home_tab_showcase deve estar presente!"
    );
    assert!(
        i18n_content.contains(r#""home_tab_showcase" => "✨ Showcase & Features""#),
        "❌ A tradução en-US de home_tab_showcase deve estar presente!"
    );
}

#[test]
fn test_html_lang_and_font_preload() {
    let lib_rs = Path::new("src/lib.rs");
    let content = fs::read_to_string(lib_rs).expect("Falha ao ler src/lib.rs");

    assert!(
        content.contains("<Html lang="),
        "❌ O atributo [lang] deve ser injetado dinamicamente no elemento <html> raiz via leptos_meta::Html!"
    );
    assert!(
        content.contains(r#"href="/fonts/cinzel-latin.woff2" as_="font""#),
        "❌ A fonte primária Cinzel deve ter tag de preload no <head> para eliminar FOUC e atraso de renderização!"
    );
}

#[test]
fn test_character_sheet_seo_and_robots() {
    let cs_rs = Path::new("src/components/views/character_sheet.rs");
    let content = fs::read_to_string(cs_rs).expect("Falha ao ler character_sheet.rs");

    assert!(
        content.contains(r#"name="description""#),
        "❌ A visualização da ficha deve conter meta description dinâmica com nome e tradição!"
    );
    assert!(
        content.contains(r#"name="robots""#) && content.contains(r#""index, follow""#) && content.contains(r#""noindex, nofollow""#),
        "❌ A visualização da ficha deve ter robots condicional ('index, follow' se pública, 'noindex, nofollow' se privada)!"
    );
}

