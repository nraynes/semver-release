use crate::{Alert, ChangeList, Commit, CommitMap, Version};

pub fn analyze_commits(
    commits: &Vec<Commit>,
    major_changes: &ChangeList,
    minor_changes: &ChangeList,
    patch_changes: &ChangeList,
    other_changes: &ChangeList,
    current_major: u32,
    current_minor: u32,
    current_patch: u32,
) -> Result<Version, Alert> {
    let mut major = current_major;
    let mut minor = current_minor;
    let mut patch = current_patch;
    let mut changes: CommitMap = CommitMap::new();
    for commit in commits.iter() {
        match major_changes.check(commit) {
            Some(kind) => {
                major += 1;
                changes.insert(&kind, commit.clone())?;
                continue;
            }
            None => {}
        };
        match minor_changes.check(commit) {
            Some(kind) => {
                minor += 1;
                changes.insert(&kind, commit.clone())?;
                continue;
            }
            None => {}
        };
        match patch_changes.check(commit) {
            Some(kind) => {
                patch += 1;
                changes.insert(&kind, commit.clone())?;
                continue;
            }
            None => {}
        };
        match other_changes.check(commit) {
            Some(kind) => {
                changes.insert(&kind, commit.clone())?;
                continue;
            }
            None => {}
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
            current_major,
            current_minor,
            current_patch,
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
