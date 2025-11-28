use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Window position and size information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowPosition {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 1200,
            height: 800,
        }
    }
}

/// Information about a tab in a window session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowTabInfo {
    pub url: String,
    pub title: String,
}

/// Complete session data for a single window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSession {
    pub window_id: usize,
    pub position: WindowPosition,
    pub tabs: Vec<WindowTabInfo>,
    pub active_tab_index: Option<usize>,
}

/// Collection of all window sessions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowSessions {
    pub windows: Vec<WindowSession>,
    pub active_window_id: Option<usize>,
}

/// Manages loading and saving window session state.
pub struct WindowSessionManager {
    session_file: PathBuf,
}

impl WindowSessionManager {
    /// Creates a new session manager with default session file path.
    pub fn new() -> Self {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".calm_windows.yml");
        Self { session_file: path }
    }

    /// Loads window sessions from disk.
    pub fn load(&self) -> WindowSessions {
        if !self.session_file.exists() {
            return WindowSessions::default();
        }

        match fs::read_to_string(&self.session_file) {
            Ok(content) => {
                serde_yaml::from_str(&content).unwrap_or_default()
            }
            Err(_) => WindowSessions::default(),
        }
    }

    /// Saves window sessions to disk.
    pub fn save(&self, sessions: &WindowSessions) -> Result<(), String> {
        let yaml = serde_yaml::to_string(sessions)
            .map_err(|e| format!("Failed to serialize window sessions: {}", e))?;

        fs::write(&self.session_file, yaml)
            .map_err(|e| format!("Failed to write window sessions: {}", e))?;

        Ok(())
    }

    /// Returns the position of the last saved window.
    pub fn get_last_window_position(&self) -> WindowPosition {
        let sessions = self.load();
        sessions.windows.first()
            .map(|w| w.position.clone())
            .unwrap_or_default()
    }

    /// Saves the state of all windows to disk.
    pub fn save_window_state(
        &self,
        windows: &HashMap<usize, (WindowPosition, Vec<WindowTabInfo>, Option<usize>)>,
        active_window_id: Option<usize>,
    ) -> Result<(), String> {
        let windows_vec: Vec<WindowSession> = windows
            .iter()
            .map(|(id, (position, tabs, active_tab))| WindowSession {
                window_id: *id,
                position: position.clone(),
                tabs: tabs.clone(),
                active_tab_index: *active_tab,
            })
            .collect();

        let sessions = WindowSessions {
            windows: windows_vec,
            active_window_id,
        };

        self.save(&sessions)
    }
}
