use crate::{Version, mock::mock_commit_map};

#[allow(dead_code)]
pub fn mock_version() -> Version {
    let (major, minor, patch) = Version::parse("v1.9.2").unwrap();
    let changes = mock_commit_map();
    Version::new(major, minor, patch, changes)
}
