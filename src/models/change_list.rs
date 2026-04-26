use crate::{Change, Commit};

/// Represents a list of change formats to use while analyzing commits.
#[derive(Debug)]
pub struct ChangeList {
    changes: Vec<Change>,
}

impl PartialEq for ChangeList {
    fn eq(&self, other: &Self) -> bool {
        if self.changes.len() != other.changes.len() {
            return false;
        }
        for i in 0..self.changes.len() {
            if self.changes[i] != other.changes[i] {
                return false;
            }
        }
        true
    }
}

impl ChangeList {
    pub fn new(changes: Vec<Change>) -> Self {
        ChangeList { changes }
    }

    /// Checks a commit to see if it matches any of the changes in this list and returns the
    /// kind of change it matched.
    ///
    /// # Example:
    ///
    /// ```
    /// # use semver_release::{Commit, Change, ChangeList};
    /// # use serde_json::json;
    /// # use chrono::DateTime;
    ///
    /// let major_changes = ChangeList::new(vec![
    ///       Change::from(&json!({
    ///           "pattern": "^(.|\n)*BREAKING CHANGE(.|\n)*$",
    ///           "kind": "BREAKING CHANGES"})).unwrap()
    /// ]);
    /// let commit = Commit::new("12345678", "John Doe", DateTime::parse_from_str("Wed Apr 22 19:12:34 2026 -0400", "%a %b %d %H:%M:%S %Y %z").unwrap(), "feat: some commit one\n\nBREAKING CHANGE: this will break the current version.");
    ///
    /// assert_eq!(major_changes.check(&commit), Some(String::from("BREAKING CHANGES")));
    /// ```
    pub fn check(&self, commit: &Commit) -> Option<String> {
        for change in self.changes.iter() {
            if change.check(commit) {
                return Some(String::from(change.kind()));
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::tests::mock;

    #[test]
    fn test_changelist_check_pattern_match_one() {
        let changelist = mock::changelist::create();
        let commit = mock::commit::create("feat(some_scope): the commit message");
        let check = changelist.check(&commit);
        assert_eq!(check, Some(String::from("Feature")));
    }

    #[test]
    fn test_changelist_check_pattern_match_two() {
        let changelist = mock::changelist::create();
        let commit = mock::commit::create("fix(some_scope): the commit message");
        let check = changelist.check(&commit);
        assert_eq!(check, Some(String::from("Fix")));
    }

    #[test]
    fn test_changelist_check_pattern_not_match_one() {
        let changelist = mock::changelist::create();
        let commit = mock::commit::create("chore: this is a chore");
        let check = changelist.check(&commit);
        assert_eq!(check, None);
    }

    #[test]
    fn test_changelist_check_pattern_not_match_two() {
        let changelist = mock::changelist::create();
        let commit = mock::commit::create("docs(readme): a test readme message");
        let check = changelist.check(&commit);
        assert_eq!(check, None);
    }
}
