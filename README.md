# Tlogger

A versatile and stylish logging library for Rust applications that provides both console output and file logging capabilities with customization options.

## Features

- 📝 Multiple log levels with selective file logging
  - Info, Warn, Error, Success, Debug levels
  - Control which levels get saved to files
  - Console output for all levels regardless of file settings

- 🎨 Rich text formatting
  - Full RGB color support with true color ANSI codes
  - Text styling (bold, italic, underline, strikethrough, dim)
  - Nested and combined styles support

- 📦 Flexible output formats
  - Single-line logging with timestamps
  - Box-style messages with titles
  - Clean file output (automatically strips ANSI codes)

- ⏰ Time-based file management
  - Configurable log rotation intervals
  - Hourly, 3-hour, 6-hour, 9-hour, 12-hour, or daily files
  - Automatic file creation and naming

- 🎯 Extensive customization options
  - Custom colors for each log level
  - Customizable symbols and borders
  - Separate colors for titles and message text

- 🔍 Development features
  - Debug mode toggle for production/development
  - Millisecond-precision timestamps
  - Error handling for file operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tlogger = "0.1.4"
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
## Log Management

### Log Intervals
Configure how frequently new log files are created:
```rust
init_logger("logs", LogInterval::OneHour).unwrap();
```

Available intervals:
- `LogInterval::OneHour` - New file every hour (e.g., `2024-02-20-14h-15h.log`)
- `LogInterval::ThreeHour` - Every 3 hours (e.g., `2024-02-20-12h-15h.log`)
- `LogInterval::SixHour` - Every 6 hours (e.g., `2024-02-20-12h-18h.log`)
- `LogInterval::NineHour` - Every 9 hours (e.g., `2024-02-20-09h-18h.log`)
- `LogInterval::TwelveHour` - Every 12 hours (e.g., `2024-02-20-12h-00h.log`)
- `LogInterval::OneDay` - One file per day (e.g., `2024-02-20-00h-24h.log`)

### Log Level Filtering
Control which types of logs are saved to files:
```rust
use tlogger::prelude::*;

// Initialize logger
init_logger("logs", LogInterval::OneHour).unwrap();

// Clear default log levels (by default all levels are saved)
clear_log_levels();

// Add only the log levels you want to save
add_log_levels!(
    LogLevel::Debug,
    LogLevel::Warn,
    LogLevel::Error
);

// Now only Debug, Warn, and Error logs will be saved to files
// Info and Success logs will still show in console but won't be saved
```

## Styling Options

### Colors
Change the colors your logs use.
All log types `(info, warn, error, success, debug)` will change the color of boxes in box style outputs,
or will change the title of single line outputs. The log types with `_text` suffix (e.g., `info_text`) will change all message text.

```rust
use tlogger::prelude::*;

customize_colors(Colors {
    info: ansi_rgb!(32, 80, 123),
    info_text: ansi_rgb!(60, 200, 30),
    ..Default::default()
});
```

### Text Styling
You can style your text using simple markdown-like syntax:
- `**text**` for bold text
- `*text*` for italic text
- `_text_` for underlined text
- `~text~` for strikethrough text
- `@text@` for dimmed text

These styles can be nested and combined:
```rust
// Single styles
info!("Server", "This text is **bold**");
info!("Server", "This text is *italic*");
info!("Server", "This text is _underlined_");
info!("Server", "This text is ~strikethrough~");
info!("Server", "This text is @dimmed@");

// Combined styles
info!("Server", "This is *italic with _underlined_ text*");
info!("Server", "This is **bold with *italic* inside**");
info!("Server", "You can mix **bold** with @dimmed@ and ~strikethrough~");

// Works in box style too
info_box!("Server", "All _styling_ **works** in *boxes* too!");
```


### Custom Styling
If you need to style text programmatically, you can use the `style_text!` macro:
```rust
let styled_text = style_text!("This is **bold** and *italic*", colors.info_text);
```

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
