use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use urlencoding;

/// Privacy-related configuration settings for the browser.
/// Controls various fingerprinting protection and tracking prevention features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    #[serde(default = "default_true")]
    pub hardware_spoofing: bool,
    #[serde(default = "default_true")]
    pub screen_normalization: bool,
    #[serde(default = "default_true")]
    pub timezone_normalization: bool,
    #[serde(default = "default_true")]
    pub battery_blocking: bool,
    #[serde(default = "default_true")]
    pub webrtc_blocking: bool,
    #[serde(default = "default_true")]
    pub media_device_blocking: bool,
    #[serde(default = "default_true")]
    pub geolocation_blocking: bool,
    #[serde(default = "default_true")]
    pub network_info_spoofing: bool,
    #[serde(default = "default_true")]
    pub storage_quota_spoofing: bool,
    #[serde(default = "default_true")]
    pub permissions_hardening: bool,
    #[serde(default = "default_true")]
    pub credentials_blocking: bool,
    #[serde(default = "default_true")]
    pub privacy_headers: bool,
    #[serde(default = "default_true")]
    pub tracking_domain_blocking: bool,
    #[serde(default = "default_true")]
    pub canvas_fingerprint_protection: bool,
    #[serde(default = "default_true")]
    pub webgl_fingerprint_protection: bool,
    #[serde(default = "default_true")]
    pub audio_fingerprint_protection: bool,
    #[serde(default = "default_true")]
    pub font_enumeration_restriction: bool,
}

fn default_true() -> bool {
    true
}

/// UI-related configuration settings for the browser interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    #[serde(default = "default_opacity")]
    pub opacity: f32,
}

fn default_opacity() -> f32 {
    1.0
}

impl Default for UiSettings {
    fn default() -> Self {
        UiSettings {
            opacity: 1.0,
        }
    }
}

impl Default for PrivacySettings {
    fn default() -> Self {
        PrivacySettings {
            hardware_spoofing: true,
            screen_normalization: true,
            timezone_normalization: true,
            battery_blocking: true,
            webrtc_blocking: true,
            media_device_blocking: true,
            geolocation_blocking: true,
            network_info_spoofing: true,
            storage_quota_spoofing: true,
            permissions_hardening: true,
            credentials_blocking: true,
            privacy_headers: true,
            tracking_domain_blocking: true,
            canvas_fingerprint_protection: true,
            webgl_fingerprint_protection: true,
            audio_fingerprint_protection: true,
            font_enumeration_restriction: true,
        }
    }
}

/// Main configuration structure for the Calm browser.
/// Contains search engine, default URL, privacy settings, and UI settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_search_engine")]
    pub search_engine: String,
    #[serde(default = "default_start_url")]
    pub default_url: String,
    #[serde(default)]
    pub privacy: PrivacySettings,
    #[serde(default)]
    pub ui: UiSettings,
}

fn default_search_engine() -> String {
    "https://start.duckduckgo.com/?q={}".to_string()
}

fn default_start_url() -> String {
    "https://start.duckduckgo.com".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            search_engine: default_search_engine(),
            default_url: default_start_url(),
            privacy: PrivacySettings::default(),
            ui: UiSettings::default(),
        }
    }
}

impl Config {
    /// Loads configuration from ~/.calm.yml file.
    /// Creates a default configuration file if one doesn't exist.
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if let Ok(contents) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_yaml::from_str(&contents) {
                return config;
            }
        }

        let default_config = Config::default();
        let _ = default_config.save();
        default_config
    }

    /// Saves the current configuration to ~/.calm.yml file.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        let yaml = serde_yaml::to_string(self)?;
        fs::write(config_path, yaml)?;
        Ok(())
    }

    fn get_config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".calm.yml")
    }

    /// Formats a search query into a complete search engine URL.
    /// Replaces `{}` placeholder in the search engine URL with the encoded query.
    pub fn format_search_url(&self, query: &str) -> String {
        self.search_engine.replace("{}", &urlencoding::encode(query))
    }
}
