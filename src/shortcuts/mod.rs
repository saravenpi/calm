use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tao::{
    event::KeyEvent,
    event_loop::{ControlFlow, EventLoopWindowTarget},
    keyboard::{Key, ModifiersState},
    window::WindowId,
};

use crate::{config::Config, utils::debouncer::Debouncer, window::BrowserWindowComponents};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shortcut {
    Reload,
    FocusUrlBar,
    ToggleDownloads,
    FocusSidebar,
    Find,
    NewTab,
    CloseTab,
    NewWindow,
    ToggleSplitView,
    Quit,
}

impl Shortcut {
    pub fn from_key_event(key_event: &KeyEvent, modifiers: ModifiersState) -> Option<Self> {
        let is_cmd_or_ctrl = if cfg!(target_os = "macos") {
            modifiers.super_key()
        } else {
            modifiers.control_key()
        };

        crate::debug_log!(
            "Key event: {:?}, Cmd/Ctrl: {}, Shift: {}, Alt: {}",
            key_event.logical_key,
            is_cmd_or_ctrl,
            modifiers.shift_key(),
            modifiers.alt_key()
        );

        if !is_cmd_or_ctrl || modifiers.alt_key() {
            return None;
        }

        if let Key::Character(c) = &key_event.logical_key {
            let ch = c.to_lowercase();
            let shift = modifiers.shift_key();

            crate::debug_log!("Matching shortcut: char='{}', shift={}", ch, shift);

            let result = match (ch.as_str(), shift) {
                ("r", false) => Some(Self::Reload),
                ("l", false) => Some(Self::FocusUrlBar),
                ("j", false) => Some(Self::ToggleDownloads),
                ("e", false) => Some(Self::FocusSidebar),
                ("f", false) => Some(Self::Find),
                ("t", false) => Some(Self::NewTab),
                ("w", false) => Some(Self::CloseTab),
                ("n", false) => Some(Self::NewWindow),
                ("s", true) => Some(Self::ToggleSplitView),
                ("q", false) => Some(Self::Quit),
                _ => None,
            };

            if let Some(ref shortcut) = result {
                crate::debug_log!("Matched shortcut: {:?}", shortcut);
            }

            result
        } else {
            None
        }
    }
}

