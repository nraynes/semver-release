use crate::models::Alert;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display, Formatter};

const COMMIT_TIME_FORMAT: &str = "%a %b %d %H:%M:%S %Y %z";

mod datetime_ser {
    use serde::de;

    use super::*;

    pub fn serialize<S>(ext: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&ext.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = match DateTime::parse_from_str(&s, COMMIT_TIME_FORMAT) {
            Ok(v) => v,
            Err(e) => return Err(de::Error::custom(e.to_string())),
        };
        Ok(dt)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters)]
pub struct Commit {
    id: String,
    author: String,

    #[serde(with = "datetime_ser")]
    timestamp: DateTime<FixedOffset>,

    message: String,
}

impl Ord for Commit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.message > other.message {
            return std::cmp::Ordering::Greater;
        } else if self.message < other.message {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Equal
    }
}

impl Eq for Commit {}

impl PartialOrd for Commit {
    fn ge(&self, other: &Self) -> bool {
        self.message >= other.message
    }

    fn gt(&self, other: &Self) -> bool {
        self.message > other.message
    }

    fn le(&self, other: &Self) -> bool {
        self.message <= other.message
    }

    fn lt(&self, other: &Self) -> bool {
        self.message < other.message
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.message == other.message {
            return Some(std::cmp::Ordering::Equal);
        } else if self.message > other.message {
            return Some(std::cmp::Ordering::Greater);
        } else if self.message < other.message {
            return Some(std::cmp::Ordering::Less);
        }
        None
    }
}

impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl Commit {
    pub fn new(id: &str, author: &str, timestamp: DateTime<FixedOffset>, message: &str) -> Self {
        Commit {
            id: id.to_string(),
            author: author.to_string(),
            timestamp,
            message: message.to_string(),
        }
    }

    /// Creates a new Commit object after converting string for timestamp to a DateTime.
    pub fn new_from_str(
        id: &str,
        author: &str,
        timestamp: &str,
        message: &str,
    ) -> Result<Self, Alert> {
        let parsed_timestamp = DateTime::parse_from_str(timestamp, COMMIT_TIME_FORMAT)?;
        Ok(Commit::new(id, author, parsed_timestamp, message))
    }

    /// Creates a new Commit object from a standard commit in text format from "git log" output.
    ///
    /// # Example:
    ///
    /// ```
    /// use semver_release::Commit;
    /// use chrono::DateTime;
    ///
    /// let c = String::from(
    ///             "490049bf36b19b30d23b4be5a4u94f71b5c6475c
    /// Author: Some Author <myemail@email.com>
    /// Date:   Tue Apr 14 17:35:15 2026 -0400
    ///
    ///     feat: added feature to get commit list
    /// ",
    /// );
    /// let commit =
    ///     Commit::new_from_commit(c).expect("Commit could not be instantiated during test.");
    /// assert_eq!(commit.id(), "490049bf36b19b30d23b4be5a4u94f71b5c6475c");
    /// assert_eq!(commit.author(), "Some Author <myemail@email.com>");
    /// assert_eq!(
    ///     commit.timestamp(),
    ///     &DateTime::parse_from_str("Tue Apr 14 17:35:15 2026 -0400", "%a %b %d %H:%M:%S %Y %z").unwrap()
    /// );
    /// assert_eq!(commit.message(), "feat: added feature to get commit list");
    /// ```
    pub fn new_from_commit(commit: String) -> Result<Self, Alert> {
        let lines: Vec<&str> = commit.split("\n").collect();
        if lines.len() > 3 {
            let id_line: (&str, &str) = lines[0].split_once(" ").unwrap_or((lines[0], ""));
            let commit_id = id_line.0.trim();
            let author_line: (&str, &str) = lines[1]
                .split_once(":")
                .ok_or("Could not parse author line of commit.")?;
            let author = author_line.1.trim();
            let date_line: (&str, &str) = lines[2]
                .split_once(":")
                .ok_or("Could not parse date line of commit.")?;
            let date = date_line.1.trim();
            let commit_end_line: usize = lines.len() - 1;
            let commit_message_untrimmed = lines[4..commit_end_line].join("\n");
            let commit_message = commit_message_untrimmed.trim();
            let object = Commit::new_from_str(commit_id, author, date, commit_message)?;
            return Ok(object);
        }
        Err(Alert::from("Commit is not valid"))
    }

    pub fn msg(&self) -> &str {
        &self.message
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_commit_new_top_commit() {
        let c = String::from(
            "490049bf36b19b30d23b4be5a4u94f71b5c6475c (HEAD -> master)
Author: Some Author <myemail@email.com>
Date:   Tue Apr 14 17:35:15 2026 -0400

    feat: added feature to get commit list
",
        );
        let commit =
            Commit::new_from_commit(c).expect("Commit could not be instantiated during test.");
        assert_eq!(commit.id, "490049bf36b19b30d23b4be5a4u94f71b5c6475c");
        assert_eq!(commit.author, "Some Author <myemail@email.com>");
        assert_eq!(
            commit.timestamp,
            DateTime::parse_from_str("Tue Apr 14 17:35:15 2026 -0400", COMMIT_TIME_FORMAT).unwrap()
        );
        assert_eq!(commit.message, "feat: added feature to get commit list");
    }

    #[test]
    fn test_commit_new_commit() {
        let c = String::from(
            "490049bf36b19b30d23b4be5a4u94f71b5c6475c
Author: Some Author <myemail@email.com>
Date:   Tue Apr 14 17:35:15 2026 -0400

    feat: added feature to get commit list
",
        );
        let commit =
            Commit::new_from_commit(c).expect("Commit could not be instantiated during test.");
        assert_eq!(commit.id, "490049bf36b19b30d23b4be5a4u94f71b5c6475c");
        assert_eq!(commit.author, "Some Author <myemail@email.com>");
        assert_eq!(
            commit.timestamp,
            DateTime::parse_from_str("Tue Apr 14 17:35:15 2026 -0400", COMMIT_TIME_FORMAT).unwrap()
        );
        assert_eq!(commit.message, "feat: added feature to get commit list");
    }
}
