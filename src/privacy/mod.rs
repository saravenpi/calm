pub mod scripts;

use crate::config::PrivacySettings;

pub use scripts::{
    get_all_privacy_scripts_with_config,
    get_content_renderer as get_content_renderer_from_privacy,
};

/// Generates a combined JavaScript initialization script with all privacy protections and UI animations.
/// This script is injected into each webview during creation.
pub fn get_combined_privacy_script_with_config(settings: &PrivacySettings) -> String {
    let mut script = String::with_capacity(30720);

    script.push_str(get_keyboard_passthrough_script());
    script.push('\n');

    script.push_str(get_all_privacy_scripts_with_config(settings).as_str());
    script.push('\n');

    script.push_str(crate::ui::get_loading_animation());
    script.push('\n');
    script.push_str(crate::ui::get_navigation_loader());
    script.push('\n');
    script.push_str(crate::ui::get_page_transitions());
    script.push('\n');
    script.push_str(crate::ui::get_interaction_animations());
    script.push('\n');
    script.push_str(crate::ui::get_audio_indicator_script());
    script.push('\n');

    script.push_str(get_content_renderer_from_privacy());
    script
}

/// Returns a script that ensures keyboard shortcuts are passed through to the native layer.
fn get_keyboard_passthrough_script() -> &'static str {
    r#"
(function() {
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    const modifierKey = isMac ? 'metaKey' : 'ctrlKey';

    const shortcuts = ['r', 'l', 'j', 'q', 't', 'w'];

    document.addEventListener('keydown', function(e) {
        if (e[modifierKey]) {
            const key = e.key.toLowerCase();
            if (shortcuts.includes(key)) {
                e.preventDefault();
                e.stopPropagation();
                e.stopImmediatePropagation();
                return false;
            }
        }
    }, true);

    document.addEventListener('keyup', function(e) {
        if (e[modifierKey]) {
            const key = e.key.toLowerCase();
            if (shortcuts.includes(key)) {
                e.preventDefault();
                e.stopPropagation();
                e.stopImmediatePropagation();
                return false;
            }
        }
    }, true);
})();
    "#
}

/// Returns a standardized user agent string for privacy and fingerprinting protection.
pub fn get_privacy_user_agent() -> &'static str {
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
}
