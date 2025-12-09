pub fn get_command_prompt_script() -> &'static str {
    r#"
        window.historyEntries = [];
        window.selectedIndex = -1;

        window.setHistory = function(history) {
            window.historyEntries = history;
            updateSuggestions();
        };

        window.hideCommandPrompt = function() {
            const input = document.getElementById('command-prompt-input');
            if (input) {
                input.value = '';
            }
            window.ipc.postMessage(JSON.stringify({action: 'hide_command_prompt'}));
        };

        let searchDebounce = null;

        window.showHistorySuggestions = function(results) {
            renderSuggestions(results);
        };

        function updateSuggestions() {
            const input = document.getElementById('command-prompt-input');
            const query = input.value.trim();

            if (!query) {
                const recent = window.historyEntries.slice(0, 8);
                renderSuggestions(recent);
                return;
            }

            // Local filter for immediate feedback
            const localMatches = window.historyEntries.filter(entry => {
                const q = query.toLowerCase();
                return entry.url.toLowerCase().includes(q) ||
                       entry.title.toLowerCase().includes(q);
            }).slice(0, 5);
            renderSuggestions(localMatches);

            // IPC search for full history
            if (searchDebounce) clearTimeout(searchDebounce);
            searchDebounce = setTimeout(() => {
                window.ipc.postMessage(JSON.stringify({
                    action: 'search_history',
                    query: query
                }));
            }, 150);
        }

        const GLOBE_ICON = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M2 12h20"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>`;

        function renderSuggestions(suggestions) {
            const suggestionsDiv = document.getElementById('command-prompt-suggestions');
            suggestionsDiv.innerHTML = '';
            window.selectedIndex = -1;

            if (suggestions.length === 0) {
                return;
            }

            suggestions.forEach((entry, index) => {
                const div = document.createElement('div');
                div.className = 'command-prompt-suggestion';
                
                // Use generic globe icon since we prioritize privacy and don't fetch 3rd party likely
                const iconHtml = GLOBE_ICON;

                div.innerHTML = `
                    <div class="suggestion-icon">${iconHtml}</div>
                    <div class="suggestion-info">
                        <div class="suggestion-title">${escapeHtml(entry.title || entry.url)}</div>
                    </div>
                    <div class="suggestion-url">${escapeHtml(entry.url)}</div>
                `;
                div.onclick = () => {
                    window.ipc.postMessage(JSON.stringify({
                        action: 'command_prompt_navigate',
                        url: entry.url
                    }));
                };
                suggestionsDiv.appendChild(div);
            });
        }

        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }

        function selectSuggestion(index) {
            const suggestions = document.querySelectorAll('.command-prompt-suggestion');
            suggestions.forEach((s, i) => {
                s.classList.toggle('selected', i === index);
            });
            window.selectedIndex = index;
        }

        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                e.preventDefault();
                e.stopPropagation();
                window.hideCommandPrompt();
            }
        });

        const input = document.getElementById('command-prompt-input');
        if (input) {
            setTimeout(() => {
                input.focus();
                input.select();
                updateSuggestions();
            }, 100);

            input.addEventListener('input', () => {
                updateSuggestions();
            });

            input.addEventListener('keydown', (e) => {
                const suggestions = document.querySelectorAll('.command-prompt-suggestion');

                if (e.key === 'ArrowDown') {
                    e.preventDefault();
                    if (window.selectedIndex < suggestions.length - 1) {
                        selectSuggestion(window.selectedIndex + 1);
                    }
                } else if (e.key === 'ArrowUp') {
                    e.preventDefault();
                    if (window.selectedIndex > 0) {
                        selectSuggestion(window.selectedIndex - 1);
                    } else {
                        selectSuggestion(-1);
                    }
                } else if (e.key === 'Enter') {
                    e.preventDefault();
                    if (window.selectedIndex >= 0 && suggestions[window.selectedIndex]) {
                        suggestions[window.selectedIndex].click();
                    } else {
                        const url = input.value.trim();
                        if (url) {
                            window.ipc.postMessage(JSON.stringify({
                                action: 'command_prompt_navigate',
                                url: url
                            }));
                        }
                    }
                }
            });
        }
    "#
}
