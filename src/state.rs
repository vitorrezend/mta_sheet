use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttributeValue {
    pub level: i32,
    pub modifier: String,
}

impl Default for AttributeValue {
    fn default() -> Self {
        Self {
            level: 0,
            modifier: String::new(),
        }
    }
}

impl AttributeValue {
    pub fn save_individual(&self, name: &str) {
        let window = web_sys::window().expect("no window");
        let storage = window.local_storage().ok().flatten().expect("no storage");
        let key = format!("attr_{}", name);
        
        if let Ok(json) = serde_json::to_string(self) {
            let _ = storage.set_item(&key, &json);
        }
    }

    pub fn load_individual(name: &str) -> Self {
        let window = web_sys::window().expect("no window");
        let storage = window.local_storage().ok().flatten().expect("no storage");
        let key = format!("attr_{}", name);

        storage.get_item(&key)
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CharacterState {
    pub attributes: HashMap<String, AttributeValue>,
    pub labels: HashMap<String, String>,
}

impl CharacterState {
    pub fn save_label(name: &str, value: &str) {
        let window = web_sys::window().expect("no window");
        let storage = window.local_storage().ok().flatten().expect("no storage");
        let key = format!("label_{}", name);
        let _ = storage.set_item(&key, value);
    }

    pub fn load_label(name: &str) -> String {
        let window = web_sys::window().expect("no window");
        let storage = window.local_storage().ok().flatten().expect("no storage");
        let key = format!("label_{}", name);
        storage.get_item(&key).ok().flatten().unwrap_or_default()
    }

    // Carregamos cada campo individualmente
    pub fn load_all(attr_names: &[&'static str], label_names: &[&'static str]) -> Self {
        let mut attributes = HashMap::new();
        for &name in attr_names {
            attributes.insert(name.to_string(), AttributeValue::load_individual(name));
        }

        let mut labels = HashMap::new();
        for &name in label_names {
            labels.insert(name.to_string(), Self::load_label(name));
        }

        Self { attributes, labels }
    }

    pub fn save_custom_list(category: &str, list: &[String]) {
        let window = web_sys::window().expect("no window");
        let storage = window.local_storage().ok().flatten().expect("no storage");
        let key = format!("custom_list_{}", category);
        if let Ok(json) = serde_json::to_string(list) {
            let _ = storage.set_item(&key, &json);
        }
    }

    pub fn load_custom_list(category: &str) -> Vec<String> {
        let window = web_sys::window().expect("no window");
        let storage = window.local_storage().ok().flatten().expect("no storage");
        let key = format!("custom_list_{}", category);
        storage.get_item(&key)
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_attribute_value_default() {
        let attr = AttributeValue::default();
        assert_eq!(attr.level, 0);
        assert_eq!(attr.modifier, "");
    }

    #[wasm_bindgen_test]
    fn test_save_and_load_individual() {
        let attr = AttributeValue {
            level: 4,
            modifier: "Test Mod".to_string(),
        };
        let name = "test_attr";
        
        // Save
        attr.save_individual(name);
        
        // Load
        let loaded = AttributeValue::load_individual(name);
        
        assert_eq!(loaded.level, 4);
        assert_eq!(loaded.modifier, "Test Mod");
    }

    #[wasm_bindgen_test]
    fn test_save_and_load_label() {
        let name = "test_label";
        let value = "Test Value";
        
        // Save
        CharacterState::save_label(name, value);
        
        // Load
        let loaded = CharacterState::load_label(name);
        
        assert_eq!(loaded, value);
    }

    #[test]
    fn test_character_state_default() {
        let state = CharacterState::default();
        assert!(state.attributes.is_empty());
        assert!(state.labels.is_empty());
    }
}
