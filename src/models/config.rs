use crate::{
    ChangeList, LogLevel, git,
    models::{Alert, Change},
};
use indexmap::IndexMap;
use rust_yaml::{Value, Yaml};
use std::fs;

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
    pub fn release_branch(&self) -> &str {
        &self.release_branch
    }

    pub fn major_changes(&self) -> &ChangeList {
        &self.major_changes
    }

    pub fn minor_changes(&self) -> &ChangeList {
        &self.minor_changes
    }

    pub fn patch_changes(&self) -> &ChangeList {
        &self.patch_changes
    }

    pub fn other_changes(&self) -> &ChangeList {
        &self.other_changes
    }

    pub fn generate_changelog(&self) -> &bool {
        &self.generate_changelog
    }

    pub fn log_level(&self) -> &LogLevel {
        &self.log_level
    }

    pub fn changelog_location(&self) -> &str {
        &self.changelog_location
    }

    pub fn git_auth_method(&self) -> &git::auth::Auth {
        &self.git_auth_method
    }

    pub fn load(yaml: Value) -> Result<Self, Alert> {
        let conf = Config::parse_yaml(yaml)?;
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

    pub fn load_from_file(file_path: String) -> Result<Self, Alert> {
        let config_file = fs::read_to_string(file_path)?;
        let yaml = Yaml::new();
        let config_contents = yaml.load_str(&config_file)?;
        let config = Config::load(config_contents)?;
        Ok(config)
    }

    fn parse_yaml(
        yaml: Value,
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
        let conf = yaml
            .as_mapping()
            .ok_or("Could not parse yaml from config file.")?;
        let master_branch = Value::from("release_branch");
        let release_branch = conf
            .get(&Value::from("release_branch"))
            .unwrap_or(&master_branch)
            .as_str()
            .ok_or("Could not get release_branch.")?;
        let major_changes = Config::parse_change_vec(conf, "major_changes")?;
        let minor_changes = Config::parse_change_vec(conf, "major_changes")?;
        let patch_changes = Config::parse_change_vec(conf, "major_changes")?;
        let other_changes = Config::parse_change_vec(conf, "major_changes")?;
        let generate_changelog = conf
            .get(&Value::from("generate_changelog"))
            .unwrap_or(&Value::from(true))
            .as_bool()
            .ok_or("Could not get generate_changelog.")?;
        let log_level = LogLevel::from_str(
            conf.get(&Value::from("log_level"))
                .unwrap_or(&Value::from("info"))
                .as_str()
                .ok_or("Could not get log_level.")?,
        )
        .ok_or("Not a valid value for LogLevel.")?;
        let default_changelog_location = Value::from("CHANGELOG.md");
        let changelog_location = conf
            .get(&Value::from("changelog_location"))
            .unwrap_or(&default_changelog_location)
            .as_str()
            .ok_or("Could not get changelog_location.")?;
        let git_auth_method_value = conf
            .get(&Value::from("git_auth_method"))
            .ok_or("Could not get git_auth_method.")?;
        let git_auth_method_str = git_auth_method_value
            .as_str()
            .ok_or("git_auth_method invalid value.")?;
        let git_auth_method = git::auth::Auth::from_str(git_auth_method_str)
            .ok_or("Invalid value for git_auth_method.")?;
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

    fn parse_change_vec(conf: &IndexMap<Value, Value>, key: &str) -> Result<ChangeList, Alert> {
        let unpacked_value = conf
            .get(&Value::from(key))
            .ok_or("Could not parse value from change vector in config.")?;
        let unpacked_vector = unpacked_value
            .as_sequence()
            .ok_or("Could not parse vector from change vector in config")?;
        let mut changes = vec![];
        for change_value in unpacked_vector.iter() {
            let change = Change::from(change_value)?;
            changes.push(change);
        }
        Ok(ChangeList::new(changes))
    }
}
