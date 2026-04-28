use crate::run_command;
use semver_common::Alert;

/// Pushes any commited changes in git to the authenticated remote repository.
pub fn push() -> Result<(), Alert> {
    run_command("git", ["push"])?;
    Ok(())
}
