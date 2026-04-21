use std::process::Command;

use crate::Alert;

pub fn authenticate(
    github_token: &str,
    github_actor: &str,
    github_repository: &str,
) -> Result<(), Alert> {
    Command::new("git")
        .args(&[
            "remote",
            "set-url",
            "origin",
            &format!(
                "https://${}:${}@github.com/{}.git",
                github_actor, github_token, github_repository
            ),
        ])
        .output()?;
    Ok(())
}
