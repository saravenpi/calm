pub fn get_page_transitions() -> &'static str {
    r#"
    (function() {
        'use strict';

        const style = document.createElement('style');
        style.id = 'calm-transitions';
        style.textContent = `
            body {
                animation: fadeIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            }
            @keyframes fadeIn {
                from {
                    opacity: 0;
                    transform: translateY(10px);
                }
                to {
                    opacity: 1;
                    transform: translateY(0);
                }
            }
        `;

        if (document.head) {
            document.head.appendChild(style);
        } else {
            document.addEventListener('DOMContentLoaded', function() {
                document.head.appendChild(style);
            });
        }

        let navigationTimeout = null;
        const originalPushState = history.pushState;
        const originalReplaceState = history.replaceState;

        const showLoaderOnNavigation = () => {
            if (window.__calmShowLoader) {
                window.__calmShowLoader();
            }

            if (navigationTimeout) {
                clearTimeout(navigationTimeout);
            }

            navigationTimeout = setTimeout(() => {
                if (window.__calmHideLoader) {
                    window.__calmHideLoader();
                }
            }, 5000);
        };

        const hideLoaderOnComplete = () => {
            if (navigationTimeout) {
                clearTimeout(navigationTimeout);
            }
            if (window.__calmHideLoader) {
                window.__calmHideLoader();
            }
        };

        history.pushState = function() {
            showLoaderOnNavigation();
            return originalPushState.apply(this, arguments);
        };

        history.replaceState = function() {
            showLoaderOnNavigation();
            return originalReplaceState.apply(this, arguments);
        };

        window.addEventListener('popstate', showLoaderOnNavigation);

        window.addEventListener('load', hideLoaderOnComplete);
        window.addEventListener('DOMContentLoaded', hideLoaderOnComplete);

        document.addEventListener('click', function(e) {
            const link = e.target.closest('a[href]');
            if (link && link.href && !link.target && !link.download) {
                try {
                    const url = new URL(link.href, window.location.href);
                    if (url.origin === window.location.origin) {
                        showLoaderOnNavigation();
                    }
                } catch (e) {
                }
            }
        }, true);

        let mutationTimeout = null;
        const observer = new MutationObserver(() => {
            if (mutationTimeout) clearTimeout(mutationTimeout);
            mutationTimeout = setTimeout(hideLoaderOnComplete, 100);
        });

        const startObserving = () => {
            if (document.body) {
                observer.observe(document.body, {
                    childList: true,
                    subtree: false,
                    attributes: false,
                    characterData: false
                });
            }
        };

        if (document.body) {
            startObserving();
        } else {
            document.addEventListener('DOMContentLoaded', startObserving);
        }
    })();
    "#
}
