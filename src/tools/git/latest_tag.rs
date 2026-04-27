use crate::run_command;

/// Gets the latest git tag from the git repository.
pub fn latest_tag() -> Option<String> {
    run_command("git", ["describe", "--abbrev=0", "--tags"]).ok()
}
