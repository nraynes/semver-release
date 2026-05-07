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
    fn test_upgrade_override_patch() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        env.commit("fix: test");
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
    fn test_upgrade_override_patch() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        env.commit("fix: test");
        env.commit("fix: test\n\nBREAKING CHANGE: test");
        env.run();
        let tag = env.latest_tag();
        assert_eq!(&tag, "v1.0.0\n");
    }

    #[test]
    fn test_upgrade_override_minor() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        env.commit("feat: test");
        env.commit("fix: test\n\nBREAKING CHANGE: test");
        env.run();
        let tag = env.latest_tag();
        assert_eq!(&tag, "v1.0.0\n");
    }

    #[test]
    fn test_upgrade_override_minor_and_patch() {
        let mut env = TestEnv::new(Some(json!({
            "push_changes": false,
        })));
        env.commit("fix: test");
        env.commit("feat: test");
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

#[test]
fn test_release_no_changelog() {
    let mut env = TestEnv::new(Some(json!({
        "generate_changelog": false,
        "push_changes": false,
    })));
    env.commit("fix: test");
    env.run();
    let tag = env.latest_tag();
    env.temp()
        .child("CHANGELOG.md")
        .assert(predicates::path::missing());
    assert_eq!(&tag, "v0.0.1\n");
}
