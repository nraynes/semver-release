#[cfg(test)]
pub mod version {
    use crate::{Version, tests::mock};

    pub fn create() -> Version {
        let (major, minor, patch) = Version::parse("v1.9.2").unwrap();
        let changes = mock::commit_map::create();
        Version::new(major, minor, patch, changes)
    }
}
