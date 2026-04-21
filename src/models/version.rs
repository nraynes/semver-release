use crate::CommitMap;

pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    changes: CommitMap,
}

impl Version {
    pub fn get(&self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn changes(&self) -> &CommitMap {
        &self.changes
    }

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
