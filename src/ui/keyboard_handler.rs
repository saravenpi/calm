/// Returns JavaScript code to capture keyboard shortcuts in webviews and send them via IPC.
pub fn get_keyboard_handler_script() -> String {
    r#"
(function() {
    console.log('[KEYBOARD] Installing keyboard shortcut handler');

    // Only capture keyboard events if we're in the tab bar or a tab
    // Don't interfere with input fields
    document.addEventListener('keydown', function(e) {
        // Allow keyboard events in input fields, textareas, and contenteditable elements
        const target = e.target;
        if (target.tagName === 'INPUT' ||
            target.tagName === 'TEXTAREA' ||
            target.tagName === 'SELECT' ||
            target.isContentEditable) {
            return; // Let the event proceed normally
        }

        // Check for our shortcuts (Cmd/Ctrl combinations)
        const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
        const modifier = isMac ? e.metaKey : e.ctrlKey;

        if (!modifier || e.altKey) {
            return; // Not a shortcut we care about
        }

        const key = e.key.toLowerCase();
        const shift = e.shiftKey;

        // Map of shortcuts
        let shortcut = null;

        if (!shift) {
            switch(key) {
                case 'r': shortcut = 'reload'; break;
                case 'l': shortcut = 'focus_url'; break;
                case 'j': shortcut = 'toggle_downloads'; break;
                case 'e': shortcut = 'focus_sidebar'; break;
                case 'f': shortcut = 'find'; break;
                case 't': shortcut = 'new_tab'; break;
                case 'w': shortcut = 'close_tab'; break;
                case 'n': shortcut = 'new_window'; break;
                case 'q': shortcut = 'quit'; break;
            }
        } else {
            if (key === 's') {
                shortcut = 'toggle_split_view';
            }
        }

        if (shortcut) {
            console.log('[KEYBOARD] Captured shortcut:', shortcut);
            e.preventDefault();
            e.stopPropagation();

            // Send to Rust via IPC
            if (window.ipc) {
                window.ipc.postMessage(JSON.stringify({
                    action: 'keyboard_shortcut',
                    shortcut: shortcut
                }));
            }
        }
    }, true); // Use capture phase to intercept before other handlers

    console.log('[KEYBOARD] Keyboard handler installed successfully');
})();
"#
    .to_string()
}
