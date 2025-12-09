use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tao::{
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::WindowId,
};

use crate::{config::Config, debug_log, window::BrowserWindowComponents};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shortcut {
    NewTab,
    NewWindow,
    CloseTab,
    Reload,
    FocusUrlBar,
    ToggleDownloads,
    ToggleSplitView,
    FocusSidebar,
    Find,
}

pub struct ShortcutManager;

impl ShortcutManager {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_shortcut(
        &self,
        shortcut: Shortcut,
        components: &BrowserWindowComponents,
        _config: &Rc<RefCell<Config>>,
        event_loop_target: &EventLoopWindowTarget<()>,
        windows_ref: &Rc<RefCell<HashMap<WindowId, BrowserWindowComponents>>>,
        focused_window_id: &Rc<RefCell<Option<WindowId>>>,
        _control_flow: &mut ControlFlow,
    ) {
        match shortcut {
            Shortcut::NewTab => {
                debug_log!("Shortcut: NewTab - Triggering command prompt");
                let _ = components.tab_bar_webview.evaluate_script(
                    "window.ipc.postMessage(JSON.stringify({action: 'keyboard_shortcut', shortcut: 'new_tab'}));"
                );
            }
            Shortcut::NewWindow => {
                debug_log!("Shortcut: NewWindow");
                let config = components.config.borrow();
                let default_url = crate::convert_file_url(&config.default_url);
                drop(config);

                match crate::window::create_browser_window(
                    event_loop_target,
                    Rc::clone(&components.config),
                    default_url,
                    false,
                ) {
                    Ok(new_components) => {
                        let new_window_id = new_components.window.id();
                        windows_ref
                            .borrow_mut()
                            .insert(new_window_id, new_components);
                        *focused_window_id.borrow_mut() = Some(new_window_id);
                    }
                    Err(e) => {
                        eprintln!("Failed to create new window: {}", e);
                    }
                }
            }
            Shortcut::CloseTab => {
                debug_log!("Shortcut: CloseTab (Cmd+W)");
                
                // Always close the current tab
                if let Some(tab_id) = components.tab_manager.borrow().get_active_tab_id() {
                    debug_log!("Closing tab {}", tab_id);
                    components.tab_manager.borrow_mut().close_tab(tab_id);
                    
                    // Check if we have any tabs left
                    let tab_count = components.tab_manager.borrow().get_tab_count();
                    debug_log!("Tabs remaining: {}", tab_count);
                    
                    if tab_count == 0 {
                        // No tabs left - create a new one with default URL
                        debug_log!("No tabs remaining - creating new tab with default URL");
                        let config = components.config.borrow();
                        let default_url = crate::convert_file_url(&config.default_url);
                        drop(config);
                        
                        if let Ok(_) = components.tab_manager.borrow_mut().create_tab(&components.window, &default_url) {
                            debug_log!("Created new default tab");
                        }
                    }
                    
                    let _ = components.tab_bar_webview.evaluate_script("window.refreshTabs();");
                }
            }
            Shortcut::Reload => {
                debug_log!("Shortcut: Reload");
                components.tab_manager.borrow().reload_active_tab();
            }
            Shortcut::FocusUrlBar => {
                debug_log!("Shortcut: FocusUrlBar");
                let script = "const urlBar = document.getElementById('url-bar'); if (urlBar) { urlBar.focus(); urlBar.select(); }";
                let _ = components.tab_bar_webview.evaluate_script(script);
            }
            Shortcut::ToggleDownloads => {
                debug_log!("Shortcut: ToggleDownloads");
                if components
                    .toggle_downloads_debouncer
                    .borrow_mut()
                    .should_execute()
                {
                    let should_show = {
                        let mut is_visible = components.sidebar_visible.borrow_mut();
                        *is_visible = !*is_visible;
                        *is_visible
                    };

                    if should_show {
                        let _ = components.download_overlay.set_visible(true);
                        std::thread::sleep(std::time::Duration::from_millis(10));
                        components
                            .tab_manager
                            .borrow_mut()
                            .resize_all_tabs(&components.window);
                        let _ = components.download_overlay.focus();
                        let _ = components.download_overlay.set_bounds(wry::Rect {
                            position: tao::dpi::LogicalPosition::new(
                                components.window.inner_size().width as i32 - 360,
                                0,
                            )
                            .into(),
                            size: tao::dpi::LogicalSize::new(
                                360,
                                components.window.inner_size().height,
                            )
                            .into(),
                        });
                    } else {
                        let _ = components.download_overlay.set_visible(false);
                        components
                            .tab_manager
                            .borrow_mut()
                            .resize_all_tabs(&components.window);
                    }
                }
            }
            Shortcut::ToggleSplitView => {
                debug_log!("Shortcut: ToggleSplitView");
                let _ = components
                    .tab_manager
                    .borrow_mut()
                    .toggle_split_view(&components.window);

                let ui_state = components.tab_manager.borrow().get_split_ui_state();
                let orientation_str = ui_state
                    .active_group_orientation
                    .as_deref()
                    .unwrap_or("vertical");
                let groups_json = components.tab_manager.borrow().get_split_groups_json();
                let script = format!(
                    "if (window.updateSplitUIState) {{ window.updateSplitUIState({}, {}, '{}'); }} if (window.setSplitGroups) {{ window.setSplitGroups({}); }}",
                    ui_state.active_tab_in_split,
                    ui_state.can_create_split,
                    orientation_str,
                    groups_json
                );
                let _ = components.tab_bar_webview.evaluate_script(&script);

                let _ = components
                    .tab_bar_webview
                    .evaluate_script("window.refreshTabs();");
            }
            Shortcut::FocusSidebar => {
                debug_log!("Shortcut: FocusSidebar");
                let _ = components.tab_bar_webview.focus();
                let script = "window.showSidebarFocus(); if (window.tabs.length > 0) { if (window.focusedTabIndex < 0) { window.updateFocusedTab(0); } else { window.updateFocusedTab(window.focusedTabIndex); } }";
                let _ = components.tab_bar_webview.evaluate_script(script);
            }
            Shortcut::Find => {
                debug_log!("Shortcut: Find");
                if let Some(active_webview) =
                    components.tab_manager.borrow().get_active_tab_webview()
                {
                    let _ = active_webview.evaluate_script("window.calmStartSearch();");
                }
            }
        }
    }
}
