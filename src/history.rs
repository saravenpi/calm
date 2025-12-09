use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn load() -> Self {
        let history_path = Self::get_history_path();

        if let Ok(contents) = fs::read_to_string(&history_path) {
            if let Ok(history) = serde_yaml::from_str::<History>(&contents) {
                return history;
            }
        }

        Self::new()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let history_dir = Self::get_history_dir();
        fs::create_dir_all(&history_dir)?;

        let history_path = Self::get_history_path();
        let yaml = serde_yaml::to_string(self)?;
        fs::write(history_path, yaml)?;
        Ok(())
    }

    pub fn add_entry(&mut self, url: String, title: String) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.entries.insert(
            0,
            HistoryEntry {
                url,
                title,
                timestamp,
            },
        );

        while self.entries.len() > 10000 {
            self.entries.pop();
        }

        let _ = self.save();
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        let _ = self.save();
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<HistoryEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|entry| {
                entry.url.to_lowercase().contains(&query_lower)
                    || entry.title.to_lowercase().contains(&query_lower)
            })
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn get_recent(&self, limit: usize) -> Vec<HistoryEntry> {
        self.entries.iter().take(limit).cloned().collect()
    }

    fn get_history_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".calm")
    }

    fn get_history_path() -> PathBuf {
        Self::get_history_dir().join("history.yml")
    }
}
