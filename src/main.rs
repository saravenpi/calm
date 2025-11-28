mod config;
mod debug;
mod privacy;
mod tabs;
mod ui;
mod url_cleaner;
mod vimium_hints;
mod window;

use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};
use tao::{
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    keyboard::ModifiersState,
    window::WindowId,
};
use global_hotkey::{
    GlobalHotKeyManager, GlobalHotKeyEvent,
    hotkey::{HotKey, Modifiers, Code},
};
use muda::{Menu, Submenu, PredefinedMenuItem};

use config::Config;
use window::{create_browser_window, BrowserWindowComponents};

const DOWNLOAD_SIDEBAR_WIDTH: i32 = 360;

/// Converts file:// URLs to calmfile://localhost URLs for custom protocol handling.
///
/// # Arguments
///
/// * `url` - The URL string to convert
///
/// # Returns
///
/// The converted URL string
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
            println!("Calm Browser v0.3.0");
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
            println!("KEYBOARD SHORTCUTS:");
            println!("    Cmd+N                   Open new window");
            println!("    Cmd+T                   Open new tab");
            println!("    Cmd+W                   Close tab/window");
            println!("    Cmd+L                   Focus URL bar");
            println!("    Cmd+R                   Reload page");
            println!();
            println!("EXAMPLES:");
            println!("    calm");
            println!("    calm https://example.com");
            println!("    calm rust programming");
            return Ok(());
        }
    }

    let config = Config::load();
    debug::set_debug_enabled(config.ui.debug);
    debug_log!("Debug mode enabled");
    debug_log!("Config loaded: vim_mode={}, debug={}", config.ui.vim_mode, config.ui.debug);

    let event_loop = EventLoop::new();

    #[cfg(target_os = "macos")]
    {
        let menu_bar = Menu::new();

        let edit_menu = Submenu::new("Edit", true);
        edit_menu.append_items(&[
            &PredefinedMenuItem::copy(None),
            &PredefinedMenuItem::cut(None),
            &PredefinedMenuItem::paste(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::select_all(None),
        ]).expect("Failed to append Edit menu items");

        menu_bar.append(&edit_menu).expect("Failed to append Edit menu");
        menu_bar.init_for_nsapp();
    }

    let (initial_url, use_welcome_html) = if args.is_empty() {
        (convert_file_url(&config.default_url), false)
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

    let mut windows: HashMap<WindowId, BrowserWindowComponents> = HashMap::new();
    let focused_window_id = Rc::new(RefCell::new(None::<WindowId>));

    let first_window = create_browser_window(
        &event_loop,
        Rc::clone(&config),
        initial_url.clone(),
        use_welcome_html,
    )?;

    let first_window_id = first_window.window.id();
    *focused_window_id.borrow_mut() = Some(first_window_id);
    windows.insert(first_window_id, first_window);

    let windows_ref = Rc::new(RefCell::new(windows));
    let last_g_key_time = Rc::new(RefCell::new(None::<Instant>));
    let last_new_tab_time = Rc::new(RefCell::new(None::<Instant>));
    let last_close_tab_time = Rc::new(RefCell::new(None::<Instant>));
    let last_new_window_time = Rc::new(RefCell::new(None::<Instant>));
    let last_toggle_downloads_time = Rc::new(RefCell::new(None::<Instant>));
    let modifiers_state = Rc::new(RefCell::new(ModifiersState::default()));

    let hotkey_manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");
    let cmd_or_ctrl = if cfg!(target_os = "macos") { Modifiers::SUPER } else { Modifiers::CONTROL };

    let hotkey_reload = HotKey::new(Some(cmd_or_ctrl), Code::KeyR);
    let hotkey_focus_url = HotKey::new(Some(cmd_or_ctrl), Code::KeyL);
    let hotkey_toggle_downloads = HotKey::new(Some(cmd_or_ctrl), Code::KeyJ);
    let hotkey_focus_sidebar = HotKey::new(Some(cmd_or_ctrl), Code::KeyE);
    let hotkey_find = HotKey::new(Some(cmd_or_ctrl), Code::KeyF);
    let hotkey_new_tab = HotKey::new(Some(cmd_or_ctrl), Code::KeyT);
    let hotkey_close_tab = HotKey::new(Some(cmd_or_ctrl), Code::KeyW);
    let hotkey_new_window = HotKey::new(Some(cmd_or_ctrl), Code::KeyN);
    let hotkey_split_view = HotKey::new(Some(cmd_or_ctrl | Modifiers::SHIFT), Code::KeyS);
    let hotkey_quit = HotKey::new(Some(cmd_or_ctrl), Code::KeyQ);

    hotkey_manager.register(hotkey_reload).expect("Failed to register Cmd+R");
    hotkey_manager.register(hotkey_focus_url).expect("Failed to register Cmd+L");
    hotkey_manager.register(hotkey_toggle_downloads).expect("Failed to register Cmd+J");
    hotkey_manager.register(hotkey_focus_sidebar).expect("Failed to register Cmd+E");
    hotkey_manager.register(hotkey_find).expect("Failed to register Cmd+F");
    hotkey_manager.register(hotkey_new_tab).expect("Failed to register Cmd+T");
    hotkey_manager.register(hotkey_close_tab).expect("Failed to register Cmd+W");
    hotkey_manager.register(hotkey_new_window).expect("Failed to register Cmd+N");
    hotkey_manager.register(hotkey_split_view).expect("Failed to register Cmd+Shift+S");
    hotkey_manager.register(hotkey_quit).expect("Failed to register Cmd+Q");

    event_loop.run(move |event, event_loop_target, control_flow| {
        *control_flow = ControlFlow::Wait;

        let mut windows_to_close = Vec::new();
        for (window_id, components) in windows_ref.borrow().iter() {
            if *components.should_quit.borrow() {
                windows_to_close.push(*window_id);
            }
        }

        for window_id in windows_to_close {
            debug_log!("Closing window: {:?}", window_id);
            windows_ref.borrow_mut().remove(&window_id);
        }

        if windows_ref.borrow().is_empty() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        if let Ok(global_hotkey_event) = GlobalHotKeyEvent::receiver().try_recv() {
            let hotkey_id = global_hotkey_event.id();
            debug_log!("GlobalHotKey event received: id={}", hotkey_id);

            let mut drained_count = 0;
            loop {
                match GlobalHotKeyEvent::receiver().try_recv() {
                    Ok(next_event) if next_event.id() == hotkey_id => {
                        drained_count += 1;
                        continue;
                    },
                    _ => break,
                }
            }
            if drained_count > 0 {
                debug_log!("Drained {} duplicate events for hotkey {}", drained_count, hotkey_id);
            }

            if hotkey_id == hotkey_quit.id() {
                *control_flow = ControlFlow::Exit;
                return;
            } else if hotkey_id == hotkey_new_window.id() {
                let now = Instant::now();
                let should_execute = {
                    let mut last_time = last_new_window_time.borrow_mut();
                    if let Some(last) = *last_time {
                        let elapsed = now.duration_since(last).as_millis();
                        if elapsed < 250 {
                            debug_log!("Cmd+N DEBOUNCED - only {}ms since last, IGNORING", elapsed);
                            false
                        } else {
                            *last_time = Some(now);
                            true
                        }
                    } else {
                        *last_time = Some(now);
                        true
                    }
                };

                if should_execute {
                    debug_log!("=== Cmd+N GlobalHotKey FIRED - creating new window ===");
                    let default_url = convert_file_url(&config.borrow().default_url);
                    match create_browser_window(
                        event_loop_target,
                        Rc::clone(&config),
                        default_url,
                        false,
                    ) {
                        Ok(new_window) => {
                            let new_window_id = new_window.window.id();
                            debug_log!("Created new window: {:?}", new_window_id);
                            *focused_window_id.borrow_mut() = Some(new_window_id);
                            windows_ref.borrow_mut().insert(new_window_id, new_window);
                        }
                        Err(e) => {
                            debug_log!("Failed to create new window: {:?}", e);
                        }
                    }
                }
            } else {
                if let Some(focused_id) = *focused_window_id.borrow() {
                    let windows = windows_ref.borrow();
                    if let Some(components) = windows.get(&focused_id) {
                        handle_hotkey(
                            hotkey_id,
                            &hotkey_reload,
                            &hotkey_focus_url,
                            &hotkey_toggle_downloads,
                            &hotkey_focus_sidebar,
                            &hotkey_find,
                            &hotkey_new_tab,
                            &hotkey_close_tab,
                            &hotkey_split_view,
                            components,
                            &config,
                            &last_new_tab_time,
                            &last_close_tab_time,
                            &last_toggle_downloads_time,
                        );
                    }
                }
            }
        }

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
                ..
            } => {
                debug_log!("Window close requested: {:?}", window_id);
                windows_ref.borrow_mut().remove(&window_id);
                if windows_ref.borrow().is_empty() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Focused(focused),
                ..
            } => {
                if focused {
                    debug_log!("Window focused: {:?}", window_id);
                    *focused_window_id.borrow_mut() = Some(window_id);
                }
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                debug_log!("Window resized: {}x{}", new_size.width, new_size.height);
                let windows = windows_ref.borrow();
                if let Some(components) = windows.get(&window_id) {
                    handle_window_resize(components);
                }
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                ..
            } => {
                debug_log!("Scale factor changed: {}", scale_factor);
                let windows = windows_ref.borrow();
                if let Some(components) = windows.get(&window_id) {
                    handle_window_resize(components);
                }
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Moved(position),
                ..
            } => {
                debug_log!("Window moved to: {:?}", position);
                let windows = windows_ref.borrow();
                if let Some(components) = windows.get(&window_id) {
                    handle_window_resize(components);
                }
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::ThemeChanged(theme),
                ..
            } => {
                debug_log!("Theme changed: {:?}", theme);
                let windows = windows_ref.borrow();
                if let Some(components) = windows.get(&window_id) {
                    handle_window_resize(components);
                }
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::ModifiersChanged(new_state),
                ..
            } => {
                *modifiers_state.borrow_mut() = new_state;
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::KeyboardInput {
                    event: key_event,
                    ..
                },
                ..
            } => {
                let windows = windows_ref.borrow();
                if let Some(components) = windows.get(&window_id) {
                    if key_event.state == ElementState::Pressed {
                        handle_keyboard_input(
                            key_event,
                            components,
                            &modifiers_state,
                            &config,
                            &last_g_key_time,
                        );
                    }
                }
            }
            _ => {}
        }
    });
}

