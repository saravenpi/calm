pub mod template;
pub mod tab_bar;
pub mod animations;
pub mod renderers;
pub mod download_overlay;
pub mod welcome;

pub use tab_bar::get_complete_tab_bar_html_with_opacity;
pub use download_overlay::get_download_overlay_html;
pub use welcome::get_welcome_html;
pub use animations::{
    get_loading_animation,
    get_navigation_loader,
    get_page_transitions,
    get_interaction_animations,
    get_audio_indicator_script,
};
