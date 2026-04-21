use crate::{Alert, Version};
use std::fs;

pub struct Changelog {
    text: String,
}

impl Changelog {
    pub fn generate(version: Version) -> Self {
        Changelog {
            text: format!("{}\n\n{}", version.get(), version.changes()),
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Alert> {
        fs::write(path, &self.text)?;
        Ok(())
    }
}
