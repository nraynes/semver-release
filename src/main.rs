use semver_common::Alert;
use semver_release::SemVer;
use std::collections::HashMap;
use std::env::{args, vars};

fn main() -> Result<(), Alert> {
    let environment_vars: HashMap<String, String> = vars().collect();
    let cli_args: Vec<String> = args().collect();
    let semver = SemVer::init(cli_args, environment_vars)?;
    semver.release()
}
