use std::{path::PathBuf, sync::OnceLock};

use logger::{LogInterval, Logger};
use styling::*;

pub mod logger;
pub mod prelude;
pub mod styling;
mod tests;

/// No log saving is used by default
///
/// This is the global logger, if enabled
/// all logs will be written into a daily log file at the provided path
pub static LOGGER: OnceLock<Logger> = OnceLock::new();
/// DEBUG is enabled by default
///
/// If you are ready to ship to production, setting this to false will prevent debug messages from being printed
/// to the console, but will still be logged to a log file.
pub static DEBUG: OnceLock<bool> = OnceLock::new();

/// Initializes the logger
///
/// Logs are generated based on the current day.
/// All logs are stored in the directory specified by `path`.
/// With the name format of `YYYY-MM-DD.log`.
#[inline]
pub fn init_logger<P: Into<PathBuf>>(path: P, log_interval: LogInterval) -> std::io::Result<()> {
    let logger = Logger::new(path, log_interval)?;
    LOGGER.set(logger).unwrap_or(());
    Ok(())
}

/// Converts RGB values to an ANSI escape code
///
/// # Example
/// ```rust
/// let red = rgb_to_ansi(255, 0, 0);       // "\x1b[38;2;255;0;0m"
/// let blue = rgb_to_ansi(0, 0, 255);      // "\x1b[38;2;0;0;255m"
/// let white = rgb_to_ansi(255, 255, 255); // "\x1b[38;2;255;255;255m"
/// ```
#[inline]
pub fn rgb_to_ansi(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

/// Converts RGB values to an ANSI background color escape code
///
/// # Example
/// ```rust
/// let red = rgb_to_ansi_bg(255, 0, 0);       // "\x1b[48;2;255;0;0m"
/// let blue = rgb_to_ansi_bg(0, 0, 255);      // "\x1b[48;2;0;0;255m"
/// let white = rgb_to_ansi_bg(255, 255, 255); // "\x1b[48;2;255;255;255m"
/// ```
#[inline]
pub fn rgb_to_ansi_bg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

/// Creates an ANSI escape code for RGB colors
///
/// # Returns
/// This returns a static string slice.
///
/// If you need just a normal string, see `rgb_to_ansi`
///
/// # Example
/// ```rust
/// println!("{}Colored text{}", ansi_rgb!(255, 0, 0), get_colors().reset);
/// ```
#[macro_export]
macro_rules! ansi_rgb {
    ($r:expr, $g:expr, $b:expr) => {
        concat!("\x1b[38;2;", $r, ";", $g, ";", $b, "m")
    };
}

/// Creates an ANSI background color escape code
///
/// # Returns
/// This returns a static string slice.
///
/// If you need just a normal string, see `rgb_to_ansi_bg`
///
/// # Example
/// ```rust
/// println!("{}Colored text{}", ansi_bg_rgb!(255, 0, 0), get_colors().reset);
/// ```
#[macro_export]
macro_rules! ansi_rgb_bg {
    ($r:expr, $g:expr, $b:expr) => {
        concat!("\x1b[48;2;", $r, ";", $g, ";", $b, "m")
    };
}

/// Debug is enabled by default
///
/// If set to false all `debug` macros will not be printed to the console, but will
/// still be logged to the log file.
#[inline]
pub fn set_debug(debug: bool) {
    DEBUG.set(debug).unwrap_or(());
}

/// Get the current timestamp in the format HH:MM:SS.SSS
#[inline]
pub fn get_timestamp() -> String {
    chrono::Local::now().format("%H:%M:%S%.3f").to_string()
}

/// Set your own colors for formatting
///
/// # Example
///
/// ```rust
/// use t_logger::prelude::*;
///
/// customize_colors(Colors {
///     info: "\x1b[96m",    // Cyan
///     warn: "\x1b[93m",    // Yellow
///     error: "\x1b[91m",   // Red
///     success: "\x1b[92m", // Green
///     debug: "\x1b[95m",   // Magenta
///     ..Default::default()
/// });
/// ```
#[inline]
pub fn customize_colors(colors: Colors) {
    _ = COLORS.set(colors);
}

/// Set your own symbols for formatting
///
/// # Example
///
/// ```rust
/// use t_logger::prelude::*;
///
/// customize_symbols(Symbols {
///     info: "i",
///     warn: "[WARN]",
///     error: "[ERROR]",
///     ..Default::default()
/// });
/// ```
#[inline]
pub fn customize_symbols(symbols: Symbols) {
    _ = SYMBOLS.set(symbols);
}

/// Set your own border characters for formatting
///
/// # Example
///
/// ```rust
/// use t_logger::prelude::*;
///
/// customize_borders(Borders {
///     top_left: "╭",
///     top_right: "╮",
///     bottom_left: "╰",
///     bottom_right: "╯",
///     horizontal: "─",
///     vertical: "│",
/// });
/// ```
#[inline]
pub fn customize_borders(borders: Borders) {
    _ = BORDERS.set(borders);
}

/// Get the current colors
#[inline]
pub fn get_colors() -> &'static Colors {
    COLORS.get_or_init(|| Colors::default())
}

