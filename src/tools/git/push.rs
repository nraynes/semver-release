use r_log::Logger;
use semver_common::{Alert, run_command};

/// Pushes any commited changes in git to the authenticated remote repository.
pub fn push(logger: &Logger) -> Result<(), Alert> {
    run_command("git", ["push"], Some(logger))?;
    Ok(())
}
