mod commit_all;
mod get_commits;
pub mod github;
mod latest_tag;
mod push;
mod tag;

pub use commit_all::commit_all;
pub use get_commits::get_commits;
pub use latest_tag::latest_tag;
pub use push::push;
pub use tag::tag;
