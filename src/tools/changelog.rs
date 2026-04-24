use crate::{Alert, Version};
use std::fs;

pub struct Changelog {
    text: String,
}

impl Changelog {
    pub fn generate(version: &Version) -> Self {
        Changelog {
            text: format!("Version: {}\n\n{}", version.get(), version.changes()),
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Alert> {
        fs::write(path, &self.text)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::tests::mock;

    use super::*;

    #[test]
    fn test_changelog_generate() {
        let version = mock::version::create();
        let changelog = Changelog::generate(&version);
        assert_eq!(
            changelog.text,
            "Version: v1.9.2\n\n## Feature\n\n- feat(scope): a test header\n- feat(scope): a test header two\n\n## Fix\n\n- fix(scope): a test header three\n\n"
        );
    }
}
