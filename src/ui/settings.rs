pub fn get_settings_html() -> String {
    let styles = r#"
        <style>
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
                color: #ffffff;
                padding: 40px;
                line-height: 1.6;
                font-size: 11px;
            }

            .settings-container {
                max-width: 800px;
                margin: 0 auto;
            }

            h1 {
                font-size: 24px;
                margin-bottom: 8px;
                font-weight: bold;
            }

            .subtitle {
                color: #999999;
                font-size: 11px;
                margin-bottom: 24px;
            }

            .setting-section {
                background: #000000;
                border: 2px solid #ffffff;
                padding: 16px;
                margin-bottom: 16px;
            }

            .setting-section h2 {
                font-size: 14px;
                margin-bottom: 12px;
                font-weight: bold;
                border-bottom: 1px solid #ffffff;
                padding-bottom: 4px;
            }

            .setting-item {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 12px 0;
                border-bottom: 1px solid #333333;
            }

            .setting-item:last-child {
                border-bottom: none;
            }

            .setting-info {
                flex: 1;
            }

            .setting-label {
                font-size: 11px;
                margin-bottom: 2px;
                font-weight: bold;
            }

            .setting-description {
                font-size: 11px;
                color: #666666;
            }

            .setting-control {
                margin-left: 16px;
            }

            input[type="text"],
            input[type="number"] {
                background: #000000;
                border: 1px solid #ffffff;
                color: #ffffff;
                padding: 6px 8px;
                font-size: 11px;
                font-family: 'gohu', monospace;
                min-width: 200px;
            }

            select {
                background: #000000;
                border: 1px solid #ffffff;
                color: #ffffff;
                padding: 6px 8px;
                font-size: 11px;
                font-family: 'gohu', monospace;
                min-width: 200px;
                appearance: none;
                -webkit-appearance: none;
                -moz-appearance: none;
                border-radius: 0;
                cursor: pointer;
                background-image: none;
                padding-right: 24px;
                background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='white' stroke-width='2' stroke-linecap='square' stroke-linejoin='miter'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
                background-repeat: no-repeat;
                background-position: right 4px center;
                background-size: 16px;
            }

            select:hover {
                background-color: #1a1a1a;
            }

            select option {
                background: #000000;
                color: #ffffff;
                padding: 4px;
            }

            input[type="text"]:focus,
            input[type="number"]:focus,
            select:focus {
                outline: none;
                border: 2px solid #ffffff;
                background: #1a1a1a;
            }

            input[type="range"] {
                -webkit-appearance: none;
                appearance: none;
                width: 200px;
                height: 4px;
                background: #333333;
                outline: none;
                border: 1px solid #ffffff;
            }

            input[type="range"]::-webkit-slider-thumb {
                -webkit-appearance: none;
                appearance: none;
                width: 16px;
                height: 16px;
                background: #ffffff;
                cursor: pointer;
                border: 2px solid #000000;
            }

            input[type="range"]::-moz-range-thumb {
                width: 16px;
                height: 16px;
                background: #ffffff;
                cursor: pointer;
                border: 2px solid #000000;
            }

            .slider-value {
                display: inline-block;
                min-width: 40px;
                margin-left: 8px;
                font-family: 'gohu', monospace;
                font-size: 11px;
            }

            input[type="checkbox"] {
                appearance: none;
                -webkit-appearance: none;
                width: 16px;
                height: 16px;
                border: 2px solid #ffffff;
                background: #000000;
                cursor: pointer;
                position: relative;
            }

            input[type="checkbox"]:checked {
                background: #000000;
            }

            input[type="checkbox"]:checked::before {
                content: '';
                position: absolute;
                left: 1px;
                top: 6px;
                width: 10px;
                height: 2px;
                background: #ffffff;
                transform: rotate(45deg);
            }

            input[type="checkbox"]:checked::after {
                content: '';
                position: absolute;
                left: 1px;
                top: 6px;
                width: 10px;
                height: 2px;
                background: #ffffff;
                transform: rotate(-45deg);
            }

            input[type="checkbox"]:hover {
                border-color: #ffffff;
                background: #1a1a1a;
            }

            input[type="checkbox"]:checked:hover {
                background: #0a0a0a;
            }

            button {
                background: #ffffff;
                color: #000000;
                border: 2px solid #ffffff;
                padding: 8px 16px;
                font-size: 11px;
                font-family: 'gohu', monospace;
                font-weight: bold;
                cursor: pointer;
            }

            button:hover {
                background: #000000;
                color: #ffffff;
            }

            button:active {
                background: #333333;
            }

            .save-indicator {
                display: inline-block;
                margin-left: 12px;
                color: #ffffff;
                font-size: 11px;
                opacity: 0;
                transition: opacity 0.3s;
                border: 1px solid #ffffff;
                padding: 4px 8px;
                background: #000000;
            }

            .save-indicator.show {
                opacity: 1;
            }

            .save-section {
                display: flex;
                justify-content: flex-end;
                align-items: center;
                margin-top: 24px;
            }
        </style>
    "#;

    let script = r#"
        <script>
            function saveSettings() {
                const settings = {
                    defaultUrl: document.getElementById('default-url').value,
                    searchEngine: document.getElementById('search-engine').value,
                    blockTrackers: document.getElementById('block-trackers').checked,
                    blockFingerprinting: document.getElementById('block-fingerprinting').checked,
                    blockCookies: document.getElementById('block-cookies').checked,
                };

                const message = JSON.stringify({
                    action: 'save_settings',
                    settings: settings
                });

                window.ipc.postMessage(message);

                const indicator = document.querySelector('.save-indicator');
                indicator.classList.add('show');
                setTimeout(() => {
                    indicator.classList.remove('show');
                }, 2000);
            }

            function loadSettings() {
                const message = JSON.stringify({
                    action: 'load_settings'
                });
                window.ipc.postMessage(message);
            }

            window.updateSettings = function(settings) {
                if (settings.defaultUrl) {
                    document.getElementById('default-url').value = settings.defaultUrl;
                }
                if (settings.searchEngine) {
                    document.getElementById('search-engine').value = settings.searchEngine;
                }
                if (settings.blockTrackers !== undefined) {
                    document.getElementById('block-trackers').checked = settings.blockTrackers;
                }
                if (settings.blockFingerprinting !== undefined) {
                    document.getElementById('block-fingerprinting').checked = settings.blockFingerprinting;
                }
                if (settings.blockCookies !== undefined) {
                    document.getElementById('block-cookies').checked = settings.blockCookies;
                }
            };

            window.addEventListener('load', loadSettings);
        </script>
    "#;

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Settings - Calm Browser</title>
    {}
