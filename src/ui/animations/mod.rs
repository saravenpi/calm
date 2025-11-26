pub mod loader;
pub mod transitions;
pub mod interactions;

pub use loader::{get_loading_animation, get_navigation_loader};
pub use transitions::get_page_transitions;
pub use interactions::{get_interaction_animations, get_audio_indicator_script};

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
