use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub startup_time: Option<Duration>,
    pub tab_creation_times: Vec<Duration>,
    pub tab_switch_times: Vec<Duration>,
    pub memory_samples: Vec<usize>,
    pub active_tabs: usize,
    pub suspended_tabs: usize,
    pub total_tabs: usize,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            startup_time: None,
            tab_creation_times: Vec::new(),
            tab_switch_times: Vec::new(),
            memory_samples: Vec::new(),
            active_tabs: 0,
            suspended_tabs: 0,
            total_tabs: 0,
        }
    }

    pub fn record_startup(&mut self, duration: Duration) {
        self.startup_time = Some(duration);
    }

    pub fn record_tab_creation(&mut self, duration: Duration) {
        self.tab_creation_times.push(duration);
        if self.tab_creation_times.len() > 100 {
            self.tab_creation_times.remove(0);
        }
    }

    pub fn record_tab_switch(&mut self, duration: Duration) {
        self.tab_switch_times.push(duration);
        if self.tab_switch_times.len() > 100 {
            self.tab_switch_times.remove(0);
        }
    }

    pub fn record_memory_sample(&mut self, memory_mb: usize) {
        self.memory_samples.push(memory_mb);
        if self.memory_samples.len() > 60 {
            self.memory_samples.remove(0);
        }
    }

    pub fn update_tab_counts(&mut self, active: usize, suspended: usize, total: usize) {
        self.active_tabs = active;
        self.suspended_tabs = suspended;
        self.total_tabs = total;
    }

    pub fn avg_tab_creation_time(&self) -> Option<Duration> {
        if self.tab_creation_times.is_empty() {
            return None;
        }
        let sum: Duration = self.tab_creation_times.iter().sum();
        Some(sum / self.tab_creation_times.len() as u32)
    }

    pub fn avg_tab_switch_time(&self) -> Option<Duration> {
        if self.tab_switch_times.is_empty() {
            return None;
        }
        let sum: Duration = self.tab_switch_times.iter().sum();
        Some(sum / self.tab_switch_times.len() as u32)
    }

    pub fn avg_memory_usage(&self) -> Option<usize> {
        if self.memory_samples.is_empty() {
            return None;
        }
        Some(self.memory_samples.iter().sum::<usize>() / self.memory_samples.len())
    }

    pub fn print_summary(&self) {
        eprintln!("=== Performance Metrics ===");
        if let Some(startup) = self.startup_time {
            eprintln!("Startup time: {:?}", startup);
        }
        if let Some(avg_create) = self.avg_tab_creation_time() {
            eprintln!("Avg tab creation: {:?}", avg_create);
        }
        if let Some(avg_switch) = self.avg_tab_switch_time() {
            eprintln!("Avg tab switch: {:?}", avg_switch);
        }
        if let Some(avg_mem) = self.avg_memory_usage() {
            eprintln!("Avg memory: {} MB", avg_mem);
        }
        eprintln!("Tabs: {} active, {} suspended, {} total",
                 self.active_tabs, self.suspended_tabs, self.total_tabs);
        eprintln!("==========================");
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TabActivityTracker {
    last_activity: HashMap<usize, Instant>,
    suspension_timeout: Duration,
}

impl TabActivityTracker {
    pub fn new(timeout_minutes: u64) -> Self {
        Self {
            last_activity: HashMap::new(),
            suspension_timeout: Duration::from_secs(timeout_minutes * 60),
        }
    }

    pub fn mark_active(&mut self, tab_id: usize) {
        self.last_activity.insert(tab_id, Instant::now());
    }

    pub fn remove_tab(&mut self, tab_id: usize) {
        self.last_activity.remove(&tab_id);
    }

    pub fn should_suspend(&self, tab_id: usize) -> bool {
        if let Some(last_active) = self.last_activity.get(&tab_id) {
            last_active.elapsed() > self.suspension_timeout
        } else {
            false
        }
    }

    pub fn get_inactive_tabs(&self) -> Vec<usize> {
        self.last_activity
            .iter()
            .filter(|(_, time)| time.elapsed() > self.suspension_timeout)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn set_timeout(&mut self, timeout_minutes: u64) {
        self.suspension_timeout = Duration::from_secs(timeout_minutes * 60);
    }
}

pub type SharedMetrics = Arc<Mutex<PerformanceMetrics>>;

pub fn create_shared_metrics() -> SharedMetrics {
    Arc::new(Mutex::new(PerformanceMetrics::new()))
}
