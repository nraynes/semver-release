use clap::Parser;
use semver_common::Alert;
use semver_release::{Args, SemVer};
use std::collections::HashMap;
use std::env::vars;

fn main() -> Result<(), Alert> {
    let environment_vars: HashMap<String, String> = vars().collect();
    let cli_args: Args = Args::parse();
    let semver = SemVer::init(cli_args, environment_vars)?;
    semver.release()
}
