use crate::{
    ChangeList, git,
    models::{Alert, Change},
};
use r_log::LogLevel;
use serde_json::{self, Value, map::Map};
use std::{fs, str::FromStr};

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
    commit_changes: bool,
    push_changes: bool,
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

    /// Whether to commit the changes made during the semantic versioning process.
    pub fn commit_changes(&self) -> &bool {
        &self.commit_changes
    }

    /// Whether to push the changes made during the semantic versioning process.
    pub fn push_changes(&self) -> &bool {
        &self.push_changes
    }

    /// Creates a new Config object when supplied a valid deserialized JSON Value.
    pub fn load(config_contents: Value) -> Result<Self, Alert> {
        let conf = config_contents
            .as_object()
            .ok_or("Could not parse config.")?;

        Ok(Config {
            release_branch: Config::parse_str_with_default(conf, "release_branch", "master")?,
            major_changes: Config::parse_changelist(conf, "major_changes")?,
            minor_changes: Config::parse_changelist(conf, "minor_changes")?,
            patch_changes: Config::parse_changelist(conf, "patch_changes")?,
            other_changes: Config::parse_changelist(conf, "other_changes")?,
            generate_changelog: Config::parse_bool_with_default(conf, "generate_changelog", true)?,
            log_level: Config::parse_loglevel_with_default(conf, "log_level", "info")?,
            changelog_location: Config::parse_str_with_default(
                conf,
                "changelog_location",
                "CHANGELOG.md",
            )?,
            git_auth_method: Config::parse_gitauthmethod(conf, "git_auth_method")?,
            commit_changes: Config::parse_bool_with_default(conf, "commit_changes", true)?,
            push_changes: Config::parse_bool_with_default(conf, "push_changes", true)?,
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

    fn parse_gitauthmethod(conf: &Map<String, Value>, key: &str) -> Result<git::auth::Auth, Alert> {
        let git_auth_method_value = conf.get(key).ok_or("Could not get git_auth_method.")?;
        let git_auth_method_str = git_auth_method_value
            .as_str()
            .ok_or("git_auth_method invalid value.")?;
        git::auth::Auth::from_str(git_auth_method_str)
    }

    fn parse_bool_with_default(
        conf: &Map<String, Value>,
        key: &str,
        default: bool,
    ) -> Result<bool, Alert> {
        Ok(conf
            .get(key)
            .unwrap_or(&Value::from(default))
            .as_bool()
            .ok_or("Could not get commit_changes.")?)
    }

    fn parse_loglevel_with_default(
        conf: &Map<String, Value>,
        key: &str,
        default: &str,
    ) -> Result<LogLevel, Alert> {
        Ok(LogLevel::from_str(
            conf.get(key)
                .unwrap_or(&Value::from(default))
                .as_str()
                .ok_or("Could not get log_level.")?,
        )
        .ok_or("Not a valid value for LogLevel.")?)
    }

    /// Parse a string from the config.
    fn parse_str_with_default(
        conf: &Map<String, Value>,
        key: &str,
        default: &str,
    ) -> Result<String, Alert> {
        Ok(String::from(
            conf.get(key)
                .unwrap_or(&Value::from(default))
                .as_str()
                .ok_or("Could not get string from key.")?,
        ))
    }

    /// Parses a list of changes from the config.
    fn parse_changelist(conf: &Map<String, Value>, key: &str) -> Result<ChangeList, Alert> {
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
    fn test_config_load_valid() {
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
            "git_auth_method": "github",
            "commit_changes": true,
            "push_changes": true
        });
        let config = Config::load(json_content).unwrap();
        assert_eq!(config.release_branch, "feature_branch");
        assert_eq!(
            config.major_changes,
            ChangeList::new(vec![mock::change::create(
                "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                "BREAKING CHANGES"
            )])
        );
        assert_eq!(
            config.minor_changes,
            ChangeList::new(vec![mock::change::create("^feat(.|\n)*$", "Features")])
        );
        assert_eq!(
            config.patch_changes,
            ChangeList::new(vec![mock::change::create("^fix(.|\n)*$", "Patches")])
        );
        assert_eq!(
            config.other_changes,
            ChangeList::new(vec![
                mock::change::create("^chore(.|\n)*$", "Maintenance Items"),
                mock::change::create("^docs(.|\n)*$", "Documentation")
            ])
        );
        assert_eq!(config.generate_changelog, false);
        assert_eq!(config.log_level, LogLevel::WARNING);
        assert_eq!(config.changelog_location, "THECHANGES.md");
        assert_eq!(config.git_auth_method, Auth::GITHUB);
        assert_eq!(config.commit_changes, true);
        assert_eq!(config.push_changes, true);
    }

    #[test]
    fn test_config_parse_str_with_default_valid() {
        let json_content = json!({
            "release_branch": "feature_branch",
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result =
            Config::parse_str_with_default(conf, "release_branch", "default_value").unwrap();
        assert_eq!(result, "feature_branch");
    }

    #[test]
    fn test_config_parse_str_with_default_invalid() {
        let json_content = json!({
            "nothing": "feature_branch",
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result =
            Config::parse_str_with_default(conf, "release_branch", "default_value").unwrap();
        assert_eq!(result, "default_value");
    }

    #[test]
    fn test_config_parse_bool_with_default_valid() {
        let json_content = json!({
            "generate_changelog": false,
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result = Config::parse_bool_with_default(conf, "generate_changelog", true).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_config_parse_bool_with_default_invalid() {
        let json_content = json!({
            "nothing": false,
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result = Config::parse_bool_with_default(conf, "generate_changelog", true).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_config_parse_loglevel_with_default_valid() {
        let json_content = json!({
            "log_level": "warning",
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result = Config::parse_loglevel_with_default(conf, "log_level", "info").unwrap();
        assert_eq!(result, LogLevel::WARNING);
    }

    #[test]
    fn test_config_parse_loglevel_with_default_invalid() {
        let json_content = json!({
            "nothing": "warning",
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result = Config::parse_loglevel_with_default(conf, "log_level", "info").unwrap();
        assert_eq!(result, LogLevel::INFO);
    }

    #[test]
    fn test_config_parse_gitauthmethod_valid() {
        let json_content = json!({
            "git_auth_method": "github",
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result = Config::parse_gitauthmethod(conf, "git_auth_method").unwrap();
        assert_eq!(result, git::auth::Auth::GITHUB);
    }

    #[test]
    fn test_config_parse_gitauthmethod_invalid() {
        let json_content = json!({
            "git_auth_method": "notvalid",
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result = Config::parse_gitauthmethod(conf, "git_auth_method");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_config_parse_changelist_valid() {
        let json_content = json!({
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
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result = Config::parse_changelist(conf, "other_changes").unwrap();
        assert_eq!(
            result,
            ChangeList::new(vec![
                mock::change::create("^chore(.|\n)*$", "Maintenance Items"),
                mock::change::create("^docs(.|\n)*$", "Documentation")
            ])
        );
    }

    #[test]
    fn test_config_parse_changelist_invalid() {
        let json_content = json!({
            "nothing": [
                {
                    "pattern": "^chore(.|\n)*$",
                    "kind": "Maintenance Items"
                },
                {
                    "pattern": "^docs(.|\n)*$",
                    "kind": "Documentation"
                }
            ],
        });
        let conf = json_content
            .as_object()
            .ok_or("Could not parse config.")
            .unwrap();
        let result = Config::parse_changelist(conf, "other_changes");
        assert_eq!(result.is_err(), true);
    }
}
