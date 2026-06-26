use clap::Parser;
use semver_common::Alert;
use semver_release::{Args, SemVer};

fn main() -> Result<(), Alert> {
    let cli_args: Args = Args::parse();
    let semver = SemVer::init(cli_args)?;
    semver.release()
}
