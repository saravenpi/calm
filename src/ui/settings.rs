use crate::ui::fonts;

/// Returns the HTML content for the settings page.
pub fn get_settings_html() -> String {
    let styles = format!(
        r#"
        <style>
            {}

            * {{
                margin: 0;
                padding: 0;
                box-sizing: border-box;
                image-rendering: pixelated;
                image-rendering: crisp-edges;
            }}

            body {{{{
                {}
                background: #0a0a0a;
                color: #e8e8e8;
                padding: 180px 40px 60px 40px;
                line-height: 1.6;
                font-size: 13px;
                min-height: 100vh;
            }}

            .settings-container {{
                max-width: 720px;
                margin: 0 auto;
            }}

            h1 {{
                font-size: 32px;
                margin-top: 20px;
                margin-bottom: 12px;
                font-weight: 700;
                letter-spacing: -0.02em;
                color: #ffffff;
                font-family: 'gohu', monospace;
            }}

            .subtitle {{
                color: #888888;
                font-size: 14px;
                margin-bottom: 40px;
                font-weight: 400;
                font-family: 'gohu', monospace;
            }}

            .setting-section {{
                background: #141414;
                border: 1px solid #2a2a2a;
                padding: 24px;
                margin-bottom: 20px;
                transition: border-color 0.2s ease;
            }}

            .setting-section:hover {{
                border-color: #3a3a3a;
            }}

            .setting-section h2 {{
                font-size: 16px;
                margin-bottom: 20px;
                font-weight: 600;
                color: #ffffff;
                letter-spacing: -0.01em;
                font-family: 'gohu', monospace;
            }}

            .setting-item {{
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 16px 0;
                border-bottom: 1px solid #222222;
            }}

            .setting-item:last-child {{
                border-bottom: none;
                padding-bottom: 0;
            }}

            .setting-item:first-child {{
                padding-top: 0;
            }}

            .setting-info {{
                flex: 1;
                max-width: 65%;
            }}

            .setting-label {{
                font-size: 14px;
                margin-bottom: 4px;
                font-weight: 500;
                color: #ffffff;
                font-family: 'gohu', monospace;
            }}

            .setting-description {{
                font-size: 13px;
                color: #888888;
                line-height: 1.4;
                font-family: 'gohu', monospace;
            }}

            .setting-control {{
                margin-left: 16px;
            }}

            input[type="text"],
            input[type="number"] {{
                background: #1a1a1a;
                border: 1px solid #333333;
                color: #e8e8e8;
                padding: 10px 14px;
                font-size: 13px;
                font-family: 'gohu', monospace;
                min-width: 280px;
                transition: all 0.2s ease;
            }}

            input[type="text"]:hover,
            input[type="number"]:hover {{
                border-color: #4a4a4a;
                background: #1f1f1f;
            }}

            select {{
                background: #1a1a1a;
                border: 1px solid #333333;
                color: #e8e8e8;
                padding: 10px 14px;
                font-size: 13px;
                font-family: 'gohu', monospace;
                min-width: 280px;
                appearance: none;
                -webkit-appearance: none;
                -moz-appearance: none;
                cursor: pointer;
                padding-right: 36px;
                background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23888888' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
                background-repeat: no-repeat;
                background-position: right 10px center;
                background-size: 18px;
                transition: all 0.2s ease;
            }}

            select:hover {{
                border-color: #4a4a4a;
                background-color: #1f1f1f;
            }}

            select option {{
                background: #1a1a1a;
                color: #e8e8e8;
                padding: 8px;
            }}

            input[type="text"]:focus,
            input[type="number"]:focus,
            select:focus {{
                outline: none;
                border: 1px solid #666666;
                background: #222222;
                box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.05);
            }}

            input[type="range"] {{
                -webkit-appearance: none;
                appearance: none;
                width: 200px;
                height: 6px;
                background: #2a2a2a;
                outline: none;
                transition: background 0.2s ease;
            }}

            input[type="range"]:hover {{
                background: #333333;
            }}

            input[type="range"]::-webkit-slider-thumb {{
                -webkit-appearance: none;
                appearance: none;
                width: 18px;
                height: 18px;
                background: #ffffff;
                cursor: pointer;
                transition: all 0.2s ease;
            }}

            input[type="range"]::-webkit-slider-thumb:hover {{
                transform: scale(1.1);
                box-shadow: 0 0 0 4px rgba(255, 255, 255, 0.1);
            }}

            input[type="range"]::-moz-range-thumb {{
                width: 18px;
                height: 18px;
                background: #ffffff;
                cursor: pointer;
                border: none;
                transition: all 0.2s ease;
            }}

            input[type="range"]::-moz-range-thumb:hover {{
                transform: scale(1.1);
                box-shadow: 0 0 0 4px rgba(255, 255, 255, 0.1);
            }}

            .slider-value {{
                display: inline-block;
                min-width: 40px;
                margin-left: 12px;
                font-family: 'gohu', monospace;
                font-size: 13px;
                color: #ffffff;
            }}

            input[type="checkbox"] {{
                appearance: none;
                -webkit-appearance: none;
                width: 44px;
                height: 24px;
                border: 2px solid #333333;
                background: #1a1a1a;
                cursor: pointer;
                position: relative;
                transition: all 0.3s ease;
            }}

            input[type="checkbox"]:hover {{
                border-color: #4a4a4a;
                background: #222222;
            }}

            input[type="checkbox"]:checked {{
                background: #ffffff;
                border-color: #ffffff;
            }}

            input[type="checkbox"]::before {{
                content: '';
                position: absolute;
                left: 2px;
                top: 2px;
                width: 16px;
                height: 16px;
                background: #666666;
                transition: all 0.3s ease;
            }}

            input[type="checkbox"]:checked::before {{
                transform: translateX(20px);
                background: #000000;
            }}

            input[type="checkbox"]:checked::after {{
                display: none;
            }}

            button {{
                background: #ffffff;
                color: #000000;
                border: none;
                padding: 12px 24px;
                font-size: 14px;
                font-family: 'gohu', monospace;
                font-weight: 600;
                cursor: pointer;
                transition: all 0.2s ease;
            }}

            button:hover {{
                background: #e8e8e8;
                transform: translateY(-1px);
                box-shadow: 0 4px 12px rgba(255, 255, 255, 0.15);
            }}

            button:active {{
                transform: translateY(0);
                box-shadow: 0 2px 6px rgba(255, 255, 255, 0.1);
            }}

            .save-indicator {{
                display: inline-block;
                margin-left: 16px;
                color: #4ade80;
                font-size: 14px;
                opacity: 0;
                transition: opacity 0.3s ease;
                padding: 12px 20px;
                background: rgba(74, 222, 128, 0.1);
                border: 1px solid rgba(74, 222, 128, 0.3);
            }}

            .save-indicator.show {{
                opacity: 1;
            }}

            .save-section {{{{
                display: flex;
                justify-content: flex-end;
                align-items: center;
                margin-top: 32px;
                padding-top: 24px;
                border-top: 1px solid #222222;
            }}}}
        </style>
    "#,
        fonts::get_gohu_font_face(),
        fonts::get_gohu_font_family()
    );

    let script = r#"
        <script>
            function saveSettings() {
                const settings = {
                    defaultUrl: document.getElementById('default-url').value,
                    searchEngine: document.getElementById('search-engine').value,
                    vimMode: document.getElementById('vim-mode').checked,
                    uiSounds: document.getElementById('ui-sounds').checked,
                    blockTrackers: document.getElementById('block-trackers').checked,
                    blockFingerprinting: document.getElementById('block-fingerprinting').checked,
                    blockCookies: document.getElementById('block-cookies').checked,
                };

                console.log('Saving settings:', settings);

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
                console.log('Loading settings...');
                const message = JSON.stringify({
                    action: 'load_settings'
                });
                window.ipc.postMessage(message);
            }

            window.updateSettings = function(settings) {
                console.log('Received settings:', settings);
                if (settings.defaultUrl) {
                    console.log('Setting default URL to:', settings.defaultUrl);
                    document.getElementById('default-url').value = settings.defaultUrl;
                }
                if (settings.searchEngine) {
                    console.log('Setting search engine to:', settings.searchEngine);
                    document.getElementById('search-engine').value = settings.searchEngine;
                }
                if (settings.vimMode !== undefined) {
                    console.log('Setting vim mode to:', settings.vimMode);
                    document.getElementById('vim-mode').checked = settings.vimMode;
                }
                if (settings.uiSounds !== undefined) {
                    console.log('Setting UI sounds to:', settings.uiSounds);
                    document.getElementById('ui-sounds').checked = settings.uiSounds;
                }
                if (settings.blockTrackers !== undefined) {
                    console.log('Setting block trackers to:', settings.blockTrackers);
                    document.getElementById('block-trackers').checked = settings.blockTrackers;
                }
                if (settings.blockFingerprinting !== undefined) {
                    console.log('Setting block fingerprinting to:', settings.blockFingerprinting);
                    document.getElementById('block-fingerprinting').checked = settings.blockFingerprinting;
                }
                if (settings.blockCookies !== undefined) {
                    console.log('Setting block cookies to:', settings.blockCookies);
                    document.getElementById('block-cookies').checked = settings.blockCookies;
                }
            };

            window.addEventListener('load', loadSettings);

            document.addEventListener('keydown', function(e) {
                if ((e.metaKey || e.ctrlKey) && e.key === 'r') {
                    e.preventDefault();
                    return false;
                }
                if (e.key === 'F5') {
                    e.preventDefault();
                    return false;
                }
            });

            window.addEventListener('beforeunload', function(e) {
                e.preventDefault();
            });
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
                    <div class="setting-description">Default search engine URL (use {{}} as query placeholder)</div>
                </div>
                <div class="setting-control">
                    <input type="text" id="search-engine" placeholder="https://start.duckduckgo.com/?q={{}}">
                </div>
            </div>
        </div>

        <div class="setting-section">
            <h2>Appearance</h2>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Vim Mode</div>
                    <div class="setting-description">Enable vim-style keyboard navigation (j/k/h/l/gg/G)</div>
                </div>
                <div class="setting-control">
                    <input type="checkbox" id="vim-mode" checked>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">UI Sounds</div>
                    <div class="setting-description">Play sounds on button clicks and keyboard shortcuts</div>
                </div>
                <div class="setting-control">
                    <input type="checkbox" id="ui-sounds" checked>
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
