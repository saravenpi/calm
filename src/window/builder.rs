use crate::config::Config;
use crate::debug_log;
use crate::tabs::TabManager;
use crate::ui;
use std::cell::RefCell;
use std::rc::Rc;
use tao::{dpi::LogicalSize, event_loop::EventLoopWindowTarget, window::WindowBuilder};
use wry::{Rect, WebView, WebViewBuilder};

#[cfg(target_os = "macos")]
use tao::platform::macos::WindowBuilderExtMacOS;

const TAB_SIDEBAR_WIDTH: u32 = 250;
const DOWNLOAD_SIDEBAR_WIDTH: i32 = 360;

/// Components that make up a complete browser window.
pub struct BrowserWindowComponents {
    pub window: Rc<tao::window::Window>,
    pub tab_manager: Rc<RefCell<TabManager>>,
    pub tab_bar_webview: Rc<WebView>,
    pub download_overlay: Rc<WebView>,
    #[allow(dead_code)]
    pub command_prompt_overlay: Rc<RefCell<Option<WebView>>>,
    #[allow(dead_code)]
    pub command_prompt_visible: Rc<RefCell<bool>>,
    pub sidebar_visible: Rc<RefCell<bool>>,
    pub should_quit: Rc<RefCell<bool>>,
    pub toggle_downloads_debouncer: Rc<RefCell<crate::utils::debouncer::Debouncer>>,
    pub config: Rc<RefCell<Config>>,
}

