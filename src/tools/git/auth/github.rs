use r_log::Logger;
use std::collections::HashMap;

use crate::{Alert, run_command};

/// Caches credentials in git for authenticating with Github as remote origin.
pub fn set_remote(env: &HashMap<String, String>, logger: &Logger) -> Result<(), Alert> {
    logger.info("Authenticating with Github");
    let actor = env
        .get("GITHUB_ACTOR")
        .ok_or("GITHUB_ACTOR not in environment variables.")?;
    let token = env
        .get("GITHUB_TOKEN")
        .ok_or("GITHUB_TOKEN not in environment variables.")?;
    let repo = env
        .get("GITHUB_REPOSITORY")
        .ok_or("GITHUB_REPOSITORY not in environment variables.")?;
    run_command(
        "git",
        [
            "remote",
            "set-url",
            "origin",
            &format!("https://${}:${}@github.com/{}.git", actor, token, repo),
        ],
    )?;
    run_command("git", ["config", "--global", "user.name", actor])?;
    Ok(())
}