/// Handles global hotkey events and dispatches them to appropriate actions.
///
/// # Arguments
///
/// * `hotkey_id` - The ID of the triggered hotkey
/// * `hotkey_reload` - HotKey for page reload
/// * `hotkey_focus_url` - HotKey for URL bar focus
/// * `hotkey_toggle_downloads` - HotKey for downloads sidebar
/// * `hotkey_focus_sidebar` - HotKey for sidebar focus
/// * `hotkey_find` - HotKey for find/search
/// * `hotkey_new_tab` - HotKey for new tab creation
/// * `hotkey_close_tab` - HotKey for tab closing
/// * `hotkey_split_view` - HotKey for split view toggle
/// * `components` - Browser window components
/// * `config` - Application configuration
/// * `last_new_tab_time` - Timestamp of last new tab action for debouncing
/// * `last_close_tab_time` - Timestamp of last close tab action for debouncing
/// * `last_toggle_downloads_time` - Timestamp of last toggle downloads action for debouncing
fn handle_hotkey(
    hotkey_id: u32,
    hotkey_reload: &HotKey,
    hotkey_focus_url: &HotKey,
    hotkey_toggle_downloads: &HotKey,
    hotkey_focus_sidebar: &HotKey,
    hotkey_find: &HotKey,
    hotkey_new_tab: &HotKey,
    hotkey_close_tab: &HotKey,
    hotkey_split_view: &HotKey,
    components: &BrowserWindowComponents,
    config: &Rc<RefCell<Config>>,
    last_new_tab_time: &Rc<RefCell<Option<Instant>>>,
    last_close_tab_time: &Rc<RefCell<Option<Instant>>>,
    last_toggle_downloads_time: &Rc<RefCell<Option<Instant>>>,
) {
    if hotkey_id == hotkey_reload.id() {
        components.tab_manager.borrow().reload_active_tab();
    } else if hotkey_id == hotkey_split_view.id() {
        debug_log!("=== Cmd+Shift+S GlobalHotKey FIRED - toggling split view ===");
        let toggled = components.tab_manager.borrow_mut().toggle_split_view(&components.window);
        if toggled {
            debug_log!("Split view enabled");
        } else {
            debug_log!("Split view disabled");
        }
        let enabled = components.tab_manager.borrow().is_split_view_enabled();
        let script = format!("if (window.updateSplitViewButtons) {{ window.updateSplitViewButtons({}); }}", enabled);
        let _ = components.tab_bar_webview.evaluate_script(&script);
    } else if hotkey_id == hotkey_focus_url.id() {
        let script = "const urlBar = document.getElementById('url-bar'); if (urlBar) { urlBar.focus(); urlBar.select(); }";
        let _ = components.tab_bar_webview.evaluate_script(script);
    } else if hotkey_id == hotkey_toggle_downloads.id() {
        let now = Instant::now();
        let should_execute = {
            let mut last_time = last_toggle_downloads_time.borrow_mut();
            if let Some(last) = *last_time {
                let elapsed = now.duration_since(last).as_millis();
                if elapsed < 250 {
                    debug_log!("Cmd+J DEBOUNCED - only {}ms since last, IGNORING", elapsed);
                    false
                } else {
                    *last_time = Some(now);
                    true
                }
            } else {
                *last_time = Some(now);
                true
            }
        };

        if should_execute {
            debug_log!("=== Cmd+J GlobalHotKey FIRED - toggling downloads sidebar ===");
            let mut is_visible = components.sidebar_visible.borrow_mut();
            *is_visible = !*is_visible;

            if *is_visible {
                let _ = components.download_overlay.set_visible(true);
                std::thread::sleep(std::time::Duration::from_millis(10));
                let script = "window.toggleVisibility(true);";
                let _ = components.download_overlay.evaluate_script(script);
                components.tab_manager.borrow_mut().resize_all_tabs_with_sidebar(
                    &components.window,
                    DOWNLOAD_SIDEBAR_WIDTH as u32,
                );
            } else {
                let script = "window.toggleVisibility(false);";
                let _ = components.download_overlay.evaluate_script(script);
                std::thread::sleep(std::time::Duration::from_millis(300));
                let _ = components.download_overlay.set_visible(false);
                components.tab_manager.borrow_mut().resize_all_tabs(&components.window);
            }
        }
    } else if hotkey_id == hotkey_focus_sidebar.id() {
        let _ = components.tab_bar_webview.focus();
        let script = "window.showSidebarFocus(); if (window.tabs.length > 0) { if (window.focusedTabIndex < 0) { window.updateFocusedTab(0); } else { window.updateFocusedTab(window.focusedTabIndex); } }";
        let _ = components.tab_bar_webview.evaluate_script(script);
    } else if hotkey_id == hotkey_find.id() {
        if let Some(active_webview) = components.tab_manager.borrow().get_active_tab_webview() {
            let _ = active_webview.evaluate_script("window.calmStartSearch();");
        }
    } else if hotkey_id == hotkey_new_tab.id() {
        let now = Instant::now();
        let should_execute = {
            let mut last_time = last_new_tab_time.borrow_mut();
            if let Some(last) = *last_time {
                let elapsed = now.duration_since(last).as_millis();
                if elapsed < 250 {
                    debug_log!("Cmd+T DEBOUNCED - only {}ms since last, IGNORING", elapsed);
                    false
                } else {
                    *last_time = Some(now);
                    true
                }
            } else {
                *last_time = Some(now);
                true
            }
        };

        if should_execute {
            debug_log!("=== Cmd+T GlobalHotKey FIRED - creating new tab ===");
            let tab_count_before = components.tab_manager.borrow().get_tab_count();
            debug_log!("Tab count before: {}", tab_count_before);

            let default_url = convert_file_url(&config.borrow().default_url);
            let tab_result = components.tab_manager.borrow_mut().create_tab(&components.window, &default_url);
            if let Ok(tab_id) = tab_result {
                debug_log!("Created tab with ID: {}", tab_id);
                components.tab_manager.borrow_mut().switch_to_tab(tab_id);
                let escaped_url = serde_json::to_string(&default_url).unwrap_or_else(|_| "\"\"".to_string());
                let script = format!(
                    "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
                    tab_id, escaped_url, tab_id, escaped_url
                );
                let _ = components.tab_bar_webview.evaluate_script(&script);
                let focus_script = "document.getElementById('url-bar')?.focus();";
                let _ = components.tab_bar_webview.evaluate_script(focus_script);
                let tab_count_after = components.tab_manager.borrow().get_tab_count();
                debug_log!("Tab count after: {}", tab_count_after);
            } else {
                debug_log!("ERROR: Failed to create tab");
            }
        }
    } else if hotkey_id == hotkey_close_tab.id() {
        let now = Instant::now();
        let should_execute = {
            let mut last_time = last_close_tab_time.borrow_mut();
            if let Some(last) = *last_time {
                let elapsed = now.duration_since(last).as_millis();
                if elapsed < 250 {
                    debug_log!("=== Cmd+W DEBOUNCED - only {}ms since last, IGNORING ===", elapsed);
                    false
                } else {
                    debug_log!("=== Cmd+W allowed - {}ms since last ===", elapsed);
                    *last_time = Some(now);
                    true
                }
            } else {
                debug_log!("=== Cmd+W first press - executing ===");
                *last_time = Some(now);
                true
            }
        };

        if should_execute {
            debug_log!("=== Cmd+W GlobalHotKey EXECUTING - closing ONE tab ===");

            let tab_count = components.tab_manager.borrow().get_tab_count();
            let active_tab_id = components.tab_manager.borrow().get_active_tab_id();

            debug_log!("Tab count before close: {}", tab_count);

            if tab_count <= 1 {
                debug_log!("Last tab - closing window");
                *components.should_quit.borrow_mut() = true;
            } else {
                if let Some(active_tab_id) = active_tab_id {
                    debug_log!("Closing ONLY tab ID: {}", active_tab_id);

                    components.tab_manager.borrow_mut().close_tab(active_tab_id);

                    let script = format!("window.removeTab({});", active_tab_id);
                    let _ = components.tab_bar_webview.evaluate_script(&script);

                    let remaining = components.tab_manager.borrow().get_tab_count();
                    debug_log!("Tab {} closed, remaining tabs: {}", active_tab_id, remaining);
                } else {
                    debug_log!("ERROR: No active tab found");
                }
            }
        }
    }
}

