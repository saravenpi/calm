pub fn get_command_prompt_html_structure() -> &'static str {
    r#"
    <div class="command-prompt-backdrop" id="command-prompt-backdrop" onclick="hideCommandPrompt()"></div>
    <div class="command-prompt-container" id="command-prompt-container">
        <div class="input-wrapper">
            <svg class="search-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M6 2h8v2H6V2zM4 6V4h2v2H4zm0 8H2V6h2v8zm2 2H4v-2h2v2zm8 0v2H6v-2h8zm2-2h-2v2h2v2h2v2h2v2h2v-2h-2v-2h-2v-2h-2v-2zm0-8h2v8h-2V6zm0 0V4h-2v2h2z" fill="currentColor"/>
            </svg>
            <input
                type="text"
                id="command-prompt-input"
                class="command-prompt-input"
                placeholder="Enter URL or search..."
                autocomplete="off"
                spellcheck="false"
            />
        </div>
        <div class="command-prompt-suggestions" id="command-prompt-suggestions"></div>
        <div class="command-prompt-hint">Press Enter to navigate • Esc to close</div>
    </div>
    "#
}
