pub fn get_tab_bar_html_structure() -> &'static str {
    r#"
    <div class="control-group">
        <button class="back-btn" id="back-btn" onclick="playUISound('cursorMove'); window.ipc.postMessage(JSON.stringify({action: 'navigate_back'}))" title="Back (Cmd+[)">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges">
                <path d="M20 11v2H8v2H6v-2H4v-2h2V9h2v2h12zM10 7H8v2h2V7zm0 0h2V5h-2v2zm0 10H8v-2h2v2zm0 0h2v2h-2v-2z"/>
            </svg>
        </button>
        <button class="forward-btn" id="forward-btn" onclick="playUISound('cursorMove'); window.ipc.postMessage(JSON.stringify({action: 'navigate_forward'}))" title="Forward (Cmd+])">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges">
                <path d="M4 11v2h12v2h2v-2h2v-2h-2V9h-2v2H4zm10-4h2v2h-2V7zm0 0h-2V5h2v2zm0 10h2v-2h-2v2zm0 0h-2v2h2v-2z"/>
            </svg>
        </button>
        <button class="new-tab-btn" onclick="playUISound('cursorMove'); handleNewTab()" title="New Tab (Cmd+T)">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges">
                <path d="M11 4h2v7h7v2h-7v7h-2v-7H4v-2h7V4z"/>
            </svg>
        </button>
    </div>
    <div id="sidebar-container">
        <div class="split-view-controls">
            <button class="split-orientation-btn" id="split-orientation-btn" onclick="window.ipc.postMessage(JSON.stringify({action: 'toggle_split_orientation'}))" title="Toggle Split Orientation">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges">
                    <path d="M2 5h20v14H2V5zm2 2v4h16V7H4zm16 6H4v4h16v-4z"/>
                </svg>
            </button>
            <button class="swap-panes-btn" id="swap-panes-btn" onclick="window.ipc.postMessage(JSON.stringify({action: 'swap_split_panes'}))" title="Swap Panes">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges">
                    <path d="M15 9V7h2v2h-2zm2 6v-2h-4v-2h4V9h2v2h2v2h-2v2h-2zm0 0v2h-2v-2h2zm-6-4v2H7v2H5v-2H3v-2h2V9h2v2h4zm-4 4h2v2H7v-2zm2-8v2H7V7h2z"/>
                </svg>
            </button>
            <button class="split-view-btn" id="split-view-btn" onclick="if (!this.disabled) { playUISound('cursorMove'); window.ipc.postMessage(JSON.stringify({action: 'toggle_split_view'})); }" title="Toggle Split View (Cmd+Shift+S)">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges">
                    <path d="M2 5h20v14H2V5zm2 2v10h7V7H4zm9 0v10h7V7h-7z"/>
                </svg>
            </button>
        </div>
        <div class="url-bar-container">
            <input type="text" class="url-bar" id="url-bar" placeholder="search or enter address" />
            <button class="reload-btn" id="reload-btn" onclick="playUISound('cursorMove'); window.ipc.postMessage(JSON.stringify({action: 'reload_tab'}))" title="Reload (Cmd+R)">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges">
                    <path d="M16 2h-2v2h2v2H4v2H2v5h2V8h12v2h-2v2h2v-2h2V8h2V6h-2V4h-2V2zM6 20h2v2h2v-2H8v-2h12v-2h2v-5h-2v5H8v-2h2v-2H8v2H6v2H4v2h2v2z"/>
                </svg>
            </button>
        </div>
        <div id="tab-bar"></div>
        <div class="bottom-controls">
            <button class="downloads-btn" id="downloads-btn" onclick="playUISound('cursorMove'); toggleDownloads()" title="Downloads">
                <div class="download-btn-content" id="download-btn-content">
                    <div class="download-icon-wrapper">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges">
                            <path d="M13 2h-2v9H9V9H7v2h2v2h2v2h2v-2h2v-2h2V9h-2v2h-2V2zm8 17v-2h-2v2H5v-2H3v4h18v-2z"/>
                        </svg>
                    </div>
                    <span class="download-badge" id="download-badge">0</span>
                </div>
                <div class="download-btn-progress" id="download-btn-progress" style="display: none;">
                    <div class="download-btn-progress-fill" id="download-btn-progress-fill"></div>
                </div>
            </button>
            <button class="settings-btn" id="settings-btn" onclick="playUISound('cursorMove'); openSettings()" title="Settings">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges">
                    <path d="M17 4h2v10h-2V4zm0 12h-2v2h2v2h2v-2h2v-2h-4zm-4-6h-2v10h2V10zm-8 2H3v2h2v6h2v-6h2v-2H5zm8-8h-2v2H9v2h6V6h-2V4zM5 4h2v6H5V4z"/>
                </svg>
            </button>
        </div>
    </div>
    "#
}
