pub mod boxed;
pub mod single;

/// Applies styling to text using markdown-like syntax
///
/// # Syntax
/// - `**text**` for bold
/// - `*text*` for italic
/// - `_text_` for underline
/// - `~text~` for strikethrough
/// - `@text@` for dim
///
/// Styles can be nested and combined.
///
/// # Example
/// ```rust
/// use tlogger::prelude::*;
///
/// let text = style_text!(
///     "This is **bold**, *italic*, and _underlined_.
///     You can also do **bold _with_ *italic* inside**",
///     colors.info_text
/// );
///
/// // Combine multiple styles
/// let status = style_text!(
///     "Status: **_CRITICAL_** - @dimmed details@",
///     colors.error_text
/// );
///
/// // Use in logging
/// info!("Server", "System status is *_important_*");
/// ```
///
/// Note: All styles are automatically stripped when written to log files,
/// preserving only the text content.
#[macro_export]
macro_rules! style_text {
    ($text:expr, $color:expr) => {{
        let do_style = $crate::get_text_styling();
        let text = $text.to_string();

        if *do_style {
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
        } else {
            // If styling is disabled, just remove the style markers
            let mut result = text;

            // Remove bold markers
            while let Some(start) = result.find("**") {
                if let Some(end) = result[start + 2..].find("**") {
                    let before = &result[..start];
                    let content = &result[start + 2..start + 2 + end];
                    let after = &result[start + 2 + end + 2..];
                    result = format!("{}{}{}", before, content, after);
                } else {
                    break;
                }
            }

            // Remove other style markers
            let markers = ["*", "_", "~", "@"];
            for marker in markers {
                while let Some(start) = result.find(marker) {
                    if let Some(end) = result[start + 1..].find(marker) {
                        let before = &result[..start];
                        let content = &result[start + 1..start + 1 + end];
                        let after = &result[start + 1 + end + 1..];
                        result = format!("{}{}{}", before, content, after);
                    } else {
                        break;
                    }
                }
            }

            format!("{}{}{}", $color, result, get_colors().reset)
        }
    }};
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
