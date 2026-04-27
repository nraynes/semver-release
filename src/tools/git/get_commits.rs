use crate::{Alert, models::Commit, run_command};
/// Gets the commits in the history of the supplied branch.
/// Returns as a vector of Commit objects.
pub fn get_commits(branch: &str, latest_tag: Option<String>) -> Result<Vec<Commit>, Alert> {
    let stdout = match latest_tag {
        None => run_command("git", ["log", branch])?,
        Some(v) => {
            let tag_arg = format!("{}..HEAD", v);
            run_command("git", ["log", branch, &tag_arg])?
        }
    };
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
    use crate::git::latest_tag;

    #[test]
    fn test_get_commits_output_all() {
        let commits = get_commits("master", None).unwrap();
        assert_eq!(commits.len() > 1, true);
    }

    #[test]
    fn test_get_commits_output_since_last_tag() {
        let commits_all = get_commits("master", None).unwrap();
        let latest_tag = latest_tag().unwrap();
        let commits_latest = get_commits("master", Some(latest_tag)).unwrap();
        assert_eq!(commits_latest.len() < commits_all.len(), true);
    }
}
