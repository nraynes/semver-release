use crate::{Alert, CommitMap};

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

    pub fn parse(version: &str) -> Result<(u32, u32, u32), Alert> {
        let separated: Vec<&str> = version[1..].split(".").collect();
        let major_version = separated
            .get(0)
            .ok_or("No current major version detected.")?;
        let minor_version = separated
            .get(1)
            .ok_or("No current minor version detected.")?;
        let patch_version = separated
            .get(2)
            .ok_or("No current patch version detected.")?;
        Ok((
            major_version.parse()?,
            minor_version.parse()?,
            patch_version.parse()?,
        ))
    }

    pub fn new(major: u32, minor: u32, patch: u32, changes: CommitMap) -> Version {
        Version {
            major,
            minor,
            patch,
            changes,
        }
    }
}
