mod test_env;

use assert_fs::{assert::PathAssert, fixture::PathChild};
use test_env::TestEnv;

use serde_json::json;

#[cfg(test)]
mod patch_release {
    use super::*;

    #[test]
    fn test_upgrade() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        env.commit("fix: test");
        env.run();
        let tag = env.latest_tag();
        assert_eq!(&tag, "v0.0.1\n");
    }
}

#[cfg(test)]
mod minor_release {
    use super::*;

    #[test]
    fn test_upgrade() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        env.commit("feat: test");
        env.run();
        let tag = env.latest_tag();
        assert_eq!(&tag, "v0.1.0\n");
    }

    #[test]
    fn test_upgrade_override_patch_offset() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        env.commit("feat: test");
        env.commit("fix: test");
        env.run();
        let tag = env.latest_tag();
        assert_eq!(&tag, "v0.1.0\n");
    }
}

#[cfg(test)]
mod major_release {
    use super::*;

    #[test]
    fn test_upgrade() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        env.commit("fix: test\n\nBREAKING CHANGE: test");
        env.run();
        let tag = env.latest_tag();
        assert_eq!(&tag, "v1.0.0\n");
    }

    #[test]
    fn test_upgrade_override_minor_offset() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        env.commit("fix: test\n\nBREAKING CHANGE: test");
        env.commit("feat: test");
        env.run();
        let tag = env.latest_tag();
        assert_eq!(&tag, "v1.0.0\n");
    }
}

#[cfg(test)]
mod changelog {
    use std::fs;

    use super::*;

    #[test]
    fn test_release_no_changelog() {
        let mut env = TestEnv::new(Some(json!({
            "generate_changelog": false,
            "push_changes": false,
        })));
        env.commit("fix: test");
        env.run();
        let tag = env.latest_tag();
        env.repo()
            .child("CHANGELOG.md")
            .assert(predicates::path::missing());
        assert_eq!(&tag, "v0.0.1\n");
    }

    #[test]
    fn test_release_changelog() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        let commits = (
            "fix: the quick brown",
            "feat: fox jumped over",
            "feat: yellow fence",
            "refactor: and then ate some grass\n\nBREAKING CHANGE: This is a breaking change.",
        );
        env.commit(commits.0);
        env.commit(commits.1);
        env.commit(commits.2);
        env.commit(commits.3);
        env.run();
        println!(
            "CHANGELOG: {}",
            fs::read_to_string(env.repo().child("CHANGELOG.md")).unwrap()
        );
        env.repo()
            .child("CHANGELOG.md")
            .assert(predicates::str::contains("v1.0.0"))
            .assert(predicates::str::contains(commits.0))
            .assert(predicates::str::contains(commits.1))
            .assert(predicates::str::contains(commits.2))
            .assert(predicates::str::contains(
                "refactor: and then ate some grass",
            ))
            .assert(predicates::str::contains(
                "BREAKING CHANGE: This is a breaking change.",
            ));
    }
}
