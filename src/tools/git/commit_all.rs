use std::process::Command;

use crate::Alert;

/// Stages all changes in git and commits those changes with a supplied message.
pub fn commit_all(message: &str) -> Result<(), Alert> {
    Command::new("git").args(["add", "."]).output()?;
    Command::new("git")
        .args(["commit", "-m", message])
        .output()?;
    Ok(())
}
