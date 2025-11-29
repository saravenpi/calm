use crate::ui::fonts;

pub fn get_styles() -> String {
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
            background: #000000;
            overflow: hidden;
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            font-size: 11px;
        }}

        .downloads-panel {{
            width: 100%;
            height: 100vh;
            background: #000000;
            display: flex;
            flex-direction: column;
            border-left: 2px solid #ffffff;
            transform: translateX(100%);
            transition: transform 0.1s linear;
            position: fixed;
            top: 0;
            right: 0;
        }}

        .downloads-content {{
            width: 100%;
            height: 100%;
            background: #000000;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }}

        .downloads-header {{
            padding: 8px;
            font-size: 11px;
            font-weight: bold;
            color: #ffffff;
            background: #000000;
            border-bottom: 1px solid #ffffff;
            flex-shrink: 0;
        }}

        .downloads-list {{
            overflow-y: auto;
            flex: 1;
            padding: 4px;
        }}

        .downloads-list::-webkit-scrollbar {{
            width: 8px;
        }}

        .downloads-list::-webkit-scrollbar-track {{
            background: #000000;
        }}

        .downloads-list::-webkit-scrollbar-thumb {{
            background: #ffffff;
        }}

        .downloads-list::-webkit-scrollbar-thumb:hover {{
            background: #cccccc;
        }}

        .downloads-empty {{
            padding: 16px 8px;
            text-align: center;
            color: #666666;
            font-size: 11px;
        }}

        .download-item {{
            background: #1a1a1a;
            border: 1px solid #ffffff;
            padding: 8px;
            margin-bottom: 4px;
            color: #ffffff;
            animation: downloadItemEnter 0.2s linear;
            transition: none;
        }}

        .download-item:hover {{
            background: #333333;
            color: #ffffff;
        }}

        .download-item.completed {{
            background: #1a4d1a;
            color: #ffffff;
            border-color: #00ff00;
        }}

        .download-item.failed {{
            background: #4d1a1a;
            color: #ffffff;
            border-color: #ff0000;
        }}

        .download-item.removing {{
            opacity: 0;
        }}

        .download-name {{
            color: inherit;
            font-size: 11px;
            font-weight: bold;
            margin-bottom: 4px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }}

        .download-progress-bar {{
            height: 8px;
            background: #333333;
            border: 1px solid #666666;
            overflow: hidden;
            margin-bottom: 4px;
        }}

        .download-progress-fill {{
            height: 100%;
            background: #ffffff;
            transition: none;
        }}

        .download-progress-fill.indeterminate {{
            width: 100%;
            background: repeating-linear-gradient(
                90deg,
                #ffffff 0px,
                #ffffff 4px,
                #000000 4px,
                #000000 8px
            );
            background-size: 16px 100%;
            animation: indeterminateProgress 0.5s infinite linear;
        }}

        .download-progress-fill.completed {{
            background: #00ff00 !important;
            animation: none !important;
            width: 100% !important;
        }}

        .download-progress-fill.failed {{
            background: #ff0000 !important;
            animation: none !important;
            width: 100% !important;
        }}

        @keyframes indeterminateProgress {{
            0% {{
                background-position: 0 0;
            }}
            100% {{
                background-position: 16px 0;
            }}
        }}

        .download-info {{
            display: flex;
            justify-content: space-between;
            font-size: 9px;
            color: inherit;
        }}

        @keyframes downloadItemEnter {{
            from {{
                opacity: 0;
            }}
            to {{
                opacity: 1;
            }}
        }}

        .download-spinner {{
            display: inline-block;
            width: 8px;
            height: 8px;
            border: 1px solid #666666;
            border-top-color: #ffffff;
            animation: spin 0.6s linear infinite;
            margin-right: 4px;
            vertical-align: middle;
        }}

        @keyframes spin {{
            to {{
                transform: rotate(360deg);
            }}
        }}

        .download-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 4px;
        }}

        .download-folder-icon {{
            cursor: pointer;
            font-size: 16px;
            padding: 2px 4px;
            flex-shrink: 0;
            opacity: 0.7;
            transition: opacity 0.1s linear;
        }}

        .download-folder-icon:hover {{
            opacity: 1;
        }}

        .download-context-menu {{
            position: fixed;
            background: #1a1a1a;
            border: 2px solid #ffffff;
            min-width: 180px;
            z-index: 10000;
            font-size: 11px;
            color: #ffffff;
            padding: 4px;
        }}

        .context-menu-item {{
            padding: 6px 8px;
            cursor: pointer;
            color: #ffffff;
            transition: background 0.05s linear;
        }}

        .context-menu-item:hover {{
            background: #333333;
        }}

        .context-menu-separator {{
            height: 1px;
            background: #666666;
            margin: 2px 0;
        }}

        .downloads-footer {{
            padding: 8px;
            border-top: 1px solid #ffffff;
            background: #000000;
            flex-shrink: 0;
            display: flex;
            justify-content: center;
        }}

        .clear-history-btn {{
            background: #1a1a1a;
            color: #ffffff;
            border: 1px solid #ffffff;
            padding: 6px 12px;
            font-size: 11px;
            font-family: 'gohu', monospace;
            cursor: pointer;
            transition: background 0.05s linear;
        }}

        .clear-history-btn:hover {{
            background: #333333;
        }}

        .clear-history-btn:active {{
            background: #ffffff;
            color: #000000;
        }}
    "#,
        fonts::get_gohu_font_face()
    )
}
