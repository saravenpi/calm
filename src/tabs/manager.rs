use super::tab::Tab;
use super::split_view::{SplitViewManager, SplitOrientation, SplitPane};
use crate::privacy;
use crate::url_cleaner;
use crate::config::Config;
use crate::vimium_hints;
use crate::debug_log;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::fs;
use tao::{dpi::{LogicalPosition, LogicalSize}, window::Window};
use wry::{Rect, WebView, WebViewBuilder};

/// Maximum number of tabs that can be open simultaneously.
const MAX_TABS: usize = 20;

/// Manages all browser tabs including creation, switching, navigation, and download handling.
pub struct TabManager {
    tabs: HashMap<usize, Tab>,
    active_tab_id: Option<usize>,
    next_tab_id: usize,
    next_download_id: Arc<Mutex<usize>>,
    tab_sidebar_width: u32,
    tab_bar_webview: Option<std::rc::Rc<WebView>>,
    download_overlay: Option<std::rc::Rc<WebView>>,
    config: std::rc::Rc<std::cell::RefCell<Config>>,
    split_view: SplitViewManager,
    current_urls: Arc<Mutex<HashMap<usize, String>>>,
    active_tab_id_shared: Arc<Mutex<Option<usize>>>,
}

/// Returns the path to the user's Downloads directory.
fn get_downloads_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join("Downloads")
}

/// Attempts to extract filename from HTTP Content-Disposition header.
fn get_filename_from_headers(url: &str) -> Option<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .ok()?;

    let response = client.head(url).send().ok()?;

    if let Some(content_disposition) = response.headers().get(reqwest::header::CONTENT_DISPOSITION) {
        if let Ok(value) = content_disposition.to_str() {
            let parsed = content_disposition::parse_content_disposition(value);
            if let Some(filename) = parsed.filename_full() {
                let sanitized = filename
                    .chars()
                    .map(|c| {
                        let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
                        if invalid_chars.contains(&c) { '_' } else { c }
                    })
                    .collect::<String>();

                if !sanitized.is_empty() {
                    return Some(sanitized);
                }
            }
        }
    }

    None
}

/// Extracts and sanitizes a filename from a URL, removing invalid characters.
fn sanitize_filename(url: &str) -> String {
    let path_part = url.split('?').next().unwrap_or(url);
    let path_part = path_part.split('#').next().unwrap_or(path_part);

    let mut filename = path_part
        .split('/')
        .last()
        .unwrap_or("download")
        .to_string();

    filename = urlencoding::decode(&filename)
        .unwrap_or_else(|_| filename.clone().into())
        .to_string();

    if filename.is_empty() || filename == "/" || filename == "download" {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        filename = format!("download_{}", timestamp);
    }

    let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    filename = filename.chars()
        .map(|c| if invalid_chars.contains(&c) { '_' } else { c })
        .collect();

    filename
}

/// Adds file extension to a file by detecting its type from content if extension is missing.
fn add_extension_if_needed(path: &PathBuf) -> PathBuf {
    if path.extension().is_some() {
        return path.clone();
    }

    if let Ok(bytes) = fs::read(path) {
        let sample_size = std::cmp::min(8192, bytes.len());
        if sample_size > 0 {
            let sample = &bytes[..sample_size];
            if let Some(kind) = infer::get(sample) {
                let new_path = path.with_extension(kind.extension());
                if let Ok(_) = fs::rename(path, &new_path) {
                    return new_path;
                }
            }
        }
    }

    path.clone()
}

impl TabManager {
    /// Creates a new TabManager instance with the specified tab sidebar width and configuration.
    pub fn new(tab_sidebar_width: u32, config: std::rc::Rc<std::cell::RefCell<Config>>) -> Self {
        Self {
            tabs: HashMap::new(),
            active_tab_id: None,
            next_tab_id: 1,
            next_download_id: Arc::new(Mutex::new(1)),
            tab_sidebar_width,
            tab_bar_webview: None,
            download_overlay: None,
            config,
            split_view: SplitViewManager::new(),
            current_urls: Arc::new(Mutex::new(HashMap::new())),
            active_tab_id_shared: Arc::new(Mutex::new(None)),
        }
    }

