use r_log::Logger;
use semver_common::run_command;

/// Gets the latest git tag from the git repository.
pub fn latest_tag(logger: &Logger) -> Option<String> {
    match run_command("git", ["describe", "--abbrev=0", "--tags"], Some(logger)) {
        Ok(v) => Some(String::from(v.trim_matches('\n'))),
        Err(_) => None,
    }
}
