/// Creates a Cyan box with the given title and message
#[macro_export]
macro_rules! info_box {
    ($title:expr, $($arg:tt)*) => {
        print!("{}", $crate::create_styled_box(
            $crate::get_colors().info,
            $crate::get_colors().info_text,
            $crate::get_symbols().info,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::get_colors().info,
            $crate::get_symbols().info,
            $title,
            $crate::get_colors().info_text,
            $($arg)*
        );

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&log) {
                    eprintln!("Error logging to file: {e}");
                }
            }
        }
    };
}

/// Creates a Yellow box with the given title and message
#[macro_export]
macro_rules! warn_box {
    ($title:expr, $($arg:tt)*) => {
        print!("{}", $crate::create_styled_box(
            $crate::get_colors().warn,
            $crate::get_colors().warn_text,
            $crate::get_symbols().warn,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::get_colors().warn,
            $crate::get_symbols().warn,
            $title,
            $crate::get_colors().warn_text,
            $($arg)*
        );

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&log) {
                    eprintln!("Error logging to file: {e}");
                }
            }
        }
    };
}

/// Creates a Red box with the given title and message
#[macro_export]
macro_rules! error_box {
    ($title:expr, $($arg:tt)*) => {
        eprint!("{}", $crate::create_styled_box(
            $crate::get_colors().error,
            $crate::get_colors().error_text,
            $crate::get_symbols().error,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::get_colors().error,
            $crate::get_symbols().error,
            $title,
            $crate::get_colors().error_text,
            $($arg)*
        );

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&log) {
                    eprintln!("Error logging to file: {e}");
                }
            }
        }
    };
}

/// Creates a Green box with the given title and message
#[macro_export]
macro_rules! success_box {
    ($title:expr, $($arg:tt)*) => {
        print!("{}", $crate::create_styled_box(
            $crate::get_colors().success,
            $crate::get_colors().success_text,
            $crate::get_symbols().success,
            $title,
            &format!($($arg)*),
            75
        ));
        let log = make_log!(
            $crate::get_colors().success,
            $crate::get_symbols().success,
            $title,
            $crate::get_colors().success,
            $($arg)*
        );

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&log) {
                    eprintln!("Error logging to file: {e}");
                }
            }
        }
    };
}

/// Creates a Magenta box with the given title and message
#[macro_export]
macro_rules! debug_box {
    ($title:expr, $($arg:tt)*) => {
        if *$crate::DEBUG.get().unwrap_or(&true) {
            print!("{}", $crate::create_styled_box(
                $crate::get_colors().debug,
                $crate::get_colors().debug_text,
                $crate::get_symbols().debug,
                $title,
                &format!($($arg)*),
                75
            ));
        }

        let log = make_log!(
            $crate::get_colors().debug,
            $crate::get_symbols().debug,
            $title,
            $crate::get_colors().debug_text,
            $($arg)*
        );

        unsafe {
            if let Some(logger) = $crate::LOGGER.get() {
                if let Err(e) = logger.log(&log) {
                    eprintln!("Error logging to file: {e}");
                }
            }
        }
    };
}
