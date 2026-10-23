//! JSON Merge Patch (RFC 7386): o salvamento automático envia só o que mudou na ficha.
//!
//! - objetos são comparados recursivamente;
//! - qualquer outro valor (inclusive listas) é substituído inteiro;
//! - `null` no patch remove a chave (a ficha volta ao valor padrão daquele campo).
//!
//! Limitação do formato: um `null` legítimo não pode ser enviado como valor. As fichas
//! não usam `null` (campos vazios são omitidos), então isso não é problema aqui.

use serde_json::{Map, Value};

/// Patch que transforma `base` em `current`, ou `None` se forem iguais.
pub fn merge_diff(base: &Value, current: &Value) -> Option<Value> {
    if base == current {
        return None;
    }
    match (base, current) {
        (Value::Object(b), Value::Object(c)) => {
            let mut patch = Map::new();
            for (key, cur_val) in c {
                match b.get(key) {
                    Some(base_val) => {
                        if let Some(sub) = merge_diff(base_val, cur_val) {
                            patch.insert(key.clone(), sub);
                        }
                    }
                    None => {
                        patch.insert(key.clone(), cur_val.clone());
                    }
                }
            }
            for key in b.keys() {
                if !c.contains_key(key) {
                    patch.insert(key.clone(), Value::Null);
                }
            }
            (!patch.is_empty()).then_some(Value::Object(patch))
        }
        _ => Some(current.clone()),
    }
}

/// Aplica um merge patch sobre `target`.
pub fn merge_apply(target: &mut Value, patch: &Value) {
    let Value::Object(patch_map) = patch else {
        *target = patch.clone();
        return;
    };
    if !target.is_object() {
        *target = Value::Object(Map::new());
    }
    if let Value::Object(target_map) = target {
        for (key, value) in patch_map {
            if value.is_null() {
                target_map.remove(key);
            } else {
                merge_apply(target_map.entry(key.clone()).or_insert(Value::Null), value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn diff_contains_only_changed_paths() {
        let base = json!({"name": "Ana", "attributes": {"Força": {"level": 2}, "Vigor": {"level": 3}}, "notes": "x"});
        let current = json!({"name": "Ana", "attributes": {"Força": {"level": 4}, "Vigor": {"level": 3}}, "notes": "x"});
        assert_eq!(merge_diff(&base, &current), Some(json!({"attributes": {"Força": {"level": 4}}})));
        assert_eq!(merge_diff(&base, &base), None);
    }

    #[test]
    fn removed_keys_become_null_and_lists_are_replaced() {
        let base = json!({"a": 1, "list": [1, 2, 3], "gone": "x"});
        let current = json!({"a": 1, "list": [1, 2]});
        assert_eq!(merge_diff(&base, &current), Some(json!({"list": [1, 2], "gone": null})));
    }

    #[test]
    fn apply_of_diff_reproduces_current() {
        let base = json!({"n": "A", "m": {"x": 1, "y": {"z": 2}}, "del": true, "arr": [1]});
        let current = json!({"n": "B", "m": {"x": 1, "y": {"z": 3, "w": 4}}, "arr": [2, 3], "novo": "sim"});
        let patch = merge_diff(&base, &current).expect("há diferenças");
        let mut target = base.clone();
        merge_apply(&mut target, &patch);
        assert_eq!(target, current);
    }

    #[test]
    fn apply_keeps_other_peoples_untouched_changes() {
        // Servidor já tem uma mudança do mestre em "health"; o jogador só mexeu em "name"
        let mut server = json!({"name": "Ana", "health": 3});
        merge_apply(&mut server, &json!({"name": "Ana Maria"}));
        assert_eq!(server, json!({"name": "Ana Maria", "health": 3}));
    }
}
