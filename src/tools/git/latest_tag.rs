use std::process::Command;

use crate::Alert;

pub fn latest_tag() -> Result<String, Alert> {
    let command_output = Command::new("git")
        .args(&["describe", "--abbrev=0", "--tags"])
        .output()?;
    Ok(String::from_utf8(command_output.stdout)?)
}
