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

            html {{
                background: #101010 !important;
            }}

            body {{{{
                {}
                background: #101010 !important;
                color: #e8e8e8;
                padding: 180px 40px 60px 40px;
                line-height: 1.6;
                font-size: 13px;
                min-height: 100vh;
            }}

            .settings-container {{
                max-width: 720px;
                margin: 0 auto;
                margin-bottom: 120px;
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
                position: fixed;
                top: 24px;
                right: 24px;
                color: #4ade80;
                font-size: 14px;
                font-family: 'gohu', monospace;
                opacity: 0;
                padding: 12px 20px;
                background: rgba(74, 222, 128, 0.15);
                border: 1px solid rgba(74, 222, 128, 0.4);
                z-index: 10000;
                pointer-events: none;
                transform: translateY(-10px);
                transition: opacity 0.2s ease, transform 0.2s ease;
                box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            }}

            .save-indicator.show {{
                opacity: 1;
                transform: translateY(0);
            }}

            .save-indicator.hide {{
                opacity: 0;
                transform: translateY(-10px);
            }}

            .save-section {{{{
                display: flex;
                justify-content: flex-end;
                align-items: center;
                margin-top: 32px;
                padding-top: 24px;
                border-top: 1px solid #222222;
            }}}}

            .shortcut-input {{
                background: #1a1a1a;
                border: 1px solid #333333;
                color: #e8e8e8;
                padding: 10px 14px;
                font-size: 13px;
                font-family: 'gohu', monospace;
                min-width: 180px;
                cursor: text;
                transition: all 0.2s ease;
                position: relative;
            }}

            .shortcut-input::placeholder {{
                color: #666666;
                opacity: 1;
            }}

            .shortcut-input:hover {{
                border-color: #4a4a4a;
                background: #1f1f1f;
            }}

            .shortcut-input.recording {{
                border-color: #ffffff;
                background: #222222;
                animation: pulse 1.5s ease-in-out infinite;
            }}

            .shortcut-input.conflict {{
                border-color: #ff6b6b;
                background: rgba(255, 107, 107, 0.1);
            }}

            @keyframes pulse {{
                0%, 100% {{ box-shadow: 0 0 0 0 rgba(255, 255, 255, 0.4); }}
                50% {{ box-shadow: 0 0 0 4px rgba(255, 255, 255, 0.1); }}
            }}

            .conflict-warning {{
                display: none;
                color: #ff6b6b;
                font-size: 12px;
                margin-top: 4px;
                font-family: 'gohu', monospace;
            }}

            .conflict-warning.show {{
                display: block;
            }}

            .update-button {{
                background: #ffffff;
                color: #000000;
                border: none;
                padding: 12px 24px;
                font-size: 14px;
                font-family: 'gohu', monospace;
                cursor: pointer;
                transition: all 0.2s ease;
                width: 100%;
                margin-bottom: 12px;
            }}

            .update-button:hover {{
                background: #e8e8e8;
                transform: translateY(-1px);
                box-shadow: 0 4px 12px rgba(255, 255, 255, 0.15);
            }}

            .update-button:disabled {{
                background: #333333;
                color: #666666;
                cursor: not-allowed;
                transform: none;
            }}

            .update-status {{
                font-size: 13px;
                color: #888888;
                font-family: 'gohu', monospace;
                margin-top: 12px;
                line-height: 1.6;
            }}

            .update-status.checking {{
                color: #ffffff;
            }}

            .update-status.available {{
                color: #4ade80;
            }}

            .update-status.error {{
                color: #ff6b6b;
            }}

            .update-progress {{
                width: 100%;
                height: 6px;
                background: #2a2a2a;
                margin-top: 12px;
                overflow: hidden;
                display: none;
            }}

            .update-progress.show {{
                display: block;
            }}

            .update-progress-bar {{
                height: 100%;
                background: #ffffff;
                width: 0%;
                transition: width 0.3s ease;
            }}

            .release-notes {{
                background: #1a1a1a;
                border: 1px solid #333333;
                padding: 16px;
                margin-top: 12px;
                font-size: 12px;
                line-height: 1.8;
                max-height: 200px;
                overflow-y: auto;
                display: none;
            }}

            .release-notes.show {{
                display: block;
            }}

            .version-info {{
                font-size: 13px;
                color: #666666;
                font-family: 'gohu', monospace;
            }}
        </style>
    "#,
        fonts::get_gohu_font_face(),
        fonts::get_gohu_font_family()
    );

    let script = r#"
        <script>
            const shortcuts = {
                'new_tab': '',
                'close_tab': '',
                'reload': '',
                'focus_url': '',
                'toggle_downloads': '',
                'focus_sidebar': '',
                'find': '',
                'new_window': '',
                'toggle_split_view': ''
            };

            let recordingKey = null;

            function formatShortcut(key) {
                const parts = [];
                if (key.includes('Cmd') || key.includes('Meta')) parts.push('Cmd');
                if (key.includes('Ctrl')) parts.push('Ctrl');
                if (key.includes('Alt') || key.includes('Option')) parts.push('Alt');
                if (key.includes('Shift')) parts.push('Shift');

                const letter = key.split('+').pop();
                if (letter && !['Cmd', 'Ctrl', 'Alt', 'Shift', 'Meta', 'Option'].includes(letter)) {
                    parts.push(letter.toUpperCase());
                }

                return parts.join('+');
            }

            function detectConflicts() {
                const values = Object.values(shortcuts).filter(v => v);
                const duplicates = values.filter((v, i) => values.indexOf(v) !== i);

                for (const [key, value] of Object.entries(shortcuts)) {
                    const input = document.getElementById(`shortcut-${key}`);
                    const warning = document.getElementById(`conflict-${key}`);

                    if (duplicates.includes(value) && value) {
                        input.classList.add('conflict');
                        warning.classList.add('show');
                        const otherKey = Object.keys(shortcuts).find(k => k !== key && shortcuts[k] === value);
                        if (otherKey) {
                            warning.textContent = `Conflict with ${otherKey.replace(/_/g, ' ')}`;
                        }
                    } else {
                        input.classList.remove('conflict');
                        warning.classList.remove('show');
                    }
                }
            }

            function handleGlobalKeydown(e) {
                if (!recordingKey) return;
                
                // Prevent default browser actions for ALL keys while recording
                e.preventDefault();
                e.stopPropagation();
                e.stopImmediatePropagation();

                recordShortcut(e, recordingKey);
                return false;
            }

            function startRecording(key) {
                if (recordingKey && recordingKey !== key) {
                    stopRecording(recordingKey);
                }

                recordingKey = key;
                const input = document.getElementById(`shortcut-${key}`);
                input.classList.add('recording');
                input.value = 'Press keys...';
                input.focus();
                
                // Add capturing listener to intercept everything
                window.addEventListener('keydown', handleGlobalKeydown, true);
            }

            function stopRecording(key) {
                const input = document.getElementById(`shortcut-${key}`);
                if (input) {
                    input.classList.remove('recording');
                    input.value = shortcuts[key] || '';
                }
                recordingKey = null;
                window.removeEventListener('keydown', handleGlobalKeydown, true);
            }

            function recordShortcut(e, key) {
                const modifiers = [];
                if (e.metaKey || e.key === 'Meta') modifiers.push('Cmd');
                if (e.ctrlKey || e.key === 'Control') modifiers.push('Ctrl');
                if (e.altKey || e.key === 'Alt') modifiers.push('Alt');
                if (e.shiftKey || e.key === 'Shift') modifiers.push('Shift');

                // Don't record if only modifiers are pressed
                if (['Meta', 'Control', 'Alt', 'Shift'].includes(e.key)) {
                    return;
                }

                let keyName = e.key;
                if (keyName.length === 1) {
                    keyName = keyName.toUpperCase();
                } else if (keyName === ' ') {
                    keyName = 'Space';
                }

                const shortcut = modifiers.length > 0
                    ? `${modifiers.join('+')}+${keyName}`
                    : keyName;

                shortcuts[key] = shortcut;
                const input = document.getElementById(`shortcut-${key}`);
                input.value = shortcut;
                
                // We must remove listener before calling stopRecording to avoid recursion if logic changes
                window.removeEventListener('keydown', handleGlobalKeydown, true);
                
                stopRecording(key);
                detectConflicts();
                
                // Trigger auto-save
                debouncedSaveSettings();
            }

            function saveSettings() {
                const hasConflicts = document.querySelectorAll('.shortcut-input.conflict').length > 0;
                if (hasConflicts) {
                    alert('Please resolve shortcut conflicts before saving.');
                    return;
                }

                const settings = {
                    defaultUrl: document.getElementById('default-url').value,
                    searchEngine: document.getElementById('search-engine').value,
                    youtubeRedirect: document.getElementById('youtube-redirect').checked,
                    invidiousInstance: document.getElementById('invidious-instance').value,
                    vimMode: document.getElementById('vim-mode').checked,
                    uiSounds: document.getElementById('ui-sounds').checked,
                    blockTrackers: document.getElementById('block-trackers').checked,
                    blockFingerprinting: document.getElementById('block-fingerprinting').checked,
                    blockCookies: document.getElementById('block-cookies').checked,
                    shortcuts: shortcuts
                };

                console.log('Saving settings:', settings);

                const message = JSON.stringify({
                    action: 'save_settings',
                    settings: settings
                });

                window.ipc.postMessage(message);

                const indicator = document.getElementById('save-indicator');
                indicator.textContent = '✓ Settings Saved';
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

            function clearHistory() {
                const button = document.getElementById('clear-history-btn');
                button.textContent = 'Clearing...';
                button.disabled = true;

                const message = JSON.stringify({
                    action: 'clear_history'
                });
                window.ipc.postMessage(message);

                setTimeout(() => {
                    button.textContent = 'Cleared!';
                    setTimeout(() => {
                        button.textContent = 'Clear History';
                        button.disabled = false;
                    }, 1500);
                }, 300);
            }

            function checkForUpdates() {
                const button = document.getElementById('check-update-btn');
                const status = document.getElementById('update-status');
                const progressBar = document.getElementById('update-progress');
                const releaseNotes = document.getElementById('release-notes');

                button.disabled = true;
                button.textContent = 'Checking...';
                status.className = 'update-status checking';
                status.textContent = 'Checking for updates...';
                progressBar.classList.remove('show');
                releaseNotes.classList.remove('show');

                const message = JSON.stringify({
                    action: 'check_for_updates'
                });

                window.ipc.postMessage(message);
            }

            function installUpdate() {
                const button = document.getElementById('install-update-btn');
                const progressBar = document.getElementById('update-progress');
                const progressFill = document.getElementById('update-progress-bar');

                button.disabled = true;
                button.textContent = 'Installing...';
                progressBar.classList.add('show');
                progressFill.style.width = '0%';

                const message = JSON.stringify({
                    action: 'install_update'
                });

                window.ipc.postMessage(message);
            }

            window.updateAvailable = function(info) {
                const button = document.getElementById('check-update-btn');
                const installBtn = document.getElementById('install-update-btn');
                const status = document.getElementById('update-status');
                const releaseNotes = document.getElementById('release-notes');

                button.disabled = false;
                button.textContent = 'Check for Updates';

                status.className = 'update-status available';
                status.innerHTML = `New version available: ${info.version}<br>Released: ${new Date(info.published_at).toLocaleDateString()}`;

                if (info.release_notes) {
                    releaseNotes.innerHTML = `<strong>Release Notes:</strong><br>${info.release_notes.replace(/\n/g, '<br>')}`;
                    releaseNotes.classList.add('show');
                }

                installBtn.style.display = 'block';
            };

            window.updateProgress = function(downloaded, total) {
                const progressFill = document.getElementById('update-progress-bar');
                const percentage = Math.round((downloaded / total) * 100);
                progressFill.style.width = `${percentage}%`;
            };

            window.updateInstalled = function() {
                const button = document.getElementById('install-update-btn');
                const status = document.getElementById('update-status');
                const progressBar = document.getElementById('update-progress');

                button.disabled = false;
                button.textContent = 'Restart to Apply';
                progressBar.classList.remove('show');

                status.className = 'update-status available';
                status.textContent = 'Update installed successfully! Restart Calm to apply.';
            };

            window.updateError = function(error) {
                const button = document.getElementById('check-update-btn');
                const status = document.getElementById('update-status');

                button.disabled = false;
                button.textContent = 'Check for Updates';

                status.className = 'update-status error';
                status.textContent = `Error: ${error}`;
            };

            window.noUpdateAvailable = function() {
                const button = document.getElementById('check-update-btn');
                const status = document.getElementById('update-status');

                button.disabled = false;
                button.textContent = 'Check for Updates';

                status.className = 'update-status';
                status.textContent = 'You are running the latest version.';
            };

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
                if (settings.youtubeRedirect !== undefined) {
                    console.log('Setting YouTube redirect to:', settings.youtubeRedirect);
                    document.getElementById('youtube-redirect').checked = settings.youtubeRedirect;
                }
                if (settings.invidiousInstance) {
                    console.log('Setting Invidious instance to:', settings.invidiousInstance);
                    document.getElementById('invidious-instance').value = settings.invidiousInstance;
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
                if (settings.shortcuts) {
                    for (const [key, value] of Object.entries(settings.shortcuts)) {
                        shortcuts[key] = value;
                        const input = document.getElementById(`shortcut-${key}`);
                        if (input) {
                            input.value = value || '';
                        }
                    }
                    detectConflicts();
                }
            };

            let saveTimeout = null;
            function debouncedSaveSettings() {
                if (saveTimeout) clearTimeout(saveTimeout);
                saveTimeout = setTimeout(saveSettings, 1000);
            }

            // Auto-save listeners
            window.addEventListener('load', function() {
                loadSettings();

                // Attach listeners to all inputs
                const textInputs = ['default-url', 'search-engine', 'invidious-instance'];
                textInputs.forEach(id => {
                    const el = document.getElementById(id);
                    if (el) el.addEventListener('input', debouncedSaveSettings);
                });

                const checkboxes = ['youtube-redirect', 'vim-mode', 'ui-sounds', 'block-trackers', 'block-fingerprinting', 'block-cookies'];
                checkboxes.forEach(id => {
                    const el = document.getElementById(id);
                    if (el) el.addEventListener('change', saveSettings);
                });
            });

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
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Browsing History</div>
                    <div class="setting-description">Clear all browsing history from command prompt suggestions</div>
                </div>
                <div class="setting-control">
                    <button id="clear-history-btn" onclick="clearHistory()">Clear History</button>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">YouTube to Invidious</div>
                    <div class="setting-description">Automatically redirect YouTube links to privacy-friendly Invidious</div>
                </div>
                <div class="setting-control">
                    <input type="checkbox" id="youtube-redirect">
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Invidious Instance</div>
                    <div class="setting-description">Custom Invidious instance domain (e.g., yewtu.be)</div>
                </div>
                <div class="setting-control">
                    <input type="text" id="invidious-instance" placeholder="yewtu.be">
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

        <div class="setting-section">
            <h2>Keyboard Shortcuts</h2>
            <p style="color: #888; font-size: 12px; margin-bottom: 16px; font-family: 'gohu', monospace;">Click on a shortcut to record new keys. Conflicts are highlighted in red.</p>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">New Tab</div>
                    <div class="setting-description">Open a new tab</div>
                </div>
                <div class="setting-control">
                    <input type="text" class="shortcut-input" id="shortcut-new_tab" readonly
                           onclick="startRecording('new_tab')"
                           onkeydown="recordShortcut(event, 'new_tab')"
                           onblur="stopRecording('new_tab')"
                           placeholder="Cmd+T">
                    <div class="conflict-warning" id="conflict-new_tab"></div>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Close Tab</div>
                    <div class="setting-description">Close the current tab</div>
                </div>
                <div class="setting-control">
                    <input type="text" class="shortcut-input" id="shortcut-close_tab" readonly
                           onclick="startRecording('close_tab')"
                           onkeydown="recordShortcut(event, 'close_tab')"
                           onblur="stopRecording('close_tab')"
                           placeholder="Cmd+W">
                    <div class="conflict-warning" id="conflict-close_tab"></div>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Reload Page</div>
                    <div class="setting-description">Reload the current page</div>
                </div>
                <div class="setting-control">
                    <input type="text" class="shortcut-input" id="shortcut-reload" readonly
                           onclick="startRecording('reload')"
                           onkeydown="recordShortcut(event, 'reload')"
                           onblur="stopRecording('reload')"
                           placeholder="Cmd+R">
                    <div class="conflict-warning" id="conflict-reload"></div>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Focus URL Bar</div>
                    <div class="setting-description">Focus the URL bar to navigate</div>
                </div>
                <div class="setting-control">
                    <input type="text" class="shortcut-input" id="shortcut-focus_url" readonly
                           onclick="startRecording('focus_url')"
                           onkeydown="recordShortcut(event, 'focus_url')"
                           onblur="stopRecording('focus_url')"
                           placeholder="Cmd+L">
                    <div class="conflict-warning" id="conflict-focus_url"></div>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Toggle Downloads</div>
                    <div class="setting-description">Show/hide downloads sidebar</div>
                </div>
                <div class="setting-control">
                    <input type="text" class="shortcut-input" id="shortcut-toggle_downloads" readonly
                           onclick="startRecording('toggle_downloads')"
                           onkeydown="recordShortcut(event, 'toggle_downloads')"
                           onblur="stopRecording('toggle_downloads')"
                           placeholder="Cmd+J">
                    <div class="conflict-warning" id="conflict-toggle_downloads"></div>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Focus Sidebar</div>
                    <div class="setting-description">Focus the sidebar tabs</div>
                </div>
                <div class="setting-control">
                    <input type="text" class="shortcut-input" id="shortcut-focus_sidebar" readonly
                           onclick="startRecording('focus_sidebar')"
                           onkeydown="recordShortcut(event, 'focus_sidebar')"
                           onblur="stopRecording('focus_sidebar')"
                           placeholder="Cmd+E">
                    <div class="conflict-warning" id="conflict-focus_sidebar"></div>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Find in Page</div>
                    <div class="setting-description">Open find in page</div>
                </div>
                <div class="setting-control">
                    <input type="text" class="shortcut-input" id="shortcut-find" readonly
                           onclick="startRecording('find')"
                           onkeydown="recordShortcut(event, 'find')"
                           onblur="stopRecording('find')"
                           placeholder="Cmd+F">
                    <div class="conflict-warning" id="conflict-find"></div>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">New Window</div>
                    <div class="setting-description">Open a new browser window</div>
                </div>
                <div class="setting-control">
                    <input type="text" class="shortcut-input" id="shortcut-new_window" readonly
                           onclick="startRecording('new_window')"
                           onkeydown="recordShortcut(event, 'new_window')"
                           onblur="stopRecording('new_window')"
                           placeholder="Cmd+N">
                    <div class="conflict-warning" id="conflict-new_window"></div>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <div class="setting-label">Toggle Split View</div>
                    <div class="setting-description">Enable/disable split view mode</div>
                </div>
                <div class="setting-control">
                    <input type="text" class="shortcut-input" id="shortcut-toggle_split_view" readonly
                           onclick="startRecording('toggle_split_view')"
                           onkeydown="recordShortcut(event, 'toggle_split_view')"
                           onblur="stopRecording('toggle_split_view')"
                           placeholder="Cmd+Shift+S">
                    <div class="conflict-warning" id="conflict-toggle_split_view"></div>
                </div>
            </div>
        </div>

        <div class="setting-section">
            <h2>Updates</h2>
            <div class="setting-item" style="flex-direction: column; align-items: stretch;">
                <div style="margin-bottom: 16px;">
                    <div class="version-info">Current version: 0.1.0</div>
                </div>
                <button class="update-button" id="check-update-btn" onclick="checkForUpdates()">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges" style="display: inline-block; vertical-align: middle; margin-right: 8px;">
                        <path d="M16 2h-2v2h2v2H4v2H2v5h2V8h12v2h-2v2h2v-2h2V8h2V6h-2V4h-2V2zM6 20h2v2h2v-2H8v-2h12v-2h2v-5h-2v5H8v-2h2v-2H8v2H6v2H4v2h2v2z"/>
                    </svg>
                    Check for Updates
                </button>
                <button class="update-button" id="install-update-btn" onclick="installUpdate()" style="display: none;">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges" style="display: inline-block; vertical-align: middle; margin-right: 8px;">
                        <path d="M12 2v12h2v-2h2v-2h2v-2h2v8h-2v2h-2v2H6v-2H4v-2H2v-8h2v2h2v2h2v2h2V2h2zm-2 18h4v-2h-4v2z"/>
                    </svg>
                    Install Update
                </button>
                <div class="update-progress" id="update-progress">
                    <div class="update-progress-bar" id="update-progress-bar"></div>
                </div>
                <div class="update-status" id="update-status">
                    Click "Check for Updates" to see if a new version is available.
                </div>
                <div class="release-notes" id="release-notes"></div>
            </div>
        </div>

        <div class="save-section">
            <span class="save-indicator" id="save-indicator">Saved</span>
        </div>
    </div>
    {}
</body>
</html>"#,
        styles, script
    )
}
