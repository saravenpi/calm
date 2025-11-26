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
            background: #000000;
            color: #ffffff;
            height: 40px;
            font-size: 11px;
        }

        #tab-bar {
            display: flex;
            align-items: center;
            background: #000000;
            border-bottom: 2px solid #ffffff;
            height: 40px;
            padding: 0 4px;
            padding-left: 80px;
            gap: 2px;
            user-select: none;
            -webkit-app-region: drag;
        }

        body::before {
            content: '';
            position: fixed;
            top: 0;
            left: 0;
            width: 80px;
            height: 40px;
            -webkit-app-region: drag;
            pointer-events: none;
        }

        .tab, .new-tab-btn, .reload-btn, .back-btn, .forward-btn, .downloads-btn, .close-tab {
            -webkit-app-region: no-drag;
        }

        .tab {
            display: flex;
            align-items: center;
            gap: 4px;
            padding: 4px 8px;
            background: #1a1a1a;
            border: 1px solid #ffffff;
            cursor: pointer;
            max-width: 200px;
            min-width: 120px;
            transition: none;
            position: relative;
            overflow: hidden;
            height: 28px;
        }

        .tab:hover {
            background: #333333;
        }

        .tab.active {
            background: #ffffff;
            color: #000000;
        }

        .tab.closing {
            animation: tabClose 0.2s linear forwards;
        }

        @keyframes tabClose {
            100% {
                opacity: 0;
                max-width: 0;
                padding: 0;
                margin: 0;
                border: 0;
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

        .new-tab-btn, .reload-btn, .back-btn, .forward-btn {
            width: 28px;
            height: 28px;
            background: #1a1a1a;
            border: 1px solid #ffffff;
            color: #ffffff;
            cursor: pointer;
            transition: none;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 0;
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
            flex: 1;
            max-width: 600px;
            height: 28px;
            background: #000000;
            border: 1px solid #ffffff;
            color: #ffffff;
            padding: 0 8px;
            font-size: 11px;
            font-family: 'gohu', monospace;
            outline: none;
            transition: none;
            margin-left: auto;
            margin-right: 4px;
        }

        .url-bar:focus {
            background: #1a1a1a;
        }

        .url-bar::placeholder {
            color: #666666;
        }

        .downloads-btn {
            width: 28px;
            height: 28px;
            background: #1a1a1a;
            border: 1px solid #ffffff;
            color: #ffffff;
            cursor: pointer;
            transition: none;
            margin-right: 4px;
            position: relative;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 0;
        }

        .downloads-btn svg {
            width: 16px;
            height: 16px;
        }

        .downloads-btn:hover {
            background: #ffffff;
            color: #000000;
        }

        .downloads-btn:active {
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
