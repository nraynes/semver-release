use crate::{
    ChangeList, git,
    models::{Alert, Change},
};
use r_log::LogLevel;
use serde_json::{self, Value, map::Map};
use std::fs;

/// Configuration object to hold all the values from the global configuration file.
pub struct Config {
    release_branch: String,
    major_changes: ChangeList,
    minor_changes: ChangeList,
    patch_changes: ChangeList,
    other_changes: ChangeList,
    generate_changelog: bool,
    log_level: LogLevel,
    changelog_location: String,
    git_auth_method: git::auth::Auth,
}

impl Config {
    /// Release branch to use for semantic versioning.
    pub fn release_branch(&self) -> &str {
        &self.release_branch
    }

    /// List of objects containing change patterns along with what kind of change
    /// a commit that matches the pattern would be for all changes that would cause
    /// the major version number to be bumped by 1.
    pub fn major_changes(&self) -> &ChangeList {
        &self.major_changes
    }

    /// List of objects containing change patterns along with what kind of change
    /// a commit that matches the pattern would be for all changes that would cause
    /// the minor version number to be bumped by 1.
    pub fn minor_changes(&self) -> &ChangeList {
        &self.minor_changes
    }

    /// List of objects containing change patterns along with what kind of change
    /// a commit that matches the pattern would be for all changes that would cause
    /// the patch version number to be bumped by 1.
    pub fn patch_changes(&self) -> &ChangeList {
        &self.patch_changes
    }

    /// List of objects containing change patterns along with what kind of change
    /// a commit that matches the pattern would be for all changes that do not bump
    /// bump the version but should be included in the changelog.
    pub fn other_changes(&self) -> &ChangeList {
        &self.other_changes
    }

    /// Whether to generate a changelog or not.
    pub fn generate_changelog(&self) -> &bool {
        &self.generate_changelog
    }

    /// What logging level should be used during runtime.
    pub fn log_level(&self) -> &LogLevel {
        &self.log_level
    }

    /// An alternate path to where the generated changelog file should be stored.
    /// Default is just the root folder with the file being named CHANGELOG.md.
    pub fn changelog_location(&self) -> &str {
        &self.changelog_location
    }

    /// Which authentication type to use for git when making push/commits.
    /// A valid option would be to use "github" if you are deploying this in a Github Action.
    pub fn git_auth_method(&self) -> &git::auth::Auth {
        &self.git_auth_method
    }

    /// Creates a new Config object when supplied a valid deserialized JSON Value.
    pub fn load(config_contents: Value) -> Result<Self, Alert> {
        let conf = Config::parse(config_contents)?;
        Ok(Config {
            release_branch: conf.0,
            major_changes: conf.1,
            minor_changes: conf.2,
            patch_changes: conf.3,
            other_changes: conf.4,
            generate_changelog: conf.5,
            log_level: conf.6,
            changelog_location: conf.7,
            git_auth_method: conf.8,
        })
    }

    /// Creates a new Config from a file at the supplied path, provided the file contains
    /// valid syntax for JSON and the config.
    pub fn load_from_file(file_path: String) -> Result<Self, Alert> {
        let config_file = fs::read_to_string(file_path)?;
        let config_contents = serde_json::from_str(&config_file)?;
        let config = Config::load(config_contents)?;
        Ok(config)
    }