/// Get the current symbols
#[inline]
pub fn get_symbols() -> &'static Symbols {
    SYMBOLS.get_or_init(|| Symbols::default())
}

/// Get the current border characters
#[inline]
pub fn get_borders() -> &'static Borders {
    BORDERS.get_or_init(|| Borders::default())
}

/// Creates a perfectly formatted box with the given title and message
pub fn create_styled_box(
    box_color: &str,
    text_color: &str,
    symbol: &str,
    title: &str,
    message: &str,
    width: usize,
) -> String {
    let message_lines: Vec<&str> = message.lines().collect();
    let mut result = String::new();

    // Get timestamp
    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
    let timestamp_display = format!("⏳ {}", timestamp);

    // Calculate spaces needed between title and timestamp
    let total_space = width - title.len() - timestamp_display.len() - symbol.len() - 1;

    // Top border with symbol, title and timestamp
    result.push_str(&format!(
        "{}{}{}─{} {}{}{}{}{}{}{}{}{}\n",
        box_color,
        get_borders().top_left,
        get_colors().bold,
        symbol,
        title,
        box_color,
        get_borders().horizontal.repeat(total_space),
        get_colors().dim,
        timestamp_display,
        get_colors().reset,
        box_color,
        get_borders().top_right,
        get_colors().reset
    ));

    // Message lines - preserve original formatting and add padding
    for line in message_lines {
        let processed_line = style_text!(line, text_color);
        let clean_processed = strip_ansi_codes(&processed_line);
        let padding = width - clean_processed.len() - 3; // Adjusted padding by 1

        result.push_str(&format!(
            "{}{} {}{}{}{}{}{}\n",
            box_color,
            get_borders().vertical,
            processed_line,
            get_colors().reset,
            box_color, // Reset to box color before padding and border
            " ".repeat(padding),
            get_borders().vertical,
            get_colors().reset,
        ));
    }

    // Bottom border
    result.push_str(&format!(
        "{}{}{}{}{}\n",
        box_color,
        get_borders().bottom_left,
        get_borders().horizontal.repeat(width - 2),
        get_borders().bottom_right,
        get_colors().reset
    ));

    result
}

/// Strips ANSI escape codes from a string
pub fn strip_ansi_codes(s: &str) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

