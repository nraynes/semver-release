use crate::run_command;
use semver_common::Alert;

/// Creates a new git tag with a supplied name and message.
pub fn tag(name: &str, message: &str) -> Result<(), Alert> {
    run_command(
        "git",
        ["tag", "-a", name, "-m", &format!("{} {}", message, name)],
    )?;
    run_command("git", ["push", "--tags"])?;
    Ok(())
}
