use std::{collections::HashMap, fmt::Display};

use crate::{Alert, Change, Commit, CommitBucket};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommitMap {
    map: HashMap<String, CommitBucket>,
}

impl Display for CommitMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buckets: Vec<&CommitBucket> = self.map.values().collect();
        buckets.sort();
        for bucket in buckets {
            writeln!(f, "{}", bucket)?;
        }
        Ok(())
    }
}

impl Default for CommitMap {
    fn default() -> Self {
        Self::new()
    }
}

impl CommitMap {
    pub fn new() -> Self {
        CommitMap {
            map: HashMap::new(),
        }
    }

    pub fn bucket(&self, key: &str) -> Option<&CommitBucket> {
        self.map.get(key)
    }

    /// Inserts a commit into a bucket with the matching key.
    /// Creates a bucket with that key if none exists already.
    pub fn insert(&mut self, change: &Change, value: Commit) -> Result<(), Alert> {
        if !self.map.contains_key(change.kind()) {
            self.map.insert(
                String::from(change.kind()),
                CommitBucket::new(change.kind(), *change.priority()),
            );
        }
        let bucket: &mut CommitBucket = self
            .map
            .get_mut(change.kind())
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
        let change_feat = mock::change::create("^feat(.|\n)*$", "Feature", 2);
        let change_fix = mock::change::create("^fix(.|\n)*$", "Fix", 3);
        let commit_one = mock::commit::create("feat(scope): a test header");
        let commit_two = mock::commit::create("feat(scope): a test header two");
        let commit_three = mock::commit::create("fix(scope): a test header two");
        let result_one = commit_map.insert(&change_feat, commit_one.clone());
        let result_two = commit_map.insert(&change_feat, commit_two.clone());
        let result_three = commit_map.insert(&change_fix, commit_three.clone());
        assert_eq!(result_one.is_ok(), true);
        assert_eq!(result_two.is_ok(), true);
        assert_eq!(result_three.is_ok(), true);

        let actual_bucket_feat = commit_map.bucket("Feature").unwrap();
        let actual_bucket_fix = commit_map.bucket("Fix").unwrap();

        let mut expected_bucket_feat =
            CommitBucket::new(change_feat.kind(), *change_feat.priority());
        expected_bucket_feat.add(commit_one);
        expected_bucket_feat.add(commit_two);

        let mut expected_bucket_fix = CommitBucket::new(change_fix.kind(), *change_fix.priority());
        expected_bucket_fix.add(commit_three);

        assert_eq!(actual_bucket_feat, &expected_bucket_feat);
        assert_eq!(actual_bucket_fix, &expected_bucket_fix);
    }

    #[test]
    fn test_commitmap_fmt() {
        let mut commit_map = CommitMap::new();
        let change_feat = mock::change::create("^feat(.|\n)*$", "Feature", 2);
        let change_fix = mock::change::create("^fix(.|\n)*$", "Fix", 3);
        let commit_one = mock::commit::create("feat(scope): a test header");
        let commit_two = mock::commit::create("feat(scope): a test header two");
        let commit_three = mock::commit::create("fix(scope): a test header three");
        commit_map.insert(&change_feat, commit_one.clone()).unwrap();
        commit_map.insert(&change_feat, commit_two.clone()).unwrap();
        commit_map
            .insert(&change_fix, commit_three.clone())
            .unwrap();

        let commit_map_str = format!("{}", commit_map);
        assert_eq!(
            commit_map_str,
            "## Feature\n\n- feat(scope): a test header\n- feat(scope): a test header two\n\n## Fix\n\n- fix(scope): a test header three\n\n"
        );
    }
}
