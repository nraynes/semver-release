use std::process::Command;

use crate::Alert;

pub fn save() -> Result<(), Alert> {
    Command::new("git")
        .args(&["commit", "-am", "semver_release_version_update"])
        .output()?;
    Ok(())
}
