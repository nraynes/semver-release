use crate::LogLevel;

pub trait Console {
    fn console(&mut self, text: String);
}

pub struct Writer {}

impl Console for Writer {
    fn console(&mut self, text: String) {
        println!("{}", text);
    }
}

impl Writer {
    pub fn new() -> Self {
        Writer {}
    }
}

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    pub fn emergency(&self, message: &str) {
        self.log(&mut Writer::new(), message, LogLevel::EMERGENCY);
    }

    pub fn alert(&self, message: &str) {
        self.log(&mut Writer::new(), message, LogLevel::ALERT);
    }

    pub fn critical(&self, message: &str) {
        self.log(&mut Writer::new(), message, LogLevel::CRITICAL);
    }

    pub fn error(&self, message: &str) {
        self.log(&mut Writer::new(), message, LogLevel::ERROR);
    }

    pub fn warning(&self, message: &str) {
        self.log(&mut Writer::new(), message, LogLevel::WARNING);
    }

    pub fn notification(&self, message: &str) {
        self.log(&mut Writer::new(), message, LogLevel::NOTIFICATION);
    }

    pub fn info(&self, message: &str) {
        self.log(&mut Writer::new(), message, LogLevel::INFO);
    }

    pub fn debug(&self, message: &str) {
        self.log(&mut Writer::new(), message, LogLevel::DEBUG);
    }

    fn log<T: Console>(&self, writer: &mut T, message: &str, level: LogLevel) {
        if self.level <= level {
            writer.console(format!("{}: {}", level, message));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockWriter {
        value: String,
    }

    impl Console for MockWriter {
        fn console(&mut self, text: String) {
            println!("{}", text);
            self.value = text;
        }
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                value: String::new(),
            }
        }

        fn value(&self) -> &str {
            &self.value
        }
    }

    fn test_logger(logger_level: LogLevel, test_level: LogLevel, expected_output: &str) {
        let mut mock_console = MockWriter::new();
        let logger = Logger::new(logger_level);
        logger.log(
            &mut mock_console,
            "The Shop Owner Sells French Fries",
            test_level,
        );
        assert_eq!(mock_console.value(), expected_output);
    }

    // Logs hidden.

    #[test]
    fn test_logger_emergency_hidden() {
        test_logger(LogLevel::DEBUG, LogLevel::EMERGENCY, "");
    }

    #[test]
    fn test_logger_alert_hidden() {
        test_logger(LogLevel::DEBUG, LogLevel::ALERT, "");
    }

    #[test]
    fn test_logger_critical_hidden() {
        test_logger(LogLevel::DEBUG, LogLevel::CRITICAL, "");
    }

    #[test]
    fn test_logger_error_hidden() {
        test_logger(LogLevel::DEBUG, LogLevel::ERROR, "");
    }

    #[test]
    fn test_logger_warning_hidden() {
        test_logger(LogLevel::DEBUG, LogLevel::WARNING, "");
    }

    #[test]
    fn test_logger_notification_hidden() {
        test_logger(LogLevel::DEBUG, LogLevel::NOTIFICATION, "");
    }

    #[test]
    fn test_logger_info_hidden() {
        test_logger(LogLevel::DEBUG, LogLevel::INFO, "");
    }

    // Logs shown.

    #[test]
    fn test_logger_emergency() {
        test_logger(
            LogLevel::EMERGENCY,
            LogLevel::EMERGENCY,
            "EMERGENCY: The Shop Owner Sells French Fries",
        );
    }

    #[test]
    fn test_logger_alert() {
        test_logger(
            LogLevel::EMERGENCY,
            LogLevel::ALERT,
            "ALERT: The Shop Owner Sells French Fries",
        );
    }

    #[test]
    fn test_logger_critical() {
        test_logger(
            LogLevel::EMERGENCY,
            LogLevel::CRITICAL,
            "CRITICAL: The Shop Owner Sells French Fries",
        );
    }

    #[test]
    fn test_logger_error() {
        test_logger(
            LogLevel::EMERGENCY,
            LogLevel::ERROR,
            "ERROR: The Shop Owner Sells French Fries",
        );
    }

    #[test]
    fn test_logger_warning() {
        test_logger(
            LogLevel::EMERGENCY,
            LogLevel::WARNING,
            "WARNING: The Shop Owner Sells French Fries",
        );
    }

    #[test]
    fn test_logger_notification() {
        test_logger(
            LogLevel::EMERGENCY,
            LogLevel::NOTIFICATION,
            "NOTIFICATION: The Shop Owner Sells French Fries",
        );
    }

    #[test]
    fn test_logger_info() {
        test_logger(
            LogLevel::EMERGENCY,
            LogLevel::INFO,
            "INFO: The Shop Owner Sells French Fries",
        );
    }

    #[test]
    fn test_logger_debug() {
        test_logger(
            LogLevel::EMERGENCY,
            LogLevel::DEBUG,
            "DEBUG: The Shop Owner Sells French Fries",
        );
    }
}
