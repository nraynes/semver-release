use std::fmt::Display;

#[derive(PartialEq, PartialOrd, Clone)]
pub enum LogLevel {
    EMERGENCY,
    ALERT,
    CRITICAL,
    ERROR,
    WARNING,
    NOTIFICATION,
    INFO,
    DEBUG,
}

impl LogLevel {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "emergency" => Some(LogLevel::EMERGENCY),
            "alert" => Some(LogLevel::ALERT),
            "critical" => Some(LogLevel::CRITICAL),
            "error" => Some(LogLevel::ERROR),
            "warning" => Some(LogLevel::WARNING),
            "notification" => Some(LogLevel::NOTIFICATION),
            "info" => Some(LogLevel::INFO),
            "debug" => Some(LogLevel::DEBUG),
            _ => None,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::EMERGENCY => write!(f, "EMERGENCY"),
            LogLevel::ALERT => write!(f, "ALERT"),
            LogLevel::CRITICAL => write!(f, "CRITICAL"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::WARNING => write!(f, "WARNING"),
            LogLevel::NOTIFICATION => write!(f, "NOTIFICATION"),
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
        }
    }
}
