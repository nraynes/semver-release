use regex::Regex;
use serde_json::Value;

use crate::models::{Alert, Commit};

/// A pattern that represents a type of change based on a commit message.
#[derive(Clone, Debug)]
pub struct Change {
    pattern: Regex,
    kind: String,
}

impl PartialEq for Change {
    fn eq(&self, other: &Self) -> bool {
        self.pattern.as_str() == other.pattern.as_str() && self.kind == other.kind
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Change {
    pub fn kind(&self) -> &String {
        &self.kind
    }

    /// Returns a new Change object when supplied with valid parsed json.
    ///
    /// # Valid JSON
    ///
    /// ```markdown
    /// {
    ///     "pattern": "^chore(.|\n)*$",
    ///     "kind": "Maintenance Items"
    /// }
    /// ```
    pub fn from(value: &Value) -> Result<Self, Alert> {
        let change_map = value
            .as_object()
            .ok_or("Change could not be parsed from config. Check syntax.")?;
        let packaged_pattern = change_map
            .get("pattern")
            .ok_or("No pattern found in change declaration in config.")?;
        let packaged_kind = change_map
            .get("kind")
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

    /// Checks a commit to see if it matches this change pattern.
    ///
    /// # Example:
    ///
    /// ```
    /// # use semver_release::{Commit, Change, ChangeList};
    /// # use serde_json::json;
    /// # use chrono::DateTime;
    ///
    /// let change = Change::from(&json!({
    ///     "pattern": "^(.|\n)*BREAKING CHANGE(.|\n)*$",
    ///     "kind": "BREAKING CHANGES"
    /// })).unwrap();
    /// let commit = Commit::new("12345678", "John Doe", DateTime::parse_from_str("Wed Apr 22 19:12:34 2026 -0400", "%a %b %d %H:%M:%S %Y %z").unwrap(), "feat: some commit one\n\nBREAKING CHANGE: this will break the current version.");
    ///
    /// assert_eq!(change.check(&commit), true);
    /// ```
    pub fn check(&self, commit: &Commit) -> bool {
        self.pattern.is_match(commit.msg())
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::{Change, tests::mock};

    #[test]
    fn test_change_new_valid() {
        let change = Change::from(&json!({
            "pattern": "^test.*$",
            "kind": "TEST"
        }));
        assert_eq!(change.is_ok(), true);
    }

    #[test]
    fn test_change_new_invalid() {
        let change = Change::from(&json!(["Invalid", { "JSON_KEY": 334 }]));
        assert_eq!(change.is_ok(), false);
    }

    #[test]
    fn test_change_check_pattern_match_one() {
        let change = mock::change::create("^feat(\n|.)*$", "Feature");
        let commit = mock::commit::create("feat(super): the message header");
        let check = change.check(&commit);
        assert_eq!(check, true);
    }

    #[test]
    fn test_change_check_pattern_match_two() {
        let change = mock::change::create("^fix(\n|.)*$", "Fix");
        let commit = mock::commit::create("fix(super): the message header");
        let check = change.check(&commit);
        assert_eq!(check, true);
    }

    #[test]
    fn test_change_check_pattern_not_match_one() {
        let change = mock::change::create("^fix(\n|.)*$", "Fix");
        let commit = mock::commit::create("feat(super): the message header");
        let check = change.check(&commit);
        assert_eq!(check, false);
    }

    #[test]
    fn test_change_check_pattern_not_match_two() {
        let change = mock::change::create("^feat(\n|.)*$", "Feature");
        let commit = mock::commit::create("fix(super): the message header");
        let check = change.check(&commit);
        assert_eq!(check, false);
    }
}
