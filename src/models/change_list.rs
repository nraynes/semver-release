use crate::{Change, Commit};

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

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl ChangeList {
    pub fn new(changes: Vec<Change>) -> Self {
        ChangeList { changes }
    }

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
    use crate::mock::mock_changelist;
    use crate::mock::mock_commit;

    #[test]
    fn test_changelist_check_pattern_match_one() {
        let changelist = mock_changelist();
        let commit = mock_commit("feat(some_scope): the commit message");
        let check = changelist.check(&commit);
        assert_eq!(check, Some(String::from("Feature")));
    }

    #[test]
    fn test_changelist_check_pattern_match_two() {
        let changelist = mock_changelist();
        let commit = mock_commit("fix(some_scope): the commit message");
        let check = changelist.check(&commit);
        assert_eq!(check, Some(String::from("Fix")));
    }

    #[test]
    fn test_changelist_check_pattern_not_match_one() {
        let changelist = mock_changelist();
        let commit = mock_commit("chore: this is a chore");
        let check = changelist.check(&commit);
        assert_eq!(check, None);
    }

    #[test]
    fn test_changelist_check_pattern_not_match_two() {
        let changelist = mock_changelist();
        let commit = mock_commit("docs(readme): a test readme message");
        let check = changelist.check(&commit);
        assert_eq!(check, None);
    }
}
