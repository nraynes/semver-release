use crate::{Alert, Config, Version, analyzer, git, parse_args};

pub struct SemVer {
    config: Config,
}

impl SemVer {
    pub fn init(args: Vec<String>) -> Result<SemVer, Alert> {
        let config_file_path: String = parse_args(args);
        let config = Config::load_from_file(config_file_path)?;
        Ok(SemVer { config })
    }

    pub fn release(&self) -> Result<(), Alert> {
        let commits = git::get_commits(self.config.release_branch())?;
        let latest_tag = git::latest_tag()?;
        let (current_major, current_minor, current_patch) = Version::parse(&latest_tag)?;
        let version = analyzer::analyze_commits(
            &commits,
            self.config.major_changes(),
            self.config.minor_changes(),
            self.config.patch_changes(),
            self.config.other_changes(),
            current_major,
            current_minor,
            current_patch,
        )?;
        git::tag(&version.get(), "SemVer-Release")?;
        Ok(())
    }
}
