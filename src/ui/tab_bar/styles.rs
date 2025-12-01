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
            background: #333333;
            z-index: 200;
            pointer-events: none;
        }}

        #sidebar-container {{
            display: flex;
            flex-direction: column;
            background: #101010;
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
            background: #101010;
            width: 100%;
            flex: 1;
            overflow-y: auto;
            overflow-x: hidden;
            padding: 12px;
            padding-top: 4px;
            gap: 6px;
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
            background: #101010;
        }}

        .tab, .new-tab-btn, .reload-btn, .back-btn, .forward-btn, .downloads-btn, .close-tab, .split-view-btn, .split-orientation-btn, .swap-panes-btn {{
            -webkit-app-region: no-drag;
        }}

        .tab {{
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 12px;
            background: #1a1a1a;
            border: 1px solid #333333;
            cursor: pointer;
            width: 100%;
            transition: none;
            position: relative;
            overflow: hidden;
            min-height: 40px;
            flex-shrink: 0;
        }}

        .tab::before {{
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            height: 100%;
            width: 0%;
            background: #ffffff;
            z-index: 0;
            transition: none;
        }}

        .tab.loading::before {{
            animation: loadingProgress 2s ease-in-out infinite;
        }}

        @keyframes loadingProgress {{
            0% {{
                width: 0%;
                opacity: 1;
            }}
            50% {{
                width: 70%;
                opacity: 1;
            }}
            100% {{
                width: 100%;
                opacity: 0;
            }}
        }}

        .tab > * {{
            position: relative;
            z-index: 1;
        }}

        .tab:hover {{
            background: #333333;
            border-color: #ffffff;
        }}

        .tab.active {{
            background: #ffffff;
            color: #000000;
            border-color: #ffffff;
        }}

        .tab.focused {{
            background: #ffffff;
            color: #000000;
            border-color: #ffffff;
        }}

        .tab.closing {{
            animation: tabClose 0.2s ease-out forwards;
        }}

        @keyframes tabClose {{
            0% {{
                opacity: 1;
                transform: translateX(0);
                max-height: 40px;
                margin-bottom: 6px;
            }}
            100% {{
                opacity: 0;
                transform: translateX(-100%);
                max-height: 0;
                margin-bottom: 0;
                padding-top: 0;
                padding-bottom: 0;
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

        .tab-favicon {{
            width: 16px;
            height: 16px;
            flex-shrink: 0;
            display: block;
            image-rendering: auto;
            background: #333333;
            opacity: 0.3;
        }}

        .tab-favicon.loaded {{
            opacity: 1;
            background: transparent;
        }}

        .tab-title {{
            flex: 1;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            font-size: 11px;
        }}

        .tab.active .tab-title {{
        }}


        .tab-audio-indicator {{
            display: none;
            font-size: 11px;
            flex-shrink: 0;
            animation: audioWave 1s ease-in-out infinite;
        }}

        .tab-audio-indicator.playing {{
            display: block;
        }}

        @keyframes audioWave {{
            0%, 100% {{
                opacity: 0.6;
                transform: scale(1);
            }}
            50% {{
                opacity: 1;
                transform: scale(1.1);
            }}
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
            border: 1px solid #333333;
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
            border-color: #ffffff;
        }}

        .new-tab-btn:active, .reload-btn:active, .back-btn:active, .forward-btn:active, .split-view-btn:active, .split-orientation-btn:active, .swap-panes-btn:active {{
            background: #101010;
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
            background: #101010;
            border: 1px solid #333333;
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
            border-color: #ffffff;
        }}

        .url-bar::placeholder {{
            color: #999999;
        }}

        .bottom-controls {{
            position: relative;
            padding: 12px 20px 12px 12px;
            display: flex;
            flex-direction: row;
            gap: 8px;
            z-index: 102;
            -webkit-app-region: no-drag;
            flex-shrink: 0;
        }}

        .downloads-btn {{
            width: 50%;
            height: 36px;
            background: #1a1a1a;
            border: 1px solid #333333;
            color: #ffffff;
            cursor: pointer;
            transition: none;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 0;
            flex-shrink: 0;
            position: relative;
            overflow: hidden;
        }}

        .settings-btn {{
            width: 50%;
            height: 36px;
            background: #1a1a1a;
            border: 1px solid #333333;
            color: #ffffff;
            cursor: pointer;
            transition: none;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 0;
            padding-right: 4px;
            flex-shrink: 0;
            position: relative;
            overflow: hidden;
        }}

        .download-btn-content {{
            display: flex;
            align-items: center;
            justify-content: center;
            width: 100%;
            height: 100%;
            position: relative;
            z-index: 1;
        }}

        .download-icon-wrapper {{
            display: flex;
            align-items: center;
            justify-content: center;
            transition: transform 0.2s ease-out;
        }}

        .downloads-btn.has-downloads .download-icon-wrapper {{
            transform: translateX(-8px);
        }}

        .download-btn-progress {{
            position: absolute;
            bottom: 0;
            left: 0;
            width: 100%;
            height: 3px;
            background: #333333;
            z-index: 0;
        }}

        .download-btn-progress-fill {{
            height: 100%;
            background: #ffffff;
            width: 0%;
            transition: width 0.3s ease;
        }}

        .downloads-btn svg, .settings-btn svg {{
            width: 16px;
            height: 16px;
        }}

        .downloads-btn:hover, .settings-btn:hover {{
            background: #ffffff;
            color: #000000;
            border-color: #ffffff;
        }}

        .downloads-btn:active, .settings-btn:active {{
            background: #101010;
            color: #ffffff;
        }}

        .downloads-btn.has-downloads {{
            background: #ffffff;
            color: #000000;
            border-color: #333333;
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
            top: 0;
            right: 0;
            bottom: 0;
            background: #333333;
            color: #ffffff;
            min-width: 24px;
            display: none;
            align-items: center;
            justify-content: center;
            font-size: 11px;
            font-family: 'gohu', monospace;
            padding: 0 6px;
            animation: badgeSlideIn 0.2s ease-out;
            overflow: hidden;
        }}

        .downloads-btn.has-downloads .download-badge {{
            display: flex;
        }}

        .download-badge.rolling {{
            animation: rollNumber 0.3s ease-in-out;
        }}

        .download-badge-spinner {{
            display: inline-block;
            width: 8px;
            height: 8px;
            border: 1px solid #666666;
            border-top-color: #ffffff;
            animation: spin 0.6s linear infinite;
        }}

        @keyframes spin {{
            to {{
                transform: rotate(360deg);
            }}
        }}

        @keyframes badgeSlideIn {{
            from {{
                transform: translateX(100%);
                opacity: 0;
            }}
            to {{
                transform: translateX(0);
                opacity: 1;
            }}
        }}

        @keyframes rollNumber {{
            0% {{
                transform: translateY(0);
                opacity: 1;
            }}
            50% {{
                transform: translateY(-100%);
                opacity: 0;
            }}
            51% {{
                transform: translateY(100%);
                opacity: 0;
            }}
            100% {{
                transform: translateY(0);
                opacity: 1;
            }}
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
