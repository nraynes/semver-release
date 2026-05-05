use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Path to the configuration file.
    #[arg(default_value = "config.semver.json")]
    pub config_path: PathBuf,
}
