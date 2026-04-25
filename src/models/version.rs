use crate::CommitMap;

/// Data structure that contains a version with a major, minor, and patch number as well as a map
/// of commits that represent changes since the previous version.
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    changes: CommitMap,
}

impl Version {
    /// Gets the version in string form. Formatted, "v<major>.<minor>.<patch>"
    pub fn get(&self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.patch)
    }

    /// Map of commits since previous version.s
    pub fn changes(&self) -> &CommitMap {
        &self.changes
    }

    /// Parse a string into the appropriate version numbers. Returns a tuple containing
    /// the major, minor, and patch version numbers extracted from the string, if available.
    pub fn parse(version: &str) -> Option<(u32, u32, u32)> {
        let separated: Vec<&str> = version[1..].split(".").collect();
        let major_version = separated.get(0)?;
        let minor_version = separated.get(1)?;
        let patch_version = separated.get(2)?;
        Some((
            major_version.parse().ok()?,
            minor_version.parse().ok()?,
            patch_version.parse().ok()?,
        ))
    }

    pub fn new(major: u32, minor: u32, patch: u32, changes: CommitMap) -> Self {
        Version {
            major,
            minor,
            patch,
            changes,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_version_parse_valid() {
        let parse_value = Version::parse("v1.9.2");
        assert_eq!(parse_value, Some((1, 9, 2)));
    }

    #[test]
    fn test_version_parse_invalid() {
        let parse_value = Version::parse("oopasdf91.9.2ff");
        assert_eq!(parse_value, None);
    }

    #[test]
    fn test_version_get() {
        let (major, minor, patch) = Version::parse("v1.9.2").unwrap();
        let changes = CommitMap::new();
        let version = Version::new(major, minor, patch, changes);
        assert_eq!(version.get(), "v1.9.2");
    }
}
