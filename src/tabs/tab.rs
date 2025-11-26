use wry::WebView;

/// Represents a single browser tab with its webview and URL.
pub struct Tab {
    pub webview: WebView,
    pub url: String,
}

impl Tab {
    /// Creates a new tab with the given URL and webview.
    pub fn new(_id: usize, url: String, webview: WebView) -> Self {
        Self { webview, url }
    }

    /// Updates the tab's current URL.
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    /// Returns the tab's current URL.
    pub fn get_url(&self) -> &str {
        &self.url
    }

    /// Makes the tab visible in the browser window.
    pub fn show(&self) {
        let _ = self.webview.set_visible(true);
    }

    /// Hides the tab from the browser window.
    pub fn hide(&self) {
        let _ = self.webview.set_visible(false);
    }

    /// Performs cleanup operations on the tab before it is closed.
    /// Stops all media playback, closes audio contexts, clears caches, and navigates to about:blank.
    pub fn cleanup(&self) {
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

        let _ = self.webview.evaluate_script(CLEANUP_SCRIPT);

        std::thread::sleep(std::time::Duration::from_millis(100));

        let _ = self.webview.load_url("about:blank");
    }
}

impl Drop for Tab {
    fn drop(&mut self) {
        self.cleanup();
    }
}
