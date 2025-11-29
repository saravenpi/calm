pub mod animations;
pub mod command_prompt;
pub mod download_overlay;
pub mod fonts;
pub mod keyboard_handler;
pub mod renderers;
pub mod settings;
pub mod sounds;
pub mod tab_bar;
pub mod template;
pub mod welcome;

pub use animations::{
    get_audio_indicator_script, get_interaction_animations, get_loading_animation,
    get_navigation_loader, get_page_transitions,
};
pub use command_prompt::get_command_prompt_html;
pub use download_overlay::get_download_overlay_html;
pub use keyboard_handler::get_keyboard_handler_script;
pub use settings::get_settings_html;
pub use sounds::get_sounds_script;
pub use tab_bar::get_complete_tab_bar_html;
pub use welcome::get_welcome_html;
