use regex::Regex;
use rust_yaml::Value;

use crate::models::{Alert, Commit};

#[derive(Clone)]
pub struct Change {
    pattern: Regex,
    kind: String,
}

impl Change {
    pub fn kind(&self) -> &String {
        &self.kind
    }

    pub fn from(value: &Value) -> Result<Self, Alert> {
        let change_map = value
            .as_mapping()
            .ok_or("Change could not be parsed from config. Check syntax.")?;
        let packaged_pattern = change_map
            .get(&Value::from("pattern"))
            .ok_or("No pattern found in change declaration in config.")?;
        let packaged_kind = change_map
            .get(&Value::from("pattern"))
            .ok_or("No kind found in change declaration in config.")?;
        let pattern_string = packaged_pattern
            .as_str()
            .ok_or("pattern in change declaration in config wrong syntax.")?;
        let pattern = Regex::new(pattern_string)?;
        let kind = packaged_kind
            .as_str()
            .ok_or("kind in change declaration in config wrong syntax.")?;
        Ok(Change {
            pattern,
            kind: String::from(kind),
        })
    }

    pub fn check(&self, commit: &Commit) -> bool {
        self.pattern.is_match(commit.msg())
    }
}
