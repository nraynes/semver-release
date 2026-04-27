use crate::{Alert, run_command};

/// Creates a new git tag with a supplied name and message.
pub fn tag(name: &str, message: &str) -> Result<(), Alert> {
    run_command("git", ["tag", "-a", name, "-m", message])?;
    Ok(())
}
