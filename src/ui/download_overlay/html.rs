pub fn get_html() -> &'static str {
    r#"
        <div class="downloads-panel" id="downloads-panel">
            <div class="downloads-content">
                <div class="downloads-header">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges" style="display: inline-block; vertical-align: middle; margin-right: 8px;">
                        <path d="M13 17V3h-2v10H9v-2H7v2h2v2h2v2h2zm8 2v-4h-2v4H5v-4H3v6h18v-2zm-8-6v2h2v-2h2v-2h-2v2h-2z"/>
                    </svg>
                    Downloads
                </div>
                <div class="downloads-empty" id="downloads-empty">No active downloads</div>
                <div class="downloads-list" id="downloads-list"></div>
            </div>
            <div class="downloads-footer">
                <button class="clear-history-btn" onclick="clearDownloadHistory()" title="Clear Download History">
                    <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor" shape-rendering="crispEdges" style="display: inline-block; vertical-align: middle; margin-right: 6px;">
                        <path d="M16 2v4h6v2h-2v14H4V8H2V6h6V2h8zm-2 2h-4v2h4V4zm0 4H6v12h12V8h-4zm-5 2h2v8H9v-8zm6 0h-2v8h2v-8z"/>
                    </svg>
                    Clear History
                </button>
            </div>
        </div>
    "#
}
