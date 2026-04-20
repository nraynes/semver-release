use std::process::Command;

use crate::Alert;

pub fn tag(name: &str, message: &str) -> Result<(), Alert> {
    Command::new("git")
        .args(&["tag", "-a", name, "-m", message])
        .output()?;
    Ok(())
}