    /// Parses the JSON contents from a serde Value to enforce config file syntax.
    fn parse(
        config_content: Value,
    ) -> Result<
        (
            String,
            ChangeList,
            ChangeList,
            ChangeList,
            ChangeList,
            bool,
            LogLevel,
            String,
            git::auth::Auth,
        ),
        Alert,
    > {
        // Parse the config.
        let conf = config_content
            .as_object()
            .ok_or("Could not parse config.")?;

        // Parse the release branch from global values. Default is the master branch.
        let master_branch = Value::from("master");
        let release_branch = conf
            .get("release_branch")
            .unwrap_or(&master_branch)
            .as_str()
            .ok_or("Could not get release_branch.")?;

        // Parse the change lists.
        let major_changes = Config::parse_change_vec(conf, "major_changes")?;
        let minor_changes = Config::parse_change_vec(conf, "minor_changes")?;
        let patch_changes = Config::parse_change_vec(conf, "patch_changes")?;
        let other_changes = Config::parse_change_vec(conf, "other_changes")?;

        // Parse changelog toggle.
        let generate_changelog = conf
            .get("generate_changelog")
            .unwrap_or(&Value::from(true))
            .as_bool()
            .ok_or("Could not get generate_changelog.")?;

        // Parse log level.
        let log_level = LogLevel::from_str(
            conf.get("log_level")
                .unwrap_or(&Value::from("info"))
                .as_str()
                .ok_or("Could not get log_level.")?,
        )
        .ok_or("Not a valid value for LogLevel.")?;

        // Parse changelog location. Default is CHANGELOG.md.
        let default_changelog_location = Value::from("CHANGELOG.md");
        let changelog_location = conf
            .get("changelog_location")
            .unwrap_or(&default_changelog_location)
            .as_str()
            .ok_or("Could not get changelog_location.")?;

        // Parse the git authentication method.
        let git_auth_method_value = conf
            .get("git_auth_method")
            .ok_or("Could not get git_auth_method.")?;
        let git_auth_method_str = git_auth_method_value
            .as_str()
            .ok_or("git_auth_method invalid value.")?;
        let git_auth_method = git::auth::Auth::from_str(git_auth_method_str)
            .ok_or("Invalid value for git_auth_method.")?;

        // Return with all the values to instantiate a Config.
        Ok((
            String::from(release_branch),
            major_changes,
            minor_changes,
            patch_changes,
            other_changes,
            generate_changelog,
            log_level,
            String::from(changelog_location),
            git_auth_method,
        ))
    }

