use semver_common::CommitBucket;
use semver_common::mock;
use semver_common::{ChangeList, Commit, Version};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum CommitType {
    MAJOR,
    MINOR,
    PATCH,
    OTHER,
}

fn mock_comparison_map() -> HashMap<CommitType, Vec<Commit>> {
    let mut comparison_map: HashMap<CommitType, Vec<Commit>> = HashMap::new();
    comparison_map.insert(CommitType::MAJOR, Vec::new());
    comparison_map.insert(CommitType::MINOR, Vec::new());
    comparison_map.insert(CommitType::PATCH, Vec::new());
    comparison_map.insert(CommitType::OTHER, Vec::new());
    comparison_map
}

pub fn mock_commits(
    commit_list: Vec<(&str, CommitType)>,
) -> (Vec<Commit>, HashMap<CommitType, Vec<Commit>>) {
    let commits: Vec<Commit> = commit_list
        .iter()
        .map(|v| mock::commit::create(v.0))
        .collect();
    let mut expected_commits = mock_comparison_map();
    commit_list.iter().for_each(|v| {
        match v.1 {
            CommitType::MAJOR => expected_commits
                .get_mut(&CommitType::MAJOR)
                .unwrap()
                .push(mock::commit::create(v.0)),
            CommitType::MINOR => expected_commits
                .get_mut(&CommitType::MINOR)
                .unwrap()
                .push(mock::commit::create(v.0)),
            CommitType::PATCH => expected_commits
                .get_mut(&CommitType::PATCH)
                .unwrap()
                .push(mock::commit::create(v.0)),
            CommitType::OTHER => expected_commits
                .get_mut(&CommitType::OTHER)
                .unwrap()
                .push(mock::commit::create(v.0)),
        };
    });
    (commits, expected_commits)
}

fn process_change_list(
    comparison_map: &mut HashMap<CommitType, Vec<Commit>>,
    change_list: &ChangeList,
    version: &Version,
    commit_type: CommitType,
) {
    for change in change_list.changes().iter() {
        comparison_map.get_mut(&commit_type).unwrap().extend(
            version
                .changes()
                .bucket(change.kind())
                .unwrap_or(&CommitBucket::new(change.kind(), *change.priority()))
                .commits()
                .clone(),
        );
    }
}

pub fn extract_actual_changes(
    version: &Version,
    major_changes: &ChangeList,
    minor_changes: &ChangeList,
    patch_changes: &ChangeList,
    other_changes: &ChangeList,
) -> HashMap<CommitType, Vec<Commit>> {
    let mut actual_changes = mock_comparison_map();
    process_change_list(
        &mut actual_changes,
        major_changes,
        version,
        CommitType::MAJOR,
    );
    process_change_list(
        &mut actual_changes,
        minor_changes,
        version,
        CommitType::MINOR,
    );
    process_change_list(
        &mut actual_changes,
        patch_changes,
        version,
        CommitType::PATCH,
    );
    process_change_list(
        &mut actual_changes,
        other_changes,
        version,
        CommitType::OTHER,
    );
    actual_changes
}
