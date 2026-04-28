use semver_common::{Alert, run_command};

/// Fetch from the remote repository to ensure commit history is updated.
pub fn fetch() -> Result<(), Alert> {
    run_command("git", ["fetch", "--tags", "--prune"])?;
    Ok(())
}
