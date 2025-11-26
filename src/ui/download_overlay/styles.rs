pub fn get_styles() -> &'static str {
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
            background: #000000;
            overflow: hidden;
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            font-size: 11px;
        }

        .downloads-panel {
            width: 100%;
            height: 100%;
            background: #000000;
            display: flex;
            flex-direction: column;
            border-left: 2px solid #ffffff;
            transform: translateX(100%);
            transition: transform 0.1s linear;
        }

        .downloads-content {
            width: 100%;
            height: 100%;
            background: #000000;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }

        .downloads-header {
            padding: 8px;
            font-size: 11px;
            font-weight: bold;
            color: #000000;
            background: #ffffff;
            border-bottom: 2px solid #ffffff;
            flex-shrink: 0;
        }

        .downloads-list {
            overflow-y: auto;
            flex: 1;
            padding: 4px;
        }

        .downloads-list::-webkit-scrollbar {
            width: 8px;
        }

        .downloads-list::-webkit-scrollbar-track {
            background: #000000;
        }

        .downloads-list::-webkit-scrollbar-thumb {
            background: #ffffff;
        }

        .downloads-list::-webkit-scrollbar-thumb:hover {
            background: #cccccc;
        }

        .downloads-empty {
            padding: 16px 8px;
            text-align: center;
            color: #666666;
            font-size: 11px;
        }

        .download-item {
            background: #1a1a1a;
            border: 1px solid #ffffff;
            padding: 8px;
            margin-bottom: 4px;
            color: #ffffff;
            animation: downloadItemEnter 0.2s linear;
            transition: none;
        }

        .download-item:hover {
            background: #333333;
            color: #ffffff;
        }

        .download-item.completed {
            background: #1a4d1a;
            color: #ffffff;
            border-color: #00ff00;
        }

        .download-item.failed {
            background: #4d1a1a;
            color: #ffffff;
            border-color: #ff0000;
        }

        .download-item.removing {
            opacity: 0;
        }

        .download-name {
            color: inherit;
            font-size: 11px;
            font-weight: bold;
            margin-bottom: 4px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        .download-progress-bar {
            height: 8px;
            background: #333333;
            border: 1px solid #666666;
            overflow: hidden;
            margin-bottom: 4px;
        }

        .download-progress-fill {
            height: 100%;
            background: #ffffff;
            transition: none;
        }

        .download-progress-fill.indeterminate {
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
        }

        .download-progress-fill.completed {
            background: #00ff00 !important;
            animation: none !important;
            width: 100% !important;
        }

        .download-progress-fill.failed {
            background: #ff0000 !important;
            animation: none !important;
            width: 100% !important;
        }

        @keyframes indeterminateProgress {
            0% {
                background-position: 0 0;
            }
            100% {
                background-position: 16px 0;
            }
        }

        .download-info {
            display: flex;
            justify-content: space-between;
            font-size: 9px;
            color: inherit;
        }

        @keyframes downloadItemEnter {
            from {
                opacity: 0;
            }
            to {
                opacity: 1;
            }
        }

        .download-spinner {
            display: inline-block;
            width: 8px;
            height: 8px;
            border: 1px solid #666666;
            border-top-color: #ffffff;
            animation: spin 0.6s linear infinite;
            margin-right: 4px;
            vertical-align: middle;
        }

        @keyframes spin {
            to {
                transform: rotate(360deg);
            }
        }
    "#
}
