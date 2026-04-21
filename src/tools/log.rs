use crate::LogLevel;

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    pub fn alert(&self, message: &str) {
        self.log(message, LogLevel::ALERT);
    }

    pub fn warning(&self, message: &str) {
        self.log(message, LogLevel::WARNING);
    }

    pub fn info(&self, message: &str) {
        self.log(message, LogLevel::INFO);
    }

    pub fn debug(&self, message: &str) {
        self.log(message, LogLevel::DEBUG);
    }

    fn log(&self, message: &str, level: LogLevel) {
        if self.level > level {
            println!("{}: {}", level, message);
        }
    }
}
