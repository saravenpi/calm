mod update_installer;
mod verification;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_REPO: &str = "saravenpi/calm";
const UPDATE_CHECK_INTERVAL_HOURS: u64 = 24;

type DownloadInfo = (String, Option<String>, Option<String>, Option<u64>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub download_url: String,
    pub signature_url: Option<String>,
    pub sha256: Option<String>,
    pub release_notes: String,
    pub published_at: String,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    body: Option<String>,
    published_at: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateState {
    last_check: Option<SystemTime>,
    last_update: Option<SystemTime>,
    pending_version: Option<String>,
    update_failed: bool,
    failure_count: u32,
}

impl Default for UpdateState {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdateState {
    pub fn new() -> Self {
        Self {
            last_check: None,
            last_update: None,
            pending_version: None,
            update_failed: false,
            failure_count: 0,
        }
    }

    pub fn load() -> Self {
        let path = Self::get_state_path();
        if let Ok(contents) = std::fs::read_to_string(&path) {
            if let Ok(state) = serde_json::from_str(&contents) {
                return state;
            }
        }
        Self::new()
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::get_state_path();
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    fn get_state_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".calm_update_state.json")
    }

    pub fn mark_check(&mut self) {
        self.last_check = Some(SystemTime::now());
        let _ = self.save();
    }

    pub fn mark_update_success(&mut self, _version: String) {
        self.last_update = Some(SystemTime::now());
        self.pending_version = None;
        self.update_failed = false;
        self.failure_count = 0;
        let _ = self.save();
    }

    pub fn mark_update_failed(&mut self) {
        self.update_failed = true;
        self.failure_count += 1;
        let _ = self.save();
    }
}

pub struct Updater {
    state: UpdateState,
}

impl Updater {
    pub fn new() -> Self {
        Self {
            state: UpdateState::load(),
        }
    }

    pub fn should_check_for_updates(&self) -> bool {
        if self.state.failure_count > 3 {
            return false;
        }

        if let Some(last_check) = self.state.last_check {
            if let Ok(elapsed) = last_check.elapsed() {
                return elapsed > Duration::from_secs(UPDATE_CHECK_INTERVAL_HOURS * 3600);
            }
        }
        true
    }

    pub fn check_for_updates(&mut self) -> Result<Option<UpdateInfo>, Box<dyn Error>> {
        self.state.mark_check();

        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            GITHUB_REPO
        );

        let client = reqwest::blocking::Client::builder()
            .user_agent(format!("Calm Browser {}", CURRENT_VERSION))
            .timeout(Duration::from_secs(10))
            .build()?;

        let response = client.get(&url).send()?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let release: GitHubRelease = response.json()?;

        let latest_version = release.tag_name.trim_start_matches('v');

        if Self::is_newer_version(latest_version, CURRENT_VERSION) {
            let (download_url, signature_url, sha256, size_bytes) =
                Self::get_download_info_for_platform(&release.assets)?;

            Ok(Some(UpdateInfo {
                version: latest_version.to_string(),
                download_url,
                signature_url,
                sha256,
                release_notes: release.body.unwrap_or_default(),
                published_at: release.published_at,
                size_bytes,
            }))
        } else {
            Ok(None)
        }
    }

    fn is_newer_version(latest: &str, current: &str) -> bool {
        let latest_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();

        let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();

        for i in 0..3 {
            let latest_part = latest_parts.get(i).unwrap_or(&0);
            let current_part = current_parts.get(i).unwrap_or(&0);

            if latest_part > current_part {
                return true;
            } else if latest_part < current_part {
                return false;
            }
        }

        false
    }

    fn get_download_info_for_platform(
        assets: &[GitHubAsset],
    ) -> Result<DownloadInfo, Box<dyn Error>> {
        #[cfg(target_os = "macos")]
        let platform_suffix = if cfg!(target_arch = "aarch64") {
            "macOS-aarch64.tar.gz"
        } else {
            "macOS-x86_64.tar.gz"
        };

        #[cfg(target_os = "linux")]
        let platform_suffix = "Linux-x86_64.tar.gz";

        #[cfg(target_os = "windows")]
        let platform_suffix = "Windows-x86_64.zip";

        let mut download_url = None;
        let mut signature_url = None;
        let mut sha256 = None;
        let mut size_bytes = None;

        for asset in assets {
            if asset.name.ends_with(platform_suffix) {
                download_url = Some(asset.browser_download_url.clone());
                size_bytes = Some(asset.size);
            } else if asset.name.ends_with(&format!("{}.sig", platform_suffix)) {
                signature_url = Some(asset.browser_download_url.clone());
            } else if asset.name.ends_with(&format!("{}.sha256", platform_suffix)) {
                sha256 = Some(asset.browser_download_url.clone());
            }
        }

        let url = download_url.ok_or("No compatible download found for your platform")?;

        Ok((url, signature_url, sha256, size_bytes))
    }

    pub fn download_and_verify_update(
        &mut self,
        update_info: &UpdateInfo,
        progress_callback: impl Fn(u64, u64),
    ) -> Result<PathBuf, Box<dyn Error>> {
        let bytes = self.download_update(update_info, &progress_callback)?;

        if let Some(ref sha256_url) = update_info.sha256 {
            verification::verify_sha256(&bytes, sha256_url)?;
        }

        if let Some(ref sig_url) = update_info.signature_url {
            verification::verify_signature(&bytes, sig_url)?;
        }

        let temp_file = self.save_update_to_temp(&bytes)?;

        Ok(temp_file)
    }

    fn download_update(
        &self,
        update_info: &UpdateInfo,
        progress_callback: &impl Fn(u64, u64),
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let response = reqwest::blocking::get(&update_info.download_url)?;

        if !response.status().is_success() {
            return Err(format!("Failed to download update: {}", response.status()).into());
        }

        let total_size = update_info
            .size_bytes
            .or_else(|| response.content_length())
            .ok_or("Content length not available")?;

        let bytes = response.bytes()?;
        progress_callback(bytes.len() as u64, total_size);

        Ok(bytes.to_vec())
    }

    fn save_update_to_temp(&self, bytes: &[u8]) -> Result<PathBuf, Box<dyn Error>> {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(format!("calm_update_{}", CURRENT_VERSION));
        std::fs::write(&temp_file, bytes)?;
        Ok(temp_file)
    }

    pub fn install_update(
        &mut self,
        update_file: PathBuf,
        version: String,
    ) -> Result<(), Box<dyn Error>> {
        match update_installer::install_update(update_file) {
            Ok(_) => {
                self.state.mark_update_success(version);
                Ok(())
            }
            Err(e) => {
                self.state.mark_update_failed();
                Err(e)
            }
        }
    }

    pub fn get_state(&self) -> &UpdateState {
        &self.state
    }
}

impl Default for Updater {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_current_version() -> &'static str {
    CURRENT_VERSION
}

pub fn format_version_info() -> String {
    format!(
        "Calm Browser v{}\nPlatform: {}-{}\nBuild: {}",
        CURRENT_VERSION,
        std::env::consts::OS,
        std::env::consts::ARCH,
        option_env!("BUILD_DATE").unwrap_or("unknown")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(Updater::is_newer_version("1.2.0", "1.1.0"));
        assert!(Updater::is_newer_version("2.0.0", "1.9.9"));
        assert!(Updater::is_newer_version("1.1.1", "1.1.0"));
        assert!(!Updater::is_newer_version("1.0.0", "1.0.0"));
        assert!(!Updater::is_newer_version("1.0.0", "1.0.1"));
        assert!(!Updater::is_newer_version("0.9.0", "1.0.0"));
    }
}
