use std::process::Command;

use crate::Alert;

/// Pushes any commited changes in git to the authenticated remote repository.
pub fn push() -> Result<(), Alert> {
    Command::new("git").args(&["push"]).output()?;
    Ok(())
}
