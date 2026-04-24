use crate::{Alert, Version};
use std::fs;

pub struct Changelog {
    text: String,
}

impl Changelog {
    /// Generates a Changelog from a Version by formatting it as a string.
    ///
    /// # Example:
    ///
    /// NOTE: Don't use .unwrap(), this is just used for the example. Return
    /// a Result wrapping your return value and error, and use ? to handle it
    /// explicitly.
    ///
    /// ```
    /// use semver_release::{Commit, CommitMap, Version, Changelog};
    /// use chrono::DateTime;
    ///
    /// let (major, minor, patch) = Version::parse("v1.9.2").unwrap();
    /// let mut changes = CommitMap::new();
    /// let random_commit = Commit::new(
    ///     "12345678",
    ///     "John Doe",
    ///     DateTime::parse_from_str("Wed Apr 22 19:12:34 2026 -0400", "%a %b %d %H:%M:%S %Y %z")
    ///         .unwrap(),
    ///     "feat(scope): a test subject",
    /// );
    /// changes.insert("Feature", random_commit).unwrap();
    ///
    /// let version = Version::new(major, minor, patch, changes);
    ///
    /// let changelog = Changelog::generate(&version);
    /// ```
    pub fn generate(version: &Version) -> Self {
        Changelog {
            text: format!("Version: {}\n\n{}", version.get(), version.changes()),
        }
    }

    /// Saves the changelog to a file.
    ///
    /// # Example:
    ///
    /// ```
    /// # use semver_release::{Commit, CommitMap, Version, Changelog};
    /// # use chrono::DateTime;
    /// #
    /// # let (major, minor, patch) = Version::parse("v1.9.2").unwrap();
    /// # let mut changes = CommitMap::new();
    /// # let random_commit = Commit::new(
    /// #     "12345678",
    /// #     "John Doe",
    /// #     DateTime::parse_from_str("Wed Apr 22 19:12:34 2026 -0400", "%a %b %d %H:%M:%S %Y %z")
    /// #         .unwrap(),
    /// #     "feat(scope): a test subject",
    /// # );
    /// # changes.insert("Feature", random_commit).unwrap();
    /// #
    /// # let version = Version::new(major, minor, patch, changes);
    /// #
    /// # let changelog = Changelog::generate(&version);
    /// let result = changelog.save("/file/path");
    /// ```
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