/// Handles window resize events by adjusting bounds of tab bar, download sidebar, and tab webviews.
///
/// # Arguments
///
/// * `components` - Browser window components to resize
fn handle_window_resize(components: &BrowserWindowComponents) {
    let window_size = components.window.inner_size();
    let scale_factor = components.window.scale_factor();

    debug_log!("Handling resize - size: {}x{}, scale: {}",
               window_size.width, window_size.height, scale_factor);

    let tab_bar_bounds = wry::Rect {
        position: tao::dpi::LogicalPosition::new(0, 0).into(),
        size: tao::dpi::LogicalSize::new(250, window_size.height).into(),
    };
    let _ = components.tab_bar_webview.set_bounds(tab_bar_bounds);

    let sidebar_x = (window_size.width as i32) - DOWNLOAD_SIDEBAR_WIDTH;
    let sidebar_bounds = wry::Rect {
        position: tao::dpi::LogicalPosition::new(sidebar_x, 0).into(),
        size: tao::dpi::LogicalSize::new(DOWNLOAD_SIDEBAR_WIDTH as u32, window_size.height).into(),
    };
    let _ = components.download_overlay.set_bounds(sidebar_bounds);

    let is_visible = *components.sidebar_visible.borrow();
    if is_visible {
        components.tab_manager.borrow_mut().resize_all_tabs_with_sidebar(&components.window, DOWNLOAD_SIDEBAR_WIDTH as u32);
    } else {
        components.tab_manager.borrow_mut().resize_all_tabs(&components.window);
    }
}

