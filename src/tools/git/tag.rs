use r_log::Logger;
use semver_common::{Alert, run_command};

/// Creates a new git tag with a supplied name and message.
pub fn tag(name: &str, message: &str, logger: &Logger) -> Result<(), Alert> {
    run_command(
        "git",
        ["tag", "-a", name, "-m", &format!("{} {}", message, name)],
        Some(logger),
    )?;
    run_command("git", ["push", "--tags"], Some(logger))?;
    Ok(())
}
