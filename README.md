# Tlogger

A versatile and stylish logging library for Rust applications that provides both console output and file logging capabilities with customization options.

## Features

- 📝 Multiple log levels (info, warn, error, success, debug)
- 🎨 Full RGB color support with true color ANSI codes
- 📦 Box-style formatted messages
- 📅 Configurable log file intervals (hourly, every 3 hours, every 6 hours, ..., Daily)
- 🔍 Debug mode toggle
- ⏰ Timestamp integration
- 💾 File logging with clean (ANSI-stripped) output
- 🎯 Fully customizable styling (colors, symbols, borders)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tlogger = "0.1.3"
```

Or use the `cargo add` command:

```bash
cargo add tlogger
```

## Usage

```rust
use tlogger::prelude::*;

fn main() {
    // Initialize logger with hourly log files
    init_logger("logs", LogInterval::OneHour).unwrap();
    // Or use other intervals: ThreeHour, SixHour, NineHour, TwelveHour, OneDay

    customize_colors(Colors {
        info: ansi_rgb!(32, 80, 123),
        debug: ansi_rgb!(60, 200, 30),
        ..Default::default()
    });

    customize_symbols(Symbols {
        debug: "⟐",
        ..Default::default()
    });

    customize_borders(Borders {
        ..Default::default()
    });

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
```

## Log Levels

- `info!()` / `info_box!()` - Cyan colored information messages
- `warn!()` / `warn_box!()` - Yellow colored warning messages
- `error!()` / `error_box!()` - Red colored error messages
- `success!()` / `success_box!()` - Green colored success messages
- `debug!()` / `debug_box!()` - Magenta colored debug messages

## Log Intervals

Configure how frequently new log files are created:
- `LogInterval::OneHour` - New file every hour (e.g., `2024-02-20-14h-15h.log`)
- `LogInterval::ThreeHour` - Every 3 hours (e.g., `2024-02-20-12h-15h.log`)
- `LogInterval::SixHour` - Every 6 hours (e.g., `2024-02-20-12h-18h.log`)
- `LogInterval::NineHour` - Every 9 hours (e.g., `2024-02-20-09h-18h.log`)
- `LogInterval::TwelveHour` - Every 12 hours (e.g., `2024-02-20-12h-00h.log`)
- `LogInterval::OneDay` - One file per day (e.g., `2024-02-20-00h-24h.log`)

## Debug Mode

Debug messages can be disabled in production while still being logged to file:
```rust
set_debug(false);  // Disables console output for debug messages
```

## File Logging

When initialized, logs are automatically saved to files based on the specified interval. All ANSI color codes are automatically stripped from the file output for better readability.

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Example Output

```
ℹ 12:34:56.789 │ Server Starting up...
✔ 12:34:56.790 │ Login User Alice connected
⚠ 12:34:56.791 │ Memory Usage at 85%
✖ 12:34:56.792 │ Database Connection failed
```

Box Style Output:
```
╭─✔Login───────────────────────────────────────────────────────⏳ 12:29:47╮
│ User Alice connected                                                    │
╰─────────────────────────────────────────────────────────────────────────╯
```
