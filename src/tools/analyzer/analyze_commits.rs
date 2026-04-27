use crate::{Alert, ChangeList, Commit, CommitMap, Version};

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
    for commit in commits.iter() {
        if let Some(change) = major_changes.check(commit) {
            major += 1;
            changes.insert(change, commit.clone())?;
            continue;
        }
        if let Some(change) = minor_changes.check(commit) {
            minor += 1;
            changes.insert(change, commit.clone())?;
            continue;
        }
        if let Some(change) = patch_changes.check(commit) {
            patch += 1;
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
    use crate::tests::mock;

    #[test]
    fn test_analyze_commits_valid() {
        let major_changes = mock::changelist::major();
        let minor_changes = mock::changelist::minor();
        let patch_changes = mock::changelist::patch();
        let other_changes = mock::changelist::other();
        let commits = vec![
            mock::commit::create("feat: some commit one"),
            mock::commit::create("feat(ai): some commit two"),
            mock::commit::create("fix(hello): some commit one"),
            mock::commit::create(
                "feat: some commit one\n\nBREAKING CHANGE: this will break the current version.",
            ),
            mock::commit::create("chore: some maintenance stuff"),
            mock::commit::create("docs(readme): updated the readme"),
        ];
        let (current_major, current_minor, current_patch) = (1u32, 7u32, 4u32);

        let new_version = analyze_commits(
            &commits,
            &major_changes,
            &minor_changes,
            &patch_changes,
            &other_changes,
            (current_major, current_minor, current_patch),
        )
        .unwrap();

        let actual_major_changes = new_version
            .changes()
            .bucket("BREAKING CHANGES")
            .unwrap()
            .commits();
        let actual_minor_changes = new_version.changes().bucket("Feature").unwrap().commits();
        let actual_patch_changes = new_version.changes().bucket("Fix").unwrap().commits();
        let actual_chore_changes = new_version
            .changes()
            .bucket("Maintenance Items")
            .unwrap()
            .commits();
        let actual_docs_changes = new_version
            .changes()
            .bucket("Documentation")
            .unwrap()
            .commits();

        let expected_major_changes = vec![mock::commit::create(
            "feat: some commit one\n\nBREAKING CHANGE: this will break the current version.",
        )];
        let expected_minor_changes = vec![
            mock::commit::create("feat: some commit one"),
            mock::commit::create("feat(ai): some commit two"),
        ];
        let expected_patch_changes = vec![mock::commit::create("fix(hello): some commit one")];
        let expected_chore_changes = vec![mock::commit::create("chore: some maintenance stuff")];
        let expected_docs_changes = vec![mock::commit::create("docs(readme): updated the readme")];

        assert_eq!(new_version.get(), "v2.9.5");
        assert_eq!(actual_major_changes, &expected_major_changes);
        assert_eq!(actual_minor_changes, &expected_minor_changes);
        assert_eq!(actual_patch_changes, &expected_patch_changes);
        assert_eq!(actual_chore_changes, &expected_chore_changes);
        assert_eq!(actual_docs_changes, &expected_docs_changes);
    }
}
