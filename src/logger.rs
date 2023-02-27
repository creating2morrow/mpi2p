use crate::args;
use chrono;
use clap::Parser;

#[derive(Debug, Clone)]
enum LogLevel {
    DEBUG,
    ERROR,
    INFO,
    WARN,
}

impl LogLevel {
    pub fn value(&self) -> String {
        match *self {
            LogLevel::DEBUG => String::from("DEBUG"),
            LogLevel::ERROR => String::from("ERROR"),
            LogLevel::INFO => String::from("INFO"),
            LogLevel::WARN => String::from("DEBUG"),
        }
    }
}

/// Log, log better than bad its good
pub struct Log{}

impl Log {
    /// Log a message for debugging.
    /// This should be removed after fixing the issue.
    pub async fn debug(msg: &str) -> () {
        Self::log(LogLevel::DEBUG, msg).await;
    }
    /// Log a message for error.
    /// This should be used in every async fn when something goes wrong.
    pub async fn error(msg: &str) -> () {
        Self::log(LogLevel::ERROR, msg).await;
    }
    /// Log a message for user information.
    /// This should be used in every async fn.
    pub async fn info(msg: &str) -> () {
        Self::log(LogLevel::INFO, msg).await;
    }
    /// Log a message for user information.
    /// This should be used sparingly when something is off but not broken.
    pub async fn warn(msg: &str) -> () {
        Self::log(LogLevel::WARN, msg).await;
    }
    /// Base for logging
    async fn log(level: LogLevel, msg: &str) -> () {
        let args = args::Args::parse();
        let set_level = args.log_level.split(",");
        let vec: Vec<String> = set_level.map(|s| String::from(s)).collect();
        if vec.contains(&level.value()) {
            println!(
                "{}",
                format!(
                    "|{:?}\t|\t|{:?}| => {}",
                    level,
                    chrono::offset::Utc::now(),
                    msg
                )
            );
        }
    }
}
