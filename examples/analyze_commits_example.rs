use semver_common::mock;
use semver_release::{
    analyzer::analyze_commits,
    mock::{CommitType, extract_actual_changes, mock_commits},
};

fn main() {
    let expected_commits = vec![
        ("feat: some commit one", CommitType::MINOR),
        ("chore: some maintenance stuff", CommitType::OTHER),
        ("fix(hello): some commit one", CommitType::PATCH),
        (
            "feat: some commit one\n\nBREAKING CHANGE: this will break the current version.",
            CommitType::MAJOR,
        ),
        ("feat(ai): some commit two", CommitType::MINOR),
        ("docs(readme): updated the readme", CommitType::OTHER),
    ];
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
        (1, 7, 4),
    )
    .unwrap();

    let actual_changes = extract_actual_changes(
        &new_version,
        &major_changes,
        &minor_changes,
        &patch_changes,
        &other_changes,
    );

    assert_eq!(new_version.get(), "v2.0.0");
    assert_eq!(actual_changes, expected_changes);
}
