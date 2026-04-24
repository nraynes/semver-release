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

    pub fn bucket(&self, key: &str) -> Option<&CommitBucket> {
        self.map.get(key)
    }

    /// Inserts a commit into a bucket with the matching key.
    /// Creates a bucket with that key if none exists already.
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
    fn test_commitmap_insert_get() {
        let mut commit_map = CommitMap::new();
        let commit_one = mock::commit::create("feat(scope): a test header");
        let commit_two = mock::commit::create("feat(scope): a test header two");
        let commit_three = mock::commit::create("fix(scope): a test header two");
        let result_one = commit_map.insert("Feature", commit_one.clone());
        let result_two = commit_map.insert("Feature", commit_two.clone());
        let result_three = commit_map.insert("Fix", commit_three.clone());
        assert_eq!(result_one.is_ok(), true);
        assert_eq!(result_two.is_ok(), true);
        assert_eq!(result_three.is_ok(), true);

        let actual_bucket_feat = commit_map.bucket("Feature").unwrap();
        let actual_bucket_fix = commit_map.bucket("Fix").unwrap();

        let mut expected_bucket_feat = CommitBucket::new("Feature");
        expected_bucket_feat.add(commit_one);
        expected_bucket_feat.add(commit_two);

        let mut expected_bucket_fix = CommitBucket::new("Fix");
        expected_bucket_fix.add(commit_three);

        assert_eq!(actual_bucket_feat, &expected_bucket_feat);
        assert_eq!(actual_bucket_fix, &expected_bucket_fix);
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
