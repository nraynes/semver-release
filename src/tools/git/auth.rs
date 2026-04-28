pub mod github;

use r_log::Logger;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use semver_common::Alert;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Auth {
    GITHUB,
}

impl Auth {
    /// Performs authentication with repository service based on which state Auth is.
    /// Supplies the environment variables from the running environment.
    pub fn authenticate(
        &self,
        env: &HashMap<String, String>,
        logger: &Logger,
    ) -> Result<(), Alert> {
        match self {
            Auth::GITHUB => {
                github::set_remote(env, logger)?;
            }
        }
        Ok(())
    }
}
