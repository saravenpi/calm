use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadEntry {
    pub id: usize,
    pub filename: String,
    pub file_path: String,
    pub total_bytes: i64,
    pub completed: bool,
    pub failed: bool,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadHistory {
    pub downloads: Vec<DownloadEntry>,
    pub next_id: usize,
}

impl Default for DownloadHistory {
    fn default() -> Self {
        Self {
            downloads: Vec::new(),
            next_id: 1,
        }
    }
}

impl DownloadHistory {
    fn get_history_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".calm/downloads.yml")
    }

    pub fn load() -> Self {
        let path = Self::get_history_path();

        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(history) = serde_yaml::from_str(&contents) {
                return history;
            }
        }

        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_history_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let yaml = serde_yaml::to_string(self)?;
        fs::write(path, yaml)?;
        Ok(())
    }

    pub fn add_download(&mut self, filename: String, file_path: String, total_bytes: i64) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.downloads.push(DownloadEntry {
            id,
            filename,
            file_path,
            total_bytes,
            completed: false,
            failed: false,
            timestamp: chrono::Utc::now().timestamp(),
        });

        let _ = self.save();
        id
    }

    pub fn update_download(
        &mut self,
        id: usize,
        completed: bool,
        failed: bool,
        filename: Option<String>,
    ) {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.id == id) {
            download.completed = completed;
            download.failed = failed;
            if let Some(name) = filename {
                download.filename = name;
            }
            let _ = self.save();
        }
    }

    pub fn clear(&mut self) {
        self.downloads.clear();
        let _ = self.save();
    }
}

pub struct DownloadManager {
    next_download_id: Arc<Mutex<usize>>,
    history: Arc<Mutex<DownloadHistory>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        let history = DownloadHistory::load();
        let next_id = history.next_id;

        Self {
            next_download_id: Arc::new(Mutex::new(next_id)),
            history: Arc::new(Mutex::new(history)),
        }
    }

    pub fn get_download_id_counter(&self) -> Arc<Mutex<usize>> {
        Arc::clone(&self.next_download_id)
    }

    pub fn get_history(&self) -> Arc<Mutex<DownloadHistory>> {
        Arc::clone(&self.history)
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}
