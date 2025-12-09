pub mod adblock;
pub mod context_menu;
pub mod core;
pub mod download_intercept;
pub mod fingerprint;
pub mod link_handler;
pub mod title_tracker;
pub mod tracking;

use crate::config::PrivacySettings;

pub use adblock::get_adblock_script;
pub use context_menu::get_script as get_context_menu_script;
pub use core::{get_dark_mode_preference, get_privacy_script};
pub use download_intercept::get_script as get_download_interceptor;
pub use fingerprint::{
    get_audio_fingerprint_protection, get_canvas_fingerprint_protection,
    get_font_fingerprint_protection, get_webgl_fingerprint_protection,
};
pub use link_handler::get_link_handler_script;
pub use title_tracker::get_title_tracker_script;
pub use tracking::get_tracking_blocker;

/// Returns the content renderer script for displaying web content.
pub fn get_content_renderer() -> &'static str {
    crate::ui::renderers::get_content_renderer()
}

/// Aggregates all enabled privacy protection scripts into a single string based on configuration.
///
/// # Arguments
///
/// * `settings` - Privacy configuration specifying which protections to enable
///
/// # Returns
///
/// Combined JavaScript code for all enabled privacy features
pub fn get_all_privacy_scripts_with_config(settings: &PrivacySettings) -> String {
    let mut scripts = Vec::new();

    scripts.push(get_privacy_script(settings).to_string());

    if settings.adblock_enabled {
        scripts.push(get_adblock_script().to_string());
    }

    if settings.tracking_domain_blocking {
        scripts.push(get_tracking_blocker().to_string());
    }

    if settings.canvas_fingerprint_protection {
        scripts.push(get_canvas_fingerprint_protection().to_string());
    }

    if settings.webgl_fingerprint_protection {
        scripts.push(get_webgl_fingerprint_protection().to_string());
    }

    if settings.audio_fingerprint_protection {
        scripts.push(get_audio_fingerprint_protection().to_string());
    }

    if settings.font_enumeration_restriction {
        scripts.push(get_font_fingerprint_protection().to_string());
    }

    scripts.push(get_dark_mode_preference().to_string());
    scripts.push(get_title_tracker_script().to_string());
    scripts.push(get_download_interceptor().to_string());
    scripts.push(get_context_menu_script().to_string());
    scripts.push(get_link_handler_script().to_string());

    scripts.join("\n")
}
