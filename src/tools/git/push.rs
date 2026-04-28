use semver_common::{Alert, run_command};

/// Pushes any commited changes in git to the authenticated remote repository.
pub fn push() -> Result<(), Alert> {
    run_command("git", ["push"])?;
    Ok(())
}
