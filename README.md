# t_logger

A versatile and stylish logging library for Rust applications that provides both console output and file logging capabilities.

## Features

- 📝 Multiple log levels (info, warn, error, success, debug)
- 🎨 Colored output with ANSI escape codes
- 📦 Box-style formatted messages
- 📅 Automatic daily log file rotation
- 🔍 Debug mode toggle
- ⏰ Timestamp integration
- 💾 File logging with clean (ANSI-stripped) output

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
t_logger = "0.1.0"
```

## Usage

```rust
use t_logger::prelude::*;

fn main() {
    // Initialize the logger with a path for log files
    init_logger("logs").unwrap();
    
    // Optional: Disable debug output (still logs to file)
    set_debug(false);

    // Simple logging
    info!("Server", "Starting up...");
    success!("Login", "User {} connected", "Alice");
    warn!("Memory", "Usage at {}%", 85);
    error!("Database", "Connection failed");
    debug!("Processing", "Items in queue: {}", 42);

    // Box-style logging
    info_box!("Server", "Detailed server information...");
    warn_box!("Memory", "Memory usage details...");
    error_box!("Database", "Connection error details...");
    success_box!("Login", "Authentication successful");
    debug_box!("Processing", "Debug information");
}
```

## Log Levels

- `info!()` / `info_box!()` - Cyan colored information messages
- `warn!()` / `warn_box!()` - Yellow colored warning messages
- `error!()` / `error_box!()` - Red colored error messages
- `success!()` / `success_box!()` - Green colored success messages
- `debug!()` / `debug_box!()` - Magenta colored debug messages

## File Logging

Logs are automatically saved to files with the format `YYYY-MM-DD.log` in the specified directory. All ANSI color codes are stripped from the file output for better readability.

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

---
