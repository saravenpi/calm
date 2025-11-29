pub fn get_tab_bar_script() -> &'static str {
    r#"
        window.tabs = [];
        window.currentUrl = '';
        window.tabAudioState = {};
        window.focusedTabIndex = -1;
        window.lastGKeyTime = 0;
        window.sidebarFocused = false;

        window.showSidebarFocus = function() {
            const tabBar = document.getElementById('tab-bar');
            if (tabBar) {
                tabBar.classList.add('sidebar-focused');
                window.sidebarFocused = true;
            }
        };

        window.hideSidebarFocus = function() {
            const tabBar = document.getElementById('tab-bar');
            if (tabBar) {
                tabBar.classList.remove('sidebar-focused');
                window.sidebarFocused = false;
            }
        };

        window.handleNewTab = function() {
            window.ipc.postMessage(JSON.stringify({action: 'new_tab'}));
        };

        window.updateUrlBar = function(url) {
            const urlBar = document.getElementById('url-bar');
            const reloadBtn = document.getElementById('reload-btn');
            const settingsBtn = document.getElementById('settings-btn');

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

            if (settingsBtn) {
                const isSettingsPage = url === 'calm://settings';
                settingsBtn.disabled = isSettingsPage;
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
                playUISound('delete');
                closeTabWithAnimation(tabId);
            };

            tabEl.appendChild(titleSpan);
            tabEl.appendChild(audioIndicator);
            tabEl.appendChild(closeBtn);

            tabEl.onclick = () => {
                playUISound('cursorMove');
                window.ipc.postMessage(JSON.stringify({action: 'switch_tab', tabId: tabId}));
                window.hideSidebarFocus();
            };

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
                const tabIndex = window.tabs.findIndex(t => t.id === tabId);
                if (tabIndex >= 0) {
                    window.focusedTabIndex = tabIndex;
                    window.updateFocusedTab(tabIndex);
                }
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

        window.updateFocusedTab = function(index) {
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('focused'));
            if (index >= 0 && index < window.tabs.length) {
                window.focusedTabIndex = index;
                const tabEl = document.querySelector(`.tab[data-tab-id="${window.tabs[index].id}"]`);
                if (tabEl) {
                    tabEl.classList.add('focused');
                    tabEl.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
                }
            }
        };

        window.focusTab = function(index) {
            if (index >= 0 && index < window.tabs.length) {
                const tabId = window.tabs[index].id;
                window.ipc.postMessage(JSON.stringify({
                    action: 'switch_tab',
                    tabId: tabId
                }));
                window.focusedTabIndex = -1;
                document.querySelectorAll('.tab').forEach(t => t.classList.remove('focused'));
            }
        };

        window.moveFocusDown = function() {
            if (window.tabs.length === 0) return;
            const newIndex = Math.min(window.focusedTabIndex + 1, window.tabs.length - 1);
            window.updateFocusedTab(newIndex);
        };

        window.moveFocusUp = function() {
            if (window.tabs.length === 0) return;
            const newIndex = Math.max(window.focusedTabIndex - 1, 0);
            window.updateFocusedTab(newIndex);
        };

        window.activateFocusedTab = function() {
            if (window.focusedTabIndex >= 0 && window.focusedTabIndex < window.tabs.length) {
                const tabId = window.tabs[window.focusedTabIndex].id;
                window.ipc.postMessage(JSON.stringify({action: 'switch_tab', tabId: tabId}));
                window.hideSidebarFocus();
            }
        };

        window.closeFocusedTab = function() {
            if (window.focusedTabIndex >= 0 && window.focusedTabIndex < window.tabs.length) {
                const tabId = window.tabs[window.focusedTabIndex].id;
                closeTabWithAnimation(tabId);
                if (window.focusedTabIndex >= window.tabs.length - 1) {
                    window.focusedTabIndex = Math.max(0, window.tabs.length - 2);
                }
                setTimeout(() => {
                    window.updateFocusedTab(window.focusedTabIndex);
                }, 50);
            }
        };

        window.jumpToFirstTab = function() {
            if (window.tabs.length > 0) {
                window.updateFocusedTab(0);
            }
        };

        window.jumpToLastTab = function() {
            if (window.tabs.length > 0) {
                window.updateFocusedTab(window.tabs.length - 1);
            }
        };

        document.addEventListener('keydown', (e) => {
            const urlBar = document.getElementById('url-bar');
            const isUrlBarFocused = document.activeElement === urlBar;

            if (isUrlBarFocused) {
                return;
            }

            if (!window.vimMode) {
                return;
            }

            if (!isUrlBarFocused) {
                if (e.key === 'j') {
                    e.preventDefault();
                    window.moveFocusDown();
                    return;
                } else if (e.key === 'k') {
                    e.preventDefault();
                    window.moveFocusUp();
                    return;
                } else if (e.key === 'Enter') {
                    e.preventDefault();
                    window.activateFocusedTab();
                    return;
                } else if (e.key === 'd') {
                    e.preventDefault();
                    window.closeFocusedTab();
                    return;
                } else if (e.key === 'n') {
                    e.preventDefault();
                    window.ipc.postMessage(JSON.stringify({action: 'new_tab'}));
                    return;
                } else if (e.key === '/') {
                    e.preventDefault();
                    if (urlBar) {
                        urlBar.focus();
                        urlBar.select();
                    }
                    return;
                } else if (e.key === 'G' && e.shiftKey) {
                    e.preventDefault();
                    window.jumpToLastTab();
                    return;
                } else if (e.key === 'g') {
                    const now = Date.now();
                    if (now - window.lastGKeyTime < 500) {
                        e.preventDefault();
                        window.jumpToFirstTab();
                        window.lastGKeyTime = 0;
                    } else {
                        window.lastGKeyTime = now;
                    }
                    return;
                }
            }
        }, false);

        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                const urlBar = document.getElementById('url-bar');
                if (urlBar && document.activeElement === urlBar) {
                    urlBar.blur();
                }
            }
        }, false);

        document.addEventListener('mouseup', (e) => {
            if (e.button === 3) {
                e.preventDefault();
                window.ipc.postMessage(JSON.stringify({action: 'navigate_back'}));
            } else if (e.button === 4) {
                e.preventDefault();
                window.ipc.postMessage(JSON.stringify({action: 'navigate_forward'}));
            }
        }, true);

        window.updateSplitViewButtons = function(enabled) {
            const splitViewBtn = document.getElementById('split-view-btn');
            const splitOrientationBtn = document.getElementById('split-orientation-btn');
            const swapPanesBtn = document.getElementById('swap-panes-btn');

            if (enabled) {
                if (splitViewBtn) splitViewBtn.classList.add('active');
                if (splitOrientationBtn) splitOrientationBtn.style.display = '';
                if (swapPanesBtn) swapPanesBtn.style.display = '';
            } else {
                if (splitViewBtn) splitViewBtn.classList.remove('active');
                if (splitOrientationBtn) splitOrientationBtn.style.display = 'none';
                if (swapPanesBtn) swapPanesBtn.style.display = 'none';
            }
        };

        Object.defineProperty(window, 'ipcMessageToWindow', {
            set: function(value) {
                if (value && value.action && window.ipc) {
                    window.ipc.postMessage(JSON.stringify(value));
                }
            },
            configurable: true
        });
    "#
}
