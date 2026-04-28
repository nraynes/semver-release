use r_log::Logger;
use semver_common::{Alert, run_command};

/// Fetch from the remote repository to ensure commit history is updated.
pub fn fetch(logger: &Logger) -> Result<(), Alert> {
    run_command("git", ["fetch", "--tags", "--prune"], Some(logger))?;
    Ok(())
}