    /// Sets the reference to the tab bar webview for IPC communication.
    pub fn set_tab_bar_webview(&mut self, webview: std::rc::Rc<WebView>) {
        self.tab_bar_webview = Some(webview);
    }

    /// Sets the reference to the download overlay webview.
    pub fn set_download_overlay(&mut self, webview: std::rc::Rc<WebView>) {
        self.download_overlay = Some(webview);
    }

    /// Creates a new tab with the specified URL.
    /// Returns the tab ID on success.
    pub fn create_tab(&mut self, window: &Window, url: &str) -> Result<usize, wry::Error> {
        self.create_tab_internal(window, url, None)
    }

    /// Creates a new tab with custom HTML content.
    /// Returns the tab ID on success.
    pub fn create_tab_with_html(&mut self, window: &Window, html: &str) -> Result<usize, wry::Error> {
        self.create_tab_internal(window, "about:blank", Some(html))
    }

    /// Internal method to create a tab with URL or HTML content.
    fn create_tab_internal(&mut self, window: &Window, url: &str, html: Option<&str>) -> Result<usize, wry::Error> {
        debug_log!("create_tab_internal called - url: {}, has_html: {}", url, html.is_some());

        if self.tabs.len() >= MAX_TABS {
            debug_log!("Max tabs reached ({}), not creating new tab", MAX_TABS);
            return Ok(self.active_tab_id.unwrap_or(1));
        }

        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;
        debug_log!("Creating new tab with id: {}", tab_id);

        let window_size = window.inner_size();
        let content_width = window_size.width.saturating_sub(self.tab_sidebar_width);

        let bounds = Rect {
            position: LogicalPosition::new(self.tab_sidebar_width as i32, 0).into(),
            size: LogicalSize::new(content_width, window_size.height).into(),
        };

        let redirected_url = url_cleaner::redirect_youtube_to_invidious(url, &self.config.borrow());
        let cleaned_url = url_cleaner::clean_url(&redirected_url).unwrap_or_else(|_| redirected_url.to_string());

        let download_id_counter = Arc::clone(&self.next_download_id);
        let current_download_id = Arc::new(Mutex::new(0usize));
        let download_id_started = Arc::clone(&current_download_id);
        let download_id_completed = Arc::clone(&current_download_id);
        let download_path_store = Arc::new(Mutex::new(PathBuf::new()));
        let download_path_started = Arc::clone(&download_path_store);
        let download_path_completed = Arc::clone(&download_path_store);
        let download_overlay_started = self.download_overlay.clone();
        let download_overlay_completed = self.download_overlay.clone();
        let tab_bar_for_ipc = self.tab_bar_webview.clone();
        let current_urls_for_ipc = Arc::clone(&self.current_urls);
        let tab_id_for_ipc = tab_id;
        let active_tab_id_for_ipc = Arc::clone(&self.active_tab_id_shared);
        let config_for_ipc = std::rc::Rc::clone(&self.config);

        let mut builder = WebViewBuilder::new();

        builder = if let Some(html_content) = html {
            builder.with_html(html_content)
        } else {
            builder.with_url(&cleaned_url)
        };

        let webview = builder
            .with_bounds(bounds)
            .with_visible(false)
            .with_user_agent(privacy::get_privacy_user_agent())
            .with_devtools(true)
            .with_clipboard(true)
            .with_asynchronous_custom_protocol("calmfile".into(), move |_webview_id, request, responder| {
                let uri = request.uri().to_string();
                let path = uri.trim_start_matches("calmfile://localhost");
                match std::fs::read(path) {
                    Ok(content) => {
                        let mime = if path.ends_with(".html") || path.ends_with(".htm") {
                            "text/html"
                        } else if path.ends_with(".css") {
                            "text/css"
                        } else if path.ends_with(".js") {
                            "application/javascript"
                        } else if path.ends_with(".json") {
                            "application/json"
                        } else if path.ends_with(".png") {
                            "image/png"
                        } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
                            "image/jpeg"
                        } else if path.ends_with(".gif") {
                            "image/gif"
                        } else if path.ends_with(".svg") {
                            "image/svg+xml"
                        } else {
                            "application/octet-stream"
                        };
                        responder.respond(wry::http::Response::builder()
                            .header("Content-Type", mime)
                            .body(content)
                            .unwrap());
                    }
                    Err(_) => {
                        responder.respond(wry::http::Response::builder()
                            .status(404)
                            .body(Vec::<u8>::new())
                            .unwrap());
                    }
                }
            })
            .with_initialization_script(&{
                debug_log!("Building initialization script for tab {}", tab_id);

                let privacy_script = privacy::get_combined_privacy_script_with_config(&self.config.borrow().privacy);
                let vimium_script = if self.config.borrow().ui.vim_mode {
                    debug_log!("Including vimium hints script (vim_mode enabled)");
                    vimium_hints::get_vimium_hints_script()
                } else {
                    debug_log!("Skipping vimium hints script (vim_mode disabled)");
                    ""
                };

                let console_override = r#"
console.log('[INIT] Script injection starting - document exists?', typeof document !== 'undefined');
console.log('[INIT] window exists?', typeof window !== 'undefined');

(function() {
    const originalLog = console.log;
    const originalError = console.error;
    const originalWarn = console.warn;

    console.log = function(...args) {
        originalLog.apply(console, args);
        if (window.ipc) {
            window.ipc.postMessage(JSON.stringify({
                action: 'console_log',
                level: 'log',
                message: args.map(a => typeof a === 'object' ? JSON.stringify(a) : String(a)).join(' ')
            }));
        }
    };

    console.error = function(...args) {
        originalError.apply(console, args);
        if (window.ipc) {
            window.ipc.postMessage(JSON.stringify({
                action: 'console_log',
                level: 'error',
                message: args.map(a => typeof a === 'object' ? JSON.stringify(a) : String(a)).join(' ')
            }));
        }
    };

    console.warn = function(...args) {
        originalWarn.apply(console, args);
        if (window.ipc) {
            window.ipc.postMessage(JSON.stringify({
                action: 'console_log',
                level: 'warn',
                message: args.map(a => typeof a === 'object' ? JSON.stringify(a) : String(a)).join(' ')
            }));
        }
    };
})();

console.log('[INIT] Console override installed');
                "#;

                let invidious_instance = self.config.borrow().invidious_instance.clone();
                let redirect_enabled = self.config.borrow().redirect_youtube_to_invidious;

                let link_interception = if redirect_enabled {
                    format!(r#"
(function() {{
    const INVIDIOUS_INSTANCE = '{}';

    function redirectYouTubeUrl(url) {{
        try {{
            const urlObj = new URL(url);
            const host = urlObj.host;

            if (host === 'youtube.com' || host === 'www.youtube.com' || host === 'm.youtube.com' || host === 'youtu.be') {{
                const path = urlObj.pathname;
                const query = urlObj.search;

                let newPath = path;
                let newQuery = query;

                if (host === 'youtu.be') {{
                    const videoId = path.substring(1).split('/')[0];
                    if (videoId) {{
                        newPath = '/watch';
                        newQuery = query ? '?v=' + videoId + '&' + query.substring(1) : '?v=' + videoId;
                    }}
                }} else if (path.startsWith('/shorts/')) {{
                    const videoId = path.substring(8).split('/')[0];
                    if (videoId) {{
                        newPath = '/watch';
                        newQuery = query ? '?v=' + videoId + '&' + query.substring(1) : '?v=' + videoId;
                    }}
                }} else if (path.startsWith('/embed/')) {{
                    const videoId = path.substring(7).split('/')[0];
                    if (videoId) {{
                        newPath = '/watch';
                        newQuery = query ? '?v=' + videoId + '&' + query.substring(1) : '?v=' + videoId;
                    }}
                }} else if (path.startsWith('/v/')) {{
                    const videoId = path.substring(3).split('/')[0];
                    if (videoId) {{
                        newPath = '/watch';
                        newQuery = query ? '?v=' + videoId + '&' + query.substring(1) : '?v=' + videoId;
                    }}
                }} else if (path.startsWith('/live/')) {{
                    const videoId = path.substring(6).split('/')[0];
                    if (videoId) {{
                        newPath = '/watch';
                        newQuery = query ? '?v=' + videoId + '&' + query.substring(1) : '?v=' + videoId;
                    }}
                }}

                return 'https://' + INVIDIOUS_INSTANCE + newPath + newQuery;
            }}
        }} catch (e) {{
            console.error('[REDIRECT] Error redirecting URL:', e);
        }}
        return null;
    }}

    function interceptNavigation() {{
        document.addEventListener('click', function(e) {{
            let target = e.target;
            while (target && target.tagName !== 'A') {{
                target = target.parentElement;
            }}

            if (target && target.tagName === 'A' && target.href) {{
                const redirected = redirectYouTubeUrl(target.href);
                if (redirected) {{
                    e.preventDefault();
                    console.log('[REDIRECT] YouTube -> Invidious:', target.href, '->', redirected);
                    window.location.href = redirected;
                    return false;
                }}
            }}
        }}, true);
    }}

    if (document.readyState === 'loading') {{
        document.addEventListener('DOMContentLoaded', interceptNavigation);
    }} else {{
        interceptNavigation();
    }}

    console.log('[INIT] YouTube->Invidious link interception installed');
}})();
                "#, invidious_instance)
                } else {
                    String::new()
                };

                let safe_privacy_script = format!(
                    "try {{\n{}\n}} catch(e) {{ console.error('[PRIVACY] Error:', e); }}",
                    privacy_script
                );

                let cfg = self.config.borrow();
                let settings_init_script = format!(r#"
(function() {{
    function initSettings() {{
        const defaultUrlInput = document.getElementById('default-url');
        const searchEngineInput = document.getElementById('search-engine');
        const vimModeCheckbox = document.getElementById('vim-mode');
        const blockTrackersCheckbox = document.getElementById('block-trackers');
        const blockFingerprintingCheckbox = document.getElementById('block-fingerprinting');

        if (defaultUrlInput && searchEngineInput && vimModeCheckbox) {{
            console.log('[SETTINGS-INIT] Settings page detected, populating values');
            defaultUrlInput.value = {};
            searchEngineInput.value = {};
            vimModeCheckbox.checked = {};
            blockTrackersCheckbox.checked = {};
            blockFingerprintingCheckbox.checked = {};
            console.log('[SETTINGS-INIT] Settings populated successfully');
        }}
    }}

    if (document.readyState === 'loading') {{
        document.addEventListener('DOMContentLoaded', initSettings);
    }} else {{
        initSettings();
    }}
}})();
                "#,
                    serde_json::to_string(&cfg.default_url).unwrap_or_else(|_| "\"\"".to_string()),
                    serde_json::to_string(&cfg.search_engine).unwrap_or_else(|_| "\"\"".to_string()),
                    cfg.ui.vim_mode,
                    cfg.privacy.tracking_domain_blocking,
                    cfg.privacy.canvas_fingerprint_protection
                );
                drop(cfg);

                let combined_script = format!("{}\n{}\n{}\n{}\n{}", console_override, link_interception, safe_privacy_script, vimium_script, settings_init_script);
                debug_log!("Initialization script size: {} bytes (console: ~600, privacy: ~{}, vimium: {})",
                    combined_script.len(),
                    safe_privacy_script.len(),
                    vimium_script.len()
                );

                combined_script
            })
            .with_download_started_handler(move |url, path| {
                let download_id = {
                    let mut id = download_id_counter.lock().unwrap();
                    let current_id = *id;
                    *id += 1;
                    current_id
                };

                let filename = get_filename_from_headers(&url)
                    .unwrap_or_else(|| sanitize_filename(&url));
                let download_path = get_downloads_dir().join(&filename);
                *path = download_path.clone();

                *download_id_started.lock().unwrap() = download_id;
                *download_path_started.lock().unwrap() = download_path.clone();

                if let Some(ref webview) = download_overlay_started {
                    let _ = webview.set_visible(true);

                    let script = format!(
                        "if (window.addDownload) {{ window.addDownload({}, {}, 0); }}",
                        download_id,
                        serde_json::to_string(&filename).unwrap_or_else(|_| "\"download\"".to_string())
                    );
                    let _ = webview.evaluate_script(&script);
                }
                true
            })
            .with_download_completed_handler(move |_url, path, success| {
                let download_id = *download_id_completed.lock().unwrap();
                let stored_path = download_path_completed.lock().unwrap().clone();

                let final_path = path.as_ref().unwrap_or(&stored_path);
                let final_filename = if success {
                    let new_path = add_extension_if_needed(final_path);
                    new_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("download")
                        .to_string()
                } else {
                    final_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("download")
                        .to_string()
                };

                if download_id > 0 {
                    if let Some(ref webview) = download_overlay_completed {
                        let script = if success {
                            format!(
                                "if (window.completeDownload) {{ window.completeDownload({}, {}); }}",
                                download_id,
                                serde_json::to_string(&final_filename).unwrap_or_else(|_| "\"download\"".to_string())
                            )
                        } else {
                            format!(
                                "if (window.failDownload) {{ window.failDownload({}); }}",
                                download_id
                            )
                        };
                        let _ = webview.evaluate_script(&script);
                    }
                }
            })
            .with_ipc_handler(move |request| {
                let body = request.body();
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(body) {
                    match data["action"].as_str() {
                        Some("update_title") => {
                            if let Some(title) = data["title"].as_str() {
                                if let Some(ref webview) = tab_bar_for_ipc {
                                    let script = format!(
                                        "window.updateTabTitle({}, {});",
                                        tab_id,
                                        serde_json::to_string(title).unwrap_or_else(|_| "\"\"".to_string())
                                    );
                                    let _ = webview.evaluate_script(&script);
                                }
                            }
                        }
                        Some("update_url") => {
                            if let Some(url) = data["url"].as_str() {
                                if let Ok(mut urls) = current_urls_for_ipc.lock() {
                                    urls.insert(tab_id_for_ipc, url.to_string());
                                }
                                let is_active = if let Ok(active_id) = active_tab_id_for_ipc.lock() {
                                    *active_id == Some(tab_id_for_ipc)
                                } else {
                                    false
                                };
                                if is_active {
                                    if let Some(ref webview) = tab_bar_for_ipc {
                                        let script = format!(
                                            "window.updateUrlBar({});",
                                            serde_json::to_string(url).unwrap_or_else(|_| "\"\"".to_string())
                                        );
                                        let _ = webview.evaluate_script(&script);
                                    }
                                }
                            }
                        }
                        Some("update_navigation_state") => {
                            if let Some(ref webview) = tab_bar_for_ipc {
                                let can_go_back = data["canGoBack"].as_bool().unwrap_or(false);
                                let can_go_forward = data["canGoForward"].as_bool().unwrap_or(false);
                                let script = format!(
                                    "window.updateNavigationButtons({}, {});",
                                    can_go_back,
                                    can_go_forward
                                );
                                let _ = webview.evaluate_script(&script);
                            }
                        }
                        Some("console_log") => {
                            let level = data["level"].as_str().unwrap_or("log");
                            let message = data["message"].as_str().unwrap_or("");
                            let prefix = match level {
                                "error" => "[BROWSER ERROR]",
                                "warn" => "[BROWSER WARN]",
                                _ => "[BROWSER]"
                            };
                            eprintln!("{} {}", prefix, message);
                        }
                        Some("load_settings") => {
                            debug_log!("=== load_settings IPC received from tab ===");
                            if let Some(ref webview) = tab_bar_for_ipc {
                                let cfg = config_for_ipc.borrow();
                                let settings_obj = serde_json::json!({
                                    "defaultUrl": cfg.default_url,
                                    "searchEngine": cfg.search_engine,
                                    "vimMode": cfg.ui.vim_mode,
                                    "blockTrackers": cfg.privacy.tracking_domain_blocking,
                                    "blockFingerprinting": cfg.privacy.canvas_fingerprint_protection,
                                    "blockCookies": true,
                                });
                                debug_log!("Settings to send from tab: {:?}", settings_obj);
                                let script = format!(
                                    "{{ const settingsTab = window.tabs.find(t => t.id === {}); if (settingsTab) {{ window.ipcMessageToTab = {{ tabId: {}, action: 'updateSettings', settings: {} }}; }} }}",
                                    tab_id_for_ipc, tab_id_for_ipc, settings_obj
                                );
                                debug_log!("Sending settings to tab via tab bar");
                                let _ = webview.evaluate_script(&script);
                                drop(cfg);
                            }
                        }
                        Some("save_settings") => {
                            debug_log!("=== save_settings IPC received from tab ===");
                            if let Some(settings) = data["settings"].as_object() {
                                debug_log!("Settings received from tab: {:?}", settings);
                                let mut cfg = config_for_ipc.borrow_mut();

                                if let Some(default_url) = settings.get("defaultUrl").and_then(|v| v.as_str()) {
                                    debug_log!("Setting default_url to: {}", default_url);
                                    cfg.default_url = default_url.to_string();
                                }
                                if let Some(search_engine) = settings.get("searchEngine").and_then(|v| v.as_str()) {
                                    debug_log!("Setting search_engine to: {}", search_engine);
                                    cfg.search_engine = search_engine.to_string();
                                }
                                if let Some(vim_mode) = settings.get("vimMode").and_then(|v| v.as_bool()) {
                                    debug_log!("Setting vim_mode to: {}", vim_mode);
                                    cfg.ui.vim_mode = vim_mode;
                                }
                                if let Some(block_trackers) = settings.get("blockTrackers").and_then(|v| v.as_bool()) {
                                    debug_log!("Setting block_trackers to: {}", block_trackers);
                                    cfg.privacy.tracking_domain_blocking = block_trackers;
                                }
                                if let Some(block_fp) = settings.get("blockFingerprinting").and_then(|v| v.as_bool()) {
                                    debug_log!("Setting block_fingerprinting to: {}", block_fp);
                                    cfg.privacy.canvas_fingerprint_protection = block_fp;
                                    cfg.privacy.webgl_fingerprint_protection = block_fp;
                                    cfg.privacy.audio_fingerprint_protection = block_fp;
                                }

                                match cfg.save() {
                                    Ok(_) => debug_log!("Settings saved successfully to ~/.calm.yml from tab"),
                                    Err(e) => debug_log!("ERROR: Failed to save settings from tab: {:?}", e),
                                }
                            } else {
                                debug_log!("ERROR: No settings object in save_settings message from tab");
                            }
                        }
                        _ => {}
                    }
                }
            })
            .build_as_child(window)?;

        debug_log!("Webview built successfully for tab {}", tab_id);

        let tab = Tab::new(tab_id, cleaned_url.clone(), webview);
        self.tabs.insert(tab_id, tab);

        if let Ok(mut urls) = self.current_urls.lock() {
            urls.insert(tab_id, cleaned_url);
        }

        debug_log!("Tab {} created and inserted into tabs map", tab_id);

        Ok(tab_id)
    }

