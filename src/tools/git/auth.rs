pub mod github;

use indexmap::IndexMap;
use r_log::Logger;
use serde::{Deserialize, Serialize};

use crate::Alert;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Auth {
    GITHUB,
}

impl Auth {
    /// Performs authentication with repository service based on which state Auth is.
    /// Supplies the environment variables from the running environment.
    pub fn authenticate(
        &self,
        env: &IndexMap<String, String>,
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
