use std::process::Command;

/// Gets the latest git tag from the git repository.
pub fn latest_tag() -> Option<String> {
    let command_output = Command::new("git")
        .args(["describe", "--abbrev=0", "--tags"])
        .output()
        .ok()?;
    String::from_utf8(command_output.stdout).ok()
}
