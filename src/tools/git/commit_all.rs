use semver_common::{Alert, run_command};

/// Stages all changes in git and commits those changes with a supplied message.
pub fn commit_all(message: &str) -> Result<(), Alert> {
    run_command("git", ["add", "."])?;
    run_command("git", ["commit", "-m", message])?;
    Ok(())
}
