mod mock_change;
mod mock_changelist;
mod mock_commit;
mod mock_commit_map;
mod mock_version;

#[cfg(test)]
pub mod objects {
    use super::*;

    pub use mock_change::change;
    pub use mock_changelist::changelist;
    pub use mock_commit::commit;
    pub use mock_commit_map::commit_map;
    pub use mock_version::version;
}
