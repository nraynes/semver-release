use crate::{Alert, models::Commit, run_command};
/// Gets the commits in the history of the supplied branch.
/// Returns as a vector of Commit objects.
pub fn get_commits(branch: &str) -> Result<Vec<Commit>, Alert> {
    let stdout = run_command("git", ["log", branch])?;
    let mut commit_list: Vec<Commit> = vec![];
    for c in stdout.split("\ncommit ") {
        match Commit::new_from_commit(c.to_string()) {
            Ok(v) => {
                println!("{}", v);
                commit_list.push(v);
            }
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
