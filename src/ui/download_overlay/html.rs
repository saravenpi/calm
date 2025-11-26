pub fn get_html() -> &'static str {
    r#"
        <div class="downloads-panel" id="downloads-panel">
            <div class="downloads-content">
                <div class="downloads-header">Downloads</div>
                <div class="downloads-empty" id="downloads-empty">No active downloads</div>
                <div class="downloads-list" id="downloads-list"></div>
            </div>
        </div>
    "#
}
