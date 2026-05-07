use std::{
    collections::HashMap,
    env::current_dir,
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use assert_fs::{
    TempDir,
    fixture::{ChildPath, FileTouch, FileWriteStr, PathChild},
};
use derive_getters::Getters;
use semver_release::Config;
use serde_json::Value;

#[derive(Getters)]
pub struct TestEnv {
    #[allow(dead_code)]
    temp: TempDir,
    repo: ChildPath,
    config_path: ChildPath,
    text_file: ChildPath,
    cargo_bin: Command,
}

impl TestEnv {
    pub fn new(overrides: Option<Value>) -> Self {
        let config = Self::get_config(overrides);
        // Setup mock project repo.
        let temp = assert_fs::TempDir::new().unwrap();
        Command::new("cargo")
            .arg("new")
            .arg("temp_proj")
            .current_dir(temp.path())
            .output()
            .unwrap();
        let repo = temp.child("temp_proj");

        // Setup semver-release configuration file.
        let config_path = repo.child("config.semver.json");
        config_path.touch().unwrap();
        config_path
            .write_str(&serde_json::to_string(&config).unwrap())
            .unwrap();

        // Add a file to write to to stage changes for commits.
        let text_file = repo.child("test_file");
        text_file.touch().unwrap();

        let cargo_bin = Self::get_cargo_bin(repo.path().to_path_buf());

        Self {
            temp,
            repo,
            config_path,
            text_file,
            cargo_bin,
        }
    }

    pub fn commit(&self, commit: &str) {
        self.text_file.write_str(commit).unwrap();
        Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(self.repo.path())
            .output()
            .unwrap();
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit)
            .current_dir(self.repo.path())
            .output()
            .unwrap();
    }

    pub fn latest_tag(&self) -> String {
        let output = Command::new("git")
            .arg("describe")
            .arg("--abbrev=0")
            .arg("--tags")
            .current_dir(self.repo.path())
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    fn get_cargo_bin(path: PathBuf) -> Command {
        let cargo_current_cmd_path = format!(
            "{}/{}",
            current_dir().unwrap().to_str().unwrap(),
            "target/debug/semver-release"
        );
        let cargo_new_cmd_path = format!("{}/{}", path.to_str().unwrap(), "semver-release");
        Command::new("cp")
            .arg(cargo_current_cmd_path)
            .arg(&cargo_new_cmd_path)
            .output()
            .unwrap();
        Command::new(cargo_new_cmd_path)
    }

    fn get_config(overrides: Option<Value>) -> Config {
        let test_config = fs::read_to_string(Path::new("tests/test.config.semver.json")).unwrap();
        let mut config_value = serde_json::from_str(&test_config).unwrap();
        if let Some(ov_value) = overrides {
            let mut config_object: HashMap<String, Value> =
                serde_json::from_value(config_value).unwrap();
            let overrides_object: HashMap<String, Value> =
                serde_json::from_value(ov_value).unwrap();
            config_object.extend(overrides_object);
            config_value = serde_json::to_value(config_object).unwrap();
        }
        serde_json::from_str(&serde_json::to_string(&config_value).unwrap()).unwrap()
    }

    pub fn run(&mut self) {
        let mut child = self
            .cargo_bin
            .arg(self.config_path.path())
            .current_dir(self.repo.path())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut child_stdout = child.stdout.take().unwrap();
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = child_stdout.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            io::stdout().write_all(&buffer[..bytes_read]).unwrap();
        }
        let _ = child.wait().unwrap();
    }
}
