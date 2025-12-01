use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabState {
    pub id: usize,
    pub url: String,
    pub title: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub tabs: Vec<TabState>,
    pub active_tab_index: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSession {
    pub windows: Vec<WindowState>,
    pub timestamp: u64,
    pub version: String,
}

impl BrowserSession {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn add_window(&mut self, window: WindowState) {
        self.windows.push(window);
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let session_path = Self::get_session_path();

        if let Some(parent) = session_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let backup_path = Self::get_session_backup_path();
        if session_path.exists() {
            let _ = fs::copy(&session_path, &backup_path);
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(session_path, json)?;
        Ok(())
    }

    pub fn load() -> Option<Self> {
        let session_path = Self::get_session_path();

        if let Ok(contents) = fs::read_to_string(&session_path) {
            if let Ok(session) = serde_json::from_str(&contents) {
                return Some(session);
            }
        }

        let backup_path = Self::get_session_backup_path();
        if let Ok(contents) = fs::read_to_string(&backup_path) {
            if let Ok(session) = serde_json::from_str(&contents) {
                return Some(session);
            }
        }

        None
    }

    pub fn clear() -> Result<(), Box<dyn std::error::Error>> {
        let session_path = Self::get_session_path();
        if session_path.exists() {
            fs::remove_file(session_path)?;
        }
        Ok(())
    }

    fn get_session_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".calm_session.json")
    }

    fn get_session_backup_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".calm_session.json.bak")
    }
}

impl Default for BrowserSession {
    fn default() -> Self {
        Self::new()
    }
}
