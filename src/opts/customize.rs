use crate::{Borders, Colors, Symbols, BORDERS, COLORS, SYMBOLS};

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

/// Strips ANSI escape codes from a string
pub fn strip_ansi_codes(s: &str) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
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
