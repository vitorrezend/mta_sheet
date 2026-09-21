use mta_sheet::components::common::wysiwyg_editor::{
    legacy_markdown_to_html, sanitize_wysiwyg_html, WysiwygEditor,
};
use mta_sheet::components::Callback;
use leptos::*;

#[test]
fn test_legacy_markdown_paragraphs_and_bold() {
    let md = "Primeiro parágrafo com **negrito importante**.\n\nSegundo parágrafo simples.";
    let html = legacy_markdown_to_html(md);
    assert!(html.contains("<p>Primeiro parágrafo com <strong>negrito importante</strong>.</p>"));
    assert!(html.contains("<p>Segundo parágrafo simples.</p>"));
}

#[test]
fn test_legacy_markdown_bullet_and_numbered_lists() {
    let md_bullets = "- Item A\n- Item B com *itálico*\n- Item C";
    let html_bullets = legacy_markdown_to_html(md_bullets);
    assert!(html_bullets.contains("<ul>"));
    assert!(html_bullets.contains("<li>Item A</li>"));
    assert!(html_bullets.contains("<li>Item B com <em>itálico</em></li>"));
    assert!(html_bullets.contains("<li>Item C</li>"));
    assert!(html_bullets.contains("</ul>"));

    let md_numbered = "1. Primeiro passo\n2. Segundo passo com **destaque**\n3. Terceiro passo";
    let html_numbered = legacy_markdown_to_html(md_numbered);
    assert!(html_numbered.contains("<ol>"));
    assert!(html_numbered.contains("<li>Primeiro passo</li>"));
    assert!(html_numbered.contains("<li>Segundo passo com <strong>destaque</strong></li>"));
    assert!(html_numbered.contains("<li>Terceiro passo</li>"));
    assert!(html_numbered.contains("</ol>"));
}

#[test]
fn test_already_formatted_html_intact() {
    let existing_html = "<p>Parágrafo existente</p><ul><li>Marcador 1</li><li>Marcador 2</li></ul>";
    let result = legacy_markdown_to_html(existing_html);
    assert_eq!(result, existing_html);
}

#[test]
fn test_sanitize_security_xss_protection() {
    let malicious = "<p>Normal</p><script>alert('hack')</script><img src='x' onerror='alert(1)'><a href='javascript:evil()'>Link</a>";
    let sanitized = sanitize_wysiwyg_html(malicious);
    assert!(!sanitized.contains("<script"));
    assert!(sanitized.contains("&lt;script"));
    assert!(!sanitized.contains("onerror="));
    assert!(!sanitized.contains("javascript:"));
}

#[test]
fn test_ssr_render_wysiwyg_editor() {
    let rendered = leptos::ssr::render_to_string(|| {
        let (val, set_val) = create_signal("Trecho com **palavra forte** e *itálica*.".to_string());
        let on_change = Callback::new(move |new_val| set_val.set(new_val));

        view! {
            <div class="test-container">
                <WysiwygEditor
                    value=val
                    on_change=on_change
                    placeholder="Digite seu texto..."
                    class="test-wysiwyg"
                    min_height="120px"
                />
            </div>
        }
    });

    assert!(!rendered.is_empty(), "SSR do WysiwygEditor não deve estar vazio");
    assert!(rendered.contains("wysiwyg-wrapper"), "Deve conter o container wrapper");
    assert!(rendered.contains("wysiwyg-toolbar"), "Deve conter a barra de ferramentas");
    assert!(rendered.contains("wysiwyg-content"), "Deve conter a área de edição contenteditable");
    assert!(rendered.contains("contenteditable=\"true\""), "Deve possuir atributo contenteditable");
    assert!(rendered.contains("data-placeholder=\"Digite seu texto...\""), "Deve possuir placeholder data attribute");

    // Valida também que o conversor produz as tags HTML semânticas esperadas
    let html = legacy_markdown_to_html("Trecho com **palavra forte** e *itálica*.");
    assert!(html.contains("<strong>palavra forte</strong>"), "Deve converter markdown em strong");
    assert!(html.contains("<em>itálica</em>"), "Deve converter markdown em em");
}
