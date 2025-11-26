pub fn get_vim_scroll_script() -> &'static str {
    r#"
(function() {
    'use strict';

    let lastGKeyTime = 0;
    const SCROLL_AMOUNT = 60;
    const HORIZONTAL_SCROLL_AMOUNT = 40;

    function isInputElement(element) {
        if (!element) return false;
        const tagName = element.tagName;
        return tagName === 'INPUT' ||
               tagName === 'TEXTAREA' ||
               tagName === 'SELECT' ||
               element.isContentEditable === true ||
               element.contentEditable === 'true';
    }

    function handleKeyDown(e) {
        if (isInputElement(document.activeElement)) {
            return;
        }

        let handled = false;

        switch(e.key) {
            case 'j':
                window.scrollBy({ top: SCROLL_AMOUNT, behavior: 'smooth' });
                handled = true;
                break;
            case 'k':
                window.scrollBy({ top: -SCROLL_AMOUNT, behavior: 'smooth' });
                handled = true;
                break;
            case 'h':
                window.scrollBy({ left: -HORIZONTAL_SCROLL_AMOUNT, behavior: 'smooth' });
                handled = true;
                break;
            case 'l':
                window.scrollBy({ left: HORIZONTAL_SCROLL_AMOUNT, behavior: 'smooth' });
                handled = true;
                break;
            case 'G':
                if (e.shiftKey) {
                    window.scrollTo({ top: document.documentElement.scrollHeight, behavior: 'smooth' });
                    handled = true;
                }
                break;
            case 'g':
                const now = Date.now();
                if (now - lastGKeyTime < 500) {
                    window.scrollTo({ top: 0, behavior: 'smooth' });
                    lastGKeyTime = 0;
                    handled = true;
                } else {
                    lastGKeyTime = now;
                }
                break;
        }

        if (handled) {
            e.preventDefault();
            e.stopPropagation();
        }
    }

    window.addEventListener('keydown', handleKeyDown, true);

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', function() {
            window.addEventListener('keydown', handleKeyDown, true);
        });
    }
})();
    "#
}
