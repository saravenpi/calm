use muda::accelerator::{Accelerator, Code, Modifiers};

pub fn parse_shortcut(shortcut: &str) -> Option<Accelerator> {
    if shortcut.is_empty() {
        return None;
    }

    let parts: Vec<&str> = shortcut.split('+').map(|s| s.trim()).collect();
    if parts.is_empty() {
        return None;
    }

    let mut modifiers = Modifiers::empty();
    let mut key_code: Option<Code> = None;

    for part in parts {
        match part.to_uppercase().as_str() {
            "CMD" | "COMMAND" | "META" | "SUPER" => {
                modifiers |= Modifiers::SUPER;
            }
            "CTRL" | "CONTROL" => {
                modifiers |= Modifiers::CONTROL;
            }
            "ALT" | "OPTION" => {
                modifiers |= Modifiers::ALT;
            }
            "SHIFT" => {
                modifiers |= Modifiers::SHIFT;
            }
            key => {
                key_code = match key {
                    "A" => Some(Code::KeyA),
                    "B" => Some(Code::KeyB),
                    "C" => Some(Code::KeyC),
                    "D" => Some(Code::KeyD),
                    "E" => Some(Code::KeyE),
                    "F" => Some(Code::KeyF),
                    "G" => Some(Code::KeyG),
                    "H" => Some(Code::KeyH),
                    "I" => Some(Code::KeyI),
                    "J" => Some(Code::KeyJ),
                    "K" => Some(Code::KeyK),
                    "L" => Some(Code::KeyL),
                    "M" => Some(Code::KeyM),
                    "N" => Some(Code::KeyN),
                    "O" => Some(Code::KeyO),
                    "P" => Some(Code::KeyP),
                    "Q" => Some(Code::KeyQ),
                    "R" => Some(Code::KeyR),
                    "S" => Some(Code::KeyS),
                    "T" => Some(Code::KeyT),
                    "U" => Some(Code::KeyU),
                    "V" => Some(Code::KeyV),
                    "W" => Some(Code::KeyW),
                    "X" => Some(Code::KeyX),
                    "Y" => Some(Code::KeyY),
                    "Z" => Some(Code::KeyZ),
                    "0" => Some(Code::Digit0),
                    "1" => Some(Code::Digit1),
                    "2" => Some(Code::Digit2),
                    "3" => Some(Code::Digit3),
                    "4" => Some(Code::Digit4),
                    "5" => Some(Code::Digit5),
                    "6" => Some(Code::Digit6),
                    "7" => Some(Code::Digit7),
                    "8" => Some(Code::Digit8),
                    "9" => Some(Code::Digit9),
                    "ENTER" | "RETURN" => Some(Code::Enter),
                    "ESCAPE" | "ESC" => Some(Code::Escape),
                    "BACKSPACE" => Some(Code::Backspace),
                    "TAB" => Some(Code::Tab),
                    "SPACE" => Some(Code::Space),
                    "MINUS" | "-" => Some(Code::Minus),
                    "EQUAL" | "=" => Some(Code::Equal),
                    "BRACKETLEFT" | "[" => Some(Code::BracketLeft),
                    "BRACKETRIGHT" | "]" => Some(Code::BracketRight),
                    "BACKSLASH" | "\\" => Some(Code::Backslash),
                    "SEMICOLON" | ";" => Some(Code::Semicolon),
                    "QUOTE" | "'" => Some(Code::Quote),
                    "BACKQUOTE" | "`" => Some(Code::Backquote),
                    "COMMA" | "," => Some(Code::Comma),
                    "PERIOD" | "." => Some(Code::Period),
                    "SLASH" | "/" => Some(Code::Slash),
                    _ => None,
                };
            }
        }
    }

    key_code.map(|code| {
        if modifiers.is_empty() {
            Accelerator::new(None, code)
        } else {
            Accelerator::new(Some(modifiers), code)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_shortcut() {
        let acc = parse_shortcut("Cmd+T");
        assert!(acc.is_some());
    }

    #[test]
    fn test_parse_with_shift() {
        let acc = parse_shortcut("Cmd+Shift+S");
        assert!(acc.is_some());
    }

    #[test]
    fn test_parse_empty() {
        let acc = parse_shortcut("");
        assert!(acc.is_none());
    }
}
