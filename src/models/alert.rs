use chrono::ParseError;
use regex::Error as RegexError;
use serde_json::Error as SerdeError;
use std::{
    convert::From, fmt::Display, io::Error as IOError, num::ParseIntError, string::FromUtf8Error,
};

/// Alert is a wrapper for all the various error types that may be returned by various
/// crate functions.
#[derive(Debug, Clone)]
pub struct Alert {
    val: String,
}

impl From<ParseError> for Alert {
    /// chrono::ParseError
    fn from(value: ParseError) -> Self {
        Alert {
            val: format!("chrono::ParseError: {}", value),
        }
    }
}

impl From<RegexError> for Alert {
    /// regex::Error
    fn from(value: RegexError) -> Self {
        Alert {
            val: format!("regex::Error: {}", value),
        }
    }
}

impl From<IOError> for Alert {
    /// io::error
    fn from(value: IOError) -> Self {
        Alert {
            val: format!("io::Error: {}", value),
        }
    }
}

impl From<ParseIntError> for Alert {
    /// num::ParseIntError
    fn from(value: ParseIntError) -> Self {
        Alert {
            val: format!("num::ParseIntError: {}", value),
        }
    }
}

impl From<SerdeError> for Alert {
    /// serde_json::Error
    fn from(value: SerdeError) -> Self {
        Alert {
            val: format!("serde_json::Error: {}", value),
        }
    }
}

impl From<FromUtf8Error> for Alert {
    /// string::FromUtf8Error
    fn from(value: FromUtf8Error) -> Self {
        Alert {
            val: format!("string::FromUtf8Error: {}", value),
        }
    }
}

impl From<&str> for Alert {
    /// &str
    fn from(value: &str) -> Self {
        Alert {
            val: String::from(value),
        }
    }
}

impl Display for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.val)
    }
}