    /// Switches the active tab to the specified tab ID.
    /// Hides the current tab and shows the target tab.
    pub fn switch_to_tab(&mut self, tab_id: usize) {
        if !self.tabs.contains_key(&tab_id) {
            return;
        }

        if let Some(current_id) = self.active_tab_id {
            if current_id == tab_id {
                return;
            }
            if let Some(current_tab) = self.tabs.get(&current_id) {
                current_tab.hide();
            }
        }

        if let Some(new_tab) = self.tabs.get(&tab_id) {
            new_tab.show();
            self.active_tab_id = Some(tab_id);

            if let Ok(mut active_id) = self.active_tab_id_shared.lock() {
                *active_id = Some(tab_id);
            }

            if let Some(ref webview) = self.tab_bar_webview {
                let url = if let Ok(urls) = self.current_urls.lock() {
                    urls.get(&tab_id).cloned().unwrap_or_else(|| new_tab.get_url().to_string())
                } else {
                    new_tab.get_url().to_string()
                };
                let escaped_url = serde_json::to_string(&url).unwrap_or_else(|_| "\"\"".to_string());
                let script = format!(
                    "window.setActiveTab({}); window.updateUrlBar({});",
                    tab_id,
                    escaped_url
                );
                let _ = webview.evaluate_script(&script);
            }
        }
    }

