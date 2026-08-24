use crate::state::CharacterData;

/// Converte a ficha de personagem em JSON formatado e dispara o download no navegador.
pub fn export_character_json(data: &CharacterData) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;

        let json_str = match serde_json::to_string_pretty(data) {
            Ok(s) => s,
            Err(err) => {
                log::error!("Erro ao serializar ficha para JSON: {:?}", err);
                return;
            }
        };

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let blob_parts = js_sys::Array::new();
                blob_parts.push(&wasm_bindgen::JsValue::from_str(&json_str));

                let mut blob_props = web_sys::BlobPropertyBag::new();
                blob_props.type_("application/json;charset=utf-8");

                if let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &blob_props) {
                    if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                        if let Ok(element) = document.create_element("a") {
                            if let Ok(a) = element.dyn_into::<web_sys::HtmlAnchorElement>() {
                                let raw_name = data.name.trim();
                                let safe_name = if raw_name.is_empty() {
                                    "ficha_mta".to_string()
                                } else {
                                    raw_name
                                        .replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "_")
                                };

                                let filename = format!("{}_mta_sheet.json", safe_name);
                                a.set_href(&url);
                                a.set_download(&filename);
                                a.click();
                                let _ = web_sys::Url::revoke_object_url(&url);
                                log::info!("Download da ficha JSON disparado: {}", filename);
                            }
                        }
                    }
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = data;
    }
}

/// Analisa uma string JSON, realiza a sanitização e recuperação resiliente de campos
/// e retorna a estrutura `CharacterData` validada.
pub fn parse_and_sanitize_sheet_json(json_str: &str) -> Result<CharacterData, String> {
    if json_str.trim().is_empty() {
        return Err("O arquivo JSON está vazio.".to_string());
    }

    if let Some(mut data) = CharacterData::from_raw_json_resilient("", json_str) {
        data.sanitize();
        Ok(data)
    } else {
        Err("O arquivo fornecido não contém um formato de ficha JSON válido.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_sanitize_valid_json() {
        let original = CharacterData::new("test-123".to_string(), "Mago Arcano".to_string());
        let json_str = serde_json::to_string(&original).unwrap();

        let parsed = parse_and_sanitize_sheet_json(&json_str).unwrap();
        assert_eq!(parsed.name, "Mago Arcano");
    }

    #[test]
    fn test_parse_and_sanitize_legacy_or_partial_json() {
        let partial_json = r#"{
            "name": "Mago Antigo",
            "attributes": {
                "Força": { "level": 4 }
            }
        }"#;

        let parsed = parse_and_sanitize_sheet_json(partial_json).unwrap();
        assert_eq!(parsed.name, "Mago Antigo");
        assert_eq!(parsed.attributes.get("Força").map(|v| v.level).unwrap_or(0), 4);
    }

    #[test]
    fn test_parse_and_sanitize_invalid_json_returns_err() {
        let invalid_json = "isto nao e um json { broken";
        let res = parse_and_sanitize_sheet_json(invalid_json);
        assert!(res.is_err());

        let empty_json = "";
        let res_empty = parse_and_sanitize_sheet_json(empty_json);
        assert!(res_empty.is_err());
    }
}
