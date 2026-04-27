#[cfg(test)]
pub mod change {
    use crate::Change;
    use serde_json::json;

    pub fn create(pattern: &str, kind: &str, priority: u32) -> Change {
        serde_json::from_value(json!({
            "pattern": pattern,
            "kind": kind,
            "priority": priority
        }))
        .unwrap()
    }
}
