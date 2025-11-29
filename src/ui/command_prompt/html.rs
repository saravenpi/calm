pub fn get_command_prompt_html_structure() -> &'static str {
    r#"
    <div class="command-prompt-backdrop" id="command-prompt-backdrop" onclick="hideCommandPrompt()"></div>
    <div class="command-prompt-container" id="command-prompt-container">
        <input
            type="text"
            id="command-prompt-input"
            class="command-prompt-input"
            placeholder="Enter URL or search..."
            autocomplete="off"
            spellcheck="false"
        />
        <div class="command-prompt-hint">Press Enter to navigate • Esc to close</div>
    </div>
    "#
}
