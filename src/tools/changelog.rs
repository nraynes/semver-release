use semver_common::{Alert, Version};
use std::fs;

pub struct Changelog {
    text: String,
}

impl Changelog {
    /// Generates a Changelog from a Version by formatting it as a string.
    pub fn generate(version: &Version) -> Self {
        Changelog {
            text: format!("# Version: {}\n\n{}", version.get(), version.changes()),
        }
    }

    /// Saves the changelog to a file.
    pub fn save(&self, path: &str) -> Result<(), Alert> {
        fs::write(path, &self.text)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use semver_common::mock;

    use super::*;

    #[test]
    fn test_changelog_generate() {
        let version = mock::version::create();
        let changelog = Changelog::generate(&version);
        assert_eq!(
            changelog.text,
            "# Version: v1.9.2\n\n## BREAKING CHANGES\n\n- fix(scope): a test header three\n\nBREAKING CHANGE: This is a breaking change.\n\n## Feature\n\n- feat(scope): a test header\n- feat(scope): a test header two\n\n## Fix\n\n- fix(scope): a test header three\n\n"
        );
    }
}
