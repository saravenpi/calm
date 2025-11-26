pub mod core;
pub mod fingerprint;
pub mod tracking;
pub mod title_tracker;
pub mod download_intercept;

use crate::config::PrivacySettings;

pub use core::{get_privacy_script, get_dark_mode_preference};
pub use fingerprint::{
    get_canvas_fingerprint_protection,
    get_webgl_fingerprint_protection,
    get_audio_fingerprint_protection,
    get_font_fingerprint_protection,
};
pub use tracking::get_tracking_blocker;
pub use title_tracker::get_title_tracker_script;
pub use download_intercept::get_script as get_download_interceptor;

pub fn get_content_renderer() -> &'static str {
    crate::ui::renderers::get_content_renderer()
}

pub fn get_all_privacy_scripts_with_config(settings: &PrivacySettings) -> String {
    let mut scripts = Vec::new();

    scripts.push(get_privacy_script(settings).to_string());

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

    scripts.join("\n")
}

