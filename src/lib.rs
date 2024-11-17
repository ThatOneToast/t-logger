use std::{path::PathBuf, sync::OnceLock};

use logger::Logger;
use styling::*;

pub mod logger;
pub mod prelude;
mod styling;
mod tests;

/// Default configurations
pub const COLORS: Colors = Colors {
    info: "\x1b[96m",    // Cyan
    warn: "\x1b[93m",    // Yellow
    error: "\x1b[91m",   // Red
    success: "\x1b[92m", // Green
    debug: "\x1b[95m",   // Magenta
    dim: "\x1b[2m",      // Dimmed
    bold: "\x1b[1m",     // Bold
    reset: "\x1b[0m",    // Reset
};

pub const SYMBOLS: Symbols = Symbols {
    info: "ℹ",
    warn: "⚠",
    error: "✖",
    success: "✔",
    debug: "⁂",
    separator: "│",
    bullet: "•",
};

pub const BORDERS: Borders = Borders {
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    horizontal: "─",
    vertical: "│",
};

pub static LOGGER: OnceLock<Logger> = OnceLock::new();
pub static DEBUG: OnceLock<bool> = OnceLock::new();

/// Initializes the logger
///
/// Logs are generated based on the current day.
/// All logs are stored in the directory specified by `path`.
/// With the name format of `YYYY-MM-DD.log`.
pub fn init_logger<P: Into<PathBuf>>(path: P) -> std::io::Result<()> {
    let logger = Logger::new(path)?;
    LOGGER.set(logger).unwrap_or(());
    Ok(())
}

/// Debug is enabled by default
///
/// If set to false all `debug` macros will not be printed to the console, but will
/// still be logged to the log file.
pub fn set_debug(debug: bool) {
    DEBUG.set(debug).unwrap_or(());
}

#[inline]
pub fn get_timestamp() -> String {
    chrono::Local::now().format("%H:%M:%S%.3f").to_string()
}

pub fn create_styled_box(
    color: &str,
    symbol: &str,
    title: &str,
    message: &str,
    width: usize,
) -> String {
    fn wrap_text(text: &str, width: usize) -> String {
        let mut wrapped = String::new();
        let mut line_length = 0;
        let mut first_word = true;

        for word in text.split_whitespace() {
            let word_length = word.len();
            if line_length + word_length + (!first_word as usize) > width - 4 {
                wrapped.push('\n');
                line_length = 0;
                first_word = true;
            }
            if !first_word {
                wrapped.push(' ');
                line_length += 1;
            }
            wrapped.push_str(word);
            line_length += word_length;
            first_word = false;
        }
        wrapped
    }

    let wrapped_message = wrap_text(message, width);
    let message_lines: Vec<&str> = wrapped_message.lines().collect();
    let mut result = String::new();

    // Get timestamp
    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
    let timestamp_display = format!("⏳ {}", timestamp);

    // Calculate spaces needed between title and timestamp
    let total_space = width - title.len() - timestamp_display.len() - symbol.len() - 1; // Adjusted for symbol and corners

    // Top border with symbol, title and timestamp
    result.push_str(&format!(
        "{}{}{}─{} {}{}{}{}{}{}{}{}\n",
        color,
        BORDERS.top_left,
        COLORS.bold,
        symbol,
        title,
        color,
        BORDERS.horizontal.repeat(total_space),
        COLORS.dim,
        timestamp_display,
        COLORS.reset,
        color,
        BORDERS.top_right
    ));

    // Message lines
    for line in message_lines {
        result.push_str(&format!(
            "{}{} {:<width$} {}{}\n",
            color,
            BORDERS.vertical,
            line,
            BORDERS.vertical,
            COLORS.reset,
            width = width - 4
        ));
    }

    // Bottom border
    result.push_str(&format!(
        "{}{}{}{}{}\n",
        color,
        BORDERS.bottom_left,
        BORDERS.horizontal.repeat(width - 2),
        BORDERS.bottom_right,
        COLORS.reset
    ));

    result
}