/// Creates a new browser window with all necessary components including tab bar, download overlay, and initial tab.
///
/// # Arguments
///
/// * `event_loop` - The event loop window target
/// * `config` - Application configuration
/// * `initial_url` - URL to load in the first tab
/// * `use_welcome_html` - Whether to show welcome page instead of initial URL
///
/// # Returns
///
/// Browser window components on success
pub fn create_browser_window<T>(
    event_loop: &EventLoopWindowTarget<T>,
    config: Rc<RefCell<Config>>,
    initial_url: String,
    use_welcome_html: bool,
) -> wry::Result<BrowserWindowComponents> {
    #[cfg(target_os = "macos")]
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Calm Browser - Privacy-Focused")
            .with_inner_size(LogicalSize::new(1200, 800))
            .with_title_hidden(true)
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .build(event_loop)
            .unwrap(),
    );

    #[cfg(not(target_os = "macos"))]
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Calm Browser - Privacy-Focused")
            .with_inner_size(LogicalSize::new(1200, 800))
            .build(event_loop)
            .unwrap(),
    );

    let tab_manager = Rc::new(RefCell::new(TabManager::new(
        TAB_SIDEBAR_WIDTH,
        Rc::clone(&config),
    )));
    let tab_bar_webview_ref: Rc<RefCell<Option<Rc<WebView>>>> = Rc::new(RefCell::new(None));
    let download_overlay_ref: Rc<RefCell<Option<Rc<WebView>>>> = Rc::new(RefCell::new(None));
    let sidebar_visible = Rc::new(RefCell::new(false));
    let should_quit = Rc::new(RefCell::new(false));
    let toggle_downloads_debouncer =
        Rc::new(RefCell::new(crate::utils::debouncer::Debouncer::new(500)));
    let command_prompt_visible = Rc::new(RefCell::new(false));
    let command_prompt_overlay_ref: Rc<RefCell<Option<WebView>>> = Rc::new(RefCell::new(None));

    let window_size = window.inner_size();

    let tab_bar_webview = Rc::new(
        WebViewBuilder::new()
            .with_html(ui::get_complete_tab_bar_html(config.borrow().ui.vim_mode, config.borrow().ui.sounds))
            .with_transparent(true)
            .with_bounds(Rect {
                position: tao::dpi::LogicalPosition::new(0, 0).into(),
                size: tao::dpi::LogicalSize::new(TAB_SIDEBAR_WIDTH, window_size.height).into(),
            })
            .with_ipc_handler({
                let tab_manager = Rc::clone(&tab_manager);
                let window = Rc::clone(&window);
                let tab_bar_webview_ref = Rc::clone(&tab_bar_webview_ref);
                let download_overlay_ref = Rc::clone(&download_overlay_ref);
                let sidebar_visible = Rc::clone(&sidebar_visible);
                let config = Rc::clone(&config);
                let should_quit = Rc::clone(&should_quit);
                let toggle_downloads_debouncer = Rc::clone(&toggle_downloads_debouncer);
                let command_prompt_visible = Rc::clone(&command_prompt_visible);
                let command_prompt_overlay_ref = Rc::clone(&command_prompt_overlay_ref);
                move |request| {
                    let body = request.body();

                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(body) {
                        match data["action"].as_str() {
                            Some("switch_tab") => {
                                if let Some(tab_id) = data["tabId"].as_u64() {
                                    tab_manager.borrow_mut().switch_to_tab(tab_id as usize);

                                    let is_in_split = tab_manager.borrow().is_tab_in_split_group(tab_id as usize);
                                    if is_in_split {
                                        tab_manager.borrow_mut().update_split_view_layout(&window, None);
                                    } else {
                                        tab_manager.borrow_mut().resize_all_tabs(&window);
                                    }

                                    if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                        let ui_state = tab_manager.borrow().get_split_ui_state();
                                        let orientation_str = ui_state.active_group_orientation.as_deref().unwrap_or("vertical");
                                        let groups_json = tab_manager.borrow().get_split_groups_json();
                                        let script = format!(
                                            "if (window.updateSplitUIState) {{ window.updateSplitUIState({}, {}, '{}'); }} if (window.setSplitGroups) {{ window.setSplitGroups({}); }}",
                                            ui_state.active_tab_in_split,
                                            ui_state.can_create_split,
                                            orientation_str,
                                            groups_json
                                        );
                                        let _ = webview.evaluate_script(&script);
                                    }
                                }
                            }
                            Some("close_tab") => {
                                if let Some(tab_id) = data["tabId"].as_u64() {
                                    debug_log!("=== IPC close_tab received for tab ID: {} ===", tab_id);
                                    let tab_count = tab_manager.borrow().get_tab_count();
                                    debug_log!("Current tab count: {}", tab_count);
                                    if tab_count == 1 {
                                        debug_log!("Last tab - closing window");
                                        *should_quit.borrow_mut() = true;
                                    } else {
                                        tab_manager.borrow_mut().close_tab(tab_id as usize);
                                        if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                            let script = format!("window.removeTab({});", tab_id);
                                            let _ = webview.evaluate_script(&script);
                                        }
                                        debug_log!("Tab {} closed via IPC", tab_id);
                                    }
                                }
                            }
                            Some("quit_app") => {
                                *should_quit.borrow_mut() = true;
                            }
                            Some("new_tab") => {
                                debug_log!("=== IPC new_tab action received ===");
                                let tab_count_before = tab_manager.borrow().get_tab_count();
                                debug_log!("IPC Tab count before: {}", tab_count_before);

                                let default_url = crate::convert_file_url(&config.borrow().default_url);
                                let tab_result =
                                    tab_manager.borrow_mut().create_tab(&window, &default_url);
                                if let Ok(tab_id) = tab_result {
                                    debug_log!("IPC Created tab with ID: {}", tab_id);
                                    tab_manager.borrow_mut().switch_to_tab(tab_id);
                                    if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                        let escaped_url = serde_json::to_string(&default_url).unwrap_or_else(|_| "\"\"".to_string());
                                        let script = format!(
                                            "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
                                            tab_id, escaped_url, tab_id, escaped_url
                                        );
                                        let _ = webview.evaluate_script(&script);

                                        let focus_script =
                                            "document.getElementById('url-bar')?.focus();";
                                        let _ = webview.evaluate_script(focus_script);
                                    }
                                    let tab_count_after = tab_manager.borrow().get_tab_count();
                                    debug_log!("IPC Tab count after: {}", tab_count_after);
                                }
                            }
                            Some("open_url_new_tab") => {
                                if let Some(url) = data["url"].as_str() {
                                    let tab_result = tab_manager.borrow_mut().create_tab(&window, url);
                                    if let Ok(tab_id) = tab_result {
                                        tab_manager.borrow_mut().switch_to_tab(tab_id);
                                        if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                            let escaped_url = serde_json::to_string(&url)
                                                .unwrap_or_else(|_| "\"\"".to_string());
                                            let script = format!(
                                                "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
                                                tab_id, escaped_url, tab_id, escaped_url
                                            );
                                            let _ = webview.evaluate_script(&script);
                                        }
                                    }
                                }
                            }
                            Some("reload_tab") => {
                                tab_manager.borrow().reload_active_tab();
                            }
                            Some("navigate_back") => {
                                tab_manager.borrow().navigate_back();
                            }
                            Some("navigate_forward") => {
                                tab_manager.borrow().navigate_forward();
                            }
                            Some("inspect_element") => {
                                tab_manager.borrow_mut().open_devtools_for_active_tab(&window);
                            }
                            Some("inspect_element_tab") => {
                                if let Some(tab_id) = data["tabId"].as_u64() {
                                    tab_manager.borrow_mut().open_devtools_for_tab(tab_id as usize, &window);
                                }
                            }
                            Some("toggle_downloads") => {
                                if toggle_downloads_debouncer.borrow_mut().should_execute() {
                                    debug_log!("=== IPC toggle_downloads FIRED ===");

                                    let should_show = {
                                        let mut is_visible = sidebar_visible.borrow_mut();
                                        *is_visible = !*is_visible;
                                        debug_log!("Download manager sidebar_visible toggled to: {}", *is_visible);
                                        *is_visible
                                    };

                                    if let Some(ref overlay) = *download_overlay_ref.borrow() {
                                        if should_show {
                                            debug_log!("Opening download manager");
                                            let _ = overlay.set_visible(true);
                                            std::thread::sleep(std::time::Duration::from_millis(10));
                                            let script = "window.toggleVisibility(true);";
                                            let _ = overlay.evaluate_script(script);
                                            tab_manager.borrow_mut().resize_all_tabs_with_sidebar(
                                                &window,
                                                DOWNLOAD_SIDEBAR_WIDTH as u32,
                                            );
                                        } else {
                                            debug_log!("Closing download manager");
                                            let script = "window.toggleVisibility(false);";
                                            let _ = overlay.evaluate_script(script);
                                            std::thread::sleep(std::time::Duration::from_millis(300));
                                            let _ = overlay.set_visible(false);
                                            tab_manager.borrow_mut().resize_all_tabs(&window);
                                        }
                                    }
                                }
                            }
                            Some("reveal_in_finder") => {
                                if let Some(file_path) = data["filePath"].as_str() {
                                    debug_log!("Revealing in Finder: {}", file_path);
                                    #[cfg(target_os = "macos")]
                                    {
                                        use std::process::Command;
                                        let _ = Command::new("open")
                                            .arg("-R")
                                            .arg(file_path)
                                            .spawn();
                                    }
                                }
                            }
                            Some("clear_download_history") => {
                                debug_log!("Clearing download history");
                                tab_manager.borrow_mut().clear_download_history();
                            }
                            Some("focus_url_bar") => {
                                let script = "const urlBar = document.getElementById('url-bar'); if (urlBar) { urlBar.focus(); urlBar.select(); }";
                                let _ = match &*tab_bar_webview_ref.borrow() {
                                    Some(ref webview) => webview.evaluate_script(script),
                                    None => Ok(()),
                                };
                            }
                            Some("open_settings") => {
                                let settings_html = ui::get_settings_html();
                                let tab_result = tab_manager.borrow_mut().create_tab_with_html(&window, &settings_html);
                                if let Ok(tab_id) = tab_result {
                                    tab_manager.borrow_mut().switch_to_tab(tab_id);

                                    let is_in_split = tab_manager.borrow().is_tab_in_split_group(tab_id);
                                    if is_in_split {
                                        tab_manager.borrow_mut().update_split_view_layout(&window, None);
                                    } else {
                                        tab_manager.borrow_mut().resize_all_tabs(&window);
                                    }

                                    if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                        let script = format!(
                                            "window.addTab({}, 'calm://settings'); window.setActiveTab({}); window.updateUrlBar('calm://settings');",
                                            tab_id, tab_id
                                        );
                                        let _ = webview.evaluate_script(&script);

                                        let ui_state = tab_manager.borrow().get_split_ui_state();
                                        let orientation_str = ui_state.active_group_orientation.as_deref().unwrap_or("vertical");
                                        let groups_json = tab_manager.borrow().get_split_groups_json();
                                        let state_script = format!(
                                            "if (window.updateSplitUIState) {{ window.updateSplitUIState({}, {}, '{}'); }} if (window.setSplitGroups) {{ window.setSplitGroups({}); }}",
                                            ui_state.active_tab_in_split,
                                            ui_state.can_create_split,
                                            orientation_str,
                                            groups_json
                                        );
                                        let _ = webview.evaluate_script(&state_script);
                                    }
                                }
                            }
                            Some("load_settings") => {
                                debug_log!("=== load_settings IPC received ===");
                                if let Some(webview) = tab_manager.borrow().get_active_tab_webview() {
                                    let cfg = config.borrow();
                                    let settings_obj = serde_json::json!({
                                        "defaultUrl": cfg.default_url,
                                        "searchEngine": cfg.search_engine,
                                        "vimMode": cfg.ui.vim_mode,
                                        "uiSounds": cfg.ui.sounds,
                                        "blockTrackers": cfg.privacy.tracking_domain_blocking,
                                        "blockFingerprinting": cfg.privacy.canvas_fingerprint_protection,
                                        "blockCookies": true,
                                    });
                                    debug_log!("Settings to send: {:?}", settings_obj);
                                    let script = format!(
                                        "if (window.updateSettings) {{ window.updateSettings({}); }}",
                                        settings_obj
                                    );
                                    debug_log!("Executing script: {}", &script[..100.min(script.len())]);
                                    let _ = webview.evaluate_script(&script);
                                } else {
                                    debug_log!("ERROR: No active webview found for load_settings");
                                }
                            }
                            Some("save_settings") => {
                                debug_log!("=== save_settings IPC received ===");
                                if let Some(settings) = data["settings"].as_object() {
                                    debug_log!("Settings received: {:?}", settings);
                                    let mut cfg = config.borrow_mut();

                                    if let Some(default_url) =
                                        settings.get("defaultUrl").and_then(|v| v.as_str())
                                    {
                                        debug_log!("Setting default_url to: {}", default_url);
                                        cfg.default_url = default_url.to_string();
                                    }
                                    if let Some(search_engine) =
                                        settings.get("searchEngine").and_then(|v| v.as_str())
                                    {
                                        debug_log!("Setting search_engine to: {}", search_engine);
                                        cfg.search_engine = search_engine.to_string();
                                    }
                                    if let Some(vim_mode) =
                                        settings.get("vimMode").and_then(|v| v.as_bool())
                                    {
                                        debug_log!("Setting vim_mode to: {}", vim_mode);
                                        cfg.ui.vim_mode = vim_mode;
                                    }
                                    if let Some(ui_sounds) =
                                        settings.get("uiSounds").and_then(|v| v.as_bool())
                                    {
                                        debug_log!("Setting ui_sounds to: {}", ui_sounds);
                                        cfg.ui.sounds = ui_sounds;
                                    }
                                    if let Some(block_trackers) =
                                        settings.get("blockTrackers").and_then(|v| v.as_bool())
                                    {
                                        debug_log!("Setting block_trackers to: {}", block_trackers);
                                        cfg.privacy.tracking_domain_blocking = block_trackers;
                                    }
                                    if let Some(block_fp) =
                                        settings.get("blockFingerprinting").and_then(|v| v.as_bool())
                                    {
                                        debug_log!("Setting block_fingerprinting to: {}", block_fp);
                                        cfg.privacy.canvas_fingerprint_protection = block_fp;
                                        cfg.privacy.webgl_fingerprint_protection = block_fp;
                                        cfg.privacy.audio_fingerprint_protection = block_fp;
                                    }

                                    match cfg.save() {
                                        Ok(_) => debug_log!("Settings saved successfully to ~/.calm.yml"),
                                        Err(e) => debug_log!("ERROR: Failed to save settings: {:?}", e),
                                    }

                                    if let Some(ui_sounds) =
                                        settings.get("uiSounds").and_then(|v| v.as_bool())
                                    {
                                        let script = if ui_sounds {
                                            "window.uiSoundsEnabled = true;"
                                        } else {
                                            "window.uiSoundsEnabled = false; window.playUISound = function() {};"
                                        };
                                        if let Some(webview) = tab_bar_webview_ref.borrow().as_ref() {
                                            let _ = webview.evaluate_script(script);
                                            debug_log!("Updated UI sounds setting to: {}", ui_sounds);
                                        }
                                    }
                                } else {
                                    debug_log!("ERROR: No settings object in save_settings message");
                                }
                            }
                            Some("navigate_url") => {
                                if let Some(url_str) = data["url"].as_str() {
                                    let cfg = config.borrow();
                                    let url = if url_str.is_empty() {
                                        crate::convert_file_url(&cfg.default_url)
                                    } else if url_str.contains("://") {
                                        crate::convert_file_url(url_str)
                                    } else {
                                        let is_likely_url = url_str.contains('.')
                                            && !url_str.contains(' ')
                                            && (url_str.starts_with("localhost")
                                                || !url_str.contains("..")
                                                    && url_str.split('.').count() >= 2);

                                        if is_likely_url {
                                            format!("https://{}", url_str)
                                        } else {
                                            cfg.format_search_url(url_str)
                                        }
                                    };
                                    drop(cfg);

                                    let active_tab_id = tab_manager.borrow().get_active_tab_id();
                                    if let Some(tab_id) = active_tab_id {
                                        tab_manager.borrow_mut().navigate_to(tab_id, &url);
                                    } else {
                                        let tab_result = tab_manager.borrow_mut().create_tab(&window, &url);
                                        if let Ok(tab_id) = tab_result {
                                            tab_manager.borrow_mut().switch_to_tab(tab_id);
                                            if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                                let escaped_url = serde_json::to_string(&url)
                                                    .unwrap_or_else(|_| "\"\"".to_string());
                                                let script = format!(
                                                    "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
                                                    tab_id, escaped_url, tab_id, escaped_url
                                                );
                                                let _ = webview.evaluate_script(&script);
                                            }
                                        }
                                    }
                                }
                            }
                            Some("toggle_split_view") => {
                                debug_log!("=== IPC toggle_split_view action received ===");
                                let _toggled = tab_manager.borrow_mut().toggle_split_view(&window);

                                if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                    let ui_state = tab_manager.borrow().get_split_ui_state();
                                    let orientation_str = ui_state.active_group_orientation.as_deref().unwrap_or("vertical");
                                    let groups_json = tab_manager.borrow().get_split_groups_json();
                                    let script = format!(
                                        "if (window.updateSplitUIState) {{ window.updateSplitUIState({}, {}, '{}'); }} if (window.setSplitGroups) {{ window.setSplitGroups({}); }}",
                                        ui_state.active_tab_in_split,
                                        ui_state.can_create_split,
                                        orientation_str,
                                        groups_json
                                    );
                                    let _ = webview.evaluate_script(&script);
                                    let _ = webview.evaluate_script("window.refreshTabs();");
                                }
                            }
                            Some("toggle_split_orientation") => {
                                debug_log!("=== IPC toggle_split_orientation action received ===");
                                tab_manager.borrow_mut().toggle_split_orientation(&window);
                                if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                    let ui_state = tab_manager.borrow().get_split_ui_state();
                                    let orientation_str = ui_state.active_group_orientation.as_deref().unwrap_or("vertical");
                                    let groups_json = tab_manager.borrow().get_split_groups_json();
                                    let script = format!(
                                        "if (window.updateSplitUIState) {{ window.updateSplitUIState({}, {}, '{}'); }} if (window.setSplitGroups) {{ window.setSplitGroups({}); }}",
                                        ui_state.active_tab_in_split,
                                        ui_state.can_create_split,
                                        orientation_str,
                                        groups_json
                                    );
                                    let _ = webview.evaluate_script(&script);
                                    let _ = webview.evaluate_script("window.refreshTabs();");
                                }
                            }
                            Some("swap_split_panes") => {
                                debug_log!("=== IPC swap_split_panes action received ===");
                                tab_manager.borrow_mut().swap_split_panes(&window);
                                if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                    let ui_state = tab_manager.borrow().get_split_ui_state();
                                    let orientation_str = ui_state.active_group_orientation.as_deref().unwrap_or("vertical");
                                    let groups_json = tab_manager.borrow().get_split_groups_json();
                                    let script = format!(
                                        "if (window.updateSplitUIState) {{ window.updateSplitUIState({}, {}, '{}'); }} if (window.setSplitGroups) {{ window.setSplitGroups({}); }}",
                                        ui_state.active_tab_in_split,
                                        ui_state.can_create_split,
                                        orientation_str,
                                        groups_json
                                    );
                                    let _ = webview.evaluate_script(&script);
                                    let _ = webview.evaluate_script("window.refreshTabs();");
                                }
                            }
                            Some("keyboard_shortcut") => {
                                if let Some(shortcut) = data["shortcut"].as_str() {
                                    debug_log!("=== IPC keyboard_shortcut '{}' received ===", shortcut);
                                    match shortcut {
                                        "reload" => {
                                            tab_manager.borrow().reload_active_tab();
                                        }
                                        "focus_url" => {
                                            let script = "const urlBar = document.getElementById('url-bar'); if (urlBar) { urlBar.focus(); urlBar.select(); }";
                                            if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                                let _ = webview.evaluate_script(script);
                                            }
                                        }
                                        "toggle_downloads" => {
                                            if toggle_downloads_debouncer.borrow_mut().should_execute() {
                                                let should_show = {
                                                    let mut is_visible = sidebar_visible.borrow_mut();
                                                    *is_visible = !*is_visible;
                                                    *is_visible
                                                };

                                                if let Some(ref overlay) = *download_overlay_ref.borrow() {
                                                    if should_show {
                                                        let _ = overlay.set_visible(true);
                                                        std::thread::sleep(std::time::Duration::from_millis(10));
                                                        let script = "window.toggleVisibility(true);";
                                                        let _ = overlay.evaluate_script(script);
                                                        tab_manager.borrow_mut().resize_all_tabs_with_sidebar(
                                                            &window,
                                                            DOWNLOAD_SIDEBAR_WIDTH as u32,
                                                        );
                                                    } else {
                                                        let script = "window.toggleVisibility(false);";
                                                        let _ = overlay.evaluate_script(script);
                                                        std::thread::sleep(std::time::Duration::from_millis(300));
                                                        let _ = overlay.set_visible(false);
                                                        tab_manager.borrow_mut().resize_all_tabs(&window);
                                                    }
                                                }
                                            }
                                        }
                                        "focus_sidebar" => {
                                            if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                                let _ = webview.focus();
                                                let script = "window.showSidebarFocus(); if (window.tabs.length > 0) { if (window.focusedTabIndex < 0) { window.updateFocusedTab(0); } else { window.updateFocusedTab(window.focusedTabIndex); } }";
                                                let _ = webview.evaluate_script(script);
                                            }
                                        }
                                        "find" => {
                                            if let Some(active_webview) = tab_manager.borrow().get_active_tab_webview() {
                                                let _ = active_webview.evaluate_script("window.calmStartSearch();");
                                            }
                                        }
                                        "new_tab" => {
                                            let is_visible = *command_prompt_visible.borrow();

                                            if is_visible {
                                                *command_prompt_visible.borrow_mut() = false;
                                                *command_prompt_overlay_ref.borrow_mut() = None;
                                            } else {
                                                let window_size = window.inner_size();
                                                let command_prompt_visible_for_ipc = Rc::clone(&command_prompt_visible);
                                                let command_prompt_overlay_for_ipc = Rc::clone(&command_prompt_overlay_ref);
                                                let tab_manager_for_prompt = Rc::clone(&tab_manager);
                                                let config_for_prompt = Rc::clone(&config);
                                                let window_for_prompt = Rc::clone(&window);
                                                let tab_bar_for_prompt = Rc::clone(&tab_bar_webview_ref);

                                                match wry::WebViewBuilder::new()
                                                    .with_html(ui::get_command_prompt_html())
                                                    .with_bounds(wry::Rect {
                                                        position: tao::dpi::LogicalPosition::new(0, 0).into(),
                                                        size: tao::dpi::LogicalSize::new(window_size.width, window_size.height).into(),
                                                    })
                                                    .with_transparent(true)
                                                    .with_ipc_handler(move |request| {
                                                        let body = request.body();
                                                        if let Ok(data) = serde_json::from_str::<serde_json::Value>(body) {
                                                            match data["action"].as_str() {
                                                                Some("hide_command_prompt") => {
                                                                    *command_prompt_visible_for_ipc.borrow_mut() = false;
                                                                    *command_prompt_overlay_for_ipc.borrow_mut() = None;
                                                                }
                                                                Some("command_prompt_navigate") => {
                                                                    if let Some(url_str) = data["url"].as_str() {
                                                                        let cfg = config_for_prompt.borrow();
                                                                        let url = if url_str.is_empty() {
                                                                            crate::convert_file_url(&cfg.default_url)
                                                                        } else if url_str.contains("://") {
                                                                            crate::convert_file_url(url_str)
                                                                        } else {
                                                                            let is_likely_url = url_str.contains('.')
                                                                                && !url_str.contains(' ')
                                                                                && (url_str.starts_with("localhost")
                                                                                    || !url_str.contains("..")
                                                                                        && url_str.split('.').count() >= 2);

                                                                            if is_likely_url {
                                                                                format!("https://{}", url_str)
                                                                            } else {
                                                                                cfg.format_search_url(url_str)
                                                                            }
                                                                        };
                                                                        drop(cfg);

                                                                        *command_prompt_visible_for_ipc.borrow_mut() = false;
                                                                        *command_prompt_overlay_for_ipc.borrow_mut() = None;

                                                                        let tab_result = tab_manager_for_prompt.borrow_mut().create_tab(&window_for_prompt, &url);
                                                                        if let Ok(tab_id) = tab_result {
                                                                            tab_manager_for_prompt.borrow_mut().switch_to_tab(tab_id);

                                                                            if let Some(ref webview) = *tab_bar_for_prompt.borrow() {
                                                                                let escaped_url = serde_json::to_string(&url).unwrap_or_else(|_| "\"\"".to_string());
                                                                                let script = format!(
                                                                                    "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
                                                                                    tab_id, escaped_url, tab_id, escaped_url
                                                                                );
                                                                                let _ = webview.evaluate_script(&script);
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                    })
                                                    .build_as_child(window.as_ref())
                                                {
                                                    Ok(webview) => {
                                                        let history = tab_manager.borrow().get_history();
                                                        let history_entries = history.borrow().get_recent(100);
                                                        let history_json = serde_json::to_string(&history_entries).unwrap_or_else(|_| "[]".to_string());
                                                        let script = format!("if (window.setHistory) {{ window.setHistory({}); }}", history_json);
                                                        let _ = webview.evaluate_script(&script);

                                                        *command_prompt_overlay_ref.borrow_mut() = Some(webview);
                                                        *command_prompt_visible.borrow_mut() = true;
                                                    }
                                                    Err(e) => {
                                                        debug_log!("Failed to create command prompt overlay: {:?}", e);
                                                    }
                                                }
                                            }
                                        }
                                        "close_tab" => {
                                            let tab_count = tab_manager.borrow().get_tab_count();
                                            if tab_count <= 1 {
                                                *should_quit.borrow_mut() = true;
                                            } else {
                                                let active_tab_id = tab_manager.borrow().get_active_tab_id();
                                                if let Some(active_tab_id) = active_tab_id {
                                                    tab_manager.borrow_mut().close_tab(active_tab_id);
                                                    if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                                        let script = format!("window.removeTab({});", active_tab_id);
                                                        let _ = webview.evaluate_script(&script);
                                                    }
                                                }
                                            }
                                        }
                                        "new_window" => {
                                            debug_log!("New window shortcut not supported from IPC (requires event loop)");
                                        }
                                        "toggle_split_view" => {
                                            let _ = tab_manager.borrow_mut().toggle_split_view(&window);
                                            if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                                let (enabled, primary_id, secondary_id, orientation) = tab_manager.borrow().get_split_view_state();
                                                let primary_str = primary_id.map_or("null".to_string(), |id| id.to_string());
                                                let secondary_str = secondary_id.map_or("null".to_string(), |id| id.to_string());
                                                let script = format!(
                                                    "if (window.updateSplitViewButtons) {{ window.updateSplitViewButtons({}); }} if (window.setSplitViewState) {{ window.setSplitViewState({}, {}, {}, '{}'); }}",
                                                    enabled, enabled, primary_str, secondary_str, orientation
                                                );
                                                let _ = webview.evaluate_script(&script);
                                            }
                                        }
                                        "quit" => {
                                            *should_quit.borrow_mut() = true;
                                        }
                                        _ => {
                                            debug_log!("Unknown keyboard shortcut: {}", shortcut);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            })
            .build_as_child(window.as_ref())?,
    );

    *tab_bar_webview_ref.borrow_mut() = Some(Rc::clone(&tab_bar_webview));
    tab_manager
        .borrow_mut()
        .set_tab_bar_webview(Rc::clone(&tab_bar_webview));

    let tab_bar_for_downloads = Rc::clone(&tab_bar_webview);
    let download_overlay = Rc::new(
        WebViewBuilder::new()
            .with_html(ui::get_download_overlay_html())
            .with_bounds(Rect {
                position: tao::dpi::LogicalPosition::new(
                    (window_size.width as i32) - DOWNLOAD_SIDEBAR_WIDTH,
                    0,
                )
                .into(),
                size: tao::dpi::LogicalSize::new(DOWNLOAD_SIDEBAR_WIDTH as u32, window_size.height)
                    .into(),
            })
            .with_visible(false)
            .with_ipc_handler(move |request| {
                let body = request.body();
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(body) {
                    match data["action"].as_str() {
                        Some("update_download_progress") => {
                            if let Some(percent) = data["percent"].as_i64() {
                                let script = format!(
                                    "if (window.updateDownloadProgress) {{ window.updateDownloadProgress({}); }}",
                                    percent
                                );
                                let _ = tab_bar_for_downloads.evaluate_script(&script);
                            }
                        }
                        Some("update_download_count") => {
                            if let Some(count) = data["count"].as_i64() {
                                let in_progress = data["inProgress"].as_bool().unwrap_or(false);
                                let script = format!(
                                    "if (window.updateDownloadCount) {{ window.updateDownloadCount({}, {}); }}",
                                    count, in_progress
                                );
                                let _ = tab_bar_for_downloads.evaluate_script(&script);
                            }
                        }
                        Some("reveal_in_finder") => {
                            if let Some(file_path) = data["filePath"].as_str() {
                                #[cfg(target_os = "macos")]
                                {
                                    use std::process::Command;
                                    let _ = Command::new("open")
                                        .arg("-R")
                                        .arg(file_path)
                                        .spawn();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            })
            .build_as_child(window.as_ref())?,
    );

    *download_overlay_ref.borrow_mut() = Some(Rc::clone(&download_overlay));
    tab_manager
        .borrow_mut()
        .set_download_overlay(Rc::clone(&download_overlay));

    {
        let manager = tab_manager.borrow();
        let history = manager.get_download_history();
        if !history.downloads.is_empty() {
            let downloads_json =
                serde_json::to_string(&history.downloads).unwrap_or_else(|_| "[]".to_string());
            let script = format!(
                "if (window.loadDownloadHistory) {{ window.loadDownloadHistory({}); }}",
                downloads_json
            );
            let _ = download_overlay.evaluate_script(&script);
            debug_log!("Loaded {} downloads from history", history.downloads.len());
        }
    }

    {
        let mut manager = tab_manager.borrow_mut();
        let tab_result = if use_welcome_html {
            let welcome_html = ui::get_welcome_html();
            manager.create_tab_with_html(&window, &welcome_html)
        } else {
            manager.create_tab(&window, &initial_url)
        };

        if let Ok(tab_id) = tab_result {
            manager.switch_to_tab(tab_id);
            let display_url = if use_welcome_html {
                "calm://welcome".to_string()
            } else {
                initial_url.clone()
            };
            let escaped_url =
                serde_json::to_string(&display_url).unwrap_or_else(|_| "\"\"".to_string());
            let url_bar_display = if use_welcome_html {
                "''".to_string()
            } else {
                escaped_url.clone()
            };
            let script = format!(
                "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
                tab_id, escaped_url, tab_id, url_bar_display
            );
            let _ = tab_bar_webview.evaluate_script(&script);

            let ui_state = manager.get_split_ui_state();
            let orientation_str = ui_state
                .active_group_orientation
                .as_deref()
                .unwrap_or("vertical");
            let state_script = format!(
                "if (window.updateSplitUIState) {{ window.updateSplitUIState({}, {}, '{}'); }}",
                ui_state.active_tab_in_split, ui_state.can_create_split, orientation_str
            );
            let _ = tab_bar_webview.evaluate_script(&state_script);
        }
    }

    Ok(BrowserWindowComponents {
        window,
        tab_manager,
        tab_bar_webview,
        download_overlay,
        command_prompt_overlay: command_prompt_overlay_ref,
        command_prompt_visible,
        sidebar_visible,
        should_quit,
        toggle_downloads_debouncer,
        config,
    })
}
