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
            background: rgba(0, 0, 0, 0.4);
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
            border: 2px solid #333333;
            padding: 0;
            z-index: 9999;
            animation: promptSlideIn 0.15s ease-out;
        }}

        .input-wrapper {{
            display: flex;
            align-items: center;
            padding: 16px;
        }}

        .search-icon {{
            width: 20px;
            height: 20px;
            margin-right: 12px;
            color: #666666;
            flex-shrink: 0;
        }}

        .command-prompt-input {{
            width: 100%;
            padding: 0;
            font-family: 'gohu', monospace;
            font-size: 16px;
            color: #ffffff;
            background: transparent;
            border: none;
            outline: none;
        }}

        .command-prompt-input::placeholder {{
            color: #444444;
        }}

        .command-prompt-input:focus {{
            background: transparent;
        }}

        .command-prompt-hint {{
            display: none;
        }}

        .command-prompt-suggestions {{
            max-height: 400px;
            overflow-y: auto;
        }}

        .command-prompt-suggestion {{
            padding: 10px 16px;
            cursor: pointer;
            border-bottom: 1px solid #1a1a1a;
            transition: background 0.05s linear;
            display: flex;
            align-items: center;
            gap: 12px;
        }}

        .command-prompt-suggestion:last-child {{
            border-bottom: none;
        }}

        .command-prompt-suggestion:hover,
        .command-prompt-suggestion.selected {{
            background: #1a1a1a;
        }}

        .suggestion-icon {{
            width: 16px;
            height: 16px;
            flex-shrink: 0;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #888888;
        }}

        .suggestion-info {{
            flex: 1;
            min-width: 0;
            display: flex;
            flex-direction: column;
            justify-content: center;
        }}

        .suggestion-title {{
            color: #ffffff;
            font-size: 13px;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            line-height: 1.2;
        }}

        .suggestion-url {{
            color: #444444;
            font-size: 11px;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            margin-left: 12px;
            max-width: 40%;
            text-align: right;
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
