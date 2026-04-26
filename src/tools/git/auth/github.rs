use std::process::Command;

use indexmap::IndexMap;

use crate::Alert;

/// Caches credentials in git for authenticating with Github as remote origin.
pub fn set_remote(env: &IndexMap<String, String>) -> Result<(), Alert> {
    Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            &format!(
                "https://${}:${}@github.com/{}.git",
                env.get("GITHUB_ACTOR")
                    .ok_or("GITHUB_ACTOR not in environment variables.")?,
                env.get("GITHUB_TOKEN")
                    .ok_or("GITHUB_TOKEN not in environment variables.")?,
                env.get("GITHUB_REPOSITORY")
                    .ok_or("GITHUB_REPOSITORY not in environment variables.")?
            ),
        ])
        .output()?;
    Ok(())
}
