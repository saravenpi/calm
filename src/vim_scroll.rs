pub fn get_vim_scroll_script() -> &'static str {
    r#"
(function() {
    'use strict';

    console.log('[CALM VIM] Script loaded and executing');

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
        console.log('[CALM VIM] Key pressed:', e.key, 'Active element:', document.activeElement.tagName);

        if (isInputElement(document.activeElement)) {
            console.log('[CALM VIM] Ignoring - input element focused');
            return;
        }

        let handled = false;

        switch(e.key) {
            case 'j':
                console.log('[CALM VIM] Scrolling down');
                window.scrollBy({ top: SCROLL_AMOUNT, behavior: 'smooth' });
                handled = true;
                break;
            case 'k':
                console.log('[CALM VIM] Scrolling up');
                window.scrollBy({ top: -SCROLL_AMOUNT, behavior: 'smooth' });
                handled = true;
                break;
            case 'h':
                console.log('[CALM VIM] Scrolling left');
                window.scrollBy({ left: -HORIZONTAL_SCROLL_AMOUNT, behavior: 'smooth' });
                handled = true;
                break;
            case 'l':
                console.log('[CALM VIM] Scrolling right');
                window.scrollBy({ left: HORIZONTAL_SCROLL_AMOUNT, behavior: 'smooth' });
                handled = true;
                break;
            case 'G':
                if (e.shiftKey) {
                    console.log('[CALM VIM] Jumping to bottom');
                    window.scrollTo({ top: document.documentElement.scrollHeight, behavior: 'smooth' });
                    handled = true;
                }
                break;
            case 'g':
                const now = Date.now();
                if (now - lastGKeyTime < 500) {
                    console.log('[CALM VIM] Jumping to top (gg)');
                    window.scrollTo({ top: 0, behavior: 'smooth' });
                    lastGKeyTime = 0;
                    handled = true;
                } else {
                    console.log('[CALM VIM] First g pressed');
                    lastGKeyTime = now;
                }
                break;
        }

        if (handled) {
            console.log('[CALM VIM] Event prevented and stopped');
            e.preventDefault();
            e.stopPropagation();
        }
    }

    console.log('[CALM VIM] Registering keyboard listener');
    document.addEventListener('keydown', handleKeyDown, true);

    console.log('[CALM VIM] Setup complete');
})();
    "#
}
