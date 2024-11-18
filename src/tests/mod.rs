#[cfg(test)]
#[test]
pub fn info() {
    use super::*;

    init_logger("Logs", LogInterval::OneHour).unwrap();
    clear_log_levels();
    add_log_levels!(LogLevel::Debug, LogLevel::Warn, LogLevel::Error);

    customize_colors(Colors {
        // info_text: crate::ansi_rgb!(32, 80, 123),
        // debug: crate::ansi_rgb!(60, 200, 30),
        warn_text: crate::ansi_rgb!(0, 255, 0),
        ..Default::default()
    });
    //
    // customize_symbols(Symbols {
    // debug: "⟐",
    // ..Default::default()
    // });
    //
    // customize_borders(Borders {
    // ..Default::default()
    // });

    info!("Server", "Starting");
    success!("Login", "User _{}_ connected", "Alice");
    debug!("Processing", "Items in queue: {}", 42);
    warn!("Memory", "Usage at {}%", 85);
    error!("Database", "* **Connection _failed_***");

    info_box!(
        "System",
        "**Your _super secure super system_ is starting up.**"
    );
    warn_box!("Memory", "Memory usage is at **{}%**", 85);
    error_box!("Database", "Database connection failed");
    success_box!("Login", "User {} connected", "Alice");
    debug_box!("Processing", "*_Items in queue:_* **{}**", 42);

    info!("Styles", "Here are all available styles:");
    info!("Bold", "**bold text**");
    info!("Italic", "*italic text*");
    info!("Underline", "_underlined text_");
    info!("Strikethrough", "~strikethrough text~");
    info!("Dim", "@dimmed text@");

    // Styles can be nested and combined
    info!("Combined", "**Bold _and underlined_ text**");
    info!("Mixed", "*Italic with **bold** and _underlined_ parts*");
    info!(
        "Complex",
        "**Bold *italic* _underlined_ ~strikethrough~ !blinking!**"
    );
}
