#[cfg(test)]
pub mod change {
    use crate::Change;
    use serde_json::json;

    pub fn create(pattern: &str, kind: &str) -> Change {
        Change::from(&json!({
            "pattern": pattern,
            "kind": kind
        }))
        .unwrap()
    }
}
