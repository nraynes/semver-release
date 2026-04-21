pub mod github;

use indexmap::IndexMap;

use crate::Alert;

pub enum Auth {
    GITHUB,
}

impl Auth {
    pub fn authenticate(&self, env: &IndexMap<String, String>) -> Result<(), Alert> {
        match self {
            Auth::GITHUB => {
                github::authenticate(env)?;
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
