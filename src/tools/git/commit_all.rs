use crate::run_command;
use semver_common::Alert;

/// Stages all changes in git and commits those changes with a supplied message.
pub fn commit_all(message: &str) -> Result<(), Alert> {
    run_command("git", ["add", "."])?;
    run_command("git", ["commit", "-m", message])?;
    Ok(())
}
