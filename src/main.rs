use semver_release::Alert;
use semver_release::SemVer;
use std::env::{args, var};

fn main() -> Result<(), Alert> {
    let semver = SemVer::init(args().collect())?;
    semver.release()
}