pub fn create_box(title: &str, message: &str, width: usize) -> String {
    let mut result = String::new();
    let message_lines: Vec<&str> = message.lines().collect();
    let max_width = width.max(message_lines.iter().map(|l| l.len()).max().unwrap_or(0) + 2);

    // Top border with title
    result.push_str(&format!("{}{}", BORDERS.top_left, BORDERS.horizontal));
    result.push_str(title);
    result.push_str(BORDERS.horizontal);
    let remaining_width = max_width - title.len() - 1;
    result.push_str(&format!(
        "{}{}\n",
        BORDERS.horizontal.repeat(remaining_width),
        BORDERS.top_right
    ));

    // Message lines
    for line in message_lines {
        result.push_str(&format!(
            "{} {:<width$} {}\n",
            BORDERS.vertical,
            line,
            BORDERS.vertical,
            width = max_width - 2
        ));
    }

    // Bottom border
    result.push_str(&format!(
        "{}{}{}\n",
        BORDERS.bottom_left,
        BORDERS.horizontal.repeat(max_width),
        BORDERS.bottom_right
    ));

    result
}

#[allow(dead_code)]
pub fn strip_ansi_codes(s: &str) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

#[macro_export]
macro_rules! info_box {
    ($title:expr, $($arg:tt)*) => {
        print!("{}", $crate::create_styled_box(
            $crate::COLORS.info,
            $crate::SYMBOLS.info,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::COLORS.debug,
            $crate::SYMBOLS.debug,
            $title,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&log);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

#[macro_export]
macro_rules! warn_box {
    ($title:expr, $($arg:tt)*) => {
        print!("{}", $crate::create_styled_box(
            $crate::COLORS.warn,
            $crate::SYMBOLS.warn,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::COLORS.debug,
            $crate::SYMBOLS.debug,
            $title,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&log);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

#[macro_export]
macro_rules! error_box {
    ($title:expr, $($arg:tt)*) => {
        eprint!("{}", $crate::create_styled_box(
            $crate::COLORS.error,
            $crate::SYMBOLS.error,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::COLORS.debug,
            $crate::SYMBOLS.debug,
            $title,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&log);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

#[macro_export]
macro_rules! success_box {
    ($title:expr, $($arg:tt)*) => {
        print!("{}", $crate::create_styled_box(
            $crate::COLORS.success,
            $crate::SYMBOLS.success,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::COLORS.debug,
            $crate::SYMBOLS.debug,
            $title,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&log);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

#[macro_export]
macro_rules! debug_box {
    ($title:expr, $($arg:tt)*) => {
        if *$crate::DEBUG.get().unwrap_or(&true) {
            print!("{}", $crate::create_styled_box(
                $crate::COLORS.debug,
                $crate::SYMBOLS.debug,
                $title,
                &format!($($arg)*),
                75
            ));
        }
        
        let log = make_log!(
            $crate::COLORS.debug,
            $crate::SYMBOLS.debug,
            $title,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&log);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

#[macro_export]
macro_rules! make_log {
    ($color:expr, $symbol:expr, $title:expr, $($arg:tt)*) => {{
        format!(
            "{}{} {} {}{}{}{} {} {}{}{}{} {}",
            $color,
            $symbol,
            $crate::COLORS.reset,
            $crate::COLORS.dim,
            $crate::get_timestamp(),
            $crate::COLORS.reset,
            $crate::COLORS.dim,
            $crate::SYMBOLS.separator,
            $crate::COLORS.reset,
            $crate::COLORS.bold,
            $title,
            $crate::COLORS.reset,
            format!($($arg)*)
        )
    }};
}

#[macro_export]
macro_rules! info {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::COLORS.info,
            $crate::SYMBOLS.info,
            $title,
            $($arg)*
        );
        println!("{msg}");

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&msg);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}

#[macro_export]
macro_rules! warn {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::COLORS.warn,
            $crate::SYMBOLS.warn,
            $title,
            $($arg)*
        );
        println!("{msg}");

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&msg);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}

#[macro_export]
macro_rules! error {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::COLORS.error,
            $crate::SYMBOLS.error,
            $title,
            $($arg)*
        );
        eprintln!("{msg}");

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&msg);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}

#[macro_export]
macro_rules! success {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::COLORS.success,
            $crate::SYMBOLS.success,
            $title,
            $($arg)*
        );
        println!("{msg}");

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&msg);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}

#[macro_export]
macro_rules! debug {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::COLORS.debug,
            $crate::SYMBOLS.debug,
            $title,
            $($arg)*
        );
        if *$crate::DEBUG.get().unwrap_or(&true) {
            println!("{msg}");
        }

        if let Some(logger) = $crate::LOGGER.get() {
            let clean_log = $crate::strip_ansi_codes(&msg);
            if let Err(e) = logger.log(&clean_log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}
