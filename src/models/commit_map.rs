use indexmap::IndexMap;

use crate::{Alert, Commit, CommitBucket};

pub struct CommitMap {
    map: IndexMap<String, CommitBucket>,
}

impl CommitMap {
    pub fn new() -> Self {
        CommitMap {
            map: IndexMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: Commit) -> Result<(), Alert> {
        let bucket: &mut CommitBucket = if self.map.contains_key(key) {
            self.map
                .get_mut(key)
                .ok_or("Could not find bucket in commit map.")?
        } else {
            &mut self
                .map
                .insert(String::from(key), CommitBucket::new(String::from(key)))
                .ok_or("Failed to add bucket to commit map.")?
        };
        bucket.add(value);
        Ok(())
    }
}
