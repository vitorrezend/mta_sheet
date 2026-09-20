use std::fs;
use std::path::Path;

#[test]
fn test_about_page_code_and_github_card() {
    let about_file = Path::new("src/components/views/about.rs");
    assert!(about_file.exists(), "src/components/views/about.rs deve existir!");

    let content = fs::read_to_string(about_file).expect("Falha ao ler about.rs");

    // Deve conter a URL oficial do repositório no GitHub
    assert!(
        content.contains("https://github.com/vitorrezend/mta_sheet"),
        "about.rs deve conter o link do repositório no GitHub"
    );

    // Deve conter o comando de clone
    assert!(
        content.contains("git clone https://github.com/vitorrezend/mta_sheet.git"),
        "about.rs deve fornecer o comando git clone"
    );

    // Deve conter o link para Issues
    assert!(
        content.contains("https://github.com/vitorrezend/mta_sheet/issues"),
        "about.rs deve conter o link para reportar issues"
    );

    // Deve mencionar a licença MIT
    assert!(
        content.contains("LICENÇA MIT"),
        "about.rs deve referenciar a licença MIT"
    );

    // Deve conter as seções de colaboração
    assert!(
        content.contains("Testes & Relato de Falhas")
            && content.contains("Código & Engenharia")
            && content.contains("Regras, Lore & Grimório")
            && content.contains("Internacionalização & Textos"),
        "about.rs deve cobrir os 4 pilares de colaboração"
    );
}

#[test]
fn test_about_routes_and_navigation() {
    // 1. Verificar registro das rotas em lib.rs
    let lib_file = Path::new("src/lib.rs");
    let lib_content = fs::read_to_string(lib_file).expect("Falha ao ler lib.rs");

    assert!(
        lib_content.contains("path=\"about\""),
        "src/lib.rs deve conter a rota 'about'"
    );
    assert!(
        lib_content.contains("path=\"colabore\""),
        "src/lib.rs deve conter a rota alternativa 'colabore'"
    );

    // 2. Verificar link na Navbar
    let navbar_file = Path::new("src/components/common/navbar.rs");
    let nav_content = fs::read_to_string(navbar_file).expect("Falha ao ler navbar.rs");

    assert!(
        nav_content.contains("href=\"/about\""),
        "navbar.rs deve conter o link para a página /about"
    );

    // 3. Verificar link no rodapé da Home
    let home_file = Path::new("src/components/views/home/mod.rs");
    let home_content = fs::read_to_string(home_file).expect("Falha ao ler home/mod.rs");

    assert!(
        home_content.contains("href=\"/about\""),
        "home/mod.rs deve conter o link para /about no rodapé"
    );

    // 4. Verificar inclusão de /about no sitemap.xml
    let seo_file = Path::new("src/server/handlers/seo.rs");
    let seo_content = fs::read_to_string(seo_file).expect("Falha ao ler seo.rs");

    assert!(
        seo_content.contains("<loc>/about</loc>"),
        "seo.rs deve incluir /about no sitemap.xml"
    );
}

#[test]
fn test_about_css_tokens_compliance() {
    let css_file = Path::new("styles/about.css");
    assert!(css_file.exists(), "styles/about.css deve existir!");

    let content = fs::read_to_string(css_file).expect("Falha ao ler styles/about.css");

    // Deve utilizar Design Tokens canônicos
    assert!(
        content.contains("var(--purple-deep)")
            && content.contains("var(--gold-primary)")
            && content.contains("var(--surface-card)"),
        "styles/about.css deve utilizar as variáveis canônicas de design"
    );

    // Deve estar importado no style.css
    let main_css = Path::new("style.css");
    let main_content = fs::read_to_string(main_css).expect("Falha ao ler style.css");

    assert!(
        main_content.contains("@import url('/styles/about.css');"),
        "style.css deve importar styles/about.css"
    );
}

#[test]
fn test_readme_bilingual_completeness() {
    let pt_readme = Path::new("README.md");
    let en_readme = Path::new("README.en.md");

    assert!(pt_readme.exists(), "README.md (Português) deve existir!");
    assert!(en_readme.exists(), "README.en.md (English) deve existir!");

    let pt_content = fs::read_to_string(pt_readme).expect("Falha ao ler README.md");
    let en_content = fs::read_to_string(en_readme).expect("Falha ao ler README.en.md");

    // Ambos devem conter links cruzados de idiomas no topo
    assert!(pt_content.contains("README.en.md"), "README.md deve linkar para README.en.md");
    assert!(en_content.contains("README.md"), "README.en.md deve linkar para README.md");

    // Ambos devem conter o repositório do GitHub
    assert!(pt_content.contains("https://github.com/vitorrezend/mta_sheet"));
    assert!(en_content.contains("https://github.com/vitorrezend/mta_sheet"));

    // Ambos devem explicar a Guerra pela Realidade / Tradições de M20
    assert!(pt_content.contains("Tradições Místicas"));
    assert!(en_content.contains("Mystick Traditions"));

    // Ambos devem conter instruções de Docker e Compilação
    assert!(pt_content.contains("docker compose up -d"));
    assert!(en_content.contains("docker compose up -d"));
    assert!(pt_content.contains("wasm-bindgen-cli"));
    assert!(en_content.contains("wasm-bindgen-cli"));

    // Ambos devem conter o aviso legal da Paradox Interactive
    assert!(pt_content.contains("Paradox Interactive AB"));
    assert!(en_content.contains("Paradox Interactive AB"));
}

