use crate::{Alert, models::Commit};
use std::process::Command;

pub fn get_commits(branch: &str) -> Result<Vec<Commit>, Alert> {
    let command_output = Command::new("git").args(&["log", branch]).output()?;
    let stdout = String::from_utf8(command_output.stdout)?;
    let mut commit_list: Vec<Commit> = vec![];
    for c in stdout.split("\ncommit ") {
        match Commit::new_from_commit(c.to_string()) {
            Ok(v) => commit_list.push(v),
            _ => continue,
        }
    }
    Ok(commit_list)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_commits_output() {
        let commits = get_commits("master").unwrap();
        assert_eq!(commits.len() > 1, true)
    }
}
