mod config;
mod privacy;
mod tabs;
mod ui;
mod url_cleaner;
mod vim_scroll;

use std::{cell::RefCell, rc::Rc};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use global_hotkey::{
    GlobalHotKeyManager, GlobalHotKeyEvent,
    hotkey::{HotKey, Modifiers, Code},
};

#[cfg(target_os = "macos")]
use tao::platform::macos::WindowBuilderExtMacOS;

use tabs::TabManager;
use config::Config;

/// Width of the tab sidebar UI element in pixels.
const TAB_SIDEBAR_WIDTH: u32 = 250;

/// Width of the download sidebar when visible, in pixels.
const DOWNLOAD_SIDEBAR_WIDTH: i32 = 360;

/// Converts file:// URLs to calmfile://localhost for custom protocol handling.
fn convert_file_url(url: &str) -> String {
    if url.starts_with("file://") {
        url.replace("file://", "calmfile://localhost")
    } else {
        url.to_string()
    }
}

fn main() -> wry::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if !args.is_empty() {
        let first_arg = &args[0];
        if first_arg == "--version" || first_arg == "-v" {
            println!("Calm Browser v0.2.0");
            println!("A privacy-focused minimalist web browser");
            println!("Built with Rust, TAO, and WRY");
            return Ok(());
        } else if first_arg == "--help" || first_arg == "-h" {
            println!("Calm Browser - Privacy-Focused Web Browser");
            println!();
            println!("USAGE:");
            println!("    calm                    Open default URL (configured in ~/.calm.yml)");
            println!("    calm <url>              Open specific URL");
            println!("    calm <search terms>     Search using configured search engine");
            println!();
            println!("OPTIONS:");
            println!("    -h, --help              Print this help information");
            println!("    -v, --version           Print version information");
            println!();
            println!("CONFIGURATION:");
            println!("    Edit ~/.calm.yml to configure default URL and search engine");
            println!();
            println!("EXAMPLES:");
            println!("    calm");
            println!("    calm https://example.com");
            println!("    calm rust programming");
            return Ok(());
        }
    }

    let config = Config::load();

    let event_loop = EventLoop::new();

    #[cfg(target_os = "macos")]
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Calm Browser - Privacy-Focused")
            .with_inner_size(tao::dpi::LogicalSize::new(1200, 800))
            .with_title_hidden(true)
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .build(&event_loop)
            .unwrap()
    );

    #[cfg(not(target_os = "macos"))]
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Calm Browser - Privacy-Focused")
            .with_inner_size(tao::dpi::LogicalSize::new(1200, 800))
            .build(&event_loop)
            .unwrap()
    );

    let (initial_url, use_welcome_html) = if args.is_empty() {
        ("".to_string(), true)
    } else {
        let first_arg = &args[0];
        let url = if first_arg.starts_with("http://") || first_arg.starts_with("https://") || first_arg.starts_with("file://") {
            first_arg.clone()
        } else if first_arg.contains("://") {
            first_arg.clone()
        } else if first_arg.contains('.') && !first_arg.contains(' ') {
            format!("https://{}", first_arg)
        } else {
            let query = args.join(" ");
            config.format_search_url(&query)
        };
        (convert_file_url(&url), false)
    };

    let config = Rc::new(RefCell::new(config));
    let tab_manager = Rc::new(RefCell::new(TabManager::new(TAB_SIDEBAR_WIDTH, config.borrow().clone())));
    let tab_bar_webview_ref: Rc<RefCell<Option<Rc<wry::WebView>>>> = Rc::new(RefCell::new(None));
    let download_overlay_ref: Rc<RefCell<Option<Rc<wry::WebView>>>> = Rc::new(RefCell::new(None));
    let sidebar_visible = Rc::new(RefCell::new(false));
    let should_quit = Rc::new(RefCell::new(false));

    let hotkey_manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");

    let cmd_or_ctrl = if cfg!(target_os = "macos") { Modifiers::SUPER } else { Modifiers::CONTROL };

    let hotkey_reload = HotKey::new(Some(cmd_or_ctrl), Code::KeyR);
    let hotkey_focus_url = HotKey::new(Some(cmd_or_ctrl), Code::KeyL);
    let hotkey_toggle_downloads = HotKey::new(Some(cmd_or_ctrl), Code::KeyJ);
    let hotkey_focus_sidebar = HotKey::new(Some(cmd_or_ctrl), Code::KeyE);
    let hotkey_quit = HotKey::new(Some(cmd_or_ctrl), Code::KeyQ);

    hotkey_manager.register(hotkey_reload).expect("Failed to register Cmd+R");
    hotkey_manager.register(hotkey_focus_url).expect("Failed to register Cmd+L");
    hotkey_manager.register(hotkey_toggle_downloads).expect("Failed to register Cmd+J");
    hotkey_manager.register(hotkey_focus_sidebar).expect("Failed to register Cmd+E");
    hotkey_manager.register(hotkey_quit).expect("Failed to register Cmd+Q");

    let window_size = window.inner_size();
    let tab_bar_webview = Rc::new(
        wry::WebViewBuilder::new()
            .with_html(&ui::get_complete_tab_bar_html())
            .with_transparent(true)
            .with_bounds(wry::Rect {
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
                move |request| {
                    let body = request.body();

                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(body) {
                        match data["action"].as_str() {
                            Some("switch_tab") => {
                                if let Some(tab_id) = data["tabId"].as_u64() {
                                    tab_manager.borrow_mut().switch_to_tab(tab_id as usize);
                                }
                            }
                            Some("close_tab") => {
                                if let Some(tab_id) = data["tabId"].as_u64() {
                                    let tab_count = tab_manager.borrow().get_tab_count();
                                    if tab_count == 1 {
                                        *should_quit.borrow_mut() = true;
                                    } else {
                                        tab_manager.borrow_mut().close_tab(tab_id as usize);
                                        if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                            let script = format!("window.removeTab({});", tab_id);
                                            let _ = webview.evaluate_script(&script);
                                        }
                                    }
                                }
                            }
                            Some("quit_app") => {
                                *should_quit.borrow_mut() = true;
                            }
                            Some("new_tab") => {
                                let welcome_html = ui::get_welcome_html();
                                let tab_result = tab_manager.borrow_mut().create_tab_with_html(&window, &welcome_html);
                                if let Ok(tab_id) = tab_result {
                                    tab_manager.borrow_mut().switch_to_tab(tab_id);
                                    if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                        let script = format!(
                                            "window.addTab({}, 'calm://welcome'); window.setActiveTab({}); window.updateUrlBar('');",
                                            tab_id,
                                            tab_id
                                        );
                                        let _ = webview.evaluate_script(&script);

                                        let focus_script = "document.getElementById('url-bar')?.focus();";
                                        let _ = webview.evaluate_script(focus_script);
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
                            Some("toggle_downloads") => {
                                let mut is_visible = sidebar_visible.borrow_mut();
                                *is_visible = !*is_visible;

                                if let Some(ref overlay) = *download_overlay_ref.borrow() {
                                    if *is_visible {
                                        let _ = overlay.set_visible(true);
                                        std::thread::sleep(std::time::Duration::from_millis(10));
                                        let script = "window.toggleVisibility(true);";
                                        let _ = overlay.evaluate_script(script);
                                        tab_manager.borrow_mut().resize_all_tabs_with_sidebar(&window, DOWNLOAD_SIDEBAR_WIDTH as u32);
                                    } else {
                                        let script = "window.toggleVisibility(false);";
                                        let _ = overlay.evaluate_script(script);
                                        std::thread::sleep(std::time::Duration::from_millis(300));
                                        let _ = overlay.set_visible(false);
                                        tab_manager.borrow_mut().resize_all_tabs(&window);
                                    }
                                }
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
                                    if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                                        let script = format!(
                                            "window.addTab({}, 'calm://settings'); window.setActiveTab({}); window.updateUrlBar('calm://settings');",
                                            tab_id,
                                            tab_id
                                        );
                                        let _ = webview.evaluate_script(&script);
                                    }
                                }
                            }
                            Some("load_settings") => {
                                if let Some(webview) = tab_manager.borrow().get_active_tab_webview() {
                                    let cfg = config.borrow();
                                    let settings_obj = serde_json::json!({
                                        "defaultUrl": cfg.default_url,
                                        "searchEngine": cfg.search_engine.split("?q=").next().unwrap_or("https://duckduckgo.com/"),
                                        "blockTrackers": cfg.privacy.tracking_domain_blocking,
                                        "blockFingerprinting": cfg.privacy.canvas_fingerprint_protection,
                                        "blockCookies": true,
                                    });
                                    let script = format!("if (window.updateSettings) {{ window.updateSettings({}); }}", settings_obj);
                                    let _ = webview.evaluate_script(&script);
                                }
                            }
                            Some("save_settings") => {
                                if let Some(settings) = data["settings"].as_object() {
                                    let mut cfg = config.borrow_mut();

                                    if let Some(default_url) = settings.get("defaultUrl").and_then(|v| v.as_str()) {
                                        cfg.default_url = default_url.to_string();
                                    }
                                    if let Some(search_engine) = settings.get("searchEngine").and_then(|v| v.as_str()) {
                                        cfg.search_engine = format!("{}{{}} ", search_engine);
                                    }
                                    if let Some(block_trackers) = settings.get("blockTrackers").and_then(|v| v.as_bool()) {
                                        cfg.privacy.tracking_domain_blocking = block_trackers;
                                    }
                                    if let Some(block_fp) = settings.get("blockFingerprinting").and_then(|v| v.as_bool()) {
                                        cfg.privacy.canvas_fingerprint_protection = block_fp;
                                        cfg.privacy.webgl_fingerprint_protection = block_fp;
                                        cfg.privacy.audio_fingerprint_protection = block_fp;
                                    }

                                    let _ = cfg.save();
                                }
                            }
                            Some("navigate_url") => {
                                if let Some(url_str) = data["url"].as_str() {
                                    let cfg = config.borrow();
                                    let url = if url_str.is_empty() {
                                        convert_file_url(&cfg.default_url)
                                    } else if url_str.contains("://") {
                                        convert_file_url(url_str)
                                    } else {
                                        let is_likely_url = url_str.contains('.') &&
                                                           !url_str.contains(' ') &&
                                                           (url_str.starts_with("localhost") ||
                                                            url_str.contains("..") == false &&
                                                            url_str.split('.').count() >= 2);

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
                                                let escaped_url = serde_json::to_string(&url).unwrap_or_else(|_| "\"\"".to_string());
                                                let script = format!(
                                                    "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
                                                    tab_id,
                                                    escaped_url,
                                                    tab_id,
                                                    escaped_url
                                                );
                                                let _ = webview.evaluate_script(&script);
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            })
            .build_as_child(window.as_ref())?
    );

    *tab_bar_webview_ref.borrow_mut() = Some(Rc::clone(&tab_bar_webview));
    tab_manager.borrow_mut().set_tab_bar_webview(Rc::clone(&tab_bar_webview));

    let download_overlay = Rc::new(
        wry::WebViewBuilder::new()
            .with_html(&ui::get_download_overlay_html())
            .with_bounds(wry::Rect {
                position: tao::dpi::LogicalPosition::new(
                    (window_size.width as i32) - DOWNLOAD_SIDEBAR_WIDTH,
                    0
                ).into(),
                size: tao::dpi::LogicalSize::new(DOWNLOAD_SIDEBAR_WIDTH as u32, window_size.height).into(),
            })
            .with_visible(false)
            .build_as_child(window.as_ref())?
    );

    *download_overlay_ref.borrow_mut() = Some(Rc::clone(&download_overlay));
    tab_manager.borrow_mut().set_download_overlay(Rc::clone(&download_overlay));

    {
        let mut manager = tab_manager.borrow_mut();
        let tab_result = if use_welcome_html {
            let welcome_html = ui::get_welcome_html();
            manager.create_tab_with_html(&window, &welcome_html)
        } else {
            manager.create_tab(&window, &initial_url)
        };

        match tab_result {
            Ok(tab_id) => {
                manager.switch_to_tab(tab_id);
                let display_url = if use_welcome_html {
                    "calm://welcome".to_string()
                } else {
                    initial_url.clone()
                };
                let escaped_url = serde_json::to_string(&display_url).unwrap_or_else(|_| "\"\"".to_string());
                let script = format!(
                    "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar('');",
                    tab_id,
                    escaped_url,
                    tab_id
                );
                let _ = tab_bar_webview.evaluate_script(&script);
            }
            Err(_) => {}
        }
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if *should_quit.borrow() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        if let Ok(global_hotkey_event) = GlobalHotKeyEvent::receiver().try_recv() {
            let hotkey_id = global_hotkey_event.id();

            loop {
                match GlobalHotKeyEvent::receiver().try_recv() {
                    Ok(next_event) if next_event.id() == hotkey_id => continue,
                    _ => break,
                }
            }

            if hotkey_id == hotkey_quit.id() {
                *control_flow = ControlFlow::Exit;
                return;
            } else if hotkey_id == hotkey_reload.id() {
                tab_manager.borrow().reload_active_tab();
            } else if hotkey_id == hotkey_focus_url.id() {
                if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                    let script = "const urlBar = document.getElementById('url-bar'); if (urlBar) { urlBar.focus(); urlBar.select(); }";
                    let _ = webview.evaluate_script(script);
                }
            } else if hotkey_id == hotkey_toggle_downloads.id() {
                let mut is_visible = sidebar_visible.borrow_mut();
                *is_visible = !*is_visible;

                if let Some(ref overlay) = *download_overlay_ref.borrow() {
                    if *is_visible {
                        let _ = overlay.set_visible(true);
                        std::thread::sleep(std::time::Duration::from_millis(10));
                        let script = "window.toggleVisibility(true);";
                        let _ = overlay.evaluate_script(script);
                        tab_manager.borrow_mut().resize_all_tabs_with_sidebar(&window, DOWNLOAD_SIDEBAR_WIDTH as u32);
                    } else {
                        let script = "window.toggleVisibility(false);";
                        let _ = overlay.evaluate_script(script);
                        std::thread::sleep(std::time::Duration::from_millis(300));
                        let _ = overlay.set_visible(false);
                        tab_manager.borrow_mut().resize_all_tabs(&window);
                    }
                }
            } else if hotkey_id == hotkey_focus_sidebar.id() {
                if let Some(ref webview) = *tab_bar_webview_ref.borrow() {
                    let script = "document.body.focus(); if (window.tabs.length > 0 && window.focusedTabIndex < 0) { window.updateFocusedTab(0); }";
                    let _ = webview.evaluate_script(script);
                }
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } | Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { .. },
                ..
            } | Event::WindowEvent {
                event: WindowEvent::Moved(_),
                ..
            } | Event::WindowEvent {
                event: WindowEvent::ThemeChanged(_),
                ..
            } => {
                let window_size = window.inner_size();
                let tab_bar_bounds = wry::Rect {
                    position: tao::dpi::LogicalPosition::new(0, 0).into(),
                    size: tao::dpi::LogicalSize::new(TAB_SIDEBAR_WIDTH, window_size.height).into(),
                };
                let _ = tab_bar_webview.set_bounds(tab_bar_bounds);

                let sidebar_x = (window_size.width as i32) - DOWNLOAD_SIDEBAR_WIDTH;
                let sidebar_bounds = wry::Rect {
                    position: tao::dpi::LogicalPosition::new(
                        sidebar_x,
                        0
                    ).into(),
                    size: tao::dpi::LogicalSize::new(DOWNLOAD_SIDEBAR_WIDTH as u32, window_size.height).into(),
                };
                let _ = download_overlay.set_bounds(sidebar_bounds);

                let is_visible = *sidebar_visible.borrow();
                if is_visible {
                    tab_manager.borrow_mut().resize_all_tabs_with_sidebar(&window, DOWNLOAD_SIDEBAR_WIDTH as u32);
                } else {
                    tab_manager.borrow_mut().resize_all_tabs(&window);
                }
            }
            _ => {}
        }
    });
}
