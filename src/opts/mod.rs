use std::path::PathBuf;

use crate::{
    logger::{LogInterval, Logger},
    DEBUG, LOGGER,
};

pub mod customize;
pub mod messages;

/// Initializes the logger
///
/// Logs are generated based on the current day.
/// All logs are stored in the directory specified by `path`.
/// With the name format of `YYYY-MM-DD.log`.
#[inline]
pub fn init_logger<P: Into<PathBuf>>(path: P, log_interval: LogInterval) -> std::io::Result<()> {
    unsafe {
        let logger = Logger::new(path, log_interval)?;
        LOGGER.set(logger).unwrap_or(());
    }
    Ok(())
}

/// Debug is enabled by default
///
/// If set to false all `debug` macros will not be printed to the console, but will
/// still be logged to the log file.
#[inline]
pub fn set_debug(debug: bool) {
    DEBUG.set(debug).unwrap_or(());
}

/// Clears the list of Log types to save to log files
///
/// By default all log types are saved to log files.
///
/// This is useful if you want to only save certain types of logs to file
/// Like not saving success logs for example as well they have succeeded.
#[inline]
pub fn clear_log_levels() {
    unsafe {
        _ = LOGGER.get_mut().unwrap().clear_log_levels();
    }
}

/// Adds log types to the list of log types to save to log files
///
/// Only save the logs of the types you need!
///
/// # Example
/// ```rust
/// use t_logger::prelude::*;
///
/// init_logger("Logs", LogInterval::OneHour).unwrap();
/// clear_log_levels(); // Want to clear always, because by default all log types are saved
/// add_log_levels!(LogLevel::Debug, LogLevel::Warn, LogLevel::Error);
/// ```
#[macro_export]
macro_rules! add_log_levels {
    ($($level:expr),+ $(,)?) => {
        unsafe {
            $(
                _ = LOGGER.get_mut().unwrap().add_log_level($level);
            )+
        }
    };
}
