use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
    #[serde(default = "default_true")]
    pub adblock_enabled: bool,
}

/// Default value function for boolean fields (returns true).
fn default_true() -> bool {
    true
}

/// Keyboard shortcut configuration.
/// All shortcuts use Cmd on macOS and Ctrl on other platforms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardShortcuts {
    #[serde(default = "default_shortcut_new_tab")]
    pub new_tab: String,
    #[serde(default = "default_shortcut_close_tab")]
    pub close_tab: String,
    #[serde(default = "default_shortcut_reload")]
    pub reload: String,
    #[serde(default = "default_shortcut_focus_url")]
    pub focus_url: String,
    #[serde(default = "default_shortcut_toggle_downloads")]
    pub toggle_downloads: String,
    #[serde(default = "default_shortcut_focus_sidebar")]
    pub focus_sidebar: String,
    #[serde(default = "default_shortcut_find")]
    pub find: String,
    #[serde(default = "default_shortcut_new_window")]
    pub new_window: String,
    #[serde(default = "default_shortcut_toggle_split_view")]
    pub toggle_split_view: String,
}

fn default_shortcut_new_tab() -> String {
    "Cmd+T".to_string()
}

fn default_shortcut_close_tab() -> String {
    "Cmd+W".to_string()
}

fn default_shortcut_reload() -> String {
    "Cmd+R".to_string()
}

fn default_shortcut_focus_url() -> String {
    "Cmd+L".to_string()
}

fn default_shortcut_toggle_downloads() -> String {
    "Cmd+J".to_string()
}

fn default_shortcut_focus_sidebar() -> String {
    "Cmd+E".to_string()
}

fn default_shortcut_find() -> String {
    "Cmd+F".to_string()
}

fn default_shortcut_new_window() -> String {
    "Cmd+N".to_string()
}

fn default_shortcut_toggle_split_view() -> String {
    "Cmd+Shift+S".to_string()
}

fn normalize_shortcut(value: &str, default: &str) -> String {
    if value.trim().is_empty() {
        default.to_string()
    } else {
        value.to_string()
    }
}

impl KeyboardShortcuts {
    pub fn normalize(&mut self) {
        self.new_tab = normalize_shortcut(&self.new_tab, &default_shortcut_new_tab());
        self.close_tab = normalize_shortcut(&self.close_tab, &default_shortcut_close_tab());
        self.reload = normalize_shortcut(&self.reload, &default_shortcut_reload());
        self.focus_url = normalize_shortcut(&self.focus_url, &default_shortcut_focus_url());
        self.toggle_downloads =
            normalize_shortcut(&self.toggle_downloads, &default_shortcut_toggle_downloads());
        self.focus_sidebar =
            normalize_shortcut(&self.focus_sidebar, &default_shortcut_focus_sidebar());
        self.find = normalize_shortcut(&self.find, &default_shortcut_find());
        self.new_window = normalize_shortcut(&self.new_window, &default_shortcut_new_window());
        self.toggle_split_view = normalize_shortcut(
            &self.toggle_split_view,
            &default_shortcut_toggle_split_view(),
        );
    }
}

impl Default for KeyboardShortcuts {
    fn default() -> Self {
        Self {
            new_tab: default_shortcut_new_tab(),
            close_tab: default_shortcut_close_tab(),
            reload: default_shortcut_reload(),
            focus_url: default_shortcut_focus_url(),
            toggle_downloads: default_shortcut_toggle_downloads(),
            focus_sidebar: default_shortcut_focus_sidebar(),
            find: default_shortcut_find(),
            new_window: default_shortcut_new_window(),
            toggle_split_view: default_shortcut_toggle_split_view(),
        }
    }
}

/// Performance-related configuration settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    #[serde(default = "default_true")]
    pub lazy_tab_loading: bool,
    #[serde(default = "default_true")]
    pub tab_suspension: bool,
    #[serde(default = "default_suspension_timeout")]
    pub suspension_timeout_minutes: u64,
    #[serde(default = "default_true")]
    pub session_restore: bool,
    #[serde(default = "default_session_save_interval")]
    pub session_save_interval_seconds: u64,
    #[serde(default = "default_max_memory_per_tab")]
    pub max_memory_per_tab_mb: usize,
}

fn default_suspension_timeout() -> u64 {
    15
}

fn default_session_save_interval() -> u64 {
    30
}

fn default_max_memory_per_tab() -> usize {
    512
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        PerformanceSettings {
            lazy_tab_loading: true,
            tab_suspension: true,
            suspension_timeout_minutes: 15,
            session_restore: true,
            session_save_interval_seconds: 30,
            max_memory_per_tab_mb: 512,
        }
    }
}

/// UI-related configuration settings for the browser interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    #[serde(default = "default_true")]
    pub vim_mode: bool,
    #[serde(default = "default_false")]
    pub debug: bool,
    #[serde(default = "default_true")]
    pub sounds: bool,
    #[serde(default)]
    pub shortcuts: KeyboardShortcuts,
}

/// Default value function for boolean fields (returns false).
fn default_false() -> bool {
    false
}

impl Default for UiSettings {
    fn default() -> Self {
        UiSettings {
            vim_mode: true,
            debug: false,
            sounds: true,
            shortcuts: KeyboardShortcuts::default(),
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
            adblock_enabled: true,
        }
    }
}

/// Main configuration structure for the Calm browser.
/// Contains search engine, default URL, privacy settings, UI settings, and redirect settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_search_engine")]
    pub search_engine: String,
    #[serde(default = "default_start_url")]
    pub default_url: String,
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    #[serde(default)]
    pub privacy: PrivacySettings,
    #[serde(default)]
    pub ui: UiSettings,
    #[serde(default)]
    pub performance: PerformanceSettings,
    #[serde(default = "default_false")]
    pub redirect_youtube_to_invidious: bool,
    #[serde(default = "default_invidious_instance")]
    pub invidious_instance: String,
}

/// Default search engine URL template with placeholder for query.
fn default_search_engine() -> String {
    "https://start.duckduckgo.com/?q={}".to_string()
}

/// Default start/home page URL.
fn default_start_url() -> String {
    "https://start.duckduckgo.com".to_string()
}

/// Default user agent string (macOS Safari 26.0 for Google sign-in compatibility).
fn default_user_agent() -> String {
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15".to_string()
}

/// Default Invidious instance for YouTube redirects.
fn default_invidious_instance() -> String {
    "yewtu.be".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            search_engine: default_search_engine(),
            default_url: default_start_url(),
            user_agent: default_user_agent(),
            privacy: PrivacySettings::default(),
            ui: UiSettings::default(),
            performance: PerformanceSettings::default(),
            redirect_youtube_to_invidious: false,
            invidious_instance: default_invidious_instance(),
        }
    }
}

impl Config {
    /// Loads configuration from ~/.calm.yml file.
    /// Creates a default configuration file if one doesn't exist.
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if let Ok(contents) = fs::read_to_string(&config_path) {
            if let Ok(mut config) = serde_yaml::from_str::<Config>(&contents) {
                config.ui.shortcuts.normalize();
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

    /// Returns the path to the configuration file (~/.calm.yml).
    fn get_config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".calm.yml")
    }

    /// Formats a search query into a complete search engine URL.
    /// Replaces `{}` placeholder in the search engine URL with the encoded query.
    pub fn format_search_url(&self, query: &str) -> String {
        self.search_engine
            .replace("{}", &urlencoding::encode(query))
    }
}