    /// Closes the tab with the specified ID and switches to another tab if needed.
    pub fn close_tab(&mut self, tab_id: usize) {
        if let Some(tab) = self.tabs.remove(&tab_id) {
            drop(tab);

            if let Ok(mut urls) = self.current_urls.lock() {
                urls.remove(&tab_id);
            }

            if self.active_tab_id == Some(tab_id) {
                self.active_tab_id = None;

                if let Ok(mut active_id) = self.active_tab_id_shared.lock() {
                    *active_id = None;
                }

                let mut tab_ids: Vec<usize> = self.tabs.keys().copied().collect();
                tab_ids.sort();

                let next_tab_id = if let Some(pos) = tab_ids.iter().position(|&id| id > tab_id) {
                    if pos > 0 {
                        Some(tab_ids[pos - 1])
                    } else {
                        Some(tab_ids[pos])
                    }
                } else if !tab_ids.is_empty() {
                    Some(tab_ids[tab_ids.len() - 1])
                } else {
                    None
                };

                if let Some(next_id) = next_tab_id {
                    self.switch_to_tab(next_id);
                }
            }
        }
    }

    /// Reloads the currently active tab.
    pub fn reload_active_tab(&self) {
        if let Some(tab_id) = self.active_tab_id {
            if let Some(tab) = self.tabs.get(&tab_id) {
                let script = "window.location.reload();";
                let _ = tab.webview.evaluate_script(script);
            }
        }
    }

