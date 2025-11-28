use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

/// Sets the global debug logging state.
///
/// # Arguments
///
/// * `enabled` - Whether debug logging should be enabled
pub fn set_debug_enabled(enabled: bool) {
    DEBUG_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Returns the current debug logging state.
///
/// # Returns
///
/// true if debug logging is enabled, false otherwise
pub fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if $crate::debug::is_debug_enabled() {
            eprintln!("[CALM DEBUG] {}", format!($($arg)*));
        }
    };
}
