//! Sanitização de HTML rico (editor WYSIWYG) baseada em lista branca via `ammonia`.
//!
//! É usada em dois pontos:
//! - no servidor, antes de persistir fichas e notas de sala (nada perigoso chega ao banco);
//! - no cliente, imediatamente antes de `set_inner_html` (neutraliza dados legados já gravados).

use std::collections::HashSet;
use std::sync::LazyLock;

/// Tags que o editor produz (toolbar + colagem de texto formatado). Nenhum atributo é permitido.
const ALLOWED_TAGS: &[&str] = &[
    "p", "br", "div", "span", "b", "strong", "i", "em", "u", "s", "strike", "del", "sub", "sup",
    "ul", "ol", "li", "h1", "h2", "h3", "h4", "h5", "h6", "blockquote", "pre", "code", "hr",
];

static SANITIZER: LazyLock<ammonia::Builder<'static>> = LazyLock::new(|| {
    let mut builder = ammonia::Builder::empty();
    builder
        .add_tags(ALLOWED_TAGS.iter().copied())
        // Remove a tag E o conteúdo (sem isso, o texto do <script> sobraria como texto solto)
        .clean_content_tags(HashSet::from(["script", "style", "noscript", "template"]));
    builder
});

/// Retorna `true` quando o texto contém algo que um navegador interpretaria como marcação
/// (`<` seguido de letra, `/`, `!` ou `?`). Textos como "Força < 3" ou "<3" não contam.
pub fn looks_like_html(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.windows(2).any(|w| {
        w[0] == b'<' && (w[1].is_ascii_alphabetic() || matches!(w[1], b'/' | b'!' | b'?'))
    })
}

/// Sanitiza um fragmento HTML mantendo apenas a formatação permitida pelo editor.
pub fn sanitize_rich_html(html: &str) -> String {
    SANITIZER.clean(html).to_string()
}

/// Sanitiza apenas se o texto parecer HTML; texto puro é devolvido intacto
/// (evita reescrever `&` ou `>` em campos simples).
pub fn sanitize_if_html(text: &str) -> Option<String> {
    if !looks_like_html(text) {
        return None;
    }
    let clean = sanitize_rich_html(text);
    (clean != text).then_some(clean)
}

/// Percorre recursivamente um JSON e sanitiza toda string que pareça HTML.
/// Retorna `true` se algum valor foi alterado.
pub fn sanitize_json_strings(value: &mut serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => match sanitize_if_html(s) {
            Some(clean) => {
                *s = clean;
                true
            }
            None => false,
        },
        serde_json::Value::Array(items) => items.iter_mut().fold(false, |changed, v| sanitize_json_strings(v) | changed),
        serde_json::Value::Object(map) => map.values_mut().fold(false, |changed, v| sanitize_json_strings(v) | changed),
        _ => false,
    }
}

/// Sanitiza todas as strings HTML de uma estrutura serializável (ficha, capela da sala...).
///
/// Percorre o JSON genericamente em vez de listar campos: qualquer campo rico novo
/// fica protegido automaticamente. Se o round-trip falhar, a estrutura não é alterada
/// e o erro é retornado para o chamador decidir (no servidor, recusar o salvamento).
pub fn sanitize_rich_text_in<T>(data: &mut T) -> Result<(), serde_json::Error>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let mut value = serde_json::to_value(&*data)?;
    if sanitize_json_strings(&mut value) {
        *data = serde_json::from_value(value)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_editor_formatting() {
        let html = "<p>Texto <strong>forte</strong> e <em>itálico</em></p><ul><li>Item</li></ul>";
        assert_eq!(sanitize_rich_html(html), html);
    }

    #[test]
    fn strips_case_variant_event_handlers() {
        let clean = sanitize_rich_html("<img src=x OnError=alert(1)><p ONCLICK=\"x()\">a</p>");
        let lower = clean.to_lowercase();
        assert!(!lower.contains("onerror"));
        assert!(!lower.contains("onclick"));
        assert!(!lower.contains("<img"));
    }

    #[test]
    fn strips_svg_and_script_content() {
        let clean = sanitize_rich_html("<svg onload=alert(1)><circle/></svg><SCRIPT>alert(2)</SCRIPT><p>ok</p>");
        assert_eq!(clean, "<p>ok</p>");
    }

    #[test]
    fn plain_text_is_untouched() {
        assert!(!looks_like_html("Força < 3 & Destreza > 2"));
        assert!(!looks_like_html("<3"));
        assert_eq!(sanitize_if_html("Força < 3 & Destreza > 2"), None);
        assert!(looks_like_html("a<b>c</b>"));
    }

    #[test]
    fn json_walk_sanitizes_nested_strings() {
        let mut v = serde_json::json!({
            "labels": { "experience": "<p>ok</p><img src=x onerror=alert(1)>", "plain": "a < b" },
            "items": [{ "description": "<iframe src=//evil></iframe><b>x</b>" }]
        });
        assert!(sanitize_json_strings(&mut v));
        assert_eq!(v["labels"]["experience"], "<p>ok</p>");
        assert_eq!(v["labels"]["plain"], "a < b");
        assert_eq!(v["items"][0]["description"], "<b>x</b>");
    }
}
