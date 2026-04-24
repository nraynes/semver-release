pub mod github;

use indexmap::IndexMap;

use crate::Alert;

#[derive(PartialEq, Debug)]
pub enum Auth {
    GITHUB,
}

impl Auth {
    /// Performs authentication with repository service based on which state Auth is.
    /// Supplies the environment variables from the running environment.
    pub fn authenticate(&self, env: &IndexMap<String, String>) -> Result<(), Alert> {
        match self {
            Auth::GITHUB => {
                github::set_remote(env)?;
            }
        }
        Ok(())
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "github" => Some(Auth::GITHUB),
            _ => None,
        }
    }
}