    /// Navigates the active tab backward in its history.
    pub fn navigate_back(&self) {
        if let Some(tab_id) = self.active_tab_id {
            if let Some(tab) = self.tabs.get(&tab_id) {
                let script = "window.history.back();";
                let _ = tab.webview.evaluate_script(script);
            }
        }
    }

    /// Navigates the active tab forward in its history.
    pub fn navigate_forward(&self) {
        if let Some(tab_id) = self.active_tab_id {
            if let Some(tab) = self.tabs.get(&tab_id) {
                let script = "window.history.forward();";
                let _ = tab.webview.evaluate_script(script);
            }
        }
    }

    /// Navigates the specified tab to a new URL.
    pub fn navigate_to(&mut self, tab_id: usize, url: &str) {
        if let Some(tab) = self.tabs.get_mut(&tab_id) {
            let redirected_url = url_cleaner::redirect_youtube_to_invidious(url, &self.config.borrow());
            let cleaned_url = url_cleaner::clean_url(&redirected_url).unwrap_or_else(|_| redirected_url.to_string());
            tab.set_url(cleaned_url.clone());
            let escaped_url = serde_json::to_string(&cleaned_url).unwrap_or_else(|_| "\"\"".to_string());
            let script = format!("window.location.href = {};", escaped_url);
            let _ = tab.webview.evaluate_script(&script);

            if let Some(ref webview) = self.tab_bar_webview {
                let update_script = format!("window.updateUrlBar({});", escaped_url);
                let _ = webview.evaluate_script(&update_script);
            }
        }
    }

