#[cfg(test)]
pub mod commit_map {
    use crate::{CommitMap, tests::mock};

    pub fn create() -> CommitMap {
        let mut commit_map = CommitMap::new();
        let commit_one = mock::commit::create("feat(scope): a test header");
        let commit_two = mock::commit::create("feat(scope): a test header two");
        let commit_three = mock::commit::create("fix(scope): a test header three");
        commit_map.insert("Feature", commit_one).unwrap();
        commit_map.insert("Feature", commit_two).unwrap();
        commit_map.insert("Fix", commit_three).unwrap();
        commit_map
    }
}
