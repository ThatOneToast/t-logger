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

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&msg) {
                    eprintln!("Error logging to file: {e}");
                }
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

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&msg) {
                    eprintln!("Error logging to file: {e}");
                }
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

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&msg) {
                    eprintln!("Error logging to file: {e}");
                }
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

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&msg) {
                    eprintln!("Error logging to file: {e}");
                }
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

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&msg) {
                    eprintln!("Error logging to file: {e}");
                }
            }
        }
    }};
}
