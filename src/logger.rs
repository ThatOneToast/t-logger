use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use chrono::{DateTime, Local, Timelike};

use crate::strip_ansi_codes;

pub enum LogInterval {
    OneHour,
    ThreeHour,
    SixHour,
    NineHour,
    TwelveHour,
    OneDay,
}

impl LogInterval {
    fn get_file_timestamp(&self, now: DateTime<Local>) -> String {
        match self {
            LogInterval::OneHour => {
                let start_hour = now.hour();
                let end_hour = (start_hour + 1) % 24;
                format!(
                    "{}-{:02}h-{:02}h",
                    now.format("%Y-%m-%d"),
                    start_hour,
                    end_hour
                )
            }
            LogInterval::ThreeHour => {
                let start_hour = (now.hour() / 3) * 3;
                let end_hour = (start_hour + 3) % 24;
                format!(
                    "{}-{:02}h-{:02}h",
                    now.format("%Y-%m-%d"),
                    start_hour,
                    end_hour
                )
            }
            LogInterval::SixHour => {
                let start_hour = (now.hour() / 6) * 6;
                let end_hour = (start_hour + 6) % 24;
                format!(
                    "{}-{:02}h-{:02}h",
                    now.format("%Y-%m-%d"),
                    start_hour,
                    end_hour
                )
            }
            LogInterval::NineHour => {
                let start_hour = (now.hour() / 9) * 9;
                let end_hour = (start_hour + 9) % 24;
                format!(
                    "{}-{:02}h-{:02}h",
                    now.format("%Y-%m-%d"),
                    start_hour,
                    end_hour
                )
            }
            LogInterval::TwelveHour => {
                let start_hour = (now.hour() / 12) * 12;
                let end_hour = (start_hour + 12) % 24;
                format!(
                    "{}-{:02}h-{:02}h",
                    now.format("%Y-%m-%d"),
                    start_hour,
                    end_hour
                )
            }
            LogInterval::OneDay => {
                format!("{}-00h-24h", now.format("%Y-%m-%d"))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Success,
}

impl LogLevel {
    // Helper function to extract log level from message
    fn from_message(message: &str) -> Option<LogLevel> {
        if message.contains("ℹ") {
            Some(LogLevel::Info)
        } else if message.contains("⚠") {
            Some(LogLevel::Warn)
        } else if message.contains("✖") {
            Some(LogLevel::Error)
        } else if message.contains("✔") {
            Some(LogLevel::Success)
        } else if message.contains("⁂") {
            Some(LogLevel::Debug)
        } else {
            None
        }
    }
}

pub struct Logger {
    base_path: PathBuf,
    log_interval: LogInterval,
    log_levels: Vec<LogLevel>,
}

impl Logger {
    pub fn new<P: Into<PathBuf>>(base_path: P, log_interval: LogInterval) -> std::io::Result<Self> {
        let base_path = base_path.into();
        fs::create_dir_all(&base_path)?;
        Ok(Logger {
            base_path,
            log_interval,
            log_levels: vec![
                LogLevel::Debug,
                LogLevel::Info,
                LogLevel::Warn,
                LogLevel::Error,
                LogLevel::Success,
            ],
        })
    }

    /// Clear all log levels
    pub fn clear_log_levels(&mut self) {
        self.log_levels.clear();
    }

    /// Add the types of logs to be saved to log files.
    pub fn add_log_level(&mut self, log_level: LogLevel) {
        self.log_levels.push(log_level);
    }

    fn get_log_file(&self) -> std::io::Result<File> {
        let now = Local::now();
        let timestamp = self.log_interval.get_file_timestamp(now);
        let file_name = format!("{}.log", timestamp);
        let file_path = self.base_path.join(file_name);

        OpenOptions::new().create(true).append(true).open(file_path)
    }

    pub fn log(&self, message: &str) -> std::io::Result<()> {
        // Check if this message's log level is in our allowed levels
        if let Some(level) = LogLevel::from_message(message) {
            if self.log_levels.contains(&level) {
                let mut file = self.get_log_file()?;
                let cleaned_message = strip_ansi_codes(message);
                file.write_all(cleaned_message.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        Ok(())
    }
}
