use crate::{Alert, Changelog, Config, Version, analyzer, git, parse_args};
use indexmap::IndexMap;
use r_log::Logger;

pub struct SemVer {
    config: Config,
    env: IndexMap<String, String>,
    logger: Logger,
}

impl SemVer {
    /// Get the current release version. Since SemVer-Release uses git tags to track versions,
    /// so this is the default. If a git tag can not be found, for example, if a repository has just
    /// been created and has not had it's first release yet, then versioning starts at 0.0.0.
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

    /// Initialize the SemVer object. This will attempt to parse arguments, read the config,
    /// setup the logger, and anything else that needs to happen before the release stage.
    /// If any of these cannot happen for some reason, an Err variant will be returned.
    pub fn init(args: Vec<String>, vars: IndexMap<String, String>) -> Result<Self, Alert> {
        let config_file_path: String = parse_args(args);
        let env = vars;
        let config = Config::load_from_file(config_file_path)?;
        let logger = Logger::new(config.log_level().clone());
        Ok(SemVer {
            config,
            env,
            logger,
        })
    }

    /// Starts the release stage. Reads the configuration that was loaded in the init
    /// stage and performs the release cycle based on this configuration.
    pub fn release(&self) -> Result<(), Alert> {
        self.logger.info("Starting Release Cycle");
        self.config.git_auth_method().authenticate(&self.env)?;
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
        if *self.config.generate_changelog() {
            let changelog = Changelog::generate(&version);
            changelog.save(self.config.changelog_location())?;
        }
        if *self.config.commit_changes() {
            git::commit_all(&format!("semver_release_version_update {}", version.get()))?;
        }
        if *self.config.push_changes() {
            git::push()?;
        }
        Ok(())
    }
}
