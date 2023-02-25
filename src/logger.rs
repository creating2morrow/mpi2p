use crate::args;
use chrono;
use clap::Parser;

#[derive(Debug, Clone)]
pub enum LogLevel {
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

pub async fn log(level: LogLevel, msg: &str) -> () {
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
