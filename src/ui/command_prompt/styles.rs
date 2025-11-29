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
            animation: backdropFadeIn 0.15s linear;
        }}

        .command-prompt-container {{
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            width: 500px;
            max-width: 90%;
            background: #000000;
            border: 2px solid #ffffff;
            padding: 16px;
            z-index: 9999;
            animation: promptSlideIn 0.2s ease-out;
        }}

        .command-prompt-input {{
            width: 100%;
            padding: 12px;
            font-family: 'gohu', monospace;
            font-size: 14px;
            color: #ffffff;
            background: #1a1a1a;
            border: 1px solid #ffffff;
            outline: none;
            margin-bottom: 8px;
        }}

        .command-prompt-input::placeholder {{
            color: #666666;
        }}

        .command-prompt-input:focus {{
            border-color: #ffffff;
            background: #000000;
        }}

        .command-prompt-hint {{
            font-size: 10px;
            color: #666666;
            text-align: center;
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
