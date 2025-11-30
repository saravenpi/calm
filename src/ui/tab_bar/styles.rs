use crate::ui::fonts;

pub fn get_tab_bar_styles() -> String {
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
            overflow: hidden;
            background: transparent;
            color: #ffffff;
            width: 250px;
            height: 100vh;
            font-size: 11px;
            position: relative;
        }}

        body::after {{
            content: '';
            position: fixed;
            top: 0;
            right: 0;
            width: 2px;
            height: 100vh;
            background: #ffffff;
            z-index: 200;
            pointer-events: none;
        }}

        #sidebar-container {{
            display: flex;
            flex-direction: column;
            background: #000000;
            width: 250px;
            height: 100vh;
            padding-top: 84px;
        }}

        .url-bar-container {{
            flex-shrink: 0;
            padding: 12px;
            padding-bottom: 0;
            -webkit-app-region: drag;
        }}

        #tab-bar {{
            display: flex;
            flex-direction: column;
            align-items: stretch;
            background: #000000;
            width: 100%;
            flex: 1;
            overflow-y: auto;
            overflow-x: hidden;
            padding: 12px;
            padding-top: 4px;
            gap: 2px;
            user-select: none;
            -webkit-app-region: drag;
            transition: border-color 0.15s ease;
        }}

        #tab-bar.sidebar-focused {{
            border-right: 2px solid #ffffff;
        }}

        body::before {{
            content: '';
            position: fixed;
            top: 0;
            left: 0;
            width: 250px;
            height: 84px;
            -webkit-app-region: drag;
            pointer-events: none;
            z-index: 100;
            background: #000000;
        }}

        .tab, .new-tab-btn, .reload-btn, .back-btn, .forward-btn, .downloads-btn, .close-tab, .split-view-btn, .split-orientation-btn, .swap-panes-btn {{
            -webkit-app-region: no-drag;
        }}

        .tab {{
            display: flex;
            align-items: center;
            gap: 4px;
            padding: 8px;
            background: #1a1a1a;
            border: 1px solid #ffffff;
            cursor: pointer;
            width: 100%;
            transition: none;
            position: relative;
            overflow: hidden;
            min-height: 32px;
            flex-shrink: 0;
        }}

        .tab:hover {{
            background: #333333;
        }}

        .tab.active {{
            background: #ffffff;
            color: #000000;
        }}

        .tab.focused {{
            background: #ffffff;
            color: #000000;
        }}

        .tab.closing {{
            animation: tabClose 0.25s ease-out forwards;
        }}

        @keyframes tabClose {{
            0% {{
                opacity: 1;
                max-height: 32px;
                padding: 8px;
                margin-bottom: 2px;
                border-width: 1px;
            }}
            50% {{
                opacity: 0;
                max-height: 32px;
                padding: 8px 0;
                margin-bottom: 2px;
                border-width: 1px;
            }}
            100% {{
                opacity: 0;
                max-height: 0;
                padding: 0;
                margin-bottom: 0;
                border-width: 0;
            }}
        }}

        .tab.opening {{
            animation: tabOpen 0.2s linear;
        }}

        @keyframes tabOpen {{
            0% {{
                opacity: 0;
            }}
            100% {{
                opacity: 1;
            }}
        }}

        .tab-title {{
            flex: 1;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            font-size: 11px;
        }}

        .tab.active .tab-title {{
            font-weight: bold;
        }}

        .tab-audio-indicator {{
            width: 8px;
            height: 8px;
            display: none;
            border: 1px solid currentColor;
        }}

        .tab-audio-indicator.playing {{
            display: block;
        }}

        .tab-close {{
            width: 12px;
            height: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 16px;
            line-height: 1;
            transition: none;
        }}

        .tab-close:hover {{
            background: #ffffff;
            color: #000000;
        }}

        .tab.active .tab-close:hover {{
            background: #000000;
            color: #ffffff;
        }}

        .control-group {{
            position: fixed;
            top: 8px;
            right: 12px;
            display: flex;
            flex-direction: row;
            gap: 2px;
            z-index: 101;
            -webkit-app-region: no-drag;
        }}

        .new-tab-btn, .reload-btn, .back-btn, .forward-btn, .split-view-btn, .split-orientation-btn, .swap-panes-btn {{
            width: 32px;
            height: 32px;
            background: #1a1a1a;
            border: 1px solid #ffffff;
            color: #ffffff;
            cursor: pointer;
            transition: none;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 0;
            flex-shrink: 0;
        }}

        .new-tab-btn:hover, .reload-btn:hover, .back-btn:hover, .forward-btn:hover, .split-view-btn:hover, .split-orientation-btn:hover, .swap-panes-btn:hover {{
            background: #ffffff;
            color: #000000;
        }}

        .new-tab-btn:active, .reload-btn:active, .back-btn:active, .forward-btn:active, .split-view-btn:active, .split-orientation-btn:active, .swap-panes-btn:active {{
            background: #000000;
            color: #ffffff;
        }}

        .split-view-btn.active {{
            background: #ffffff;
            color: #000000;
        }}

        .back-btn:disabled, .forward-btn:disabled {{
            opacity: 0.3;
            cursor: not-allowed;
        }}

        .back-btn:disabled:hover, .forward-btn:disabled:hover {{
            background: #1a1a1a;
            color: #ffffff;
        }}

        .url-bar {{
            width: 100%;
            height: 32px;
            background: #000000;
            border: 1px solid #ffffff;
            color: #ffffff;
            padding: 0 8px;
            font-size: 11px;
            font-family: 'gohu', monospace;
            outline: none;
            transition: none;
            -webkit-app-region: no-drag;
            user-select: text;
            -webkit-user-select: text;
        }}

        .url-bar:focus {{
            background: #1a1a1a;
        }}

        .url-bar::placeholder {{
            color: #999999;
        }}

        .bottom-controls {{
            position: relative;
            padding: 12px;
            display: flex;
            flex-direction: row;
            gap: 8px;
            z-index: 102;
            -webkit-app-region: no-drag;
            flex-shrink: 0;
        }}

        .downloads-btn, .settings-btn {{
            width: 50%;
            height: 36px;
            background: #1a1a1a;
            border: 1px solid #ffffff;
            color: #ffffff;
            cursor: pointer;
            transition: none;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 16px;
            flex-shrink: 0;
        }}

        .downloads-btn svg, .settings-btn svg {{
            width: 16px;
            height: 16px;
        }}

        .downloads-btn:hover, .settings-btn:hover {{
            background: #ffffff;
            color: #000000;
        }}

        .downloads-btn:active, .settings-btn:active {{
            background: #000000;
            color: #ffffff;
        }}

        .downloads-btn.has-downloads {{
            background: #ffffff;
            color: #000000;
        }}

        .downloads-btn.pulse {{
            animation: downloadPulse 0.3s linear;
        }}

        @keyframes downloadPulse {{
            0%, 100% {{
                border-width: 1px;
            }}
            50% {{
                border-width: 2px;
            }}
        }}

        .download-badge {{
            position: absolute;
            top: -6px;
            right: -6px;
            background: #ffffff;
            color: #000000;
            min-width: 12px;
            height: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 8px;
            font-weight: bold;
            padding: 0 2px;
            border: 1px solid #000000;
            animation: badgeAppear 0.2s linear;
        }}

        @keyframes badgeAppear {{
            0% {{
                opacity: 0;
            }}
            100% {{
                opacity: 1;
            }}
        }}

        .split-view-controls {{
            position: fixed;
            top: 44px;
            right: 12px;
            display: flex;
            gap: 2px;
            z-index: 101;
            -webkit-app-region: no-drag;
        }}
    "#,
        fonts::get_gohu_font_face()
    )
}
