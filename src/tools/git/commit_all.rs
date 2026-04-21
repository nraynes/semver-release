use std::process::Command;

use crate::Alert;

pub fn commit_all(message: &str) -> Result<(), Alert> {
    Command::new("git").args(&["add", "."]).output()?;
    Command::new("git")
        .args(&["commit", "-m", message])
        .output()?;
    Ok(())
}