/// Handles keyboard input events for vim-style navigation and controls.
///
/// # Arguments
///
/// * `key_event` - The keyboard event
/// * `components` - Browser window components
/// * `modifiers_state` - Current state of modifier keys
/// * `config` - Application configuration
/// * `last_g_key_time` - Timestamp for 'gg' detection (double-g to scroll to top)
fn handle_keyboard_input(
    key_event: tao::event::KeyEvent,
    components: &BrowserWindowComponents,
    modifiers_state: &Rc<RefCell<ModifiersState>>,
    config: &Rc<RefCell<Config>>,
    last_g_key_time: &Rc<RefCell<Option<Instant>>>,
) {
    let modifiers = *modifiers_state.borrow();

    debug_log!("Key pressed: {:?}, modifiers: ctrl={}, super={}, alt={}, shift={}",
        key_event.logical_key,
        modifiers.control_key(),
        modifiers.super_key(),
        modifiers.alt_key(),
        modifiers.shift_key()
    );

    if key_event.logical_key == tao::keyboard::Key::Enter {
        debug_log!("Enter key - focusing selected tab");
        let script = "if (window.focusedTabIndex >= 0) { window.focusTab(window.focusedTabIndex); }";
        let _ = components.tab_bar_webview.evaluate_script(script);
        return;
    }

    if !config.borrow().ui.vim_mode {
        debug_log!("Vim mode disabled, skipping");
        return;
    }

    let action_script = match &key_event.logical_key {
        tao::keyboard::Key::Character(c) => {
            let ch = c.to_string();
            let ch_lower = ch.to_lowercase();

            match ch_lower.as_str() {
                "j" => Some("if (document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA' && document.activeElement.tagName !== 'SELECT' && !document.activeElement.isContentEditable && !(window.calmGetHintMode && window.calmGetHintMode()) && !(window.calmIsSearchMode && window.calmIsSearchMode())) { window.scrollBy({top: 60, behavior: 'smooth'}); }"),
                "k" => Some("if (document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA' && document.activeElement.tagName !== 'SELECT' && !document.activeElement.isContentEditable && !(window.calmGetHintMode && window.calmGetHintMode()) && !(window.calmIsSearchMode && window.calmIsSearchMode())) { window.scrollBy({top: -60, behavior: 'smooth'}); }"),
                "h" => Some("if (document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA' && document.activeElement.tagName !== 'SELECT' && !document.activeElement.isContentEditable && !(window.calmGetHintMode && window.calmGetHintMode()) && !(window.calmIsSearchMode && window.calmIsSearchMode())) { window.scrollBy({left: -40, behavior: 'smooth'}); }"),
                "l" => Some("if (document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA' && document.activeElement.tagName !== 'SELECT' && !document.activeElement.isContentEditable && !(window.calmGetHintMode && window.calmGetHintMode()) && !(window.calmIsSearchMode && window.calmIsSearchMode())) { window.scrollBy({left: 40, behavior: 'smooth'}); }"),
                "g" => {
                    let is_uppercase = ch.chars().next().unwrap().is_uppercase();
                    if is_uppercase {
                        Some("if (document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA' && document.activeElement.tagName !== 'SELECT' && !document.activeElement.isContentEditable && !(window.calmGetHintMode && window.calmGetHintMode()) && !(window.calmIsSearchMode && window.calmIsSearchMode())) { window.scrollTo({top: document.documentElement.scrollHeight, behavior: 'smooth'}); }")
                    } else {
                        let mut last_time = last_g_key_time.borrow_mut();
                        let now = Instant::now();

                        if let Some(last) = *last_time {
                            if now.duration_since(last).as_millis() < 500 {
                                *last_time = None;
                                Some("if (document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA' && document.activeElement.tagName !== 'SELECT' && !document.activeElement.isContentEditable && !(window.calmGetHintMode && window.calmGetHintMode()) && !(window.calmIsSearchMode && window.calmIsSearchMode())) { window.scrollTo({top: 0, behavior: 'smooth'}); }")
                            } else {
                                *last_time = Some(now);
                                None
                            }
                        } else {
                            *last_time = Some(now);
                            None
                        }
                    }
                }
                "f" => {
                    let is_uppercase = ch.chars().next().unwrap().is_uppercase();
                    if is_uppercase {
                        Some("if (document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA' && document.activeElement.tagName !== 'SELECT' && !document.activeElement.isContentEditable && window.calmShowHints) { window.calmShowHints(true); }")
                    } else {
                        Some("if (document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA' && document.activeElement.tagName !== 'SELECT' && !document.activeElement.isContentEditable && window.calmShowHints) { window.calmShowHints(false); }")
                    }
                }
                "/" => Some("if (document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA' && document.activeElement.tagName !== 'SELECT' && !document.activeElement.isContentEditable && !(window.calmGetHintMode && window.calmGetHintMode()) && window.calmStartSearch) { window.calmStartSearch(); }"),
                _ => None
            }
        }
        _ => None
    };

    if let Some(script) = action_script {
        debug_log!("Executing script: {}", &script[..100.min(script.len())]);
        if let Some(active_webview) = components.tab_manager.borrow().get_active_tab_webview() {
            let result = active_webview.evaluate_script(script);
            debug_log!("Script result: {:?}", result);
        } else {
            debug_log!("No active webview found!");
        }
    } else {
        debug_log!("No action script for this key");
    }
}
