pub fn get_tab_bar_styles() -> &'static str {
    r#"
        @font-face {
            font-family: 'gohu';
            src: url('data:application/font-woff2;charset=utf-8;base64,d09GMgABAAAAABEYAA4AAAAAJKAAABDEAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAGhYbHhyBbAZgAIEICghWCYM8EQwKgdRgg7hzC4NAAAE2AiQDhx4EIAWDAAeFPQyBZxu8IqOQHkO2HySh4MbHmL/M3v+TQLAbdqUBAhsOACi1kcDKiixZqgMYVqyqKju0e3//b7Pb1SIRqC5SN6ErCDmT0DSReCdh0kgkn2Dz/P//n3m/+cXsO7PvzO07c+/M3DszJJKS5kkk0kmkk0gkkdwk0kmke+/eJvfuPQe9e5vce/emL7lJokTy3ntPIin33ntP7r33nvz/f5tdYKuqKqmqomor87+ft3e21N7ZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9s2afbNm36zZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9s2afbNm36zZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9s2afbNm36zZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9v0HAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA') format('woff2');
            font-weight: normal;
            font-style: normal;
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
            image-rendering: pixelated;
            image-rendering: crisp-edges;
        }

        body {
            font-family: 'gohu', monospace;
            overflow: hidden;
            background: transparent;
            color: #ffffff;
            width: 250px;
            height: 100vh;
            font-size: 11px;
        }

        #tab-bar {
            display: flex;
            flex-direction: column;
            align-items: stretch;
            background: #000000;
            border-right: 2px solid #ffffff;
            width: 250px;
            height: 100vh;
            padding: 4px;
            padding-top: 50px;
            padding-bottom: 44px;
            gap: 2px;
            user-select: none;
            -webkit-app-region: drag;
            overflow-y: auto;
            overflow-x: hidden;
        }

        body::before {
            content: '';
            position: fixed;
            top: 0;
            left: 0;
            width: 250px;
            height: 50px;
            -webkit-app-region: drag;
            pointer-events: none;
            z-index: 100;
            background: #000000;
            border-right: 2px solid #ffffff;
        }

        .tab, .new-tab-btn, .reload-btn, .back-btn, .forward-btn, .downloads-btn, .close-tab {
            -webkit-app-region: no-drag;
        }

        .tab {
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
        }

        .tab:hover {
            background: #333333;
        }

        .tab.active {
            background: #ffffff;
            color: #000000;
        }

        .tab.focused {
            outline: 2px solid #ffffff;
            outline-offset: -2px;
        }

        .tab.closing {
            animation: tabClose 0.25s ease-out forwards;
        }

        @keyframes tabClose {
            0% {
                opacity: 1;
                max-height: 32px;
                padding: 8px;
                margin-bottom: 2px;
                border-width: 1px;
            }
            50% {
                opacity: 0;
                max-height: 32px;
                padding: 8px 0;
                margin-bottom: 2px;
                border-width: 1px;
            }
            100% {
                opacity: 0;
                max-height: 0;
                padding: 0;
                margin-bottom: 0;
                border-width: 0;
            }
        }

        .tab.opening {
            animation: tabOpen 0.2s linear;
        }

        @keyframes tabOpen {
            0% {
                opacity: 0;
            }
            100% {
                opacity: 1;
            }
        }

        .tab-title {
            flex: 1;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            font-size: 11px;
        }

        .tab.active .tab-title {
            font-weight: bold;
        }

        .tab-audio-indicator {
            width: 8px;
            height: 8px;
            display: none;
            border: 1px solid currentColor;
        }

        .tab-audio-indicator.playing {
            display: block;
        }

        .tab-close {
            width: 12px;
            height: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 16px;
            line-height: 1;
            transition: none;
        }

        .tab-close:hover {
            background: #ffffff;
            color: #000000;
        }

        .tab.active .tab-close:hover {
            background: #000000;
            color: #ffffff;
        }

        .control-group {
            position: fixed;
            top: 8px;
            right: 4px;
            display: flex;
            flex-direction: row;
            gap: 2px;
            z-index: 101;
            -webkit-app-region: no-drag;
        }

        .new-tab-btn, .reload-btn, .back-btn, .forward-btn {
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
        }

        .new-tab-btn:hover, .reload-btn:hover, .back-btn:hover, .forward-btn:hover {
            background: #ffffff;
            color: #000000;
        }

        .new-tab-btn:active, .reload-btn:active, .back-btn:active, .forward-btn:active {
            background: #000000;
            color: #ffffff;
        }

        .back-btn:disabled, .forward-btn:disabled {
            opacity: 0.3;
            cursor: not-allowed;
        }

        .back-btn:disabled:hover, .forward-btn:disabled:hover {
            background: #1a1a1a;
            color: #ffffff;
        }

        .url-bar {
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
            flex-shrink: 0;
            margin-bottom: 4px;
        }

        .url-bar:focus {
            background: #1a1a1a;
        }

        .url-bar::placeholder {
            color: #999999;
        }

        .bottom-controls {
            position: fixed;
            bottom: 4px;
            left: 4px;
            width: 242px;
            display: flex;
            flex-direction: row;
            gap: 2px;
            z-index: 102;
            -webkit-app-region: no-drag;
        }

        .downloads-btn, .settings-btn {
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
            padding: 0;
            flex-shrink: 0;
        }

        .downloads-btn svg, .settings-btn svg {
            width: 16px;
            height: 16px;
        }

        .downloads-btn:hover, .settings-btn:hover {
            background: #ffffff;
            color: #000000;
        }

        .downloads-btn:active, .settings-btn:active {
            background: #000000;
            color: #ffffff;
        }

        .downloads-btn.has-downloads {
            background: #ffffff;
            color: #000000;
        }

        .downloads-btn.pulse {
            animation: downloadPulse 0.3s linear;
        }

        @keyframes downloadPulse {
            0%, 100% {
                border-width: 1px;
            }
            50% {
                border-width: 2px;
            }
        }

        .download-badge {
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
        }

        @keyframes badgeAppear {
            0% {
                opacity: 0;
            }
            100% {
                opacity: 1;
            }
        }
    "#
}
