use crate::ui::fonts;

pub fn get_command_prompt_styles() -> String {
    format!(
        r#"
        {}

        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
            image-rendering: pixelated;
            image-rendering: crisp-edges;
        }}

        body {{
            font-family: 'gohu', monospace;
            background: transparent;
            overflow: hidden;
            margin: 0;
            padding: 0;
            width: 100vw;
            height: 100vh;
            font-size: 11px;
            position: relative;
        }}

        .command-prompt-backdrop {{
            position: fixed;
            top: 0;
            left: 0;
            width: 100vw;
            height: 100vh;
            background: rgba(0, 0, 0, 0.7);
            cursor: pointer;
            z-index: 9998;
            animation: backdropFadeIn 0.1s linear;
        }}

        .command-prompt-container {{
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            width: 600px;
            max-width: 90%;
            background: #101010;
            border: 2px solid #ffffff;
            padding: 0;
            z-index: 9999;
            animation: promptSlideIn 0.15s ease-out;
        }}

        .command-prompt-input {{
            width: 100%;
            padding: 16px;
            font-family: 'gohu', monospace;
            font-size: 16px;
            color: #ffffff;
            background: #101010;
            border: none;
            outline: none;
        }}

        .command-prompt-input::placeholder {{
            color: #666666;
        }}

        .command-prompt-input:focus {{
            background: #101010;
        }}

        .command-prompt-hint {{
            display: none;
        }}

        @keyframes backdropFadeIn {{
            from {{
                opacity: 0;
            }}
            to {{
                opacity: 1;
            }}
        }}

        @keyframes promptSlideIn {{
            from {{
                transform: translate(-50%, calc(-50% - 20px));
                opacity: 0;
            }}
            to {{
                transform: translate(-50%, -50%);
                opacity: 1;
            }}
        }}
    "#,
        fonts::get_gohu_font_face()
    )
}
