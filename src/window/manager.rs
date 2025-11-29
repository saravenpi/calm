use super::browser_window::BrowserWindow;
use super::session::{WindowPosition, WindowSessionManager, WindowTabInfo};
use crate::config::Config;
use crate::ui;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop::EventLoopWindowTarget,
    window::{WindowBuilder, WindowId},
};
use wry::{Rect, WebViewBuilder};

#[cfg(target_os = "macos")]
use tao::platform::macos::WindowBuilderExtMacOS;

#[allow(dead_code)]
const TAB_SIDEBAR_WIDTH: u32 = 250;
#[allow(dead_code)]
const DOWNLOAD_SIDEBAR_WIDTH: i32 = 360;

/// Manages multiple browser windows and their lifecycle.
#[allow(dead_code)]
pub struct WindowManager {
    windows: HashMap<WindowId, BrowserWindow>,
    window_id_map: HashMap<usize, WindowId>,
    next_window_id: usize,
    focused_window_id: Option<WindowId>,
    session_manager: WindowSessionManager,
    config: Rc<RefCell<Config>>,
}

#[allow(dead_code)]
impl WindowManager {
    /// Creates a new window manager.
    pub fn new(config: Rc<RefCell<Config>>) -> Self {
        Self {
            windows: HashMap::new(),
            window_id_map: HashMap::new(),
            next_window_id: 1,
            focused_window_id: None,
            session_manager: WindowSessionManager::new(),
            config,
        }
    }

    /// Creates a new window at the specified position.
    pub fn create_window<T>(
        &mut self,
        event_loop: &EventLoopWindowTarget<T>,
        position: Option<WindowPosition>,
    ) -> Result<WindowId, String> {
        let window_pos = position.unwrap_or_else(|| {
            let last_pos = self.session_manager.get_last_window_position();
            WindowPosition {
                x: last_pos.x + 30,
                y: last_pos.y + 30,
                width: last_pos.width,
                height: last_pos.height,
            }
        });

        #[cfg(target_os = "macos")]
        let window = WindowBuilder::new()
            .with_title("Calm Browser - Privacy-Focused")
            .with_inner_size(LogicalSize::new(window_pos.width, window_pos.height))
            .with_position(LogicalPosition::new(window_pos.x, window_pos.y))
            .with_title_hidden(true)
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .build(event_loop)
            .map_err(|e| format!("Failed to create window: {}", e))?;

        #[cfg(not(target_os = "macos"))]
        let window = WindowBuilder::new()
            .with_title("Calm Browser - Privacy-Focused")
            .with_inner_size(LogicalSize::new(window_pos.width, window_pos.height))
            .with_position(LogicalPosition::new(window_pos.x, window_pos.y))
            .build(event_loop)
            .map_err(|e| format!("Failed to create window: {}", e))?;

        let window_id = window.id();
        let browser_window_id = self.next_window_id;
        self.next_window_id += 1;

        let browser_window = BrowserWindow::new(
            browser_window_id,
            Rc::new(window),
            TAB_SIDEBAR_WIDTH,
            Rc::clone(&self.config),
        );

        self.windows.insert(window_id, browser_window);
        self.window_id_map.insert(browser_window_id, window_id);
        self.focused_window_id = Some(window_id);

        Ok(window_id)
    }

    /// Sets up webviews for a window (tab bar and download overlay).
    pub fn setup_window_webviews(
        &mut self,
        window_id: WindowId,
        ipc_handler: Box<dyn Fn(wry::http::Request<String>) + 'static>,
    ) -> Result<(), String> {
        let browser_window = self.windows.get(&window_id).ok_or("Window not found")?;

        let window_size = browser_window.window.inner_size();

        let tab_bar_webview = Rc::new(
            WebViewBuilder::new()
                .with_html(&ui::get_complete_tab_bar_html(
                    self.config.borrow().ui.vim_mode,
                    self.config.borrow().ui.sounds,
                ))
                .with_transparent(true)
                .with_bounds(Rect {
                    position: LogicalPosition::new(0, 0).into(),
                    size: LogicalSize::new(TAB_SIDEBAR_WIDTH, window_size.height).into(),
                })
                .with_ipc_handler(ipc_handler)
                .build_as_child(browser_window.window.as_ref())
                .map_err(|e| format!("Failed to create tab bar: {}", e))?,
        );

        let download_overlay = Rc::new(
            WebViewBuilder::new()
                .with_html(&ui::get_download_overlay_html())
                .with_bounds(Rect {
                    position: LogicalPosition::new(
                        (window_size.width as i32) - DOWNLOAD_SIDEBAR_WIDTH,
                        0,
                    )
                    .into(),
                    size: LogicalSize::new(DOWNLOAD_SIDEBAR_WIDTH as u32, window_size.height)
                        .into(),
                })
                .with_visible(false)
                .build_as_child(browser_window.window.as_ref())
                .map_err(|e| format!("Failed to create download overlay: {}", e))?,
        );

        browser_window.set_tab_bar_webview(tab_bar_webview);
        browser_window.set_download_overlay(download_overlay);

        Ok(())
    }

