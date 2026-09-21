use leptos::*;
use crate::components::Callback;
use crate::components::common::wysiwyg_editor::{WysiwygEditor, legacy_markdown_to_html, sanitize_wysiwyg_html};
use crate::i18n::Language;

// Re-exporta utilitários para compatibilidade
pub use crate::components::common::wysiwyg_editor::{
    legacy_markdown_to_html as rote_legacy_markdown_to_html,
    sanitize_wysiwyg_html as rote_sanitize_wysiwyg_html,
};

/// Editor WYSIWYG Real para a Descrição Narrativa & Mecânica do Grimório.
/// Permite edição rica direta no documento com negrito, itálico, sublinhado
/// e listas visíveis em tempo real, sem necessidade de códigos Markdown ou botão de prévia.
#[component]
pub fn RoteDescriptionEditor(
    #[prop(into)] description: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] lang: Signal<Language>,
) -> impl IntoView {
    let placeholder_sig = Signal::derive(move || match lang.try_get().unwrap_or_default() {
        Language::PtBr => "Descreva o procedimento mágico, narrativa visual do feitiço, paradas de dados, dificuldade, gastos de quintessência, regras de paradoxo e efeitos mecânicos...".to_string(),
        Language::EnUs => "Describe the magical procedure, visual manifestation, dice pool, difficulty, quintessence cost, paradox, and mechanical effects...".to_string(),
    });

    view! {
        <div class="rote-desc-wrap">
            <div class="rote-desc-header">
                <label class="rote-desc-label">
                    <span class="desc-icon">"📜"</span>
                    {move || match lang.get() {
                        Language::PtBr => "DESCRIÇÃO NARRATIVA & EFEITOS MECÂNICOS:",
                        Language::EnUs => "NARRATIVE DESCRIPTION & MECHANICS:",
                    }}
                </label>
            </div>

            <WysiwygEditor
                value=description
                on_change=on_change
                placeholder=placeholder_sig
                class="rote-wysiwyg-custom"
                min_height="95px"
                lang=lang
            />

            <div class="rote-desc-hint">
                <span class="hint-icon">"💡"</span>
                {move || match lang.try_get().unwrap_or_default() {
                    Language::PtBr => "Editor visual: selecione qualquer trecho para aplicar negrito, itálico ou listas. Suporta atalhos (Ctrl+B, Ctrl+I, Ctrl+U). Tab insere recuo e Esc desfaz o foco.",
                    Language::EnUs => "WYSIWYG editor: select any text to apply bold, italic or lists. Supports shortcuts (Ctrl+B, Ctrl+I, Ctrl+U). Tab indents and Esc unfocuses.",
                }}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_markdown_to_html_paragraphs_and_bold() {
        let md = "Juntar os itens\n\n- As **Blueprints Originais**.\n- A Escritura.";
        let html = legacy_markdown_to_html(md);
        assert!(html.contains("<p>Juntar os itens</p>"));
        assert!(html.contains("<ul><li>As <strong>Blueprints Originais</strong>.</li>"));
        assert!(html.contains("<li>A Escritura.</li></ul>"));
    }

    #[test]
    fn test_legacy_markdown_to_html_italic() {
        let md = "Efeito com *palavra itálica*.";
        let html = legacy_markdown_to_html(md);
        assert!(html.contains("<em>palavra itálica</em>"));
    }

    #[test]
    fn test_already_html_preserved() {
        let rich = "<p>Texto já <strong>formatado</strong></p>";
        let html = legacy_markdown_to_html(rich);
        assert_eq!(html, rich);
    }

    #[test]
    fn test_sanitize_wysiwyg_html_strips_scripts() {
        let dangerous = "<p>Texto</p><script>alert('xss')</script>";
        let cleaned = sanitize_wysiwyg_html(dangerous);
        assert!(!cleaned.contains("<script"));
    }
}
