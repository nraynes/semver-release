use crate::Change;
use serde_json::json;

#[allow(dead_code)]
pub fn mock_change(pattern: &str, kind: &str) -> Change {
    Change::from(&json!({
        "pattern": pattern,
        "kind": kind
    }))
    .unwrap()
}