    /// Returns a reference to a window by its ID.
    pub fn get_window(&self, window_id: &WindowId) -> Option<&BrowserWindow> {
        self.windows.get(window_id)
    }

    /// Returns a mutable reference to a window by its ID.
    pub fn get_window_mut(&mut self, window_id: &WindowId) -> Option<&mut BrowserWindow> {
        self.windows.get_mut(window_id)
    }

    /// Returns a reference to the currently focused window.
    pub fn get_focused_window(&self) -> Option<&BrowserWindow> {
        self.focused_window_id
            .as_ref()
            .and_then(|id| self.windows.get(id))
    }

    /// Returns the ID of the currently focused window.
    pub fn get_focused_window_id(&self) -> Option<WindowId> {
        self.focused_window_id
    }

    /// Sets which window is currently focused.
    pub fn set_focused_window(&mut self, window_id: WindowId) {
        self.focused_window_id = Some(window_id);
    }

    /// Closes a window and returns true if all windows are now closed.
    pub fn close_window(&mut self, window_id: WindowId) -> bool {
        if let Some(browser_window) = self.windows.remove(&window_id) {
            self.window_id_map.remove(&browser_window.window_id);

            if self.focused_window_id == Some(window_id) {
                self.focused_window_id = self.windows.keys().next().copied();
            }

            self.windows.is_empty()
        } else {
            false
        }
    }

    /// Returns whether the application should quit (no windows remaining).
    pub fn should_quit(&self) -> bool {
        self.windows.is_empty()
    }

    /// Returns the total number of open windows.
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Returns a vector of all window IDs.
    pub fn get_all_window_ids(&self) -> Vec<WindowId> {
        self.windows.keys().copied().collect()
    }

    /// Resizes a window and its components.
    pub fn resize_window(&self, window_id: &WindowId) {
        if let Some(browser_window) = self.windows.get(window_id) {
            let window_size = browser_window.window.inner_size();

            if let Some(ref webview) = *browser_window.tab_bar_webview.borrow() {
                let tab_bar_bounds = Rect {
                    position: LogicalPosition::new(0, 0).into(),
                    size: LogicalSize::new(TAB_SIDEBAR_WIDTH, window_size.height).into(),
                };
                let _ = webview.set_bounds(tab_bar_bounds);
            }

            if let Some(ref overlay) = *browser_window.download_overlay.borrow() {
                let sidebar_x = (window_size.width as i32) - DOWNLOAD_SIDEBAR_WIDTH;
                let sidebar_bounds = Rect {
                    position: LogicalPosition::new(sidebar_x, 0).into(),
                    size: LogicalSize::new(DOWNLOAD_SIDEBAR_WIDTH as u32, window_size.height)
                        .into(),
                };
                let _ = overlay.set_bounds(sidebar_bounds);
            }

            let is_visible = *browser_window.sidebar_visible.borrow();
            if is_visible {
                browser_window
                    .tab_manager
                    .borrow_mut()
                    .resize_all_tabs_with_sidebar(
                        &browser_window.window,
                        DOWNLOAD_SIDEBAR_WIDTH as u32,
                    );
            } else {
                browser_window
                    .tab_manager
                    .borrow_mut()
                    .resize_all_tabs(&browser_window.window);
            }
        }
    }

    /// Saves the current window session state to disk.
    pub fn save_session(&self) -> Result<(), String> {
        let mut window_states = HashMap::new();

        for (_window_id, browser_window) in &self.windows {
            let position = {
                let pos = browser_window
                    .window
                    .outer_position()
                    .unwrap_or(tao::dpi::PhysicalPosition::new(0, 0));
                let size = browser_window.window.inner_size();
                WindowPosition {
                    x: pos.x,
                    y: pos.y,
                    width: size.width,
                    height: size.height,
                }
            };

            let tabs: Vec<WindowTabInfo> = vec![];

            let active_tab = browser_window.tab_manager.borrow().get_active_tab_id();

            window_states.insert(browser_window.window_id, (position, tabs, active_tab));
        }

        let active_window_id = self
            .focused_window_id
            .and_then(|id| self.windows.get(&id))
            .map(|w| w.window_id);

        self.session_manager
            .save_window_state(&window_states, active_window_id)
    }
}
