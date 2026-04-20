use crate::{Change, Commit};

pub struct ChangeList {
    changes: Vec<Change>,
}

impl ChangeList {
    pub fn new(changes: Vec<Change>) -> ChangeList {
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
