pub fn get_command_prompt_script() -> &'static str {
    r#"
        window.hideCommandPrompt = function() {
            const input = document.getElementById('command-prompt-input');
            if (input) {
                input.value = '';
            }
            window.ipc.postMessage(JSON.stringify({action: 'hide_command_prompt'}));
        };

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
            }, 100);

            input.addEventListener('keydown', (e) => {
                if (e.key === 'Enter') {
                    e.preventDefault();
                    const url = input.value.trim();
                    if (url) {
                        window.ipc.postMessage(JSON.stringify({
                            action: 'command_prompt_navigate',
                            url: url
                        }));
                    }
                }
            });
        }
    "#
}
