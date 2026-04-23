use crate::{Alert, ChangeList, Commit, CommitMap, Version};

pub fn analyze_commits(
    commits: &Vec<Commit>,
    major_changes: &ChangeList,
    minor_changes: &ChangeList,
    patch_changes: &ChangeList,
    other_changes: &ChangeList,
    current_major: u32,
    current_minor: u32,
    current_patch: u32,
) -> Result<Version, Alert> {
    let mut major = current_major;
    let mut minor = current_minor;
    let mut patch = current_patch;
    let mut changes: CommitMap = CommitMap::new();
    for commit in commits.iter() {
        match major_changes.check(commit) {
            Some(kind) => {
                major += 1;
                changes.insert(&kind, commit.clone())?;
            }
            None => {}
        };
        match minor_changes.check(commit) {
            Some(kind) => {
                minor += 1;
                changes.insert(&kind, commit.clone())?;
            }
            None => {}
        };
        match patch_changes.check(commit) {
            Some(kind) => {
                patch += 1;
                changes.insert(&kind, commit.clone())?;
            }
            None => {}
        };
        match other_changes.check(commit) {
            Some(kind) => changes.insert(&kind, commit.clone())?,
            None => {}
        };
    }
    Ok(Version::new(major, minor, patch, changes))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_analyze_commits_valid() {}
}
