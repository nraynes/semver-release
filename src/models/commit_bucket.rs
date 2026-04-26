use std::fmt::Display;

use crate::Commit;

/// A container for a kind of change pattern that represents a commit that matches said pattern,
/// as well as a vector of Commit objects that match the pattern for that kind of change.
#[derive(Debug)]
pub struct CommitBucket {
    kind: String,
    commits: Vec<Commit>,
}

impl PartialEq for CommitBucket {
    fn eq(&self, other: &Self) -> bool {
        if self.commits.len() != other.commits.len() || self.kind != other.kind {
            return false;
        }
        for i in 0..self.commits.len() {
            if self.commits[i] != other.commits[i] {
                return false;
            }
        }
        true
    }
}

impl Display for CommitBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "## {}\n", self.kind)?;
        for commit in self.commits.iter() {
            write!(f, "- {}", commit)?;
        }
        Ok(())
    }
}

impl CommitBucket {
    pub fn kind(&self) -> &String {
        &self.kind
    }

    pub fn commits(&self) -> &Vec<Commit> {
        &self.commits
    }

    pub fn new(kind: &str) -> Self {
        CommitBucket {
            kind: String::from(kind),
            commits: vec![],
        }
    }

    pub fn add(&mut self, commit: Commit) {
        self.commits.push(commit)
    }
}

#[cfg(test)]
mod test {
    use crate::tests::mock;

    use super::*;

    #[test]
    fn test_commitbucket_add() {
        let mut bucket = CommitBucket::new("Feature");
        let commit_one = mock::commit::create("feat: this is a test one");
        let commit_two = mock::commit::create("feat: this is a test two");
        bucket.add(commit_one);
        bucket.add(commit_two);

        assert_eq!(bucket.kind(), "Feature");
        assert_eq!(bucket.commits()[0].message(), "feat: this is a test one");
        assert_eq!(bucket.commits()[1].message(), "feat: this is a test two");
    }

    #[test]
    fn test_commitbucket_fmt() {
        let mut bucket = CommitBucket::new("Feature");
        let commit_one = mock::commit::create("feat: this is a test one");
        let commit_two = mock::commit::create("feat: this is a test two");
        bucket.add(commit_one);
        bucket.add(commit_two);

        let displayed = format!("{}", bucket);
        assert_eq!(
            &displayed,
            "## Feature\n\n- feat: this is a test one\n- feat: this is a test two\n"
        );
    }
}
