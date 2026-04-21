use std::fmt::Display;

use crate::Commit;

pub struct CommitBucket {
    kind: String,
    commits: Vec<Commit>,
}

impl Display for CommitBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "## {}\n\n", self.kind)?;
        for commit in self.commits.iter() {
            write!(f, "    {}\n", commit)?;
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

    pub fn new(kind: String) -> Self {
        CommitBucket {
            kind,
            commits: vec![],
        }
    }

    pub fn add(&mut self, commit: Commit) {
        self.commits.push(commit)
    }
}
