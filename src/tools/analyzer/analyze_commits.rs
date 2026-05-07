use semver_common::{Alert, ChangeList, Commit, CommitMap, Version};

/// Analyzes a list of commits against a set of patterns that define whether a commit message counts
/// as a major, minor, or patch version upgrade. Saves all matched commits including commits that match other
/// patterns that do not affect the version number, but are used in changelog generation. Packages the final version
/// with a hash map structure containing all of the commits and the type of change they represent.
pub fn analyze_commits(
    commits: &[Commit],
    major_changes: &ChangeList,
    minor_changes: &ChangeList,
    patch_changes: &ChangeList,
    other_changes: &ChangeList,
    current_version: (u32, u32, u32),
) -> Result<Version, Alert> {
    let mut major = current_version.0;
    let mut minor = current_version.1;
    let mut patch = current_version.2;
    let mut changes: CommitMap = CommitMap::new();
    let mut minor_lock = false;
    let mut patch_lock = false;
    for commit in commits.iter() {
        if let Some(change) = major_changes.check(commit) {
            major += 1;
            minor = 0;
            patch = 0;
            changes.insert(change, commit.clone())?;
            minor_lock = true;
            patch_lock = true;
            continue;
        }
        if let Some(change) = minor_changes.check(commit) {
            if !minor_lock {
                minor += 1;
                patch = 0;
            }
            changes.insert(change, commit.clone())?;
            patch_lock = true;
            continue;
        }
        if let Some(change) = patch_changes.check(commit) {
            if !patch_lock {
                patch += 1;
            }
            changes.insert(change, commit.clone())?;
            continue;
        }
        if let Some(change) = other_changes.check(commit) {
            changes.insert(change, commit.clone())?;
            continue;
        };
    }
    Ok(Version::new(major, minor, patch, changes))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    use semver_common::{CommitBucket, mock};

    #[derive(PartialEq, Eq, Hash, Debug)]
    enum CommitType {
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

    fn mock_commits(
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
                    .unwrap_or(&CommitBucket::new(change.kind(), change.priority().clone()))
                    .commits()
                    .clone(),
            );
        }
    }

    fn extract_actual_changes(
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

    fn test_analyze_commits(
        expected_commits: Vec<(&str, CommitType)>,
        mock_version_num: (u32, u32, u32),
        expected_version: &str,
    ) {
        let (commits, expected_changes) = mock_commits(expected_commits);

        let major_changes = mock::changelist::major();
        let minor_changes = mock::changelist::minor();
        let patch_changes = mock::changelist::patch();
        let other_changes = mock::changelist::other();

        let new_version = analyze_commits(
            &commits,
            &major_changes,
            &minor_changes,
            &patch_changes,
            &other_changes,
            mock_version_num,
        )
        .unwrap();

        let actual_changes = extract_actual_changes(
            &new_version,
            &major_changes,
            &minor_changes,
            &patch_changes,
            &other_changes,
        );

        assert_eq!(new_version.get(), expected_version);
        assert_eq!(actual_changes, expected_changes);
    }

    #[test]
    fn test_analyze_commits_random_order() {
        test_analyze_commits(
            vec![
                ("feat: some commit one", CommitType::MINOR),
                ("chore: some maintenance stuff", CommitType::OTHER),
                ("fix(hello): some commit one", CommitType::PATCH),
                (
                    "feat: some commit one\n\nBREAKING CHANGE: this will break the current version.",
                    CommitType::MAJOR,
                ),
                ("feat(ai): some commit two", CommitType::MINOR),
                ("docs(readme): updated the readme", CommitType::OTHER),
            ],
            (1, 7, 4),
            "v2.0.0",
        );
    }

    #[test]
    fn test_analyze_commits_no_history() {
        test_analyze_commits(vec![], (1, 7, 4), "v1.7.4");
    }

    #[test]
    fn test_analyze_commits_descending_order() {
        test_analyze_commits(
            vec![
                (
                    "feat: some commit one\n\nBREAKING CHANGE: this will break the current version.",
                    CommitType::MAJOR,
                ),
                ("feat: some commit one", CommitType::MINOR),
                ("feat(ai): some commit two", CommitType::MINOR),
                ("fix(hello): some commit one", CommitType::PATCH),
                ("chore: some maintenance stuff", CommitType::OTHER),
                ("docs(readme): updated the readme", CommitType::OTHER),
            ],
            (4, 0, 2),
            "v5.0.0",
        );
    }

    #[test]
    fn test_analyze_commits_ascending_order() {
        test_analyze_commits(
            vec![
                ("feat: some commit one", CommitType::MINOR),
                ("feat(ai): some commit two", CommitType::MINOR),
                ("fix(hello): some commit one", CommitType::PATCH),
                (
                    "feat: some commit one\n\nBREAKING CHANGE: this will break the current version.",
                    CommitType::MAJOR,
                ),
                ("chore: some maintenance stuff", CommitType::OTHER),
                ("docs(readme): updated the readme", CommitType::OTHER),
            ],
            (1, 7, 4),
            "v2.0.0",
        );
    }
}
