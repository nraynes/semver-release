use crate::{Changelog, Config, analyzer, git, parse_args, plugins};
use r_log::Logger;
use semver_common::{Alert, Version};
use std::collections::HashMap;

pub struct SemVer {
    config: Config,
    env: HashMap<String, String>,
    logger: Logger,
}

impl SemVer {
    /// Initialize the SemVer object. This will attempt to parse arguments, read the config,
    /// setup the logger, and anything else that needs to happen before the release stage.
    /// If any of these cannot happen for some reason, an Err variant will be returned.
    pub fn init(args: Vec<String>, env: HashMap<String, String>) -> Result<Self, Alert> {
        let config_file_path: String = parse_args(args);
        let config = Config::load_from_file(config_file_path)?;
        let logger = Logger::new(config.log_level().clone());
        logger.info("Initialization complete");
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

        // Authenticate git with the chosen method.
        self.config
            .git_auth_method()
            .authenticate(&self.env, &self.logger)?;

        // Fetch commits from remote origin.
        self.logger.info("Fetching commit history");
        git::fetch(&self.logger)?;

        // Get the current version and whether a tag exists already.
        self.logger.info("Getting current version");
        let latest_tag = git::latest_tag(&self.logger);
        let current_version = match &latest_tag {
            Some(v) => Version::parse(v).unwrap_or_default(),
            None => (0, 0, 0),
        };

        // Get the commits since the last version, or all of them if no tag was present.
        self.logger.info("Acquiring commits");
        let commits = git::get_commits(&latest_tag, &self.logger)?;

        // Analyze the list of commits.
        self.logger.info("Analyzing commits");
        let version = analyzer::analyze_commits(
            &commits,
            self.config.major_changes(),
            self.config.minor_changes(),
            self.config.patch_changes(),
            self.config.other_changes(),
            current_version,
        )?;

        // Tag with the new version if new version exists.
        if match latest_tag {
            Some(v) => version.get() != v,
            None => true,
        } {
            self.logger.info("Tagging version");
            git::tag(&version.get(), "tag version update", &self.logger)?;
        }

        // Generate the changelog.
        if *self.config.generate_changelog() {
            self.logger.info("Generating changelog");
            let changelog = Changelog::generate(&version);
            changelog.save(self.config.changelog_location())?;
        }

        // Run plugins.
        plugins::run(&self.config, &self.logger, &version)?;

        // Commit the changes.
        if *self.config.commit_changes() {
            self.logger.info("Committing changes");
            git::commit_all(
                &format!("semver_release_version_update {}", version.get()),
                &self.logger,
            )?;
        }

        // Push the changes.
        if *self.config.push_changes() {
            self.logger.info("Pushing changes");
            git::push(&self.logger)?;
        }

        self.logger.info("Done");
        Ok(())
    }
}
