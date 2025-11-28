use crate::tabs::TabManager;
use crate::config::Config;
use std::rc::Rc;
use std::cell::RefCell;
use tao::window::Window;
use wry::WebView;

/// Represents a browser window with its associated components.
pub struct BrowserWindow {
    pub window_id: usize,
    pub window: Rc<Window>,
    pub tab_manager: Rc<RefCell<TabManager>>,
    pub tab_bar_webview: Rc<RefCell<Option<Rc<WebView>>>>,
    pub download_overlay: Rc<RefCell<Option<Rc<WebView>>>>,
    pub sidebar_visible: Rc<RefCell<bool>>,
}

impl BrowserWindow {
    /// Creates a new browser window with the specified configuration.
    pub fn new(
        window_id: usize,
        window: Rc<Window>,
        tab_sidebar_width: u32,
        config: std::rc::Rc<std::cell::RefCell<Config>>,
    ) -> Self {
        Self {
            window_id,
            window,
            tab_manager: Rc::new(RefCell::new(TabManager::new(tab_sidebar_width, config))),
            tab_bar_webview: Rc::new(RefCell::new(None)),
            download_overlay: Rc::new(RefCell::new(None)),
            sidebar_visible: Rc::new(RefCell::new(false)),
        }
    }

    /// Returns the number of tabs in this window.
    pub fn get_tab_count(&self) -> usize {
        self.tab_manager.borrow().get_tab_count()
    }

    /// Returns true if the window has at least one tab.
    pub fn has_tabs(&self) -> bool {
        self.get_tab_count() > 0
    }

    /// Sets the tab bar webview for this window.
    pub fn set_tab_bar_webview(&self, webview: Rc<WebView>) {
        *self.tab_bar_webview.borrow_mut() = Some(webview.clone());
        self.tab_manager.borrow_mut().set_tab_bar_webview(webview);
    }

    /// Sets the download overlay webview for this window.
    pub fn set_download_overlay(&self, overlay: Rc<WebView>) {
        *self.download_overlay.borrow_mut() = Some(overlay.clone());
        self.tab_manager.borrow_mut().set_download_overlay(overlay);
    }
}
