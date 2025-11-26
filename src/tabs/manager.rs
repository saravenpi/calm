use super::tab::Tab;
use crate::privacy;
use crate::url_cleaner;
use crate::config::Config;
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
    tab_bar_height: u32,
    tab_bar_webview: Option<std::rc::Rc<WebView>>,
    download_overlay: Option<std::rc::Rc<WebView>>,
    config: Config,
}

fn get_downloads_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join("Downloads")
}

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
    /// Creates a new TabManager instance with the specified tab bar height and configuration.
    pub fn new(tab_bar_height: u32, config: Config) -> Self {
        Self {
            tabs: HashMap::new(),
            active_tab_id: None,
            next_tab_id: 1,
            next_download_id: Arc::new(Mutex::new(1)),
            tab_bar_height,
            tab_bar_webview: None,
            download_overlay: None,
            config,
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

    fn create_tab_internal(&mut self, window: &Window, url: &str, html: Option<&str>) -> Result<usize, wry::Error> {
        if self.tabs.len() >= MAX_TABS {
            return Ok(self.active_tab_id.unwrap_or(1));
        }

        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;

        let window_size = window.inner_size();
        let content_height = window_size.height.saturating_sub(self.tab_bar_height);

        let bounds = Rect {
            position: LogicalPosition::new(0, self.tab_bar_height as i32).into(),
            size: LogicalSize::new(window_size.width, content_height).into(),
        };

        let cleaned_url = url_cleaner::clean_url(url).unwrap_or_else(|_| url.to_string());

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
            .with_initialization_script(&privacy::get_combined_privacy_script_with_config(&self.config.privacy))
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
                        _ => {}
                    }
                }
            })
            .build_as_child(window)?;

        let tab = Tab::new(tab_id, cleaned_url, webview);
        self.tabs.insert(tab_id, tab);

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

            if let Some(ref webview) = self.tab_bar_webview {
                let url = new_tab.get_url();
                let escaped_url = serde_json::to_string(url).unwrap_or_else(|_| "\"\"".to_string());
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

            if self.active_tab_id == Some(tab_id) {
                self.active_tab_id = None;

                if let Some((&next_id, _)) = self.tabs.iter().next() {
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
            let cleaned_url = url_cleaner::clean_url(url).unwrap_or_else(|_| url.to_string());
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

    /// Resizes all tabs to fit the current window size.
    pub fn resize_all_tabs(&mut self, window: &Window) {
        let window_size = window.inner_size();
        let content_height = window_size.height.saturating_sub(self.tab_bar_height);

        let bounds = Rect {
            position: LogicalPosition::new(0, self.tab_bar_height as i32).into(),
            size: LogicalSize::new(window_size.width, content_height).into(),
        };

        for tab in self.tabs.values() {
            let _ = tab.webview.set_bounds(bounds);
        }
    }

    /// Resizes all tabs to fit the window size minus the sidebar width.
    pub fn resize_all_tabs_with_sidebar(&mut self, window: &Window, sidebar_width: u32) {
        let window_size = window.inner_size();
        let content_height = window_size.height.saturating_sub(self.tab_bar_height);
        let content_width = window_size.width.saturating_sub(sidebar_width);

        let bounds = Rect {
            position: LogicalPosition::new(0, self.tab_bar_height as i32).into(),
            size: LogicalSize::new(content_width, content_height).into(),
        };

        for tab in self.tabs.values() {
            let _ = tab.webview.set_bounds(bounds);
        }
    }
}
