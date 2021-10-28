// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    // Note: A clever solution = https://exercism.org/tracks/rust/exercises/semi-structured-logs/solutions/13W
    match level {
        LogLevel::Info => {
            String::from(format!("[INFO]: {}", message))
        },
        LogLevel::Error => {
            String::from(format!("[ERROR]: {}", message))
        },
        LogLevel::Warning => {
            String::from(format!("[WARNING]: {}", message))
        },
    }
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
