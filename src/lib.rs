pub mod game;
pub mod i18n;
pub mod story;
pub mod time;
pub mod tui;

use std::sync::atomic::{AtomicBool, Ordering};

/// Global flag set by the Ctrl+C handler
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// Mark the process as interrupted (called from Ctrl+C handler)
pub fn set_interrupted() {
    INTERRUPTED.store(true, Ordering::Relaxed);
}

/// Check if Ctrl+C was pressed (used by tui::run)
pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::Relaxed)
}
