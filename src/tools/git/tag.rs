use std::process::Command;

use crate::Alert;

/// Creates a new git tag with a supplied name and message.
pub fn tag(name: &str, message: &str) -> Result<(), Alert> {
    Command::new("git")
        .args(&["tag", "-a", name, "-m", message])
        .output()?;
    Ok(())
}