pub struct ShortcutManager {
    reload_debouncer: Rc<RefCell<Debouncer>>,
    new_tab_debouncer: Rc<RefCell<Debouncer>>,
    close_tab_debouncer: Rc<RefCell<Debouncer>>,
    new_window_debouncer: Rc<RefCell<Debouncer>>,
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self {
            reload_debouncer: Rc::new(RefCell::new(Debouncer::new(250))),
            new_tab_debouncer: Rc::new(RefCell::new(Debouncer::new(250))),
            close_tab_debouncer: Rc::new(RefCell::new(Debouncer::new(250))),
            new_window_debouncer: Rc::new(RefCell::new(Debouncer::new(250))),
        }
    }

    pub fn handle_shortcut(
        &self,
        shortcut: Shortcut,
        components: &BrowserWindowComponents,
        config: &Rc<RefCell<Config>>,
        event_loop_target: &EventLoopWindowTarget<()>,
        windows_ref: &Rc<RefCell<HashMap<WindowId, BrowserWindowComponents>>>,
        focused_window_id: &Rc<RefCell<Option<WindowId>>>,
        control_flow: &mut ControlFlow,
    ) {
        match shortcut {
            Shortcut::Reload => {
                if self.reload_debouncer.borrow_mut().should_execute() {
                    crate::debug_log!("Shortcut: Reload");
                    components.tab_manager.borrow().reload_active_tab();
                }
            }
            Shortcut::FocusUrlBar => {
                crate::debug_log!("Shortcut: Focus URL bar");
                let script = "const urlBar = document.getElementById('url-bar'); if (urlBar) { urlBar.focus(); urlBar.select(); }";
                let _ = components.tab_bar_webview.evaluate_script(script);
            }
            Shortcut::ToggleDownloads => {
                if components
                    .toggle_downloads_debouncer
                    .borrow_mut()
                    .should_execute()
                {
                    crate::debug_log!("Shortcut: Toggle downloads");
                    self.toggle_downloads(components);
                }
            }
            Shortcut::FocusSidebar => {
                crate::debug_log!("Shortcut: Focus sidebar");
                let _ = components.tab_bar_webview.focus();
                let script = "window.showSidebarFocus(); if (window.tabs.length > 0) { if (window.focusedTabIndex < 0) { window.updateFocusedTab(0); } else { window.updateFocusedTab(window.focusedTabIndex); } }";
                let _ = components.tab_bar_webview.evaluate_script(script);
            }
            Shortcut::Find => {
                crate::debug_log!("Shortcut: Find");
                if let Some(active_webview) =
                    components.tab_manager.borrow().get_active_tab_webview()
                {
                    let _ = active_webview.evaluate_script("window.calmStartSearch();");
                }
            }
            Shortcut::NewTab => {
                if self.new_tab_debouncer.borrow_mut().should_execute() {
                    crate::debug_log!("Shortcut: New tab");
                    self.open_new_tab(components);
                }
            }
            Shortcut::CloseTab => {
                if self.close_tab_debouncer.borrow_mut().should_execute() {
                    crate::debug_log!("Shortcut: Close tab");
                    self.close_tab(components);
                }
            }
            Shortcut::NewWindow => {
                if self.new_window_debouncer.borrow_mut().should_execute() {
                    crate::debug_log!("Shortcut: New window");
                    self.create_new_window(
                        config,
                        event_loop_target,
                        windows_ref,
                        focused_window_id,
                    );
                }
            }
            Shortcut::ToggleSplitView => {
                crate::debug_log!("Shortcut: Toggle split view");
                let _toggled = components
                    .tab_manager
                    .borrow_mut()
                    .toggle_split_view(&components.window);
                let enabled = components.tab_manager.borrow().is_split_view_enabled();
                let script = format!(
                    "if (window.updateSplitViewButtons) {{ window.updateSplitViewButtons({}); }}",
                    enabled
                );
                let _ = components.tab_bar_webview.evaluate_script(&script);
            }
            Shortcut::Quit => {
                crate::debug_log!("Shortcut: Quit");
                *control_flow = ControlFlow::Exit;
            }
        }
    }

    fn toggle_downloads(&self, components: &BrowserWindowComponents) {
        const DOWNLOAD_SIDEBAR_WIDTH: i32 = 360;

        let should_show = {
            let mut is_visible = components.sidebar_visible.borrow_mut();
            *is_visible = !*is_visible;
            *is_visible
        };

        if should_show {
            let _ = components.download_overlay.set_visible(true);
            std::thread::sleep(std::time::Duration::from_millis(10));
            let script = "window.toggleVisibility(true);";
            let _ = components.download_overlay.evaluate_script(script);
            components
                .tab_manager
                .borrow_mut()
                .resize_all_tabs_with_sidebar(&components.window, DOWNLOAD_SIDEBAR_WIDTH as u32);
        } else {
            let script = "window.toggleVisibility(false);";
            let _ = components.download_overlay.evaluate_script(script);
            std::thread::sleep(std::time::Duration::from_millis(300));
            let _ = components.download_overlay.set_visible(false);
            components
                .tab_manager
                .borrow_mut()
                .resize_all_tabs(&components.window);
        }
    }

    fn open_new_tab(&self, components: &BrowserWindowComponents) {
        let is_visible = *components.command_prompt_visible.borrow();

        if is_visible {
            *components.command_prompt_visible.borrow_mut() = false;
            *components.command_prompt_overlay.borrow_mut() = None;
        } else {
            let window_size = components.window.inner_size();
            let command_prompt_visible_for_ipc = Rc::clone(&components.command_prompt_visible);
            let command_prompt_overlay_for_ipc = Rc::clone(&components.command_prompt_overlay);
            let tab_manager_for_prompt = Rc::clone(&components.tab_manager);
            let config_for_prompt = Rc::clone(&components.config);
            let window_for_prompt = Rc::clone(&components.window);
            let tab_bar_for_prompt = Rc::clone(&components.tab_bar_webview);

            match wry::WebViewBuilder::new()
                .with_html(&crate::ui::get_command_prompt_html())
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
                                                || url_str.contains("..") == false
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

                                        let escaped_url = serde_json::to_string(&url).unwrap_or_else(|_| "\"\"".to_string());
                                        let script = format!(
                                            "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
                                            tab_id, escaped_url, tab_id, escaped_url
                                        );
                                        let _ = tab_bar_for_prompt.evaluate_script(&script);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                })
                .build_as_child(components.window.as_ref())
            {
                Ok(webview) => {
                    *components.command_prompt_overlay.borrow_mut() = Some(webview);
                    *components.command_prompt_visible.borrow_mut() = true;
                }
                Err(e) => {
                    crate::debug_log!("Failed to create command prompt overlay: {:?}", e);
                }
            }
        }
    }

    fn close_tab(&self, components: &BrowserWindowComponents) {
        let tab_count = components.tab_manager.borrow().get_tab_count();
        let active_tab_id = components.tab_manager.borrow().get_active_tab_id();

        if tab_count <= 1 {
            crate::debug_log!("Last tab - closing window");
            *components.should_quit.borrow_mut() = true;
        } else {
            if let Some(active_tab_id) = active_tab_id {
                components.tab_manager.borrow_mut().close_tab(active_tab_id);
                let script = format!("window.removeTab({});", active_tab_id);
                let _ = components.tab_bar_webview.evaluate_script(&script);
                crate::debug_log!("Tab {} closed", active_tab_id);
            }
        }
    }

    fn create_new_window(
        &self,
        config: &Rc<RefCell<Config>>,
        event_loop_target: &EventLoopWindowTarget<()>,
        windows_ref: &Rc<RefCell<HashMap<WindowId, BrowserWindowComponents>>>,
        focused_window_id: &Rc<RefCell<Option<WindowId>>>,
    ) {
        let default_url = crate::convert_file_url(&config.borrow().default_url);
        match crate::window::create_browser_window(
            event_loop_target,
            Rc::clone(config),
            default_url,
            false,
        ) {
            Ok(new_window) => {
                let new_window_id = new_window.window.id();
                crate::debug_log!("Created new window: {:?}", new_window_id);
                *focused_window_id.borrow_mut() = Some(new_window_id);
                windows_ref.borrow_mut().insert(new_window_id, new_window);
            }
            Err(e) => {
                crate::debug_log!("Failed to create new window: {:?}", e);
            }
        }
    }
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}
