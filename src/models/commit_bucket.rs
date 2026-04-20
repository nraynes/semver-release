use crate::Commit;

pub struct CommitBucket {
    kind: String,
    commits: Vec<Commit>,
}

impl CommitBucket {
    pub fn kind(&self) -> &String {
        &self.kind
    }

    pub fn commits(&self) -> &Vec<Commit> {
        &self.commits
    }

    pub fn new(kind: String) -> CommitBucket {
        CommitBucket {
            kind,
            commits: vec![],
        }
    }

    pub fn add(&mut self, commit: Commit) {
        self.commits.push(commit)
    }
}
