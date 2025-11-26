pub fn get_tab_bar_script() -> &'static str {
    r#"
        window.tabs = [];
        window.currentUrl = '';
        window.tabAudioState = {};

        window.handleNewTab = function() {
            window.ipc.postMessage(JSON.stringify({action: 'new_tab'}));
        };

        window.updateUrlBar = function(url) {
            const urlBar = document.getElementById('url-bar');
            const reloadBtn = document.getElementById('reload-btn');
            if (urlBar) {
                if (url === 'about:blank' || url === '') {
                    urlBar.value = '';
                } else {
                    const displayUrl = url.replace(/^calmfile:\/\/localhost/, 'file://');
                    urlBar.value = displayUrl;
                }
                window.currentUrl = url;
            }

            if (reloadBtn) {
                const isCalmPage = url.startsWith('calm://');
                reloadBtn.disabled = isCalmPage;
            }
        };

        window.updateNavigationButtons = function(canGoBack, canGoForward) {
            const backBtn = document.getElementById('back-btn');
            const forwardBtn = document.getElementById('forward-btn');
            if (backBtn) backBtn.disabled = !canGoBack;
            if (forwardBtn) forwardBtn.disabled = !canGoForward;
        };

        const urlBar = document.getElementById('url-bar');
        if (urlBar) {
            urlBar.addEventListener('keydown', (e) => {
                if (e.key === 'Enter') {
                    e.preventDefault();
                    const url = urlBar.value.trim();
                    if (url) {
                        window.ipc.postMessage(JSON.stringify({action: 'navigate_url', url: url}));
                    }
                }
            });

            urlBar.addEventListener('focus', () => {
                urlBar.select();
            });
        }

        window.addTab = function(tabId, url) {
            const tab = {
                id: tabId,
                url: url,
                title: getDisplayTitle(url)
            };
            window.tabs.push(tab);
            window.tabAudioState[tabId] = false;

            const tabEl = document.createElement('div');
            tabEl.className = 'tab opening';
            tabEl.dataset.tabId = tabId;

            const titleSpan = document.createElement('span');
            titleSpan.className = 'tab-title';
            titleSpan.textContent = tab.title;

            const audioIndicator = document.createElement('span');
            audioIndicator.className = 'tab-audio-indicator';
            audioIndicator.id = `audio-indicator-${tabId}`;
            audioIndicator.textContent = '🔊';

            const closeBtn = document.createElement('span');
            closeBtn.className = 'tab-close';
            closeBtn.textContent = '×';
            closeBtn.onclick = (event) => {
                event.stopPropagation();
                closeTabWithAnimation(tabId);
            };

            tabEl.appendChild(titleSpan);
            tabEl.appendChild(audioIndicator);
            tabEl.appendChild(closeBtn);

            tabEl.onclick = () => window.ipc.postMessage(JSON.stringify({action: 'switch_tab', tabId: tabId}));

            const tabBar = document.getElementById('tab-bar');
            if (tabBar) {
                tabBar.appendChild(tabEl);
            }

            setTimeout(() => {
                tabEl.classList.remove('opening');
            }, 300);
        };

        window.closeTabWithAnimation = function(tabId) {
            const tabEl = document.querySelector(`.tab[data-tab-id="${tabId}"]`);
            if (tabEl) {
                tabEl.classList.add('closing');
                setTimeout(() => {
                    window.ipc.postMessage(JSON.stringify({action: 'close_tab', tabId: tabId}));
                }, 300);
            }
        };

        window.removeTab = function(tabId) {
            const tabEl = document.querySelector(`.tab[data-tab-id="${tabId}"]`);
            if (tabEl) {
                tabEl.remove();
            }
            window.tabs = window.tabs.filter(t => t.id !== tabId);
            delete window.tabAudioState[tabId];
        };

        window.setActiveTab = function(tabId) {
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            const tabEl = document.querySelector(`.tab[data-tab-id="${tabId}"]`);
            if (tabEl) {
                tabEl.classList.add('active');
            }
        };

        window.updateTabTitle = function(tabId, title) {
            const tab = window.tabs.find(t => t.id === tabId);
            if (tab) {
                tab.title = title;
                const tabEl = document.querySelector(`.tab[data-tab-id="${tabId}"] .tab-title`);
                if (tabEl) {
                    tabEl.textContent = title || 'New Tab';
                }
            }
        };

        window.updateTabAudioState = function(tabId, isPlaying) {
            window.tabAudioState[tabId] = isPlaying;
            const indicator = document.getElementById(`audio-indicator-${tabId}`);
            if (indicator) {
                if (isPlaying) {
                    indicator.classList.add('playing');
                } else {
                    indicator.classList.remove('playing');
                }
            }
        };

        function getDisplayTitle(url) {
            if (url === 'about:blank' || url === '') {
                return 'New Tab';
            }
            try {
                const urlObj = new URL(url);
                return urlObj.hostname || 'New Tab';
            } catch {
                return 'New Tab';
            }
        }

        window.toggleDownloads = function() {
            window.ipc.postMessage(JSON.stringify({action: 'toggle_downloads'}));
        };

        window.openSettings = function() {
            window.ipc.postMessage(JSON.stringify({action: 'open_settings'}));
        };

        window.updateDownloadCount = function(count) {
            const badge = document.getElementById('download-badge');
            const btn = document.getElementById('downloads-btn');
            if (badge && btn) {
                if (count > 0) {
                    badge.textContent = count;
                    badge.style.display = 'flex';
                    btn.classList.add('has-downloads');
                    btn.classList.add('pulse');
                    setTimeout(() => btn.classList.remove('pulse'), 600);
                } else {
                    badge.style.display = 'none';
                    btn.classList.remove('has-downloads');
                }
            }
        };

        window.getActiveTabId = function() {
            const activeTab = document.querySelector('.tab.active');
            if (activeTab) {
                return parseInt(activeTab.dataset.tabId);
            }
            return null;
        };

        let lastShortcutEvent = { key: '', time: 0 };
        const KEY_DEBOUNCE_MS = 300;

        document.addEventListener('keydown', (e) => {
            if ((e.metaKey || e.ctrlKey) && e.key === 't') {
                const now = Date.now();
                const eventKey = 'meta+t';

                if (lastShortcutEvent.key === eventKey && (now - lastShortcutEvent.time) < KEY_DEBOUNCE_MS) {
                    e.preventDefault();
                    e.stopImmediatePropagation();
                    return false;
                }

                lastShortcutEvent = { key: eventKey, time: now };
                e.preventDefault();
                e.stopImmediatePropagation();
                window.ipc.postMessage(JSON.stringify({action: 'new_tab'}));
                return false;
            }

            if ((e.metaKey || e.ctrlKey) && e.key === 'w') {
                const now = Date.now();
                const eventKey = 'meta+w';

                if (lastShortcutEvent.key === eventKey && (now - lastShortcutEvent.time) < KEY_DEBOUNCE_MS) {
                    e.preventDefault();
                    e.stopImmediatePropagation();
                    return false;
                }

                lastShortcutEvent = { key: eventKey, time: now };
                e.preventDefault();
                e.stopImmediatePropagation();
                const activeTab = document.querySelector('.tab.active');
                if (activeTab) {
                    const tabId = parseInt(activeTab.dataset.tabId);
                    window.ipc.postMessage(JSON.stringify({action: 'close_tab', tabId: tabId}));
                }
                return false;
            }

            if (e.key === 'Escape') {
                const urlBar = document.getElementById('url-bar');
                if (urlBar && document.activeElement === urlBar) {
                    urlBar.blur();
                }
            }
        }, true);

        document.addEventListener('mouseup', (e) => {
            if (e.button === 3) {
                e.preventDefault();
                window.ipc.postMessage(JSON.stringify({action: 'navigate_back'}));
            } else if (e.button === 4) {
                e.preventDefault();
                window.ipc.postMessage(JSON.stringify({action: 'navigate_forward'}));
            }
        }, true);
    "#
}
