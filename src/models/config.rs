use crate::{ChangeList, git, models::Alert};
use derive_getters::Getters;
use r_log::LogLevel;
use serde::{Deserialize, Serialize};
use std::fs;

fn default_loglevel() -> LogLevel {
    LogLevel::INFO
}

fn default_true() -> bool {
    true
}

fn default_release_branch() -> String {
    String::from("master")
}

fn default_changelog_location() -> String {
    String::from("CHANGELOG.md")
}

/// Configuration object to hold all the values from the global configuration file.
#[derive(Serialize, Deserialize, Debug, Getters)]
pub struct Config {
    #[serde(default = "default_release_branch")]
    release_branch: String,

    major_changes: ChangeList,
    minor_changes: ChangeList,
    patch_changes: ChangeList,
    other_changes: ChangeList,

    #[serde(default = "default_true")]
    generate_changelog: bool,

    #[serde(default = "default_loglevel")]
    log_level: LogLevel,

    #[serde(default = "default_changelog_location")]
    changelog_location: String,

    git_auth_method: git::auth::Auth,

    #[serde(default = "default_true")]
    commit_changes: bool,

    #[serde(default = "default_true")]
    push_changes: bool,
}

impl Config {
    /// Creates a new Config from a file at the supplied path, provided the file contains
    /// valid syntax for JSON and the config.
    pub fn load_from_file(file_path: String) -> Result<Self, Alert> {
        let config_file = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&config_file)?;
        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use serde_json::{Value, json};

    use crate::{git::auth::Auth, tests::mock};

    use super::*;

    #[test]
    fn test_config_load_valid() {
        let json_content: Value = json!({
            "release_branch": "feature_branch",
            "major_changes": {
                "changes": [
                    {
                        "pattern": "^(.|\n)*BREAKING_CHANGE(.|\n)*$",
                        "kind": "BREAKING CHANGES"
                    }
                ]
            },
            "minor_changes": {
                "changes": [
                    {
                        "pattern": "^feat(.|\n)*$",
                        "kind": "Features"
                    }
                ]
            },
            "patch_changes": {
                "changes": [
                    {
                        "pattern": "^fix(.|\n)*$",
                        "kind": "Patches"
                    }
                ]
            },
            "other_changes": {
                "changes": [
                    {
                        "pattern": "^chore(.|\n)*$",
                        "kind": "Maintenance Items"
                    },
                    {
                        "pattern": "^docs(.|\n)*$",
                        "kind": "Documentation"
                    }
                ]
            },
            "generate_changelog": false,
            "log_level": "WARNING",
            "changelog_location": "THECHANGES.md",
            "git_auth_method": "GITHUB",
            "commit_changes": true,
            "push_changes": true
        });
        let config: Config = serde_json::from_value(json_content).unwrap();
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
}
