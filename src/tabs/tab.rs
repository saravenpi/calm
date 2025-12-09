use std::time::Instant;
use wry::WebView;
use std::rc::Rc;

#[allow(dead_code)]
pub enum TabState {
    Unloaded { url: String },
    Loaded { webview: Rc<WebView>, url: String },
    Suspended { url: String, title: Option<String> },
}

/// Represents a single browser tab with its webview and URL.
pub struct Tab {
    pub state: TabState,
    #[allow(dead_code)]
    pub id: usize,
    #[allow(dead_code)]
    pub title: Option<String>,
    pub last_accessed: Instant,
}

impl Tab {
    /// Creates a new tab with the given URL and webview.
    pub fn new(id: usize, url: String, webview: Rc<WebView>) -> Self {
        Self {
            id,
            state: TabState::Loaded {
                url: url.clone(),
                webview,
            },
            title: None,
            last_accessed: Instant::now(),
        }
    }

    /// Creates a new unloaded tab (for lazy loading).
    #[allow(dead_code)]
    pub fn new_unloaded(id: usize, url: String) -> Self {
        Self {
            id,
            state: TabState::Unloaded { url },
            title: None,
            last_accessed: Instant::now(),
        }
    }

    /// Loads the tab by creating its webview.
    #[allow(dead_code)]
    pub fn load(&mut self, webview: Rc<WebView>) {
        if let TabState::Unloaded { url } = &self.state {
            let url = url.clone();
            self.state = TabState::Loaded { url, webview };
            self.last_accessed = Instant::now();
        }
    }

    /// Suspends the tab to save memory.
    #[allow(dead_code)]
    pub fn suspend(&mut self) {
        if matches!(self.state, TabState::Loaded { .. }) {
            let old_state = std::mem::replace(
                &mut self.state,
                TabState::Suspended {
                    url: String::new(),
                    title: None,
                },
            );

            if let TabState::Loaded { url, webview: _ } = old_state {
                self.state = TabState::Suspended {
                    url,
                    title: self.title.clone(),
                };
            }
        }
    }

    /// Checks if the tab is loaded.
    #[allow(dead_code)]
    pub fn is_loaded(&self) -> bool {
        matches!(self.state, TabState::Loaded { .. })
    }

    /// Checks if the tab is suspended.
    #[allow(dead_code)]
    pub fn is_suspended(&self) -> bool {
        matches!(self.state, TabState::Suspended { .. })
    }

    /// Checks if the tab is unloaded.
    #[allow(dead_code)]
    pub fn is_unloaded(&self) -> bool {
        matches!(self.state, TabState::Unloaded { .. })
    }

    /// Gets a reference to the webview if the tab is loaded.
    pub fn webview(&self) -> Option<&WebView> {
        if let TabState::Loaded { webview, .. } = &self.state {
            Some(webview)
        } else {
            None
        }
    }

    /// Updates the tab's current URL.
    pub fn set_url(&mut self, url: String) {
        match &mut self.state {
            TabState::Loaded {
                url: current_url, ..
            } => *current_url = url,
            TabState::Unloaded { url: current_url } => *current_url = url,
            TabState::Suspended {
                url: current_url, ..
            } => *current_url = url,
        }
    }

    /// Returns the tab's current URL.
    pub fn get_url(&self) -> &str {
        match &self.state {
            TabState::Loaded { url, .. } => url,
            TabState::Unloaded { url } => url,
            TabState::Suspended { url, .. } => url,
        }
    }

    /// Sets the tab's title.
    #[allow(dead_code)]
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    /// Makes the tab visible in the browser window.
    pub fn show(&self) {
        if let TabState::Loaded { webview, .. } = &self.state {
            let _ = webview.set_visible(true);
        }
    }

    /// Hides the tab from the browser window.
    pub fn hide(&self) {
        if let TabState::Loaded { webview, .. } = &self.state {
            let _ = webview.set_visible(false);
        }
    }

    /// Performs cleanup operations on the tab before it is closed.
    /// Stops all media playback, closes audio contexts, clears caches, and navigates to about:blank.
    pub fn cleanup(&self) {
        if let TabState::Loaded { webview, .. } = &self.state {
            const CLEANUP_SCRIPT: &str = r#"
                (function() {
                    if (window.__calmStopAllAudio) {
                        window.__calmStopAllAudio();
                    }

                    document.querySelectorAll('audio, video').forEach(media => {
                        media.pause();
                        media.currentTime = 0;
                        media.src = '';
                        media.load();
                        media.remove();
                    });

                    document.querySelectorAll('video, audio').forEach(media => {
                        if (media.srcObject) {
                            const tracks = media.srcObject.getTracks();
                            tracks.forEach(track => track.stop());
                            media.srcObject = null;
                        }
                    });

                    if (window.AudioContext || window.webkitAudioContext) {
                        const AudioContext = window.AudioContext || window.webkitAudioContext;
                        if (window.audioContext && window.audioContext instanceof AudioContext) {
                            window.audioContext.close();
                        }
                    }

                    if (window.caches) {
                        caches.keys().then(keys => keys.forEach(key => caches.delete(key)));
                    }

                    window.stop();
                })();
            "#;

            let _ = webview.evaluate_script(CLEANUP_SCRIPT);

            std::thread::sleep(std::time::Duration::from_millis(100));

            let _ = webview.load_url("about:blank");
        }
    }

    /// Marks the tab as accessed, updating its last_accessed timestamp.
    pub fn mark_accessed(&mut self) {
        self.last_accessed = Instant::now();
    }

    /// Returns the duration since the tab was last accessed.
    #[allow(dead_code)]
    pub fn time_since_last_access(&self) -> std::time::Duration {
        self.last_accessed.elapsed()
    }
}

impl Drop for Tab {
    fn drop(&mut self) {
        self.cleanup();
    }
}
