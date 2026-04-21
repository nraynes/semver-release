use std::process::Command;

use crate::Alert;

pub fn push() -> Result<(), Alert> {
    Command::new("git").args(&["push"]).output()?;
    Ok(())
}
