#[cfg(test)]
#[test]
pub fn info() {
    use super::*;
    
    init_logger("Logs", LogInterval::OneHour).unwrap();

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
    success!("Login", "User {} connected", "Alice");
    debug!("Processing", "Items in queue: {}", 42);
    warn!("Memory", "Usage at {}%", 85);
    error!("Database", "Connection failed");

    info_box!("System", "Your super secure super system is starting up.");
    warn_box!("Memory", "Memory usage is at {}%", 85);
    error_box!("Database", "Database connection failed");
    success_box!("Login", "User {} connected", "Alice");
    debug_box!("Processing", "Items in queue: {}", 42);
}
