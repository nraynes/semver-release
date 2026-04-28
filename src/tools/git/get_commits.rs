use r_log::Logger;
use semver_common::{Alert, Commit, run_command};

/// Gets the commits in the history of the supplied branch.
/// Returns as a vector of Commit objects.
pub fn get_commits(latest_tag: &Option<String>, logger: &Logger) -> Result<Vec<Commit>, Alert> {
    let stdout: String = match latest_tag {
        None => run_command("git", ["log"], Some(logger))?,
        Some(v) => {
            let tag_arg = format!("{}..HEAD", v);
            run_command("git", ["log", &tag_arg], Some(logger))?
        }
    };
    let mut commit_list: Vec<Commit> = vec![];
    for c in stdout.split("\ncommit ") {
        match Commit::new_from_commit(c.to_string()) {
            Ok(v) => commit_list.push(v),
            Err(_) => continue,
        }
    }
    Ok(commit_list)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::git::latest_tag;
    use r_log::LogLevel;

    #[test]
    fn test_get_commits_output_all() {
        let logger = Logger::new(LogLevel::DEBUG);
        let commits = get_commits(&None, &logger).unwrap();
        assert_eq!(commits.len() > 1, true);
    }

    #[test]
    fn test_get_commits_output_since_last_tag() {
        let logger = Logger::new(LogLevel::DEBUG);
        let commits_all = get_commits(&None, &logger).unwrap();
        let latest_tag = latest_tag(&logger).unwrap();
        let commits_latest = get_commits(&Some(latest_tag), &logger).unwrap();
        assert_eq!(commits_latest.len() < commits_all.len(), true);
    }
}
