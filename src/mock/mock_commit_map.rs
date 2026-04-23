use crate::{CommitMap, mock::mock_commit};

#[allow(dead_code)]
pub fn mock_commit_map() -> CommitMap {
    let mut commit_map = CommitMap::new();
    let commit_one = mock_commit("feat(scope): a test header");
    let commit_two = mock_commit("feat(scope): a test header two");
    let commit_three = mock_commit("fix(scope): a test header three");
    commit_map.insert("Feature", commit_one).unwrap();
    commit_map.insert("Feature", commit_two).unwrap();
    commit_map.insert("Fix", commit_three).unwrap();
    commit_map
}