/// Creates a Cyan box with the given title and message
#[macro_export]
macro_rules! info_box {
    ($title:expr, $($arg:tt)*) => {
        print!("{}", $crate::create_styled_box(
            $crate::get_colors().info,
            $crate::get_colors().info_text,
            $crate::get_symbols().info,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::get_colors().info,
            $crate::get_symbols().info,
            $title,
            $crate::get_colors().info_text,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

/// Creates a Yellow box with the given title and message
#[macro_export]
macro_rules! warn_box {
    ($title:expr, $($arg:tt)*) => {
        print!("{}", $crate::create_styled_box(
            $crate::get_colors().warn,
            $crate::get_colors().warn_text,
            $crate::get_symbols().warn,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::get_colors().warn,
            $crate::get_symbols().warn,
            $title,
            $crate::get_colors().warn_text,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

/// Creates a Red box with the given title and message
#[macro_export]
macro_rules! error_box {
    ($title:expr, $($arg:tt)*) => {
        eprint!("{}", $crate::create_styled_box(
            $crate::get_colors().error,
            $crate::get_colors().error_text,
            $crate::get_symbols().error,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::get_colors().error,
            $crate::get_symbols().error,
            $title,
            $crate::get_colors().error_text,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

/// Creates a Green box with the given title and message
#[macro_export]
macro_rules! success_box {
    ($title:expr, $($arg:tt)*) => {
        print!("{}", $crate::create_styled_box(
            $crate::get_colors().success,
            $crate::get_colors().success_text,
            $crate::get_symbols().success,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::get_colors().success,
            $crate::get_symbols().success,
            $title,
            $crate::get_colors().success,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

/// Creates a Magenta box with the given title and message
#[macro_export]
macro_rules! debug_box {
    ($title:expr, $($arg:tt)*) => {
        if *$crate::DEBUG.get().unwrap_or(&true) {
            print!("{}", $crate::create_styled_box(
                $crate::get_colors().debug,
                $crate::get_colors().debug_text,
                $crate::get_symbols().debug,
                $title,
                &format!($($arg)*),
                75
            ));
        }

        let log = make_log!(
            $crate::get_colors().debug,
            $crate::get_symbols().debug,
            $title,
            $crate::get_colors().debug_text,
            $($arg)*
        );

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&log) {
                eprintln!("Error logging to file: {e}");
            }
        }
    };
}

/// Creates a log message with the given title, symbol, and message
#[macro_export]
macro_rules! make_log {
    ($color:expr, $symbol:expr, $title:expr, $textcolor:expr, $($arg:tt)*) => {{
        let message = format!($($arg)*);
        let processed_message = style_text!(message, $textcolor);

        format!(
            "{}{} {} {}{}{}{} {} {}{}{}{}{} {}",
            $color,
            $symbol,
            $crate::get_colors().reset,
            $crate::get_colors().dim,
            $crate::get_timestamp(),
            $crate::get_colors().reset,
            $crate::get_colors().dim,
            $crate::get_symbols().separator,
            $crate::get_colors().reset,
            $crate::get_colors().bold,
            $color,
            $title,
            $crate::get_colors().reset,
            processed_message
        )
    }};
}

#[macro_export]
macro_rules! style_text {
    ($text:expr, $color:expr) => {{
        let text = $text.to_string();
        let mut result = text.clone();

        // Bold (process first as it uses double markers)
        while let Some(start) = result.find("**") {
            if let Some(end) = result[start + 2..].find("**") {
                let before = &result[..start];
                let content = &result[start + 2..start + 2 + end];
                let after = &result[start + 2 + end + 2..];
                result = format!("{}{}\x1b[1m{}\x1b[22m{}", before, $color, content, after);
            } else {
                break;
            }
        }

        // Process other styles
        let styles = [
            ("*", "\x1b[3m", "\x1b[23m"), // Italic
            ("_", "\x1b[4m", "\x1b[24m"), // Underline
            ("~", "\x1b[9m", "\x1b[29m"), // Strikethrough
            ("@", "\x1b[2m", "\x1b[22m"), // Dim
        ];

        for (marker, start_code, end_code) in styles {
            while let Some(start) = result.find(marker) {
                if let Some(end) = result[start + 1..].find(marker) {
                    let before = &result[..start];
                    let content = &result[start + 1..start + 1 + end];
                    let after = &result[start + 1 + end + 1..];
                    result = format!(
                        "{}{}{}{}{}{}",
                        before, $color, start_code, content, end_code, after
                    );
                } else {
                    break;
                }
            }
        }

        format!("{}{}{}", $color, result, get_colors().reset)
    }};
}

/// Creates a single line message with a symbol timestamp, title, and message
#[macro_export]
macro_rules! info {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::get_colors().info,
            $crate::get_symbols().info,
            $title,
            $crate::get_colors().info_text,
            $($arg)*
        );
        println!("{msg}");

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&msg) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}

/// Creates a single line message with a symbol timestamp, title, and message
#[macro_export]
macro_rules! warn {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::get_colors().warn,
            $crate::get_symbols().warn,
            $title,
            $crate::get_colors().warn_text,
            $($arg)*
        );
        println!("{msg}");

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&msg) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}
/// Creates a single line message with a symbol timestamp, title, and message
#[macro_export]
macro_rules! error {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::get_colors().error,
            $crate::get_symbols().error,
            $title,
            $crate::get_colors().error_text,
            $($arg)*
        );
        eprintln!("{msg}");

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&msg) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}
/// Creates a single line message with a symbol timestamp, title, and message
#[macro_export]
macro_rules! success {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::get_colors().success,
            $crate::get_symbols().success,
            $title,
            $crate::get_colors().success_text,
            $($arg)*
        );
        println!("{msg}");

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&msg) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}

/// Creates a single line message with a symbol timestamp, title, and message
#[macro_export]
macro_rules! debug {
    ($title:expr, $($arg:tt)*) => {{
        let msg = make_log!(
            $crate::get_colors().debug,
            $crate::get_symbols().debug,
            $title,
            $crate::get_colors().debug_text,
            $($arg)*
        );
        if *$crate::DEBUG.get().unwrap_or(&true) {
            println!("{msg}");
        }

        if let Some(logger) = $crate::LOGGER.get() {
            if let Err(e) = logger.log(&msg) {
                eprintln!("Error logging to file: {e}");
            }
        }
    }};
}
