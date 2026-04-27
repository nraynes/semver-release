//! This is the lib part of the SemVer-Release tool. It provides a struct called SemVer
//! that can be initialized with the configuration file. You can then call release on result
//! to start the semantic release workflow.
//!
//! # Examples
//!
//! See main.rs in the source code for usage.

mod mock;
mod models;
mod semver;
mod tools;
mod utils;

pub use models::Alert;
pub use models::Change;
pub use models::ChangeList;
pub use models::Commit;
pub use models::CommitBucket;
pub use models::CommitMap;
pub use models::Config;
pub use models::Version;
pub use semver::SemVer;
pub use tools::Changelog;
pub use tools::analyzer;
pub use tools::git;
pub use utils::parse_args;
pub use utils::run_command;

#[cfg(test)]
pub mod tests {
    use super::*;

    pub use mock::objects as mock;
}
