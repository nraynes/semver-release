use chrono;
use regex;
use serde_json;
use std::{convert::From, fmt::Display, io, num, string};

/// Alert is a wrapper for all the various error types that may be returned by various
/// crate functions.
#[derive(Debug, Clone)]
pub struct Alert {
    val: String,
}

impl From<chrono::ParseError> for Alert {
    fn from(value: chrono::ParseError) -> Self {
        Alert {
            val: format!("chrono::ParseError: {}", value),
        }
    }
}

impl From<regex::Error> for Alert {
    fn from(value: regex::Error) -> Self {
        Alert {
            val: format!("regex::Error: {}", value),
        }
    }
}

impl From<io::Error> for Alert {
    fn from(value: io::Error) -> Self {
        Alert {
            val: format!("io::Error: {}", value),
        }
    }
}

impl From<num::ParseIntError> for Alert {
    fn from(value: num::ParseIntError) -> Self {
        Alert {
            val: format!("num::ParseIntError: {}", value),
        }
    }
}

impl From<serde_json::Error> for Alert {
    fn from(value: serde_json::Error) -> Self {
        Alert {
            val: format!("serde_json::Error: {}", value),
        }
    }
}

impl From<string::FromUtf8Error> for Alert {
    fn from(value: string::FromUtf8Error) -> Self {
        Alert {
            val: format!("string::FromUtf8Error: {}", value),
        }
    }
}

impl From<&str> for Alert {
    fn from(value: &str) -> Self {
        Alert {
            val: String::from(value),
        }
    }
}

impl Display for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
