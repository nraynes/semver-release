use crate::{Change, Commit};
use serde::{Deserialize, Serialize};

/// Represents a list of change formats to use while analyzing commits.
#[derive(Serialize, Deserialize, Debug)]
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
    pub fn check(&self, commit: &Commit) -> Option<&Change> {
        for change in self.changes.iter() {
            if change.check(commit).is_ok() {
                return Some(&change);
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
        assert_eq!(
            check,
            Some(&mock::change::create("^feat(\n|.)*$", "Feature", 2))
        );
    }

    #[test]
    fn test_changelist_check_pattern_match_two() {
        let changelist = mock::changelist::create();
        let commit = mock::commit::create("fix(some_scope): the commit message");
        let check = changelist.check(&commit);
        assert_eq!(check, Some(&mock::change::create("^fix(\n|.)*$", "Fix", 3)));
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
