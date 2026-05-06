mod test_env;

use test_env::TestEnv;

use serde_json::json;

#[test]
fn test_release_patch_upgrade() {
    let mut env = TestEnv::new(Some(json!({
        "push_changes": false,
    })));
    env.commit("fix: test");
    env.run();
    let tag = env.latest_tag();
    assert_eq!(&tag, "v0.0.1\n");
}

#[test]
fn test_release_minor_upgrade() {
    let mut env = TestEnv::new(Some(json!({
        "push_changes": false,
    })));
    env.commit("feat: test");
    env.run();
    let tag = env.latest_tag();
    assert_eq!(&tag, "v0.1.0\n");
}

#[test]
fn test_release_major_upgrade() {
    let mut env = TestEnv::new(Some(json!({
        "push_changes": false,
    })));
    env.commit("fix: test\n\nBREAKING CHANGE: test");
    env.run();
    let tag = env.latest_tag();
    assert_eq!(&tag, "v1.0.0\n");
}

// #[test]
// fn test_release_no_changelog() {
//     let mut env = TestEnv::new(Some(json!({
//         "push_changes": false,
//     })));
//     env.commit("fix: test");
//     env.run();
//     let tag = env.latest_tag();
//     assert_eq!(&tag, "v0.0.1\n");
// }
