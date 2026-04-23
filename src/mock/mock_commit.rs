use crate::Commit;
use chrono::DateTime;

#[allow(dead_code)]
pub fn mock_commit(message: &str) -> Commit {
    Commit::new(
        "12345678",
        "John Doe",
        DateTime::parse_from_str("Wed Apr 22 19:12:34 2026 -0400", "%a %b %d %H:%M:%S %Y %z")
            .unwrap(),
        message,
    )
}