    /// Returns the ID of the currently active tab, if any.
    pub fn get_active_tab_id(&self) -> Option<usize> {
        self.active_tab_id
    }

    /// Returns the total number of open tabs.
    pub fn get_tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Returns a reference to the active tab's webview if there is one.
    pub fn get_active_tab_webview(&self) -> Option<&wry::WebView> {
        self.active_tab_id.and_then(|id| self.tabs.get(&id).map(|tab| &tab.webview))
    }

    /// Resizes all tabs to fit the current window size.
    pub fn resize_all_tabs(&mut self, window: &Window) {
        if self.split_view.state().enabled {
            self.update_split_view_layout(window, None);
        } else {
            let window_size = window.inner_size();
            let content_width = window_size.width.saturating_sub(self.tab_sidebar_width);

            let bounds = Rect {
                position: LogicalPosition::new(self.tab_sidebar_width as i32, 0).into(),
                size: LogicalSize::new(content_width, window_size.height).into(),
            };

            for tab in self.tabs.values() {
                let _ = tab.webview.set_bounds(bounds);
            }
        }
    }

    /// Resizes all tabs to fit the window size minus the download sidebar width.
    pub fn resize_all_tabs_with_sidebar(&mut self, window: &Window, download_sidebar_width: u32) {
        if self.split_view.state().enabled {
            self.update_split_view_layout(window, Some(download_sidebar_width));
        } else {
            let window_size = window.inner_size();
            let content_width = window_size.width.saturating_sub(self.tab_sidebar_width).saturating_sub(download_sidebar_width);

            let bounds = Rect {
                position: LogicalPosition::new(self.tab_sidebar_width as i32, 0).into(),
                size: LogicalSize::new(content_width, window_size.height).into(),
            };

            for tab in self.tabs.values() {
                let _ = tab.webview.set_bounds(bounds);
            }
        }
    }

