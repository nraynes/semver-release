pub mod github;

use std::str::FromStr;

use indexmap::IndexMap;
use r_log::Logger;

use crate::Alert;

#[derive(PartialEq, Debug)]
pub enum Auth {
    GITHUB,
}

impl FromStr for Auth {
    type Err = Alert;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "github" => Ok(Auth::GITHUB),
            _ => Err(Alert::from("Cannot instantiate Auth from invalid string.")),
        }
    }
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
