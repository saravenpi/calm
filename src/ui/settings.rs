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
                background: #101010;
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
                letter-spacing: -0.02em;
                color: #ffffff;
                font-family: 'gohu', monospace;
            }}

            .subtitle {{
                color: #888888;
                font-size: 14px;
                margin-bottom: 40px;
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
                font-family: 'gohu', monospace;
                opacity: 0;
                padding: 12px 20px;
                background: rgba(74, 222, 128, 0.1);
                border: 1px solid rgba(74, 222, 128, 0.3);
                transform: translateX(-20px);
                transition: opacity 0.3s ease, transform 0.3s ease;
            }}

            .save-indicator.show {{
                opacity: 1;
                transform: translateX(0);
            }}

            .save-indicator.hide {{
                opacity: 0;
                transform: translateX(-20px);
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

                const indicator = document.getElementById('save-indicator');
                indicator.classList.remove('hide');
                indicator.classList.add('show');
                setTimeout(() => {
                    indicator.classList.remove('show');
                    indicator.classList.add('hide');
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

            let isTabActive = true;
            let faviconTransitionTimeout = null;

            function interpolateColor(from, to, steps, currentStep) {
                const fromVal = parseInt(from, 16);
                const toVal = parseInt(to, 16);
                const step = (toVal - fromVal) / steps;
                const val = Math.round(fromVal + (step * currentStep));
                return val.toString(16).padStart(2, '0');
            }

            function setFaviconColor(color) {
                const link = document.querySelector('link[rel="icon"]');
                if (link) {
                    link.remove();
                }
                const newLink = document.createElement('link');
                newLink.rel = 'icon';
                newLink.type = 'image/svg+xml';
                newLink.href = `data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='%23${color}'%3E%3Cpath d='M17 4h2v10h-2V4zm0 12h-2v2h2v2h2v-2h2v-2h-4zm-4-6h-2v10h2V10zm-8 2H3v2h2v6h2v-6h2v-2H5zm8-8h-2v2H9v2h6V6h-2V4zM5 4h2v6H5V4z'/%3E%3C/svg%3E`;
                document.head.appendChild(newLink);

                // Notify tab bar to update display
                if (window.ipc) {
                    window.ipc.postMessage(JSON.stringify({
                        action: 'update_favicon',
                        favicon: newLink.href
                    }));
                }
            }

            function updateFavicon(active) {
                if (faviconTransitionTimeout) {
                    clearTimeout(faviconTransitionTimeout);
                }

                isTabActive = active;
                const fromColor = active ? 'ffffff' : '000000';
                const toColor = active ? '000000' : 'ffffff';
                const steps = 4;
                let currentStep = 0;

                function animateStep() {
                    if (currentStep <= steps) {
                        const r = interpolateColor(fromColor.substr(0, 2), toColor.substr(0, 2), steps, currentStep);
                        const g = interpolateColor(fromColor.substr(2, 2), toColor.substr(2, 2), steps, currentStep);
                        const b = interpolateColor(fromColor.substr(4, 2), toColor.substr(4, 2), steps, currentStep);
                        const color = r + g + b;

                        setFaviconColor(color);
                        currentStep++;

                        if (currentStep <= steps) {
                            faviconTransitionTimeout = setTimeout(animateStep, 30);
                        }
                    }
                }

                animateStep();
            }

            window.onTabActive = function() {
                updateFavicon(true);
            };

            window.onTabInactive = function() {
                updateFavicon(false);
            };

            window.addEventListener('load', function() {
                updateFavicon(true);
            });

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
    <link rel="icon" type="image/svg+xml" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='%23ffffff'%3E%3Cpath d='M17 4h2v10h-2V4zm0 12h-2v2h2v2h2v-2h2v-2h-4zm-4-6h-2v10h2V10zm-8 2H3v2h2v6h2v-6h2v-2H5zm8-8h-2v2H9v2h6V6h-2V4zM5 4h2v6H5V4z'/%3E%3C/svg%3E">
    {}
</head>
<body>
    <div class="settings-container">
        <h1 style="display: flex; align-items: center;">
            <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges" style="margin-right: 12px;">
                <path d="M17 4h2v10h-2V4zm0 12h-2v2h2v2h2v-2h2v-2h-4zm-4-6h-2v10h2V10zm-8 2H3v2h2v6h2v-6h2v-2H5zm8-8h-2v2H9v2h6V6h-2V4zM5 4h2v6H5V4z"/>
            </svg>
            Settings
        </h1>
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
            <button onclick="saveSettings()">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges" style="display: inline-block; vertical-align: middle; margin-right: 8px;">
                    <path d="M4 2h14v2H4v16h2v-6h12v6h2V6h2v16H2V2h2zm4 18h8v-4H8v4zM20 6h-2V4h2v2zM6 6h9v4H6V6z"/>
                </svg>
                Save Settings
            </button>
            <span class="save-indicator" id="save-indicator">Saved</span>
        </div>
    </div>
    {}
</body>
</html>"#,
        styles, script
    )
}
