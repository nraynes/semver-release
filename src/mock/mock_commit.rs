#[cfg(test)]
pub mod commit {
    use crate::Commit;
    use chrono::DateTime;

    pub fn create(message: &str) -> Commit {
        Commit::new(
            "12345678",
            "John Doe",
            DateTime::parse_from_str("Wed Apr 22 19:12:34 2026 -0400", "%a %b %d %H:%M:%S %Y %z")
                .unwrap(),
            message,
        )
    }
}
