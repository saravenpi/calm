pub fn get_vim_scroll_script() -> &'static str {
    r#"
        (function() {
            let lastGKeyTime = 0;
            const SCROLL_AMOUNT = 60;
            const HORIZONTAL_SCROLL_AMOUNT = 40;

            document.addEventListener('keydown', (e) => {
                const activeElement = document.activeElement;
                const isInputField = activeElement && (
                    activeElement.tagName === 'INPUT' ||
                    activeElement.tagName === 'TEXTAREA' ||
                    activeElement.isContentEditable
                );

                if (isInputField) {
                    return;
                }

                if (e.key === 'j') {
                    e.preventDefault();
                    window.scrollBy({ top: SCROLL_AMOUNT, behavior: 'smooth' });
                } else if (e.key === 'k') {
                    e.preventDefault();
                    window.scrollBy({ top: -SCROLL_AMOUNT, behavior: 'smooth' });
                } else if (e.key === 'h') {
                    e.preventDefault();
                    window.scrollBy({ left: -HORIZONTAL_SCROLL_AMOUNT, behavior: 'smooth' });
                } else if (e.key === 'l') {
                    e.preventDefault();
                    window.scrollBy({ left: HORIZONTAL_SCROLL_AMOUNT, behavior: 'smooth' });
                } else if (e.key === 'G' && e.shiftKey) {
                    e.preventDefault();
                    window.scrollTo({ top: document.documentElement.scrollHeight, behavior: 'smooth' });
                } else if (e.key === 'g') {
                    const now = Date.now();
                    if (now - lastGKeyTime < 500) {
                        e.preventDefault();
                        window.scrollTo({ top: 0, behavior: 'smooth' });
                        lastGKeyTime = 0;
                    } else {
                        lastGKeyTime = now;
                    }
                }
            }, true);
        })();
    "#
}
