use derive_getters::Getters;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::models::{Alert, Commit};

/// A pattern that represents a type of change based on a commit message.
#[derive(Serialize, Deserialize, Clone, Debug, Getters)]
pub struct Change {
    priority: u32,
    pattern: String,
    kind: String,
}

impl PartialEq for Change {
    fn eq(&self, other: &Self) -> bool {
        self.pattern.as_str() == other.pattern.as_str() && self.kind == other.kind
    }
}

impl Change {
    /// Checks a commit to see if it matches this change pattern.
    pub fn check(&self, commit: &Commit) -> Result<(), Alert> {
        let r = Regex::new(&self.pattern)?;
        match r.is_match(commit.msg()) {
            true => Ok(()),
            false => Err(Alert::from("Pattern did not match")),
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::{Value, json};

    use crate::{Change, tests::mock};

    #[test]
    fn test_change_new_valid() {
        let content: Value = json!({
            "priority": 1,
            "pattern": "^test.*$",
            "kind": "TEST"
        });
        let change: Result<Change, serde_json::Error> = serde_json::from_value(content);
        assert_eq!(change.is_ok(), true);
    }

    #[test]
    fn test_change_new_invalid() {
        let content: Value = json!(["Invalid", { "JSON_KEY": 334 }]);
        let change: Result<Change, serde_json::Error> = serde_json::from_value(content);
        assert_eq!(change.is_ok(), false);
    }

    #[test]
    fn test_change_check_pattern_match_one() {
        let change = mock::change::create("^feat(\n|.)*$", "Feature", 2);
        let commit = mock::commit::create("feat(super): the message header");
        let check = change.check(&commit);
        assert_eq!(check.is_ok(), true);
    }

    #[test]
    fn test_change_check_pattern_match_two() {
        let change = mock::change::create("^fix(\n|.)*$", "Fix", 3);
        let commit = mock::commit::create("fix(super): the message header");
        let check = change.check(&commit);
        assert_eq!(check.is_ok(), true);
    }

    #[test]
    fn test_change_check_pattern_not_match_one() {
        let change = mock::change::create("^fix(\n|.)*$", "Fix", 3);
        let commit = mock::commit::create("feat(super): the message header");
        let check = change.check(&commit);
        assert_eq!(check.is_ok(), false);
    }

    #[test]
    fn test_change_check_pattern_not_match_two() {
        let change = mock::change::create("^feat(\n|.)*$", "Feature", 2);
        let commit = mock::commit::create("fix(super): the message header");
        let check = change.check(&commit);
        assert_eq!(check.is_ok(), false);
    }
}
