use std::sync::OnceLock;

use logger::Logger;
use prelude::strip_ansi_codes;
use styling::*;

pub mod logger;
pub mod opts;
pub mod prelude;
pub mod styling;
mod tests;

/// No log saving is used by default
///
/// This is the global logger, if enabled
/// all logs will be written into a daily log file at the provided path
pub static mut LOGGER: OnceLock<Logger> = OnceLock::new();
/// DEBUG is enabled by default
///
/// If you are ready to ship to production, setting this to false will prevent debug messages from being printed
/// to the console, but will still be logged to a log file.
pub static DEBUG: OnceLock<bool> = OnceLock::new();

/// Text Styling is on by default
///
/// Text styling is the ability to have underlines, bold, italics, etc. in your logs
///
/// # Example usage of a Test Style
/// ```rust
/// fn printme() {
///     info!("Title", "THis message _here has underlines_");
/// }
/// ```
///
/// if toggled to off these styles will be ignored.
pub static mut TEXT_STYLING: OnceLock<bool> = OnceLock::new();

#[inline]
pub fn text_styling_on() {
    unsafe { TEXT_STYLING.set(true).unwrap() };
}

#[inline]
pub fn text_styling_off() {
    unsafe { TEXT_STYLING.set(false).unwrap() };
}

#[inline]
pub fn get_text_styling() -> &'static bool {
    unsafe { TEXT_STYLING.get_or_init(|| true) }
}

/// Get the current timestamp in the format HH:MM:SS.SSS
#[inline]
pub fn get_timestamp() -> String {
    chrono::Local::now().format("%H:%M:%S%.3f").to_string()
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
    mut width: usize,
) -> String {
    let mut result = String::new();

    // Get timestamp
    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
    let timestamp_display = format!("⏳ {}", timestamp);

    // Calculate minimum required width based on title and timestamp
    let minimum_width = title.len() + timestamp_display.len() + symbol.len() + 4;

    // Adjust width if necessary
    if width < minimum_width {
        width = minimum_width;
    }

    // Calculate spaces needed between title and timestamp
    let total_space = width - title.len() - timestamp_display.len() - symbol.len() - 2;

    // Top border with symbol, title and timestamp
    result.push_str(&format!(
        "{}{}{} {} {}{}{}{}{}{}{}{}{}\n", // Added space after left border
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

    // Process message
    let processed_message = style_text!(message, text_color);
    let clean_message = strip_ansi_codes(&processed_message);
    let max_line_width = width.saturating_sub(4); // Account for borders and padding

    // Split message by explicit line breaks first
    for paragraph in clean_message.split('\n') {
        if paragraph.is_empty() {
            // Handle empty lines
            result.push_str(&format!(
                "{}{} {}{}{}\n",
                box_color,
                get_borders().vertical,
                " ".repeat(width.saturating_sub(4)),
                get_borders().vertical,
                get_colors().reset
            ));
            continue;
        }

        // Word wrap within each paragraph
        let mut current_line = String::new();
        let mut current_width = 0;
        let words: Vec<&str> = paragraph.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            let word_width = word.len();

            // If a single word is longer than max_line_width, split it
            if word_width > max_line_width {
                // Print current line if not empty
                if !current_line.is_empty() {
                    let padding = width.saturating_sub(current_width + 4);
                    result.push_str(&format!(
                        "{}{} {}{}{}{}{}{}\n",
                        box_color,
                        get_borders().vertical,
                        style_text!(&current_line, text_color),
                        get_colors().reset,
                        box_color,
                        " ".repeat(padding),
                        get_borders().vertical,
                        get_colors().reset
                    ));
                }

                // Split long word into chunks
                let mut remaining = word.to_string();
                while !remaining.is_empty() {
                    let (chunk, rest) = if remaining.len() > max_line_width {
                        remaining.split_at(max_line_width)
                    } else {
                        (remaining.as_str(), "")
                    };

                    let padding = width.saturating_sub(chunk.len() + 4);
                    result.push_str(&format!(
                        "{}{} {}{}{}{}{}{}\n",
                        box_color,
                        get_borders().vertical,
                        style_text!(chunk, text_color),
                        get_colors().reset,
                        box_color,
                        " ".repeat(padding),
                        get_borders().vertical,
                        get_colors().reset
                    ));

                    remaining = rest.to_string();
                }
                current_line.clear();
                current_width = 0;
                continue;
            }

            let space_width = if current_width == 0 { 0 } else { 1 };

            if current_width + space_width + word_width <= max_line_width {
                // Add word to current line
                if current_width > 0 {
                    current_line.push(' ');
                    current_width += 1;
                }
                current_line.push_str(word);
                current_width += word_width;

                // Print line if it's the last word
                if i == words.len() - 1 {
                    let padding = width.saturating_sub(current_width + 4);
                    result.push_str(&format!(
                        "{}{} {}{}{}{}{}{}\n",
                        box_color,
                        get_borders().vertical,
                        style_text!(&current_line, text_color),
                        get_colors().reset,
                        box_color,
                        " ".repeat(padding),
                        get_borders().vertical,
                        get_colors().reset
                    ));
                }
            } else {
                // Print current line and start new one
                let padding = width.saturating_sub(current_width + 4);
                result.push_str(&format!(
                    "{}{} {}{}{}{}{}{}\n",
                    box_color,
                    get_borders().vertical,
                    style_text!(&current_line, text_color),
                    get_colors().reset,
                    box_color,
                    " ".repeat(padding),
                    get_borders().vertical,
                    get_colors().reset
                ));

                // Start new line with current word
                current_line = word.to_string();
                current_width = word_width;

                // Print the word if it's the last one
                if i == words.len() - 1 {
                    let padding = width.saturating_sub(current_width + 4);
                    result.push_str(&format!(
                        "{}{} {}{}{}{}{}{}\n",
                        box_color,
                        get_borders().vertical,
                        style_text!(&current_line, text_color),
                        get_colors().reset,
                        box_color,
                        " ".repeat(padding),
                        get_borders().vertical,
                        get_colors().reset
                    ));
                }
            }
        }
    }

    // Bottom border
    result.push_str(&format!(
        "{}{} {}{} {}\n", // Added spaces for alignment
        box_color,
        get_borders().bottom_left,
        get_borders().horizontal.repeat(width - 4), // Adjusted width
        get_borders().bottom_right,
        get_colors().reset
    ));

    result
}
