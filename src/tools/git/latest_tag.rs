use crate::run_command;

/// Gets the latest git tag from the git repository.
pub fn latest_tag() -> Option<String> {
    match run_command("git", ["describe", "--abbrev=0", "--tags"]) {
        Ok(v) => Some(String::from(v.trim_matches('\n'))),
        Err(_) => None,
    }
}
