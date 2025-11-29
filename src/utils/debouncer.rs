use std::time::Instant;

/// A debouncer that prevents actions from executing too frequently.
/// Useful for preventing rapid-fire events like keyboard shortcuts or button clicks.
pub struct Debouncer {
    last_time: Option<Instant>,
    threshold_ms: u128,
}

impl Debouncer {
    /// Creates a new debouncer with the specified threshold in milliseconds.
    ///
    /// # Arguments
    ///
    /// * `threshold_ms` - Minimum time in milliseconds between allowed executions
    pub fn new(threshold_ms: u128) -> Self {
        Self {
            last_time: None,
            threshold_ms,
        }
    }

    /// Checks if enough time has passed since the last execution.
    ///
    /// # Returns
    ///
    /// true if the action should execute, false if it should be debounced
    pub fn should_execute(&mut self) -> bool {
        let now = Instant::now();
        match self.last_time {
            Some(last) if now.duration_since(last).as_millis() < self.threshold_ms => false,
            _ => {
                self.last_time = Some(now);
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_debouncer_allows_first_execution() {
        let mut debouncer = Debouncer::new(100);
        assert!(debouncer.should_execute());
    }

    #[test]
    fn test_debouncer_blocks_rapid_execution() {
        let mut debouncer = Debouncer::new(100);
        assert!(debouncer.should_execute());
        assert!(!debouncer.should_execute());
    }

    #[test]
    fn test_debouncer_allows_after_threshold() {
        let mut debouncer = Debouncer::new(50);
        assert!(debouncer.should_execute());
        thread::sleep(Duration::from_millis(60));
        assert!(debouncer.should_execute());
    }
}
