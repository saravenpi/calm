use base64::{engine::general_purpose::STANDARD, Engine};

/// Returns base64-encoded cursor move sound as a data URL.
pub fn get_cursor_move_sound() -> String {
    let sound_data = include_bytes!("../../../assets/sounds/cursor_move.mp3");
    format!("data:audio/mpeg;base64,{}", STANDARD.encode(sound_data))
}

/// Returns base64-encoded delete sound as a data URL.
pub fn get_delete_sound() -> String {
    let sound_data = include_bytes!("../../../assets/sounds/delete.mp3");
    format!("data:audio/mpeg;base64,{}", STANDARD.encode(sound_data))
}

/// Returns base64-encoded startup sound as a data URL.
pub fn get_startup_sound() -> String {
    let sound_data = include_bytes!("../../../assets/sounds/startup.mp3");
    format!("data:audio/mpeg;base64,{}", STANDARD.encode(sound_data))
}

/// Returns JavaScript code for playing UI sounds, with sounds enabled or disabled based on configuration.
///
/// # Arguments
///
/// * `sounds_enabled` - Whether UI sounds should be enabled
pub fn get_sounds_script(sounds_enabled: bool) -> String {
    if !sounds_enabled {
        return String::from(
            r#"
            window.uiSoundsEnabled = false;
            window.playUISound = function() {};
        "#,
        );
    }

    format!(
        r#"
        window.uiSoundsEnabled = true;

        const cursorMoveData = '{}';
        const deleteData = '{}';
        const startupData = '{}';

        const baseSounds = {{
            cursorMove: new Audio(cursorMoveData),
            delete: new Audio(deleteData),
            startup: new Audio(startupData)
        }};

        Object.values(baseSounds).forEach(sound => {{
            sound.volume = 0.3;
            sound.preload = 'auto';
            sound.load();
        }});

        window.playUISound = function(soundName) {{
            if (!window.uiSoundsEnabled) return;

            const baseSound = baseSounds[soundName];
            if (baseSound) {{
                const sound = baseSound.cloneNode();
                sound.volume = 0.3;
                sound.play().catch(err => {{}});
            }}
        }};
    "#,
        get_cursor_move_sound(),
        get_delete_sound(),
        get_startup_sound()
    )
}