    /// Parses a list of changes from the config.
    fn parse_change_vec(conf: &Map<String, Value>, key: &str) -> Result<ChangeList, Alert> {
        let unpacked_value = conf
            .get(key)
            .ok_or("Could not parse value from change vector in config.")?;
        let unpacked_mapping = unpacked_value
            .as_array()
            .ok_or("Could not parse vector from change vector in config")?;
        let mut changes = vec![];
        for change_value in unpacked_mapping.iter() {
            let change = Change::from(change_value)?;
            changes.push(change);
        }
        Ok(ChangeList::new(changes))
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::{git::auth::Auth, tests::mock};

    use super::*;

    #[test]
    fn test_config_parse_valid() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": false,
            "log_level": "warning",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "github"
        });
        let (
            release_branch,
            major_changes,
            minor_changes,
            patch_changes,
            other_changes,
            generate_changelog,
            log_level,
            changelog_location,
            git_auth_method,
        ) = Config::parse(json_content).unwrap();
        assert_eq!(release_branch, "feature_branch");
        assert_eq!(
            major_changes,
            ChangeList::new(vec![mock::change::create(
                "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                "BREAKING CHANGES"
            )])
        );
        assert_eq!(
            minor_changes,
            ChangeList::new(vec![mock::change::create("^feat(.|\n)*$", "Features")])
        );
        assert_eq!(
            patch_changes,
            ChangeList::new(vec![mock::change::create("^fix(.|\n)*$", "Patches")])
        );
        assert_eq!(
            other_changes,
            ChangeList::new(vec![
                mock::change::create("^chore(.|\n)*$", "Maintenance Items"),
                mock::change::create("^docs(.|\n)*$", "Documentation")
            ])
        );
        assert_eq!(generate_changelog, false);
        assert_eq!(log_level, LogLevel::WARNING);
        assert_eq!(changelog_location, "THECHANGES.md");
        assert_eq!(git_auth_method, Auth::GITHUB);
    }

    #[test]
    fn test_config_parse_default_changelog_location() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": false,
            "log_level": "warning",
            "git_auth_method": "github"
        });
        let (
            release_branch,
            major_changes,
            minor_changes,
            patch_changes,
            other_changes,
            generate_changelog,
            log_level,
            changelog_location,
            git_auth_method,
        ) = Config::parse(json_content).unwrap();
        assert_eq!(release_branch, "feature_branch");
        assert_eq!(
            major_changes,
            ChangeList::new(vec![mock::change::create(
                "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                "BREAKING CHANGES"
            )])
        );
        assert_eq!(
            minor_changes,
            ChangeList::new(vec![mock::change::create("^feat(.|\n)*$", "Features")])
        );
        assert_eq!(
            patch_changes,
            ChangeList::new(vec![mock::change::create("^fix(.|\n)*$", "Patches")])
        );
        assert_eq!(
            other_changes,
            ChangeList::new(vec![
                mock::change::create("^chore(.|\n)*$", "Maintenance Items"),
                mock::change::create("^docs(.|\n)*$", "Documentation")
            ])
        );
        assert_eq!(generate_changelog, false);
        assert_eq!(log_level, LogLevel::WARNING);
        assert_eq!(changelog_location, "CHANGELOG.md");
        assert_eq!(git_auth_method, Auth::GITHUB);
    }

    #[test]
    fn test_config_parse_default_release_branch() {
        let json_content: Value = json!({
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": false,
            "log_level": "warning",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "github"
        });
        let (
            release_branch,
            major_changes,
            minor_changes,
            patch_changes,
            other_changes,
            generate_changelog,
            log_level,
            changelog_location,
            git_auth_method,
        ) = Config::parse(json_content).unwrap();
        assert_eq!(release_branch, "master");
        assert_eq!(
            major_changes,
            ChangeList::new(vec![mock::change::create(
                "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                "BREAKING CHANGES"
            )])
        );
        assert_eq!(
            minor_changes,
            ChangeList::new(vec![mock::change::create("^feat(.|\n)*$", "Features")])
        );
        assert_eq!(
            patch_changes,
            ChangeList::new(vec![mock::change::create("^fix(.|\n)*$", "Patches")])
        );
        assert_eq!(
            other_changes,
            ChangeList::new(vec![
                mock::change::create("^chore(.|\n)*$", "Maintenance Items"),
                mock::change::create("^docs(.|\n)*$", "Documentation")
            ])
        );
        assert_eq!(generate_changelog, false);
        assert_eq!(log_level, LogLevel::WARNING);
        assert_eq!(changelog_location, "THECHANGES.md");
        assert_eq!(git_auth_method, Auth::GITHUB);
    }

    #[test]
    fn test_config_parse_invalid_release_branch() {
        let json_content: Value = json!({
            "release_branch": 888,
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": false,
            "log_level": "warning",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "github"
        });
        let result = Config::parse(json_content);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_config_parse_invalid_major_changes() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                "BREAKING CHANGE"
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": false,
            "log_level": "warning",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "github"
        });
        let result = Config::parse(json_content);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_config_parse_invalid_minor_changes() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": true,
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": false,
            "log_level": "warning",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "github"
        });
        let result = Config::parse(json_content);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_config_parse_invalid_patch_changes() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": 88.8
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": false,
            "log_level": "warning",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "github"
        });
        let result = Config::parse(json_content);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_config_parse_invalid_other_changes() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                [
                    "Hello",
                    "world",
                    false
                ],
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": false,
            "log_level": "warning",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "github"
        });
        let result = Config::parse(json_content);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_config_parse_invalid_generate_changelog() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": 9007,
            "log_level": "warning",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "github"
        });
        let result = Config::parse(json_content);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_config_parse_invalid_log_level() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": true,
            "log_level": "notavalidlevel",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "github"
        });
        let result = Config::parse(json_content);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_config_parse_invalid_changelog_location() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": true,
            "log_level": "warning",
            "changelog_location": false,
            "git_auth_method": "github"
        });
        let result = Config::parse(json_content);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_config_parse_invalid_git_auth_method() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": [
                {
                    "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                    "kind": "BREAKING CHANGES"
                }
            ],
            "minor_changes": [
                {
                    "pattern": "^feat(.|\n)*$",
                    "kind": "Features"
                }
            ],
            "patch_changes": [
                {
                    "pattern": "^fix(.|\n)*$",
                    "kind": "Patches"
                }
            ],
            "other_changes": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
            "generate_changelog": false,
            "log_level": "warning",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "notavalidrepo"
        });
        let result = Config::parse(json_content);
        assert_eq!(result.is_ok(), false);
    }
}