    /// Toggles split view mode on/off and returns whether it was enabled.
    pub fn toggle_split_view(&mut self, window: &Window) -> bool {
        let tab_ids: Vec<usize> = self.tabs.keys().copied().collect();
        let toggled = self.split_view.toggle_split_view(self.active_tab_id, &tab_ids);

        if toggled {
            self.update_split_view_layout(window, None);
        } else {
            self.resize_all_tabs(window);
        }

        toggled
    }

    /// Returns whether split view mode is currently enabled.
    pub fn is_split_view_enabled(&self) -> bool {
        self.split_view.state().enabled
    }

    /// Toggles the split view orientation between horizontal and vertical.
    pub fn toggle_split_orientation(&mut self, window: &Window) {
        if self.split_view.state().enabled {
            self.split_view.state_mut().toggle_orientation();
            self.update_split_view_layout(window, None);
        }
    }

    /// Swaps the primary and secondary panes in split view.
    pub fn swap_split_panes(&mut self) {
        if self.split_view.state().enabled {
            self.split_view.state_mut().swap_panes();
        }
    }

    /// Sets which pane is active in split view mode.
    pub fn set_split_active_pane(&mut self, pane: SplitPane) {
        if self.split_view.state().enabled {
            self.split_view.state_mut().set_active_pane(pane);
            if let Some(tab_id) = self.split_view.state().get_active_tab_id() {
                self.active_tab_id = Some(tab_id);
            }
        }
    }

    /// Updates the layout and bounds of split view panes.
    fn update_split_view_layout(&mut self, window: &Window, download_sidebar_width: Option<u32>) {
        let state = self.split_view.state();

        if !state.enabled {
            return;
        }

        let (primary_bounds, secondary_bounds) = state.calculate_bounds(
            window,
            self.tab_sidebar_width,
            download_sidebar_width,
        );

        for (tab_id, tab) in &self.tabs {
            if state.primary_tab_id == Some(*tab_id) {
                let _ = tab.webview.set_bounds(primary_bounds);
                tab.show();
            } else if state.secondary_tab_id == Some(*tab_id) {
                let _ = tab.webview.set_bounds(secondary_bounds);
                tab.show();
            } else {
                tab.hide();
            }
        }
    }

    /// Returns the current split view state with primary tab, secondary tab, and orientation.
    pub fn get_split_view_state(&self) -> Option<(usize, usize, SplitOrientation)> {
        let state = self.split_view.state();
        if state.enabled {
            if let (Some(primary), Some(secondary)) = (state.primary_tab_id, state.secondary_tab_id) {
                return Some((primary, secondary, state.orientation));
            }
        }
        None
    }
}
