pub mod interactions;
pub mod loader;
pub mod transitions;

pub use interactions::{get_audio_indicator_script, get_interaction_animations};
pub use loader::{get_loading_animation, get_navigation_loader};
pub use transitions::get_page_transitions;

/// Returns all animation scripts combined into a single string.
#[allow(dead_code)]
pub fn get_all_animation_scripts() -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}",
        get_loading_animation(),
        get_navigation_loader(),
        get_page_transitions(),
        get_interaction_animations(),
        get_audio_indicator_script()
    )
}