</head>
<body>
    <div class="settings-container">
        <h1>Settings</h1>
        <p class="subtitle">Customize your Calm Browser experience</p>

        <div class="setting-section">
            <h2>General</h2>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Default URL</div>
                    <div class="setting-description">The page that opens when you start Calm</div>
                </div>
                <div class="setting-control">
                    <input type="text" id="default-url" placeholder="https://example.com">
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Search Engine</div>
                    <div class="setting-description">Default search engine for queries</div>
                </div>
                <div class="setting-control">
                    <select id="search-engine">
                        <option value="https://duckduckgo.com/?q=">DuckDuckGo</option>
                        <option value="https://www.google.com/search?q=">Google</option>
                        <option value="https://www.bing.com/search?q=">Bing</option>
                        <option value="https://search.brave.com/search?q=">Brave</option>
                    </select>
                </div>
            </div>
        </div>

        <div class="setting-section">
            <h2>Privacy</h2>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Block Trackers</div>
                    <div class="setting-description">Prevent tracking scripts from running</div>
                </div>
                <div class="setting-control">
                    <input type="checkbox" id="block-trackers" checked>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Block Fingerprinting</div>
                    <div class="setting-description">Protect against browser fingerprinting</div>
                </div>
                <div class="setting-control">
                    <input type="checkbox" id="block-fingerprinting" checked>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Block Third-Party Cookies</div>
                    <div class="setting-description">Prevent third-party cookies from being set</div>
                </div>
                <div class="setting-control">
                    <input type="checkbox" id="block-cookies" checked>
                </div>
            </div>
        </div>

        <div class="save-section">
            <button onclick="saveSettings()">Save Settings</button>
            <span class="save-indicator">✓ Saved</span>
        </div>
    </div>
    {}
</body>
</html>"#,
        styles, script
    )
}
