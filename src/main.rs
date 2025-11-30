mod config;
mod debug;
mod downloads;
mod errors;
mod ipc;
mod privacy;
mod shortcuts;
mod single_instance;
mod tabs;
mod ui;
mod url_cleaner;
mod utils;
mod vimium_hints;
mod window;

use muda::{Menu, MenuItem, PredefinedMenuItem, Submenu, accelerator::Accelerator};
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};
use tao::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::ModifiersState,
    window::WindowId,
};

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
pub fn convert_file_url(url: &str) -> String {
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
    debug_log!(
        "Config loaded: vim_mode={}, debug={}",
        config.ui.vim_mode,
        config.ui.debug
    );

    if !single_instance::SingleInstance::is_single() {
        let url_to_send = if args.is_empty() {
            convert_file_url(&config.default_url)
        } else {
            let first_arg = &args[0];
            let url = if first_arg.starts_with("http://")
                || first_arg.starts_with("https://")
                || first_arg.starts_with("file://")
            {
                first_arg.clone()
            } else if first_arg.contains("://") {
                first_arg.clone()
            } else if first_arg.contains('.') && !first_arg.contains(' ') {
                format!("https://{}", first_arg)
            } else {
                let query = args.join(" ");
                config.format_search_url(&query)
            };
            convert_file_url(&url)
        };

        debug_log!("Sending URL to existing instance: {}", url_to_send);
        if let Err(e) = single_instance::SingleInstance::send_to_existing(&url_to_send) {
            eprintln!("Failed to send URL to existing instance: {}", e);
            return Ok(());
        }
        println!("Opened URL in existing Calm instance");
        return Ok(());
    }

    debug_log!("Starting new Calm instance");

    let event_loop = EventLoop::new();

    #[cfg(target_os = "macos")]
    let menu_items = {
        let menu_bar = Menu::new();

        let file_menu = Submenu::new("File", true);
        let new_tab_item = MenuItem::new(
            "New Tab",
            true,
            Some(Accelerator::new(Some(muda::accelerator::Modifiers::SUPER), muda::accelerator::Code::KeyT)),
        );
        let new_window_item = MenuItem::new(
            "New Window",
            true,
            Some(Accelerator::new(Some(muda::accelerator::Modifiers::SUPER), muda::accelerator::Code::KeyN)),
        );
        let close_tab_item = MenuItem::new(
            "Close Tab",
            true,
            Some(Accelerator::new(Some(muda::accelerator::Modifiers::SUPER), muda::accelerator::Code::KeyW)),
        );

        file_menu
            .append_items(&[
                &new_tab_item,
                &new_window_item,
                &PredefinedMenuItem::separator(),
                &close_tab_item,
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::quit(None),
            ])
            .expect("Failed to append File menu items");

        let edit_menu = Submenu::new("Edit", true);
        edit_menu
            .append_items(&[
                &PredefinedMenuItem::copy(None),
                &PredefinedMenuItem::cut(None),
                &PredefinedMenuItem::paste(None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::select_all(None),
            ])
            .expect("Failed to append Edit menu items");

        let view_menu = Submenu::new("View", true);
        let reload_item = MenuItem::new(
            "Reload",
            true,
            Some(Accelerator::new(Some(muda::accelerator::Modifiers::SUPER), muda::accelerator::Code::KeyR)),
        );
        let toggle_downloads_item = MenuItem::new(
            "Toggle Downloads",
            true,
            Some(Accelerator::new(Some(muda::accelerator::Modifiers::SUPER), muda::accelerator::Code::KeyJ)),
        );
        let toggle_split_view_item = MenuItem::new(
            "Toggle Split View",
            true,
            Some(Accelerator::new(Some(muda::accelerator::Modifiers::SUPER | muda::accelerator::Modifiers::SHIFT), muda::accelerator::Code::KeyS)),
        );

        view_menu
            .append_items(&[
                &reload_item,
                &PredefinedMenuItem::separator(),
                &toggle_downloads_item,
                &toggle_split_view_item,
            ])
            .expect("Failed to append View menu items");

        let navigate_menu = Submenu::new("Navigate", true);
        let focus_url_item = MenuItem::new(
            "Focus URL Bar",
            true,
            Some(Accelerator::new(Some(muda::accelerator::Modifiers::SUPER), muda::accelerator::Code::KeyL)),
        );
        let focus_sidebar_item = MenuItem::new(
            "Focus Sidebar",
            true,
            Some(Accelerator::new(Some(muda::accelerator::Modifiers::SUPER), muda::accelerator::Code::KeyE)),
        );
        let find_item = MenuItem::new(
            "Find in Page",
            true,
            Some(Accelerator::new(Some(muda::accelerator::Modifiers::SUPER), muda::accelerator::Code::KeyF)),
        );

        navigate_menu
            .append_items(&[
                &focus_url_item,
                &focus_sidebar_item,
                &PredefinedMenuItem::separator(),
                &find_item,
            ])
            .expect("Failed to append Navigate menu items");

        menu_bar
            .append(&file_menu)
            .expect("Failed to append File menu");
        menu_bar
            .append(&edit_menu)
            .expect("Failed to append Edit menu");
        menu_bar
            .append(&view_menu)
            .expect("Failed to append View menu");
        menu_bar
            .append(&navigate_menu)
            .expect("Failed to append Navigate menu");
        menu_bar.init_for_nsapp();

        (
            new_tab_item,
            new_window_item,
            close_tab_item,
            reload_item,
            toggle_downloads_item,
            toggle_split_view_item,
            focus_url_item,
            focus_sidebar_item,
            find_item,
        )
    };

    let (initial_url, use_welcome_html) = if args.is_empty() {
        (convert_file_url(&config.default_url), false)
    } else {
        let first_arg = &args[0];
        let url = if first_arg.starts_with("http://")
            || first_arg.starts_with("https://")
            || first_arg.starts_with("file://")
        {
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
    let modifiers_state = Rc::new(RefCell::new(ModifiersState::default()));
    let shortcut_manager = shortcuts::ShortcutManager::new();

    let url_receiver = match single_instance::SingleInstance::start_listener() {
        Ok(rx) => {
            debug_log!("IPC listener started successfully");
            Some(rx)
        }
        Err(e) => {
            eprintln!("Failed to start IPC listener: {}", e);
            None
        }
    };

    event_loop.run(move |event, event_loop_target, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[cfg(target_os = "macos")]
        if let Ok(menu_event) = muda::MenuEvent::receiver().try_recv() {
            debug_log!("Menu event received: {:?}", menu_event.id());

            if let Some(focused_id) = *focused_window_id.borrow() {
                let windows = windows_ref.borrow();
                if let Some(components) = windows.get(&focused_id) {
                    if menu_event.id() == menu_items.0.id() {
                        shortcut_manager.handle_shortcut(
                            shortcuts::Shortcut::NewTab,
                            components,
                            &config,
                            event_loop_target,
                            &windows_ref,
                            &focused_window_id,
                            control_flow,
                        );
                    } else if menu_event.id() == menu_items.1.id() {
                        shortcut_manager.handle_shortcut(
                            shortcuts::Shortcut::NewWindow,
                            components,
                            &config,
                            event_loop_target,
                            &windows_ref,
                            &focused_window_id,
                            control_flow,
                        );
                    } else if menu_event.id() == menu_items.2.id() {
                        shortcut_manager.handle_shortcut(
                            shortcuts::Shortcut::CloseTab,
                            components,
                            &config,
                            event_loop_target,
                            &windows_ref,
                            &focused_window_id,
                            control_flow,
                        );
                    } else if menu_event.id() == menu_items.3.id() {
                        shortcut_manager.handle_shortcut(
                            shortcuts::Shortcut::Reload,
                            components,
                            &config,
                            event_loop_target,
                            &windows_ref,
                            &focused_window_id,
                            control_flow,
                        );
                    } else if menu_event.id() == menu_items.4.id() {
                        shortcut_manager.handle_shortcut(
                            shortcuts::Shortcut::ToggleDownloads,
                            components,
                            &config,
                            event_loop_target,
                            &windows_ref,
                            &focused_window_id,
                            control_flow,
                        );
                    } else if menu_event.id() == menu_items.5.id() {
                        shortcut_manager.handle_shortcut(
                            shortcuts::Shortcut::ToggleSplitView,
                            components,
                            &config,
                            event_loop_target,
                            &windows_ref,
                            &focused_window_id,
                            control_flow,
                        );
                    } else if menu_event.id() == menu_items.6.id() {
                        shortcut_manager.handle_shortcut(
                            shortcuts::Shortcut::FocusUrlBar,
                            components,
                            &config,
                            event_loop_target,
                            &windows_ref,
                            &focused_window_id,
                            control_flow,
                        );
                    } else if menu_event.id() == menu_items.7.id() {
                        shortcut_manager.handle_shortcut(
                            shortcuts::Shortcut::FocusSidebar,
                            components,
                            &config,
                            event_loop_target,
                            &windows_ref,
                            &focused_window_id,
                            control_flow,
                        );
                    } else if menu_event.id() == menu_items.8.id() {
                        shortcut_manager.handle_shortcut(
                            shortcuts::Shortcut::Find,
                            components,
                            &config,
                            event_loop_target,
                            &windows_ref,
                            &focused_window_id,
                            control_flow,
                        );
                    }
                }
            }
        }

        if let Some(ref receiver) = url_receiver {
            while let Ok(url) = receiver.try_recv() {
                debug_log!("Received URL from another instance: {}", url);

                if let Some(focused_id) = *focused_window_id.borrow() {
                    let windows = windows_ref.borrow();
                    if let Some(components) = windows.get(&focused_id) {
                        match components.tab_manager.borrow_mut().create_tab(&components.window, &url) {
                            Ok(tab_id) => {
                                debug_log!("Created new tab {} for URL: {}", tab_id, url);
                                components.tab_manager.borrow_mut().switch_to_tab(tab_id);

                                let escaped_url = serde_json::to_string(&url).unwrap_or_else(|_| "\"\"".to_string());
                                let script = format!(
                                    "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
                                    tab_id, escaped_url, tab_id, escaped_url
                                );
                                let _ = components.tab_bar_webview.evaluate_script(&script);
                            }
                            Err(e) => {
                                debug_log!("Failed to create tab for URL: {:?}", e);
                            }
                        }
                    }
                }
            }
        }

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
                window_id: _window_id,
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
                        let modifiers = *modifiers_state.borrow();

                        if let Some(shortcut) = shortcuts::Shortcut::from_key_event(&key_event, modifiers) {
                            shortcut_manager.handle_shortcut(
                                shortcut,
                                components,
                                &config,
                                event_loop_target,
                                &windows_ref,
                                &focused_window_id,
                                control_flow,
                            );
                        } else {
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
            }
            _ => {}
        }
    });
}

/// Handles window resize events by updating bounds of all UI components.
///
/// # Arguments
///
/// * `components` - Browser window components to resize
#[allow(clippy::too_many_arguments)]
fn handle_window_resize(components: &BrowserWindowComponents) {
    let window_size = components.window.inner_size();
    let scale_factor = components.window.scale_factor();

    debug_log!(
        "Handling resize - size: {}x{}, scale: {}",
        window_size.width,
        window_size.height,
        scale_factor
    );

    let tab_bar_bounds = wry::Rect {
        position: tao::dpi::PhysicalPosition::new(0, 0).into(),
        size: tao::dpi::PhysicalSize::new(250.0 * scale_factor, window_size.height as f64).into(),
    };
    let _ = components.tab_bar_webview.set_bounds(tab_bar_bounds);

    let sidebar_x = window_size.width as i32 - (DOWNLOAD_SIDEBAR_WIDTH as f64 * scale_factor) as i32;
    let sidebar_bounds = wry::Rect {
        position: tao::dpi::PhysicalPosition::new(sidebar_x, 0).into(),
        size: tao::dpi::PhysicalSize::new(DOWNLOAD_SIDEBAR_WIDTH as f64 * scale_factor, window_size.height as f64).into(),
    };
    let _ = components.download_overlay.set_bounds(sidebar_bounds);

    let is_visible = *components.sidebar_visible.borrow();
    if is_visible {
        components
            .tab_manager
            .borrow_mut()
            .resize_all_tabs_with_sidebar(&components.window, DOWNLOAD_SIDEBAR_WIDTH as u32);
    } else {
        components
            .tab_manager
            .borrow_mut()
            .resize_all_tabs(&components.window);
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

    debug_log!(
        "Key pressed: {:?}, modifiers: ctrl={}, super={}, alt={}, shift={}",
        key_event.logical_key,
        modifiers.control_key(),
        modifiers.super_key(),
        modifiers.alt_key(),
        modifiers.shift_key()
    );

    if key_event.logical_key == tao::keyboard::Key::Enter {
        debug_log!("Enter key - focusing selected tab");
        let script =
            "if (window.focusedTabIndex >= 0) { window.focusTab(window.focusedTabIndex); }";
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
        _ => None,
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
