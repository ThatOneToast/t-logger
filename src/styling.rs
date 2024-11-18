use std::sync::OnceLock;

/// Colors using ANSI escape codes
pub struct Colors {
    pub info: &'static str,
    pub info_text: &'static str,
    pub warn: &'static str,
    pub warn_text: &'static str,
    pub error: &'static str,
    pub error_text: &'static str,
    pub success: &'static str,
    pub success_text: &'static str,
    pub debug: &'static str,
    pub debug_text: &'static str,
    pub dim: &'static str,
    pub bold: &'static str,
    pub reset: &'static str,
}

/// Symbols used in logging
pub struct Symbols {
    pub info: &'static str,
    pub warn: &'static str,
    pub error: &'static str,
    pub success: &'static str,
    pub debug: &'static str,
    pub separator: &'static str,
    pub bullet: &'static str,
}

/// Border characters for optional boxing
pub struct Borders {
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub horizontal: &'static str,
    pub vertical: &'static str,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            info: "\x1b[96m",         // Bright Cyan
            info_text: "\x1b[36m",    // Dark Cyan
            warn: "\x1b[93m",         // Bright Yellow
            warn_text: "\x1b[33m",    // Dark Yellow
            error: "\x1b[91m",        // Bright Red
            error_text: "\x1b[31m",   // Dark Red
            success: "\x1b[92m",      // Bright Green
            success_text: "\x1b[32m", // Dark Green
            debug: "\x1b[95m",        // Bright Magenta
            debug_text: "\x1b[35m",   // Dark Magenta
            dim: "\x1b[2m",           // Dimmed
            bold: "\x1b[1m",          // Bold
            reset: "\x1b[0m",         // Reset
        }
    }
}

impl Default for Symbols {
    fn default() -> Self {
        Self {
            info: "ℹ",
            warn: "⚠",
            error: "✖",
            success: "✔",
            debug: "⁂",
            separator: "│",
            bullet: "•",
        }
    }
}

impl Default for Borders {
    fn default() -> Self {
        Self {
            top_left: "╭",
            top_right: "╮",
            bottom_left: "╰",
            bottom_right: "╯",
            horizontal: "─",
            vertical: "│",
        }
    }
}

pub static COLORS: OnceLock<Colors> = OnceLock::new();
pub static SYMBOLS: OnceLock<Symbols> = OnceLock::new();
pub static BORDERS: OnceLock<Borders> = OnceLock::new();
