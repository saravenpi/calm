pub fn get_tab_bar_html_structure() -> &'static str {
    r#"
    <div class="control-group">
        <button class="back-btn" id="back-btn" onclick="window.ipc.postMessage(JSON.stringify({action: 'navigate_back'}))" title="Back" disabled>
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" shape-rendering="crispEdges">
                <rect x="8" y="4" width="2" height="2"/>
                <rect x="6" y="6" width="2" height="2"/>
                <rect x="4" y="8" width="2" height="2"/>
                <rect x="6" y="10" width="2" height="2"/>
                <rect x="8" y="12" width="2" height="2"/>
                <rect x="8" y="6" width="6" height="2"/>
                <rect x="8" y="10" width="6" height="2"/>
            </svg>
        </button>
        <button class="forward-btn" id="forward-btn" onclick="window.ipc.postMessage(JSON.stringify({action: 'navigate_forward'}))" title="Forward" disabled>
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" shape-rendering="crispEdges">
                <rect x="6" y="4" width="2" height="2"/>
                <rect x="8" y="6" width="2" height="2"/>
                <rect x="10" y="8" width="2" height="2"/>
                <rect x="8" y="10" width="2" height="2"/>
                <rect x="6" y="12" width="2" height="2"/>
                <rect x="2" y="6" width="6" height="2"/>
                <rect x="2" y="10" width="6" height="2"/>
            </svg>
        </button>
        <button class="reload-btn" id="reload-btn" onclick="window.ipc.postMessage(JSON.stringify({action: 'reload_tab'}))" title="Reload (Cmd+R)">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" shape-rendering="crispEdges">
                <rect x="4" y="2" width="6" height="2"/>
                <rect x="10" y="4" width="2" height="2"/>
                <rect x="12" y="6" width="2" height="2"/>
                <rect x="12" y="8" width="2" height="2"/>
                <rect x="10" y="10" width="2" height="2"/>
                <rect x="4" y="12" width="6" height="2"/>
                <rect x="2" y="10" width="2" height="2"/>
                <rect x="2" y="6" width="2" height="2"/>
                <rect x="2" y="4" width="2" height="2"/>
                <rect x="10" y="2" width="2" height="2"/>
                <rect x="12" y="10" width="2" height="2"/>
            </svg>
        </button>
        <button class="new-tab-btn" onclick="handleNewTab()" title="New Tab (Cmd+T)">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" shape-rendering="crispEdges">
                <rect x="7" y="2" width="2" height="12"/>
                <rect x="2" y="7" width="12" height="2"/>
            </svg>
        </button>
    </div>
    <div id="tab-bar">
        <input type="text" class="url-bar" id="url-bar" placeholder="search or enter address" />
    </div>
    <div class="bottom-controls">
        <button class="downloads-btn" id="downloads-btn" onclick="toggleDownloads()" title="Downloads">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" shape-rendering="crispEdges">
                <rect x="7" y="2" width="2" height="8"/>
                <rect x="5" y="8" width="2" height="2"/>
                <rect x="9" y="8" width="2" height="2"/>
                <rect x="3" y="10" width="2" height="2"/>
                <rect x="11" y="10" width="2" height="2"/>
                <rect x="2" y="12" width="12" height="2"/>
            </svg>
            <span class="download-badge" id="download-badge" style="display: none;">0</span>
        </button>
        <button class="settings-btn" id="settings-btn" onclick="openSettings()" title="Settings">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" shape-rendering="crispEdges">
                <rect x="7" y="0" width="2" height="2"/>
                <rect x="7" y="14" width="2" height="2"/>
                <rect x="0" y="7" width="2" height="2"/>
                <rect x="14" y="7" width="2" height="2"/>
                <rect x="3" y="3" width="2" height="2"/>
                <rect x="11" y="3" width="2" height="2"/>
                <rect x="3" y="11" width="2" height="2"/>
                <rect x="11" y="11" width="2" height="2"/>
                <rect x="6" y="6" width="4" height="4"/>
            </svg>
        </button>
    </div>
    "#
}
