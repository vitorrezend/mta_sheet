use std::fs;
use std::path::Path;

#[test]
fn test_viewport_meta_tag_configured() {
    let lib_rs = Path::new("src/lib.rs");
    assert!(lib_rs.exists(), "src/lib.rs deve existir!");

    let content = fs::read_to_string(lib_rs).expect("Falha ao ler src/lib.rs");
    assert!(
        content.contains(r#"<Meta name="viewport" content="width=device-width, initial-scale=1.0"/>"#)
            || content.contains(r#"<meta name="viewport""#),
        "❌ A meta tag de viewport responsivo (width=device-width, initial-scale=1.0) deve estar presente no HTML raiz!"
    );
}

#[test]
fn test_global_box_sizing_and_body_overflow_configured() {
    let vars_css = Path::new("styles/01-variables.css");
    assert!(vars_css.exists(), "styles/01-variables.css deve existir!");

    let content = fs::read_to_string(vars_css).expect("Falha ao ler styles/01-variables.css");

    // Box sizing universal
    assert!(
        content.contains("box-sizing: border-box"),
        "❌ styles/01-variables.css deve conter 'box-sizing: border-box' universal para prevenir estouro de largura por padding!"
    );

    // Proteção de overflow horizontal no body
    assert!(
        content.contains("overflow-x: hidden"),
        "❌ styles/01-variables.css deve conter proteção contra rolagem horizontal ('overflow-x: hidden') para mobile!"
    );
}

#[test]
fn test_navbar_has_mobile_media_query_and_wrapping() {
    let vars_css = Path::new("styles/01-variables.css");
    let content = fs::read_to_string(vars_css).expect("Falha ao ler styles/01-variables.css");

    // Verifica se a navbar possui media query para telas móveis
    assert!(
        content.contains("@media (max-width: 680px)") || content.contains("@media (max-width: 768px)"),
        "❌ styles/01-variables.css deve conter media query para a barra de navegação superior (.navbar-container)!"
    );

    // Verifica que a navbar em mobile permite quebra (wrap) ou empilhamento para não espremer botões
    assert!(
        content.contains("flex-wrap: wrap"),
        "❌ A navbar deve permitir 'flex-wrap: wrap' em mobile para evitar corte de botões na lateral!"
    );
}

#[test]
fn test_home_page_has_mobile_responsiveness_rules() {
    let home_css = Path::new("styles/home.css");
    assert!(home_css.exists(), "styles/home.css deve existir!");

    let content = fs::read_to_string(home_css).expect("Falha ao ler styles/home.css");

    // Verifica media query na Home
    assert!(
        content.contains("@media (max-width: 640px)"),
        "❌ styles/home.css deve conter bloco de regras para mobile (@media (max-width: 640px))!"
    );

    // Verifica que o card de visitante empilha verticalmente para o botão não vazar da tela
    assert!(
        content.contains(".visitor-banner") && content.contains("flex-direction: column"),
        "❌ O banner de visitante (.visitor-banner) deve adotar 'flex-direction: column' em mobile para que o botão 'Entrar / Cadastrar' não vaze a borda!"
    );

    // Verifica que o botão de login no banner ocupa largura total no mobile
    assert!(
        content.contains(".visitor-login-btn") && content.contains("width: 100%"),
        "❌ O botão de login do banner (.visitor-login-btn) deve ter 'width: 100%' em mobile para área de toque confortável!"
    );

    // Verifica que a barra de navegação do Drive (pastas/fichas) empilha em mobile
    assert!(
        content.contains(".drive-nav-bar") && content.contains("flex-direction: column"),
        "❌ A barra de navegação de pastas (.drive-nav-bar) deve adotar 'flex-direction: column' em mobile para não cortar o botão '+ Nova Pasta'!"
    );

    // Verifica que o grid de cards passa a 1 coluna em telas estreitas
    assert!(
        content.contains(".character-cards-grid") && content.contains("grid-template-columns: 1fr"),
        "❌ O grid de fichas (.character-cards-grid) deve ajustar para 'grid-template-columns: 1fr' em telas móveis!"
    );

    // Verifica que o botão de criação de ficha mobile possui alto contraste com tokens canônicos
    assert!(
        content.contains(".mobile-create-toggle-btn") && content.contains("var(--surface-paper"),
        "❌ O botão de nova ficha mobile (.mobile-create-toggle-btn) deve possuir alto contraste usando 'var(--surface-paper)' para texto visível!"
    );
    assert!(
        content.contains(".mobile-create-toggle-btn") && content.contains("var(--purple-accent"),
        "❌ O botão de nova ficha mobile (.mobile-create-toggle-btn) deve utilizar a paleta púrpura mística canônica do projeto!"
    );
}

#[test]
fn test_sheet_type_selector_mobile_responsiveness() {
    let gm_css = Path::new("styles/09-gods-and-monsters.css");
    assert!(gm_css.exists(), "styles/09-gods-and-monsters.css deve existir!");

    let content = fs::read_to_string(gm_css).expect("Falha ao ler styles/09-gods-and-monsters.css");

    // Verifica que o seletor de arquétipos não estoura telas pequenas
    assert!(
        content.contains("@media (max-width: 480px)") && content.contains(".sheet-type-selector"),
        "❌ O seletor de tipo de ficha (.sheet-type-selector) deve ter adaptação mobile para não estourar telas de 320px/360px!"
    );
}

#[test]
fn test_auth_page_mobile_responsiveness() {
    let auth_css = Path::new("styles/auth.css");
    assert!(auth_css.exists(), "styles/auth.css deve existir!");

    let content = fs::read_to_string(auth_css).expect("Falha ao ler styles/auth.css");

    // Container ocupa 100% de largura
    assert!(
        content.contains(".auth-page-container") && content.contains("width: 100%"),
        "❌ .auth-page-container deve ter 'width: 100%' para centralizar corretamente na viewport móvel!"
    );

    // Card é responsivo com largura total
    assert!(
        content.contains(".auth-card") && content.contains("width: 100%"),
        "❌ .auth-card deve ter 'width: 100%' para se adaptar a qualquer tamanho de smartphone!"
    );

    // Possui media query de mobile
    assert!(
        content.contains("@media (max-width: 640px)"),
        "❌ styles/auth.css deve conter regras dedicadas para mobile (@media (max-width: 640px))!"
    );

    // O layout da ficha isola seu alinhamento usando :has(.sheet-container)
    let sheet_layout = Path::new("styles/03-sheet-layout.css");
    let sheet_content = fs::read_to_string(sheet_layout).expect("Falha ao ler styles/03-sheet-layout.css");
    assert!(
        sheet_content.contains("body:has(.sheet-container)"),
        "❌ styles/03-sheet-layout.css deve escopar 'body:has(.sheet-container)' para não desalinhar a tela de login!"
    );
}

#[test]
fn test_modal_overlays_touch_action_and_scroll_lock() {
    // 1. Compêndio M20 não pode conter 'touch-action: none' no overlay
    let compendium_css = Path::new("styles/12-compendium.css");
    assert!(compendium_css.exists(), "styles/12-compendium.css deve existir!");
    let compendium_content = fs::read_to_string(compendium_css).expect("Falha ao ler styles/12-compendium.css");
    assert!(
        !compendium_content.contains("touch-action: none"),
        "❌ .practice-compendium-overlay NÃO deve ter 'touch-action: none' pois trava toques e scroll no mobile! Use 'manipulation'."
    );
    assert!(
        compendium_content.contains(".practice-compendium-overlay") && compendium_content.contains("touch-action: manipulation"),
        "❌ .practice-compendium-overlay deve ter 'touch-action: manipulation' para garantir scroll e toques responsivos no mobile!"
    );

    // 2. Compêndio de Armas não pode conter 'touch-action: none' no overlay
    let magic_combat_css = Path::new("styles/05-page2-magic-combat.css");
    assert!(magic_combat_css.exists(), "styles/05-page2-magic-combat.css deve existir!");
    let combat_content = fs::read_to_string(magic_combat_css).expect("Falha ao ler styles/05-page2-magic-combat.css");
    assert!(
        !combat_content.contains("touch-action: none"),
        "❌ .weapon-compendium-overlay NÃO deve ter 'touch-action: none'! Use 'manipulation'."
    );
    assert!(
        combat_content.contains(".weapon-compendium-overlay") && combat_content.contains("touch-action: manipulation"),
        "❌ .weapon-compendium-overlay deve ter 'touch-action: manipulation' para navegação touch no mobile!"
    );

    // 3. Modal Overlay canônico em styles/02-common.css deve ter overscroll-behavior e touch-action
    let common_css = Path::new("styles/02-common.css");
    assert!(common_css.exists(), "styles/02-common.css deve existir!");
    let common_content = fs::read_to_string(common_css).expect("Falha ao ler styles/02-common.css");
    assert!(
        common_content.contains(".modal-overlay") && common_content.contains("touch-action: manipulation"),
        "❌ .modal-overlay deve conter 'touch-action: manipulation' para suporte a toque fluido em modais!"
    );
}

#[test]
fn test_profile_page_mobile_responsiveness_and_card_constraints() {
    let profile_css = Path::new("styles/profile.css");
    assert!(profile_css.exists(), "styles/profile.css deve existir!");
    let css_content = fs::read_to_string(profile_css).expect("Falha ao ler styles/profile.css");

    // 1. Deve conter media queries de mobile e tablet
    assert!(
        css_content.contains("@media (max-width: 768px)") && css_content.contains("@media (max-width: 480px)"),
        "❌ styles/profile.css deve conter regras dedicadas para mobile (@media (max-width: 768px) e @media (max-width: 480px))!"
    );

    // 2. Deve limitar altura e corte do retrato nos cards
    assert!(
        css_content.contains(".feed-card-portrait-box") && css_content.contains("height: 180px"),
        "❌ .feed-card-portrait-box deve ter altura fixa (180px) para impedir estouro de fotos gigantes!"
    );
    assert!(
        css_content.contains(".feed-card-portrait-img") && css_content.contains("object-fit: cover"),
        "❌ .feed-card-portrait-img deve conter 'object-fit: cover' para preenchimento harmônico do retrato!"
    );

    // 3. Grid de cards deve se ajustar para 1 coluna no mobile
    assert!(
        css_content.contains("grid-template-columns: 1fr"),
        "❌ O grid de cards do perfil deve ajustar para 1 coluna ('grid-template-columns: 1fr') em telas estreitas!"
    );

    // 4. Componente profile.rs não deve usar classes legadas quebradas
    let profile_rs = Path::new("src/components/views/profile.rs");
    assert!(profile_rs.exists(), "src/components/views/profile.rs deve existir!");
    let rs_content = fs::read_to_string(profile_rs).expect("Falha ao ler src/components/views/profile.rs");

    assert!(
        !rs_content.contains("class=\"feed-grid\""),
        "❌ src/components/views/profile.rs não deve usar classe legada 'feed-grid'! Use 'feed-cards-grid'."
    );
    assert!(
        !rs_content.contains("class=\"feed-card-portrait-wrapper\""),
        "❌ src/components/views/profile.rs não deve usar 'feed-card-portrait-wrapper'! Use 'feed-card-portrait-box'."
    );
    assert!(
        !rs_content.contains("class=\"feed-card-portrait\""),
        "❌ src/components/views/profile.rs não deve usar 'feed-card-portrait'! Use 'feed-card-portrait-img'."
    );
    assert!(
        !rs_content.contains("class=\"feed-card-content\""),
        "❌ src/components/views/profile.rs não deve usar 'feed-card-content'! Use 'feed-card-body'."
    );
}

#[test]
fn test_app_shell_and_global_navigation_integrity() {
    // 1. Garante que src/lib.rs implementa o AppLayout com Navbar e Outlet
    let lib_rs = Path::new("src/lib.rs");
    assert!(lib_rs.exists(), "src/lib.rs deve existir!");
    let lib_content = fs::read_to_string(lib_rs).expect("Falha ao ler src/lib.rs");

    assert!(
        lib_content.contains("pub fn AppLayout() -> impl IntoView") && lib_content.contains("<crate::components::Navbar />") && lib_content.contains("<Outlet />"),
        "❌ src/lib.rs deve implementar o componente 'AppLayout' renderizando <Navbar /> e <Outlet />!"
    );

    // 2. Garante que as rotas filhas estejam contidas dentro de AppLayout
    assert!(
        lib_content.contains(r#"<Route path="" view=AppLayout>"#),
        "❌ As rotas da aplicação devem estar aninhadas dentro do layout pai <Route path=\"\" view=AppLayout>!"
    );

    // 3. Garante que o App Shell e animação de transição estejam definidos em styles/01-variables.css
    let vars_css = Path::new("styles/01-variables.css");
    assert!(vars_css.exists(), "styles/01-variables.css deve existir!");
    let vars_content = fs::read_to_string(vars_css).expect("Falha ao ler styles/01-variables.css");

    assert!(
        vars_content.contains(".app-shell") && vars_content.contains(".app-main-content"),
        "❌ styles/01-variables.css deve conter as classes canônicas do App Shell (.app-shell e .app-main-content)!"
    );
    assert!(
        vars_content.contains("fadeInPage"),
        "❌ styles/01-variables.css deve conter animação de transição suave 'fadeInPage' para trocas de rota sem piscar!"
    );

    // 4. Garante que FeedPage e ProfilePage possuem barra de navegação de retorno (breadcrumb)
    let feed_rs = Path::new("src/components/views/feed.rs");
    assert!(feed_rs.exists(), "src/components/views/feed.rs deve existir!");
    let feed_content = fs::read_to_string(feed_rs).expect("Falha ao ler src/components/views/feed.rs");

    assert!(
        feed_content.contains("page-breadcrumb-nav") && feed_content.contains("breadcrumb-back-link"),
        "❌ src/components/views/feed.rs deve conter a barra de retorno com 'page-breadcrumb-nav' e 'breadcrumb-back-link'!"
    );

    let profile_rs = Path::new("src/components/views/profile.rs");
    let profile_content = fs::read_to_string(profile_rs).expect("Falha ao ler src/components/views/profile.rs");
    assert!(
        profile_content.contains("page-breadcrumb-nav") && profile_content.contains("breadcrumb-back-link"),
        "❌ src/components/views/profile.rs deve conter a barra de retorno com 'page-breadcrumb-nav' e 'breadcrumb-back-link'!"
    );

    // 5. Garante que styles/feed.css possui responsividade e cabeçalho místico
    let feed_css = Path::new("styles/feed.css");
    assert!(feed_css.exists(), "styles/feed.css deve existir!");
    let feed_css_content = fs::read_to_string(feed_css).expect("Falha ao ler styles/feed.css");

    assert!(
        feed_css_content.contains("@media (max-width: 768px)") && feed_css_content.contains("@media (max-width: 480px)"),
        "❌ styles/feed.css deve conter media queries de mobile (@media (max-width: 768px) e @media (max-width: 480px))!"
    );
    assert!(
        feed_css_content.contains("var(--purple-deep)") && feed_css_content.contains("var(--gold-light)"),
        "❌ styles/feed.css deve utilizar a paleta mística canônica (purple-deep e gold-light) no cabeçalho!"
    );
}
