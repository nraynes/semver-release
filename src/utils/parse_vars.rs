use indexmap::IndexMap;

use crate::{Alert, Env};

pub fn parse_vars(vars: IndexMap<String, String>) -> Result<Env, Alert> {
    let github_token = vars
        .get("GITHUB_TOKEN")
        .ok_or("GITHUB_TOKEN not in environment variables.")?;
    let github_actor = vars
        .get("GITHUB_ACTOR")
        .ok_or("GITHUB_ACTOR not in environment variables.")?;
    let github_repository = vars
        .get("GITHUB_REPOSITORY")
        .ok_or("GITHUB_REPOSITORY not in environment variables.")?;
    Ok(Env::new(github_token, github_actor, github_repository))
}
