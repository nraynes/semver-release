use crate::{Alert, Config, Version, analyzer, git, log::Logger, parse_args};

pub struct SemVer {
    config: Config,
    logger: Logger,
}

impl SemVer {
    fn current_version(&self) -> (u32, u32, u32) {
        let latest_tag = match git::latest_tag() {
            Some(v) => v,
            None => return (0, 0, 0),
        };
        match Version::parse(&latest_tag) {
            Some(v) => v,
            None => (0, 0, 0),
        }
    }

    pub fn init(args: Vec<String>) -> Result<Self, Alert> {
        let config_file_path: String = parse_args(args);
        let config = Config::load_from_file(config_file_path)?;
        let logger = Logger::new(config.log_level().clone());
        Ok(SemVer { config, logger })
    }

    pub fn release(&self) -> Result<(), Alert> {
        let commits = git::get_commits(self.config.release_branch())?;
        let (current_major, current_minor, current_patch) = self.current_version();
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
