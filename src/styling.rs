/// Colors using ANSI escape codes
pub struct Colors {
    pub info: &'static str,
    pub warn: &'static str,
    pub error: &'static str,
    pub success: &'static str,
    pub debug: &'static str,
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
