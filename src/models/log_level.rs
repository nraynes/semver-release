use std::fmt::Display;

#[derive(PartialEq, PartialOrd, Clone)]
pub enum LogLevel {
    ALERT,
    WARNING,
    INFO,
    DEBUG,
}

impl LogLevel {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "alert" => Some(LogLevel::ALERT),
            "warning" => Some(LogLevel::WARNING),
            "info" => Some(LogLevel::INFO),
            "debug" => Some(LogLevel::DEBUG),
            _ => None,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
