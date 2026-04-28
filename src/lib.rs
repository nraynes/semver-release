//! This is the lib part of the SemVer-Release tool. It provides a struct called SemVer
//! that can be initialized with the configuration file. You can then call release on result
//! to start the semantic release workflow.
//!
//! # Examples
//!
//! See main.rs in the source code for usage.

mod models;
mod semver;
mod tools;
mod utils;

pub use models::Config;
pub use semver::SemVer;
pub use tools::Changelog;
pub use tools::analyzer;
pub use tools::git;
pub use utils::parse_args;
pub use utils::plugins;
