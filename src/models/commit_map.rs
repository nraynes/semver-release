use std::fmt::Display;

use indexmap::IndexMap;

use crate::{Alert, Commit, CommitBucket};

pub struct CommitMap {
    map: IndexMap<String, CommitBucket>,
}

impl Display for CommitMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bucket in self.map.values().into_iter() {
            write!(f, "{}\n", bucket)?;
        }
        Ok(())
    }
}

impl CommitMap {
    pub fn new() -> Self {
        CommitMap {
            map: IndexMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: Commit) -> Result<(), Alert> {
        if !self.map.contains_key(key) {
            self.map.insert(String::from(key), CommitBucket::new(key));
        }
        let bucket: &mut CommitBucket = self
            .map
            .get_mut(key)
            .ok_or("Could not find bucket in commit map.")?;
        bucket.add(value);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::tests::mock;

    use super::*;

    #[test]
    fn test_commitmap_insert() {
        let mut commit_map = CommitMap::new();
        let commit_one = mock::commit::create("feat(scope): a test header");
        let result = commit_map.insert("Feature", commit_one);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_commitmap_fmt() {
        let mut commit_map = CommitMap::new();
        let commit_one = mock::commit::create("feat(scope): a test header");
        let commit_two = mock::commit::create("feat(scope): a test header two");
        let commit_three = mock::commit::create("fix(scope): a test header three");
        commit_map.insert("Feature", commit_one).unwrap();
        commit_map.insert("Feature", commit_two).unwrap();
        commit_map.insert("Fix", commit_three).unwrap();

        let commit_map_str = format!("{}", commit_map);
        assert_eq!(
            commit_map_str,
            "## Feature\n\n- feat(scope): a test header\n- feat(scope): a test header two\n\n## Fix\n\n- fix(scope): a test header three\n\n"
        );
    }
}
